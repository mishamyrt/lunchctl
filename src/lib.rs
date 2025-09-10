use thiserror::Error;

mod control;
mod agent;
mod os;

pub use control::LaunchControllable;
pub use agent::LaunchAgent;

/// Error types for Launch Agent configuration.
#[derive(Error, Debug)]
pub enum LaunchAgentError {
    #[error("Failed to process plist")]
    PListError(#[from] plist::Error),

    #[error("Failed to write plist")]
    WriteError(#[from] std::io::Error),

    #[error("Failed to run launchctl command. Exit code: {0}, Output: {1}")]
    CommandFailed(i32, String),
}

/// Result type for launchctl operations.
pub type LaunchctlResult<T> = Result<T, LaunchAgentError>;
