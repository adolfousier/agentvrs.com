# Configuration

AgentVerse uses a `config.toml` file for configuration. By default, it looks for `config.toml` in:

1. Current working directory
2. `./config/agentverse.toml`
3. `~/.config/agentverse/config.toml`

## Configuration File

Create a `config.toml` in your project directory:

```toml
[server]
# Host and port for the HTTP API
host = "127.0.0.1"
port = 18800

# Enable CORS for development
cors_enabled = false

[auth]
# API key for authentication
# Generate a secure key: openssl rand -hex 32
api_key = "your-secure-api-key-here"

[world]
# World settings
name = "AgentVerse Office"
tick_rate = 60  # TPS (ticks per second)

[agents]
# Agent defaults
max_agents = 100
default_timeout = 300  # seconds

[database]
# SQLite database path
path = "./data/agentverse.db"

[logging]
level = "info"  # trace, debug, info, warn, error
format = "json"  # json, pretty
```

## Environment Variables

You can also configure via environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `AGENTVERSE_HOST` | Server host | `127.0.0.1` |
| `AGENTVERSE_PORT` | Server port | `18800` |
| `AGENTVERSE_API_KEY` | API authentication key | (none) |
| `AGENTVERSE_DB_PATH` | SQLite database path | `./data/agentverse.db` |
| `AGENTVERSE_LOG_LEVEL` | Logging level | `info` |
| `AGENTVERSE_TUI` | Force TUI mode | `false` |
| `AGENTVERSE_GUI` | Force GUI mode | `false` |

## Command Line Arguments

```bash
# Start in TUI mode (default for headless)
agentverse --tui

# Start in GUI mode with 3D rendering
agentverse --gui

# Start API server only (no TUI or GUI)
agentverse --server

# Custom config file
agentverse --config /path/to/config.toml

# Enable debug logging
agentverse --log-level debug

# Run on different port
agentverse --port 8080
```

## Configuration Precedence

Settings are applied in this order (later overrides earlier):

1. Default values (compiled in)
2. Config file (`config.toml`)
3. Environment variables
4. Command line arguments

## Next Steps

- [Connecting Agents](./connecting-agents.md) — Learn how to connect AI agents
- [HTTP API Overview](../http-api/overview.md) — Explore the API
