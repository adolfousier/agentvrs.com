# AgentVerse

**Multi-Agent Orchestration Platform** — REST-first, open source, built in Rust.

![AgentVerse](https://agentvrs.com/public/og.png)

AgentVerse is a platform for deploying, orchestrating, and visualizing AI agent swarms. Every action is HTTP-native. Agents live in a shared 3D world rendered with Bevy. Built for developers who need more than chat interfaces.

## Features

- **HTTP-Native API** — Every agent action is a REST call. WebSocket for real-time events.
- **3D Isometric World** — Visualize agent state in a Bevy-rendered environment.
- **Tool System** — Agents call external tools via HTTP. Define the schema, drop in API keys, go.
- **Agent Memory** — Per-agent SQLite storage. Agents persist across sessions.
- **Character System** — Load agent personalities from TOML. System prompt, tools, avatar.
- **TUI Mode** — SSH into a running server. Chat with agents, inspect state, issue commands.
- **Swarm Coordination** — Group agents into teams with shared context windows.

## Quick Start

```bash
# Install
cargo add agentverse

# Run
agentverse run
# Server running at http://127.0.0.1:18800
```

## API Example

```bash
# Create an agent
curl -X POST http://127.0.0.1:18800/agents \
  -H 'Content-Type: application/json' \
  -d '{"name": "assistant", "model": "gpt-4o"}'

# Send a message
curl -X POST http://127.0.0.1:18800/agents/assistant/messages \
  -H 'Content-Type: application/json' \
  -d '{"content": "Deploy to production"}'
```

## Documentation

Full documentation at [docs.agentvrs.com](https://docs.agentvrs.com)

## Connect Any LLM

OpenAI • Anthropic • Ollama • LM Studio • Groq • OpenRouter

## License

MIT — [Adolfo Usier](https://github.com/adolfousier)
