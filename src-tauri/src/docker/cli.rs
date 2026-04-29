//! Docker CLI wrapper for running `docker` commands.

use crate::utils::json::{parse_json, parse_ndjson};
use serde::de::DeserializeOwned;
use std::process::{Command, Output};

/// Docker CLI entrypoint.
pub struct Docker {
    binary: String,
    args: Vec<String>,
    errors: Vec<String>,
    log_level: Option<log::Level>,
    log_prefix: Option<String>,
}

impl Docker {
    /// Creates a new Docker command builder with the specified command.
    ///
    /// Example: `Docker::cmd("version")` creates a command for `docker version`.
    pub fn cmd(command: &str) -> Self {
        Self {
            binary: "docker".to_string(),
            args: vec![command.to_string()],
            errors: vec![],
            log_level: None,
            log_prefix: None,
        }
    }

    /// Appends a single argument to the command.
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Appends multiple arguments to the command.
    pub fn args<TArgs, TItem>(mut self, args: TArgs) -> Self
    where
        TArgs: IntoIterator<Item = TItem>,
        TItem: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn filter(mut self, key: &str, value: &str) -> Self {
        let is_dangerous = value.chars().any(|c| matches!(c, '\0' | '\n' | '\r'));
        if is_dangerous {
            self.errors
                .push(format!("Invalid filter: {}={}", key, value));
            return self;
        }

        self.arg("--filter").arg(format!("{}={}", key, value))
    }

    /// Shortcut for appending `--format json` to the command.
    ///
    /// Use this before calling `.run_json()` or `.run_ndjson()` functions.
    pub fn json(self) -> Self {
        self.args(["--format", "json"])
    }

    /// Marks the command to be logged with the specified level.
    pub fn log(mut self, level: log::Level) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Marks the command to be logged with the specified level and prefix.
    pub fn log_prefix(mut self, level: log::Level, prefix: impl Into<String>) -> Self {
        self.log_prefix = Some(prefix.into());
        self.log(level)
    }

    /// Runs the command. Non-zero exit returns stderr as `Err`.
    pub fn run(self) -> Result<String, String> {
        let output = self.exec()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).into_owned())
        }
    }

    /// Runs the command, treating a non-zero exit with useful stdout as success.
    ///
    /// Use when the command may write partial output on failure — e.g. `docker version`
    /// prints client info on stdout even when the daemon is unreachable and it exits non-zero.
    ///
    /// Outcomes:
    /// - `Err(stderr_or_io)` — spawn/IO failure (docker binary missing, permission denied).
    /// - `Ok(Some(stdout))` — zero exit OR non-zero exit but stdout had content.
    /// - `Ok(None)` — non-zero exit with empty stdout (nothing useful to return).
    ///
    /// Stderr is discarded in the non-zero + non-empty stdout case (= safe success).
    pub fn run_safe(self) -> Result<Option<String>, String> {
        let output = self.exec()?;
        if output.status.success() || !output.stdout.is_empty() {
            Ok(Some(String::from_utf8_lossy(&output.stdout).into_owned()))
        } else {
            Ok(None)
        }
    }

    /// [`Docker::run`] + parses stdout as a single JSON value.
    pub fn run_json<T: DeserializeOwned>(self) -> Result<T, String> {
        parse_json(&self.run()?)
    }

    /// [`Docker::run_safe`] + parses stdout as a single JSON value.
    pub fn run_json_safe<T: DeserializeOwned>(self) -> Result<Option<T>, String> {
        self.run_safe()?.map(|s| parse_json(&s)).transpose()
    }

    /// [`Docker::run`] + parses stdout as NDJSON (one value per line).
    pub fn run_ndjson<T: DeserializeOwned>(self) -> Result<Vec<T>, String> {
        parse_ndjson(&self.run()?)
    }

    /// [`Docker::run_safe`] + parses stdout as NDJSON (one value per line).
    pub fn run_ndjson_safe<T: DeserializeOwned>(self) -> Result<Option<Vec<T>>, String> {
        self.run_safe()?.map(|s| parse_ndjson(&s)).transpose()
    }

    fn exec(&self) -> Result<Output, String> {
        let mut cmd = Command::new(&self.binary);
        cmd.args(&self.args);
        if let Some(level) = self.log_level {
            match &self.log_prefix {
                Some(p) => log::log!(level, "{} — {:?}", p, cmd),
                None => log::log!(level, "{:?}", cmd),
            }
        }

        if !self.errors.is_empty() {
            return Err(self.errors.join(", "));
        }

        cmd.output().map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_sets_subcommand() {
        let d = Docker::cmd("ps");
        assert_eq!(d.args, vec!["ps"]);
    }

    #[test]
    fn arg_appends_single() {
        let d = Docker::cmd("ps").arg("-a");
        assert_eq!(d.args, vec!["ps", "-a"]);
    }

    #[test]
    fn args_appends_many() {
        let d = Docker::cmd("version").args(["--format", "json"]);
        assert_eq!(d.args, vec!["version", "--format", "json"]);
    }

    #[test]
    fn json_appends_format_flag() {
        let d = Docker::cmd("version").json();
        assert_eq!(d.args, vec!["version", "--format", "json"]);
    }

    // TODO:
    // - Errors
    // - Run
}
