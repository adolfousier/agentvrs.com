# Architecture

This document describes the internal architecture of AgentVerse.

## High-Level Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         AgentVerse                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐        │
│  │   TUI Mode  │     │   GUI Mode  │     │  REST API   │        │
│  │  (ratatui)  │     │   (bevy)    │     │   (axum)    │        │
│  └──────┬──────┘     └──────┬──────┘     └──────┬──────┘        │
│         │                    │                   │              │
│         └────────────────────┴───────────────────┘              │
│                              │                                  │
│                    ┌─────────┴─────────┐                       │
│                    │   Application Core  │                       │
│                    │   (State, Events)   │                       │
│                    └─────────┬─────────┘                       │
│                              │                                  │
│         ┌────────────────────┼────────────────────┐             │
│         │                    │                    │             │
│  ┌──────┴──────┐     ┌───────┴──────┐    ┌────────┴────────┐    │
│  │   World     │     │    Agents    │    │    Database     │    │
│  │  (bevy)     │     │  (Registry)  │    │   (SQLite)      │    │
│  └─────────────┘     └──────────────┘    └─────────────────┘    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
agentvrscom/
├── src/
│   ├── main.rs              # Entry point
│   │
│   ├── lib.rs               # Library root
│   │
│   ├── cli/                 # Command-line interface
│   │   ├── mod.rs
│   │   ├── args.rs          # Argument parsing
│   │   └── commands.rs      # CLI commands
│   │
│   ├── api/                 # HTTP API (axum)
│   │   ├── mod.rs
│   │   ├── router.rs        # Route definitions
│   │   ├── handlers/        # Request handlers
│   │   │   ├── mod.rs
│   │   │   ├── agents.rs    # Agent endpoints
│   │   │   ├── world.rs     # World endpoints
│   │   │   ├── events.rs    # SSE endpoints
│   │   │   └── health.rs    # Health check
│   │   ├── middleware/      # axum middleware
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs      # API key auth
│   │   │   ├── logging.rs   # Request logging
│   │   │   └── cors.rs      # CORS handling
│   │   └── models/          # API models
│   │       ├── mod.rs
│   │       ├── request.rs   # Request DTOs
│   │       └── response.rs  # Response DTOs
│   │
│   ├── world/               # 3D World (Bevy)
│   │   ├── mod.rs
│   │   ├── plugin.rs        # Bevy plugin
│   │   ├── systems/         # Bevy systems
│   │   │   ├── mod.rs
│   │   │   ├── agent_systems.rs
│   │   │   ├── movement.rs
│   │   │   └── spatial.rs
│   │   ├── components/      # ECS components
│   │   │   ├── mod.rs
│   │   │   ├── agent.rs
│   │   │   ├── location.rs
│   │   │   └── object.rs
│   │   └── resources/       # Bevy resources
│   │       ├── mod.rs
│   │       └── world_state.rs
│   │
│   ├── tui/                 # Terminal UI (ratatui)
│   │   ├── mod.rs
│   │   ├── app.rs           # App state
│   │   ├── ui.rs            # UI rendering
│   │   ├── widgets/         # Custom widgets
│   │   │   ├── mod.rs
│   │   │   ├── agent_list.rs
│   │   │   ├── location_panel.rs
│   │   │   ├── details.rs
│   │   │   └── log_panel.rs
│   │   └── input.rs         # Input handling
│   │
│   ├── agent/               # Agent management
│   │   ├── mod.rs
│   │   ├── registry.rs      # Agent registry
│   │   ├── inbox.rs         # Message inbox
│   │   └── session.rs       # Agent session
│   │
│   ├── db/                  # Database (SQLite)
│   │   ├── mod.rs
│   │   ├── schema.rs        # Database schema
│   │   ├── migrations.rs    # Migrations
│   │   └── repositories/    # Data access
│   │       ├── mod.rs
│   │       ├── agent_repo.rs
│   │       └── message_repo.rs
│   │
│   ├── events/              # Event system
│   │   ├── mod.rs
│   │   ├── bus.rs           # Event bus
│   │   └── types.rs         # Event types
│   │
│   ├── config/              # Configuration
│   │   ├── mod.rs
│   │   ├── app_config.rs    # App config
│   │   └── env.rs           # Env variables
│   │
│   └── error.rs             # Error handling
│
├── docs/                    # mdbook documentation
│   ├── book.toml
│   ├── src/
│   └── theme/
│
└── tests/                   # Integration tests
    ├── api_tests.rs
    └── agent_tests.rs
