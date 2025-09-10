use crate::agent::LaunchAgent;
use crate::os::{get_user_id, run_shell};
use crate::LaunchctlResult;

/// Trait for controlling launch agents via launchctl.
pub trait LaunchControllable {
    /// Bootstrap the launch agent.
    fn bootstrap(&self) -> LaunchctlResult<()>;

    /// Boot out the launch agent.
    fn boot_out(&self) -> LaunchctlResult<()>;

    /// Check if the launch agent is running.
    fn is_running(&self) -> LaunchctlResult<bool>;
}

impl LaunchAgent {
    /// Format a launchctl command.
    /// If the command is empty, it will return an empty string.
    fn format_command(&self, command: &str) -> String {
        if command.is_empty() {
            return String::new();
        }
        format!(
            "launchctl {} gui/{} '{}'",
            command,
            get_user_id(),
            self.path().display()
        )
    }

    fn format_bootstrap_command(&self) -> String {
        self.format_command("bootstrap")
    }

    fn format_boot_out_command(&self) -> String {
        self.format_command("bootout")
    }

    fn format_print_command(&self) -> String {
        format!("launchctl print gui/{}/{}", get_user_id(), self.label)
    }

    /// Check if the output contains agent is running indicator.
    fn check_is_running(output: &str) -> bool {
        output.contains("state = running")
    }
}

impl LaunchControllable for LaunchAgent {
    /// Bootstrap the launch agent.
    fn bootstrap(&self) -> LaunchctlResult<()> {
        let cmd = self.format_bootstrap_command();
        run_shell(&cmd).map(|_| ())
    }

    /// Boot out the launch agent.
    /// It means not only stop, but also deactivate the launch agent.
    fn boot_out(&self) -> LaunchctlResult<()> {
        let cmd = self.format_boot_out_command();
        run_shell(&cmd).map(|_| ())
    }

    /// Check if the launch agent is running.
    fn is_running(&self) -> LaunchctlResult<bool> {
        let cmd = self.format_print_command();

        let output = run_shell(&cmd)?;
        Ok(LaunchAgent::check_is_running(&output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_command() {
        let agent = LaunchAgent::new("test");
        let agent_path = agent.path().display().to_string();
        let user_id = get_user_id();

        assert_eq!(
            agent.format_command("subcommand"),
            format!("launchctl subcommand gui/{user_id} '{agent_path}'")
        );
        assert_eq!(
            agent.format_command("manageruid"),
            format!("launchctl manageruid gui/{user_id} '{agent_path}'")
        );
        assert_eq!(agent.format_command(""), "");
    }

    #[test]
    fn test_format_bootstrap_command() {
        let agent = LaunchAgent::new("test");
        let user_id = get_user_id();
        let agent_path = agent.path().display().to_string();

        assert_eq!(
            agent.format_bootstrap_command(),
            format!("launchctl bootstrap gui/{user_id} '{agent_path}'")
        );
    }

    #[test]
    fn test_format_bootout_command() {
        let agent = LaunchAgent::new("test");
        let user_id = get_user_id();
        let agent_path = agent.path().display().to_string();

        assert_eq!(
            agent.format_boot_out_command(),
            format!("launchctl bootout gui/{user_id} '{agent_path}'")
        );
    }

    #[test]
    fn test_check_info_command() {
        let agent = LaunchAgent::new("test");
        let user_id = get_user_id();

        assert_eq!(
            agent.format_print_command(),
            format!("launchctl print gui/{user_id}/test")
        );
    }

    #[test]
    fn test_check_is_running() {
        let output = "
{
        domain = gui/501 [100003]
        asid = 100003

        jetsam memory limit (active) = (unlimited)
        jetsam memory limit (inactive) = (unlimited)
        jetsamproperties category = daemon
        jetsam thread limit = 32
        cpumon = default
        job state = running
        probabilistic guard malloc policy = {
                activation rate = 1/1000
                sample rate = 1/0
        }

        properties = keepalive | runatload | inferred program | managed LWCR | has LWCR
}
        ";
        assert!(LaunchAgent::check_is_running(output));

        let output = "
        {
            domain = gui/501 [100003]
            asid = 100003
        }
        ";
        assert!(!LaunchAgent::check_is_running(output));
    }
}
