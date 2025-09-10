# ajam-launchctl

A Rust crate for working with macOS launch agents via `launchctl`, providing an easy way to create, manage, and control background processes on macOS.

## Features

- Create and configure launch agents
- Manage launch agent autostart behavior
- Bootstrap (start) and bootout (stop) launch agents
- Check if a launch agent is running
- Read and write launch agent configurations as plist files

## Usage

### Creating a Launch Agent

```rust
use ajam_launchctl::{LaunchAgent, LaunchControllable};

// Create a new launch agent
let mut agent = LaunchAgent::new("com.example.myapp");

// Configure the agent
agent.program_arguments = vec!["myapp".to_string(), "--daemon".to_string()];
agent.run_at_load = true;  // Start at login
agent.keep_alive = true;   // Restart if process exits

// Write the configuration to ~/Library/LaunchAgents/
agent.write().expect("Failed to write launch agent configuration");

// Bootstrap (start) the agent
agent.bootstrap().expect("Failed to bootstrap agent");

// Check if running
let is_running = agent.is_running().expect("Failed to check agent status");
println!("Agent running: {}", is_running);

// Stop the agent
agent.boot_out().expect("Failed to stop agent");
```

### Reading an Existing Launch Agent

```rust
use ajam_launchctl::LaunchAgent;

if LaunchAgent::exists("com.example.myapp") {
    let agent = LaunchAgent::from_file("com.example.myapp")
        .expect("Failed to read launch agent");
    
    println!("Program arguments: {:?}", agent.program_arguments);
}
```

