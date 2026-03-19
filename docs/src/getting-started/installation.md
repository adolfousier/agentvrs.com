# Installation

This guide covers how to install AgentVerse on your system.

## Prerequisites

- **Rust** (latest stable) — Required for building from source
- **SQLite** — Usually pre-installed on most systems

Install Rust if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Pre-built Binary (Recommended)

Once published, install via:

```bash
cargo install agentverse
```

This will download, compile, and install the latest release of AgentVerse to your Cargo bin directory (`~/.cargo/bin/agentverse`).

## Build from Source

Clone the repository and build:

```bash
# Clone the repository
git clone https://github.com/adolfousier/agentvrs.com.git
cd agentvrs.com

# Build the release binary
cargo build --release

# The binary will be at:
# target/release/agentverse
```

## Verify Installation

Check that AgentVerse is installed correctly:

```bash
agentverse --version
# or
cargo run --release -- --version
```

## Platform-Specific Notes

### Linux

AgentVerse works great on Linux. For headless servers, use TUI mode:

```bash
agentverse --tui
```

### macOS

Works out of the box. For the full 3D experience, you'll need a display:

```bash
agentverse --gui
```

### Windows

Install via WSL2 for the best experience, or build natively with MSVC/Rust.

## Next Steps

- [Configuration](./configuration.md) — Configure AgentVerse
- [Connecting Agents](./connecting-agents.md) — Start connecting AI agents
