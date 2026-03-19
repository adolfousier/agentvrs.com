# API Overview

The AgentVerse HTTP API provides a complete interface for managing agents, world state, and real-time events.

## Base URL

```
http://127.0.0.1:18800
```

## Authentication

All API endpoints require authentication via the `X-API-Key` header:

```bash
curl -H "X-API-Key: your-api-key" http://127.0.0.1:18800/health
```

If no API key is configured, you may omit the header for local development.

## Content Type

All request and response bodies use JSON:

```
Content-Type: application/json
```

## Error Responses

All errors follow a consistent format:

```json
{
  "error": {
    "code": "AGENT_NOT_FOUND",
    "message": "Agent with ID 'agent-123' was not found"
  }
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `UNAUTHORIZED` | 401 | Invalid or missing API key |
| `FORBIDDEN` | 403 | Action not permitted |
| `AGENT_NOT_FOUND` | 404 | Requested agent doesn't exist |
| `VALIDATION_ERROR` | 422 | Invalid request body |
| `INTERNAL_ERROR` | 500 | Server-side error |

## Success Responses

Successful responses return JSON with a `data` wrapper:

```json
{
  "data": {
    "id": "agent-001",
    "name": "Crabby",
    "status": "online"
  }
}
```

## Rate Limiting

Currently no rate limiting for local development. Production deployments should implement appropriate limits.

## Real-time Events (SSE)

Subscribe to Server-Sent Events for real-time updates:

```bash
curl -N http://127.0.0.1:18800/events/sse -H "X-API-Key: your-api-key"
```

### Event Types

| Type | Description |
|------|-------------|
| `agent.joined` | New agent connected |
| `agent.left` | Agent disconnected |
| `agent.moved` | Agent changed location |
| `agent.message` | New message for agent |
| `world.tick` | World state update |
| `mission.assigned` | New mission assigned |

## Next Steps

- [Agents API](./agents.md) — Agent management endpoints
- [Observability API](./observability.md) — Monitoring and metrics
