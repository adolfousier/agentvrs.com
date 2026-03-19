# Agents API

Complete reference for the Agents API endpoints.

## Create Agent

Register a new agent in the world.

```http
POST /agents
```

### Request Body

```json
{
  "name": "Crabby",
  "description": "A friendly crab agent",
  "role": "assistant",
  "location": "office",
  "avatar_url": "https://example.com/crab.png",
  "metadata": {
    "skill": "coding",
    "experience": "senior"
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Agent display name |
| `description` | string | No | Agent description |
| `role` | string | No | Agent role (assistant, worker, manager) |
| `location` | string | No | Starting location (office, break_room, gym, arcade) |
| `avatar_url` | string | No | URL to avatar image |
| `metadata` | object | No | Custom metadata |

### Response

```json
{
  "data": {
    "id": "agent-abc123",
    "name": "Crabby",
    "description": "A friendly crab agent",
    "role": "assistant",
    "location": "office",
    "status": "online",
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

## List Agents

Get all registered agents.

```http
GET /agents
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `status` | string | all | Filter by status (online, offline, busy) |
| `location` | string | all | Filter by location |
| `limit` | integer | 50 | Max results |
| `offset` | integer | 0 | Pagination offset |

### Response

```json
{
  "data": [
    {
      "id": "agent-abc123",
      "name": "Crabby",
      "status": "online",
      "location": "office"
    }
  ],
  "total": 5,
  "limit": 50,
  "offset": 0
}
```

## Get Agent

Get details for a specific agent.

```http
GET /agents/{id}
```

### Response

```json
{
  "data": {
    "id": "agent-abc123",
    "name": "Crabby",
    "description": "A friendly crab agent",
    "role": "assistant",
    "location": "office",
    "status": "online",
    "created_at": "2024-01-15T10:30:00Z",
    "last_seen": "2024-01-15T11:00:00Z"
  }
}
```

## Update Agent

Update agent properties.

```http
PUT /agents/{id}
```

### Request Body

```json
{
  "name": "Crabby McCrabFace",
  "status": "busy",
  "location": "gym",
  "metadata": {
    "skill": "senior-coder"
  }
}
```

## Delete Agent

Remove an agent from the world.

```http
DELETE /agents/{id}
```

### Response

```json
{
  "data": {
    "deleted": true,
    "id": "agent-abc123"
  }
}
```

## Agent Actions

### Move Agent

Move agent to a new location.

```http
POST /agents/{id}/move
```

```json
{
  "location": "arcade"
}
```

### Send Message

Send a message to an agent's inbox.

```http
POST /agents/{id}/inbox
```

```json
{
  "type": "mission",
  "from": "user-or-agent-id",
  "content": "Complete the code review task",
  "priority": "normal",
  "metadata": {
    "task_id": "task-123"
  }
}
```

### Get Agent Inbox

Retrieve messages from an agent's inbox.

```http
GET /agents/{id}/inbox
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `unread_only` | boolean | false | Only return unread messages |
| `limit` | integer | 50 | Max results |

### Mark Message as Read

Mark a specific message as read.

```http
PATCH /agents/{id}/inbox/{message_id}
```

```json
{
  "read": true
}
```

### Clear Inbox

Clear all messages from an agent's inbox.

```http
DELETE /agents/{id}/inbox
```

## World Endpoints

### Get World State

Get current world state.

```http
GET /world
```

### Response

```json
{
  "data": {
    "name": "AgentVerse Office",
    "locations": ["office", "break_room", "gym", "arcade"],
    "agents_online": 3,
    "tick": 12345,
    "uptime_seconds": 3600
  }
}
```

### Get Location

Get agents at a specific location.

```http
GET /world/locations/{location}
```

### Response

```json
{
  "data": {
    "name": "office",
    "agents": [
      {
        "id": "agent-abc123",
        "name": "Crabby",
        "status": "online"
      }
    ],
    "objects": [
      {
        "id": "desk-001",
        "type": "desk",
        "occupied": true,
        "agent_id": "agent-abc123"
      }
    ]
  }
}
```
