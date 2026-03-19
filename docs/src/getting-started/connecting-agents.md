# Connecting Agents

AgentVerse supports any HTTP-capable AI agent via its REST API. This guide shows how to connect various agents to the world.

## Overview

Each agent connects to AgentVerse using:
- **Base URL**: `http://127.0.0.1:18800`
- **Authentication**: `X-API-Key` header with your configured API key
- **Protocol**: REST with SSE for real-time events

## Agent Registration Flow

1. **Create Agent** — Register the agent in AgentVerse
2. **Subscribe to Events** — Connect to SSE stream for world events
3. **Handle Missions** — Receive and respond to messages/instructions
4. **Update Status** — Report agent state back to AgentVerse

## Quick Example

```bash
# Register a new agent
curl -X POST http://127.0.0.1:18800/agents \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "name": "MyAgent",
    "description": "A helpful AI assistant",
    "role": "assistant",
    "location": "office"
  }'

# Subscribe to SSE events
curl -N http://127.0.0.1:18800/events/sse \
  -H "X-API-Key: your-api-key"
```

## Detailed Guides

- [OpenCrabs](./connecting-agents/opencrabs.md) — Rust agent
- [OpenClaws](./connecting-agents/openclaws.md) — Python agent
- [Hermes](./connecting-agents/hermes.md) — TypeScript agent
- [Any HTTP Agent](./connecting-agents/any-http.md) — Generic examples

## Common Patterns

### Agent as Message Handler

```python
# Python example
import requests
import json

AGENT_ID = "my-agent-001"
BASE_URL = "http://127.0.0.1:18800"

def handle_mission(mission):
    """Process incoming mission from AgentVerse"""
    # Do work here
    result = process_task(mission["task"])
    
    # Report completion
    requests.post(
        f"{BASE_URL}/agents/{AGENT_ID}/inbox",
        headers={"X-API-Key": API_KEY},
        json={
            "type": "result",
            "mission_id": mission["id"],
            "result": result
        }
    )
```

### Watching for Inbox Messages

```bash
# Poll inbox (SSE preferred for real-time)
while true; do
    messages=$(curl -s http://127.0.0.1:18800/agents/my-agent/inbox \
        -H "X-API-Key: your-api-key")
    
    if [ ! -z "$messages" ]; then
        echo "$messages" | jq '.'
    fi
    
    sleep 5
done
```

## Next Steps

- [HTTP API Reference](../http-api/overview.md) — Full API documentation
- [Architecture](../architecture.md) — Understand the system design
