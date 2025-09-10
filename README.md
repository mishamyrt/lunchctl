# lunchctl

Lightweight Rust library for creating and controlling macOS Launch Agents (launchd) via `launchctl`. It helps you generate plist files in `~/Library/LaunchAgents`, start/stop agents, and query their state.

## Features

- Create and serialize Launch Agents to plist
- Bootstrap (start) and bootout (stop) agents via `launchctl`
- Check agent running state
- Read existing agent configs from disk
- Small, focused API

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
lunchctl = { git = "https://github.com/mishamyrt/lunchctl" }
```

## Quick start

```rust
use lunchctl::{LaunchAgent, LaunchControllable};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use a unique label for your job (maps to ~/Library/LaunchAgents/<label>.plist)
    let mut agent = LaunchAgent::new("co.example.myapp");

    // A harmless long-running command for demonstration
    agent.program_arguments = vec![
        "/usr/bin/tail".to_string(), "-f".to_string(), "/dev/null".to_string()
    ];
    agent.run_at_load = true;
    agent.keep_alive = true;

    // Write, start, check, then stop and remove
    agent.write()?;
    agent.bootstrap()?;
    println!("Running: {}", agent.is_running()?);
    agent.boot_out()?;
    agent.remove()?;

    Ok(())
}
```

## Read an existing agent

```rust
use lunchctl::LaunchAgent;

if LaunchAgent::exists("co.example.myapp") {
    let agent = LaunchAgent::from_file("co.example.myapp")?;
    println!("Args: {:?}", agent.program_arguments);
}
```

## Examples

- Basic end-to-end example: [`examples/basic.rs`](examples/basic.rs)

Run with:

```bash
cargo run --example basic
```

## Requirements

- macOS with `launchctl`
- The library writes plist files to `~/Library/LaunchAgents` for the current user

## License

MIT — see [`LICENSE`](LICENSE) for details.

