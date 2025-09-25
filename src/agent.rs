use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use derive_builder::Builder;

use serde::{Deserialize, Serialize};

use crate::LaunchAgentError;

/// The path to the null device.
pub(crate) const DEV_NULL: &str = "/dev/null";

/// Launch Agent configuration.
///
/// A Launch Agent is a macOS mechanism for automatically starting user-level processes
/// at login or in response to specific events. Launch Agents are described using plist files,
/// which are placed in the `~/Library/LaunchAgents` directory.
///
/// Each agent can be configured to start automatically, restart on failure, redirect output,
/// and pass arguments to the launched process. Agents are managed using the `launchctl` utility.
///
/// More information:
/// [`https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html`](Apple Developer Documentation)
#[derive(Deserialize, Clone, Serialize, Builder)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchAgent {
    #[builder(setter(into))]
    pub label: String,

    #[builder(default, setter(each = "arg"))]
    pub program_arguments: Vec<String>,

    #[builder(default = "PathBuf::from(DEV_NULL)", setter(into))]
    pub standard_out_path: PathBuf,

    #[builder(default = "PathBuf::from(DEV_NULL)", setter(into))]
    pub standard_error_path: PathBuf,

    #[builder(default)]
    pub keep_alive: bool,

    #[builder(default)]
    pub run_at_load: bool,

    #[builder(default)]
    pub process_type: ProcessType,
}

#[derive(Clone)]
pub enum ProcessType {
    /// Background jobs are generally processes that do work that was not
    /// directly requested by the user. The resource limits applied to
    /// Background jobs are intended to prevent them from disrupting the
    /// user experience.
    Background,
    /// Standard jobs are equivalent to no `ProcessType` being set.
    Standard,
    /// Adaptive jobs move between the Background and Interactive classifications
    /// based on activity over XPC connections.
    Adaptive,
    /// Interactive jobs run with the same resource limitations as apps,
    /// that is to say, none. Interactive jobs are critical to maintaining
    /// a responsive user experience, and this key should only be used if
    /// an app's ability to be responsive depends on it, and cannot be made
    /// Adaptive.
    Interactive,
}

impl Default for ProcessType {
    fn default() -> Self {
        Self::Standard
    }
}

impl Serialize for ProcessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Background => "background",
            Self::Standard => "standard",
            Self::Adaptive => "adaptive",
            Self::Interactive => "interactive",
        })
    }
}

impl<'de> Deserialize<'de> for ProcessType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "background" => Self::Background,
            "standard" => Self::Standard,
            "adaptive" => Self::Adaptive,
            "interactive" => Self::Interactive,
            _ => return Err(serde::de::Error::custom("invalid process type")),
        })
    }
}

impl LaunchAgent {
    /// Create a new Launch Agent configuration.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            program_arguments: vec![],
            standard_out_path: PathBuf::from(DEV_NULL),
            standard_error_path: PathBuf::from(DEV_NULL),
            keep_alive: false,
            run_at_load: false,
            process_type: ProcessType::default(),
        }
    }

    /// Check if a Launch Agent configuration exists.
    pub fn exists(label: &str) -> bool {
        let path = Self::path_for(label);
        path.exists()
    }

    /// Loads a Launch Agent configuration from `~/Library/LaunchAgents` by agent label.
    pub fn from_file(label: &str) -> Result<Self, LaunchAgentError> {
        let path = Self::path_for(label);

        let agent = plist::from_file(path)?;

        Ok(agent)
    }

    /// Returns the path to the Launch Agent configuration file for the given label.
    fn path_for(label: &str) -> PathBuf {
        let home = std::env::var("HOME").unwrap();
        let file_name = format!("{label}.plist");
        PathBuf::from(home)
            .join("Library")
            .join("LaunchAgents")
            .join(file_name)
    }
}

impl LaunchAgent {
    /// Writes the Launch Agent configuration to the current user's `LaunchAgents` directory.
    pub fn write(&self) -> Result<(), LaunchAgentError> {
        let path = Self::path_for(&self.label);
        let mut file = File::create(path)?;
        self.to_writer(&mut file)?;
        Ok(())
    }