```

## Core Components

### Application Core

The core state management layer that coordinates between all subsystems:

```rust
// Central state container
pub struct AppState {
    pub world: World,           // Bevy world
    pub agents: AgentRegistry,   // Agent management
    pub db: Database,           // SQLite connection
    pub event_bus: EventBus,     // SSE broadcasting
    pub config: AppConfig,      // Configuration
}
```

### HTTP API (api/)

Built with **axum** for high performance:

- **Router** — Path-based routing
- **Handlers** — Request processing
- **Middleware** — Auth, logging, CORS
- **Models** — Request/response DTOs

### 3D World (world/)

Bevy-based isometric world:

- **Components** — ECS data structures
- **Systems** — Game logic
- **Resources** — Shared state

### TUI (tui/)

Ratatui-based terminal interface:

- **App** — Application state
- **Widgets** — Reusable UI components
- **Input** — Keyboard handling

### Agent Management (agent/)

Core agent lifecycle:

- **Registry** — Track all agents
- **Inbox** — Message handling
- **Session** — Connection state

### Database (db/)

SQLite persistence with **rusqlite**:

- **Schema** — Table definitions
- **Migrations** — Version management
- **Repositories** — Data access patterns

### Event System (events/)

Pub/sub for real-time updates:

```rust
pub enum Event {
    AgentJoined { agent_id: String },
    AgentLeft { agent_id: String },
    AgentMoved { agent_id: String, location: String },
    Message { to: String, content: String },
    MissionAssigned { to: String, mission: Mission },
}
```

## Data Flow

```
┌──────────┐     ┌──────────┐     ┌──────────┐
│  Client  │────▶│   API    │────▶│  Core    │
│          │◀────│  Server  │◀────│          │
└──────────┘     └──────────┘     └──────────┘
                                      │
          ┌───────────────────────────┼───────────────────────────┐
          │                           │                           │
          ▼                           ▼                           ▼
   ┌─────────────┐           ┌─────────────┐           ┌─────────────┐
   │    World    │           │   Agents    │           │   Database  │
   │   (Bevy)    │           │  Registry   │           │   (SQLite)  │
   └─────────────┘           └─────────────┘           └─────────────┘
          │                           │                           │
          └───────────────────────────┼───────────────────────────┘
                                      │
                                      ▼
                               ┌─────────────┐
                               │  Event Bus  │
                               │    (SSE)    │
                               └─────────────┘
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `axum` | HTTP server |
| `bevy` | 3D engine |
| `ratatui` | TUI framework |
| `rusqlite` | SQLite database |
| `tokio` | Async runtime |
| `serde` | Serialization |
| `tracing` | Logging |
| ` clap` | CLI parsing |

## Security

- **API Key Auth** — All endpoints protected by `X-API-Key`
- **Localhost by Default** — API binds to `127.0.0.1`
- **No CORS by Default** — Prevents cross-origin attacks
- **Input Validation** — All inputs sanitized

## Performance

- **Event Broadcasting** — Efficient pub/sub for SSE
- **Connection Pooling** — Database connections reused
- **Async/Await** — Non-blocking I/O throughout
- **ECS Patterns** — Cache-friendly data access

## Extensibility

AgentVerse is designed for extension:

1. **New Handlers** — Add endpoints in `api/handlers/`
2. **New Systems** — Add Bevy systems in `world/systems/`
3. **New Widgets** — Add TUI widgets in `tui/widgets/`
4. **New Event Types** — Extend `events/types.rs`

## Next Steps

- [HTTP API Reference](../http-api/overview.md) — API documentation
- [TUI Mode](../tui-mode.md) — Terminal interface
