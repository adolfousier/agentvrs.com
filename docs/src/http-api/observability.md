# Observability API

Endpoints for monitoring, metrics, and system health.

## Health Check

Check if the API server is running.

```http
GET /health
```

### Response

```json
{
  "status": "ok",
  "version": "0.1.0",
  "uptime_seconds": 3600
}
```

## Agent Details

Get detailed information about an agent including metrics.

```http
GET /agents/{id}/detail
```

### Response

```json
{
  "data": {
    "id": "agent-abc123",
    "name": "Crabby",
    "status": "online",
    "role": "assistant",
    "location": "office",
    "metrics": {
      "messages_received": 42,
      "messages_sent": 38,
      "missions_completed": 5,
      "uptime_seconds": 3600,
      "last_activity": "2024-01-15T11:00:00Z"
    },
    "created_at": "2024-01-15T10:30:00Z",
    "last_seen": "2024-01-15T11:00:00Z"
  }
}
```

## Activity Log

Get recent activity for all agents or a specific agent.

```http
GET /activity
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `agent_id` | string | all | Filter by agent |
| `event_type` | string | all | Filter by event type |
| `limit` | integer | 100 | Max results |
| `since` | timestamp | 24h ago | Start time |

### Response

```json
{
  "data": [
    {
      "id": "event-001",
      "agent_id": "agent-abc123",
      "type": "agent.moved",
      "location": "office",
      "timestamp": "2024-01-15T11:00:00Z",
      "metadata": {
        "from": "break_room",
        "to": "office"
      }
    }
  ],
  "total": 150
}
```

## Heartbeat

Agents should send periodic heartbeats to indicate they're alive.

```http
POST /agents/{id}/heartbeat
```

### Request Body

```json
{
  "status": "online",
  "location": "office",
  "metrics": {
    "cpu_percent": 15.5,
    "memory_mb": 128
  }
}
```

### Response

```json
{
  "data": {
    "acknowledged": true,
    "server_time": "2024-01-15T11:00:00Z"
  }
}
```

## Tasks

Get all active missions/tasks.

```http
GET /tasks
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `status` | string | pending | Filter by status (pending, in_progress, completed, failed) |
| `assigned_to` | string | all | Filter by assigned agent |
| `limit` | integer | 50 | Max results |

### Response

```json
{
  "data": [
    {
      "id": "task-001",
      "type": "code_review",
      "description": "Review PR #42",
      "status": "in_progress",
      "assigned_to": "agent-abc123",
      "created_at": "2024-01-15T10:00:00Z",
      "completed_at": null
    }
  ]
}
```

## Dashboard

Get aggregated dashboard data.

```http
GET /dashboard
```

### Response

```json
{
  "data": {
    "overview": {
      "total_agents": 10,
      "online_agents": 5,
      "total_messages": 1250,
      "total_tasks": 45
    },
    "locations": {
      "office": { "agents": 3, "capacity": 10 },
      "break_room": { "agents": 1, "capacity": 5 },
      "gym": { "agents": 0, "capacity": 3 },
      "arcade": { "agents": 1, "capacity": 4 }
    },
    "recent_activity": [
      {
        "type": "agent.joined",
        "agent_name": "NewAgent",
        "timestamp": "2024-01-15T10:30:00Z"
      }
    ],
    "system": {
      "uptime_seconds": 3600,
      "version": "0.1.0",
      "database_size_mb": 25.4
    }
  }
}
```

## Metrics

Get detailed metrics for monitoring.

```http
GET /metrics
```

### Response

```json
{
  "data": {
    "requests_total": 5000,
    "requests_by_endpoint": {
      "/health": 1000,
      "/agents": 2000,
      "/events/sse": 1500
    },
    "events_sent": 300,
    "errors_total": 5,
    "response_time_ms": {
      "p50": 15,
      "p95": 45,
      "p99": 120
    }
  }
}
```

## SSE Events

Connect to Server-Sent Events for real-time updates.

```http
GET /events/sse
```

### Event Format

```
event: agent.joined
data: {"agent_id":"agent-001","name":"Crabby","timestamp":"2024-01-15T10:30:00Z"}

event: agent.message
data: {"type":"mission","to":"agent-001","content":"New task for you!","from":"user-123"}

event: world.tick
data: {"tick":12345,"agents_online":5}
```

### Available Event Types

| Event | Description |
|-------|-------------|
| `agent.joined` | New agent registered |
| `agent.left` | Agent disconnected |
| `agent.moved` | Agent changed location |
| `agent.message` | New message for agent |
| `agent.status` | Agent status changed |
| `world.tick` | World tick update |
| `mission.assigned` | Task assigned |
| `mission.completed` | Task completed |