    /// Removes the Launch Agent configuration from the current user's `LaunchAgents` directory.
    pub fn remove(&self) -> Result<(), LaunchAgentError> {
        let path = Self::path_for(&self.label);
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Returns the path to the Launch Agent configuration file for the given label.
    pub fn path(&self) -> PathBuf {
        Self::path_for(&self.label)
    }

    /// Writes the Launch Agent configuration to provided writer.
    fn to_writer<W: Write>(&self, writer: W) -> Result<(), LaunchAgentError> {
        plist::to_writer_xml(writer, self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_format_plist() {
        let agent = LaunchAgent {
            label: "co.myrt.ajam".to_string(),
            program_arguments: vec!["ajam".to_string(), "run".to_string()],
            standard_out_path: PathBuf::from(DEV_NULL),
            standard_error_path: PathBuf::from(DEV_NULL),
            keep_alive: false,
            run_at_load: false,
            process_type: ProcessType::default(),
        };

        let mut buf = BufWriter::new(Vec::new());

        agent.to_writer(&mut buf).unwrap();

        let plist = String::from_utf8(buf.into_inner().unwrap()).unwrap();

        assert!(plist.contains("</dict>"));
        assert!(plist.contains("<key>Label</key>"));
        assert!(plist.contains("<key>ProgramArguments</key>"));
        assert!(plist.contains("<key>StandardOutPath</key>"));
        assert!(plist.contains("<key>StandardErrorPath</key>"));
        assert!(plist.contains("<key>KeepAlive</key>"));
        assert!(plist.contains("<key>RunAtLoad</key>"));

        assert!(plist.contains("co.myrt.ajam"));
    }

    #[test]
    fn test_path() {
        let agent = LaunchAgent {
            label: "co.myrt.ajam".to_string(),
            program_arguments: vec![],
            standard_out_path: PathBuf::from(DEV_NULL),
            standard_error_path: PathBuf::from(DEV_NULL),
            keep_alive: false,
            run_at_load: false,
            process_type: ProcessType::default(),
        };
        let path = PathBuf::from("Library/LaunchAgents/co.myrt.ajam.plist");
        let abs_path = PathBuf::from(std::env::var("HOME").unwrap()).join(path);
        assert_eq!(agent.path(), abs_path);
    }

    #[test]
    fn test_write() {
        let label = format!("co.myrt.ajam.test.{}", rand::random_range(0.0..=1e9));

        let agent = LaunchAgent {
            label,
            program_arguments: vec![],
            standard_out_path: PathBuf::from(DEV_NULL),
            standard_error_path: PathBuf::from(DEV_NULL),
            keep_alive: false,
            run_at_load: false,
            process_type: ProcessType::default(),
        };
        let path = agent.path();

        agent.write().unwrap();
        assert!(path.exists());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_remove() {
        let label = format!("co.myrt.ajam.test.{}", rand::random_range(0.0..=1e9));
        let agent = LaunchAgent {
            label,
            program_arguments: vec![],
            standard_out_path: PathBuf::from(DEV_NULL),
            standard_error_path: PathBuf::from(DEV_NULL),
            keep_alive: false,
            run_at_load: false,
            process_type: ProcessType::default(),
        };
        let path = agent.path();

        agent.write().unwrap();
        assert!(path.exists());

        agent.remove().unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn test_exists() {
        let label = format!("co.myrt.ajam.test.{}", rand::random_range(0.0..=1e9));
        let agent = LaunchAgent {
            label: label.clone(),
            program_arguments: vec![],
            standard_out_path: PathBuf::from(DEV_NULL),
            standard_error_path: PathBuf::from(DEV_NULL),
            keep_alive: false,
            run_at_load: false,
            process_type: ProcessType::default(),
        };

        assert!(!LaunchAgent::exists(&label));

        agent.write().unwrap();
        assert!(LaunchAgent::exists(&label));

        agent.remove().unwrap();
        assert!(!LaunchAgent::exists(&label));
    }
}
