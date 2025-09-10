#![allow(clippy::print_stdout)]

use lunchctl::{LaunchAgent, LaunchControllable};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a unique label to avoid collisions with existing agents
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let label = format!("co.myrt.lunchctl.example.{timestamp}");

    // Configure the launch agent
    let mut agent = LaunchAgent::new(&label);
    agent.program_arguments = vec![
        "/usr/bin/tail".to_string(),
        "-f".to_string(),
        "/dev/null".to_string(),
    ];
    agent.keep_alive = true;
    agent.run_at_load = true;

    println!("Writing plist to {}", agent.path().display());
    agent.write()?;

    println!("Bootstrapping '{}'", agent.label);
    agent.bootstrap()?;

    // Give launchd a moment to start the job
    thread::sleep(Duration::from_millis(300));

    let running = agent.is_running()?;
    println!("Is running: {running}");

    println!("Booting out '{}'", agent.label);
    agent.boot_out()?;

    println!("Removing plist {}", agent.path().display());
    agent.remove()?;

    Ok(())
}
