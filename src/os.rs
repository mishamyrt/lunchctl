use std::process::Command;

use crate::{LaunchAgentError, LaunchctlResult};

/// Run a shell command.
pub(crate) fn run_shell(command: &str) -> LaunchctlResult<String> {
    let output =
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| {
                LaunchAgentError::CommandFailed(
                    e.raw_os_error().unwrap_or(1),
                    e.to_string(),
                )
            })?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Get the user ID.
pub(crate) fn get_user_id() -> u32 {
    unsafe { libc::geteuid() }
}
