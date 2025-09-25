#![allow(clippy::print_stdout)]

use lunchctl::{LaunchAgentBuilder, LaunchControllable};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let label = format!("co.myrt.lunchctl.builder.{timestamp}");

    let agent = LaunchAgentBuilder::default()
        .label(label)
        .arg("/usr/bin/tail".into())
        .arg("-f".into())
        .arg("/dev/null".into())
        .keep_alive(true)
        .run_at_load(true)
        .build()?;

    println!("Writing plist to {}", agent.path().display());
    agent.write()?;

    println!("Bootstrapping '{}'", agent.label);
    agent.bootstrap()?;

    thread::sleep(Duration::from_millis(300));
    println!("Is running: {}", agent.is_running()?);

    println!("Booting out '{}'", agent.label);
    agent.boot_out()?;

    println!("Removing plist {}", agent.path().display());
    agent.remove()?;

    Ok(())
}


