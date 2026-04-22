use std::process::Command;

pub struct DockerCli;

impl DockerCli {
    pub fn new() -> Self {
        Self
    }

    pub fn build_args(&self, subcommand: &str, extra: &[&str]) -> Vec<String> {
        let mut args = vec![subcommand.to_string()];
        args.extend(extra.iter().map(|s| s.to_string()));
        args
    }

    pub fn run(&self, args: &[String]) -> Result<String, String> {
        let output = Command::new("docker")
            .args(args)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).into_owned())
        }
    }

    pub fn run_allow_failure(&self, args: &[String]) -> Result<String, String> {
        let output = Command::new("docker")
            .args(args)
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_args_subcommand_first() {
        let cli = DockerCli::new();
        let args = cli.build_args("ps", &[]);
        assert_eq!(args, vec!["ps"]);
    }

    #[test]
    fn build_args_appends_extra() {
        let cli = DockerCli::new();
        let args = cli.build_args("ps", &["-a", "--no-trunc"]);
        assert_eq!(args, vec!["ps", "-a", "--no-trunc"]);
    }

    #[test]
    fn build_args_empty_extra_no_trailing() {
        let cli = DockerCli::new();
        let args = cli.build_args("images", &[]);
        assert_eq!(args.len(), 1);
        assert_eq!(args[0], "images");
    }
}
