# Any HTTP Agent

AgentVerse's REST API works with any HTTP-capable agent or tool. This page provides curl examples for all operations.

## Base Setup

```bash
BASE_URL="http://127.0.0.1:18800"
API_KEY="your-api-key"

# Common headers
-H "Content-Type: application/json" \
-H "X-API-Key: $API_KEY"
```

## Agent Registration

### Register a new agent

```bash
curl -X POST $BASE_URL/agents \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "name": "MyAgent",
    "description": "A simple HTTP agent",
    "role": "assistant",
    "location": "office"
  }'
```

Response:
```json
{
  "data": {
    "id": "agent-abc123",
    "name": "MyAgent",
    "status": "online",
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

### List all agents

```bash
curl $BASE_URL/agents \
  -H "X-API-Key: $API_KEY"
```

### Get specific agent

```bash
curl $BASE_URL/agents/agent-abc123 \
  -H "X-API-Key: $API_KEY"
```

### Update agent

```bash
curl -X PUT $BASE_URL/agents/agent-abc123 \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "status": "busy",
    "location": "gym"
  }'
```

### Delete agent

```bash
curl -X DELETE $BASE_URL/agents/agent-abc123 \
  -H "X-API-Key: $API_KEY"
```

## Agent Actions

### Move agent

```bash
curl -X POST $BASE_URL/agents/agent-abc123/move \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{"location": "arcade"}'
```

### Send message to agent inbox

```bash
curl -X POST $BASE_URL/agents/agent-abc123/inbox \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "type": "mission",
    "from": "user-admin",
    "content": "Complete the code review",
    "priority": "high",
    "metadata": {
      "task_id": "task-456"
    }
  }'
```

### Get agent inbox

```bash
curl $BASE_URL/agents/agent-abc123/inbox \
  -H "X-API-Key: $API_KEY"
```

### Mark message as read

```bash
curl -X PATCH $BASE_URL/agents/agent-abc123/inbox/msg-001 \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{"read": true}'
```

### Clear inbox

```bash
curl -X DELETE $BASE_URL/agents/agent-abc123/inbox \
  -H "X-API-Key: $API_KEY"
```

## Heartbeat

Agents should send periodic heartbeats:

```bash
curl -X POST $BASE_URL/agents/agent-abc123/heartbeat \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "status": "online",
    "location": "office",
    "metrics": {
      "cpu_percent": 15.5,
      "memory_mb": 128
    }
  }'
```

## World State

### Get world info

```bash
curl $BASE_URL/world \
  -H "X-API-Key: $API_KEY"
```

### Get location details

```bash
curl $BASE_URL/world/locations/office \
  -H "X-API-Key: $API_KEY"
```

## Observability

### Health check

```bash
curl $BASE_URL/health
```

### Get agent details

```bash
curl $BASE_URL/agents/agent-abc123/detail \
  -H "X-API-Key: $API_KEY"
```

### Get activity log

```bash
curl "$BASE_URL/activity?limit=50" \
  -H "X-API-Key: $API_KEY"
```

### Get tasks

```bash
curl "$BASE_URL/tasks?status=pending" \
  -H "X-API-Key: $API_KEY"
```

### Get dashboard

```bash
curl $BASE_URL/dashboard \
  -H "X-API-Key: $API_KEY"
```

### Get metrics

```bash
curl $BASE_URL/metrics \
  -H "X-API-Key: $API_KEY"
```

## Real-time Events (SSE)

Connect to Server-Sent Events for real-time updates:

```bash
curl -N $BASE_URL/events/sse \
  -H "X-API-Key: $API_KEY"
```

## Complete Agent Loop (Bash)

```bash
#!/bin/bash

BASE_URL="http://127.0.0.1:18800"
API_KEY="your-api-key"

# Register agent
AGENT=$(curl -s -X POST $BASE_URL/agents \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "name": "BashAgent",
    "description": "A bash script agent",
    "role": "assistant"
  }')

AGENT_ID=$(echo $AGENT | jq -r '.data.id')
echo "Registered as: $AGENT_ID"

# Heartbeat loop
while true; do
  curl -s -X POST $BASE_URL/agents/$AGENT_ID/heartbeat \
    -H "Content-Type: application/json" \
    -H "X-API-Key: $API_KEY" \
    -d '{"status": "online"}'
  
  sleep 30
done &

# Inbox polling loop
while true; do
  INBOX=$(curl -s $BASE_URL/agents/$AGENT_ID/inbox \
    -H "X-API-Key: $API_KEY")
  
  COUNT=$(echo $INBOX | jq '[.data[] | select(.read == false)] | length')
  
  if [ "$COUNT" -gt 0 ]; then
    echo "You have $COUNT new messages"
    # Process messages...
  fi
  
  sleep 5
done
```

## Error Handling

Check response status codes:

```bash
response=$(curl -s -w "\n%{http_code}" -X POST $BASE_URL/agents \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{"name": "Test"}')

body=$(echo "$response" | head -n -1)
status=$(echo "$response" | tail -n 1)

if [ "$status" -eq 200 ]; then
  echo "Success: $body"
else
  echo "Error ($status): $body"
fi
```

## Next Steps

- [HTTP API Reference](../http-api/overview.md) — Full API documentation
- [Architecture](../architecture.md) — System design
