//! Docker CLI wrapper for running `docker` commands.

use crate::utils::json::{parse_json, parse_ndjson};
use serde::de::DeserializeOwned;
use std::process::{Command, Output};

/// Docker CLI entrypoint.
pub struct Docker {
    binary: String,
    args: Vec<String>,
}

impl Docker {
    /// Creates a new Docker command builder with the specified subcommand.
    ///
    /// Example: `Docker::cmd("version")` creates a command for `docker version`.
    pub fn cmd(subcommand: &str) -> Self {
        Self {
            binary: "docker".to_string(),
            args: vec![subcommand.to_string()],
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

    /// Shortcut for appending `--format json` to the command.
    ///
    /// Use this before calling `.run_json()` or `.run_ndjson()` functions.
    pub fn json(self) -> Self {
        self.args(["--format", "json"])
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
        Command::new(&self.binary)
            .args(&self.args)
            .output()
            .map_err(|e| e.to_string())
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
}
