# HTTPS/WSS Server Implementation Guide

## Overview

A complete combined HTTPS and WebSocket Secure (WSS) server implementation with:
- ✅ Static file serving (downloaded from GitHub)
- ✅ Webhook POST request handling with pattern matching
- ✅ WebSocket support with manual handshake
- ✅ Database-backed session tracking (HTTP & WSS)
- ✅ SQLite database for all persistence
- ✅ Manual WebSocket handshake (SHA-1 + Base64)

## Architecture

### Protocol Detection Flow
```
Client → TLS Handshake → Read HTTP Request → Check Headers
                                ↓
                    ┌───────────┴──────────┐
                    ↓                      ↓
            Upgrade: websocket?        Regular HTTP?
                    ↓                      ↓
          Manual WS Handshake        GET or POST?
                    ↓                      ↓
            WebSocket Stream          ┌────┴────┐
                    ↓                 ↓         ↓
            Echo + Track          Static    Webhook
                                  File      Handler
```

## Files Created/Modified

### Storage Models
- `mobile/src/storage/models/webhook.rs` - Webhook patterns, config, request logging
- `mobile/src/storage/models/session.rs` - HTTP/WSS session tracking
- `mobile/src/storage/models/mod.rs` - Export new modules

### Controllers
- `mobile/src/calc/webhook.rs` - Webhook pattern matching & validation
- `mobile/src/calc/session.rs` - Session management
- `mobile/src/calc/mod.rs` - Export new modules

### Server & Client
- `mobile/src/tungstenite/server.rs` - Combined HTTPS/WSS server with DB integration
- `mobile/src/tungstenite/client.rs` - Test client (HTTP GET/POST + WebSocket)

### CLI Commands
- `mobile/src/cli/commands/wss.rs` - WSS server/client management commands
- `mobile/src/cli/commands/webhook.rs` - Webhook management commands

### Dependencies
- `mobile/Cargo.toml` - Added `sha1 = "0.10"` (base64, zip, ureq already present)

## Database Schema

### webhook_config
```sql
CREATE TABLE webhook_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    logging_enabled INTEGER NOT NULL DEFAULT 0
);
```

### webhook_allow_patterns
```sql
CREATE TABLE webhook_allow_patterns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL
);
```

### webhook_requests
```sql
CREATE TABLE webhook_requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern TEXT NOT NULL,
    path TEXT NOT NULL,
    method TEXT NOT NULL,
    headers TEXT NOT NULL,
    body TEXT NOT NULL,
    remote_addr TEXT NOT NULL,
    received_at INTEGER NOT NULL
);
```

### sessions
```sql
CREATE TABLE sessions (
    session_id TEXT PRIMARY KEY,
    domain TEXT NOT NULL,
    session_type TEXT NOT NULL,  -- 'http' or 'wss'
    connected_at INTEGER NOT NULL,
    last_seen INTEGER NOT NULL,
    request_count INTEGER NOT NULL,
    remote_addr TEXT NOT NULL
);
```

## Usage

### 1. Setup Webhook Patterns

Before starting the server, configure webhook patterns:

```bash
# Enable webhook logging
dure webhook enable-logging --db dure-server.db

# Add webhook patterns
dure webhook add-pattern "/webhook/*" --db dure-server.db
dure webhook add-pattern "/api/v1/callback" --db dure-server.db
dure webhook add-pattern "*" --db dure-server.db  # Allow all

# List patterns
dure webhook list-patterns --db dure-server.db
```

**Output:**
```
Webhook Allow Patterns:
ID     Pattern                        Created
--------------------------------------------------------
1      /webhook/*                     2m ago
2      /api/v1/callback              1m ago
3      *                             30s ago
```

### 2. Start Server

```bash
# Start with default settings
dure wss server example.com

# Start with custom database
dure wss server example.com --db /path/to/db.sqlite

# Start without downloading static files
dure wss server example.com --no-download

# Custom address and stats interval
dure wss server example.com --addr 0.0.0.0:8443 --stats-interval 30
```

**Server Output:**
```
✓ Static files already present at "serv"
✓ Database initialized: dure-server.db
Loading TLS configuration...
  Certificate: /home/user/.acme.sh/example.com/example.com.cer
  Private key: /home/user/.acme.sh/example.com/example.com.key
🚀 Combined HTTPS/WSS server with Database
   Domain: example.com
   Listening on: https://0.0.0.0:443
   Static files: "serv"
   Database: dure-server.db
   Server ID: dure-server-12345

Protocol detection:
   GET /path -> Serve static file
   POST /webhook -> Check patterns & log to DB
   GET / (Upgrade: websocket) -> WebSocket + session tracking

Press Ctrl+C to stop
```

### 3. Test Static File Serving (HTTPS GET)

```bash
# Test with curl
curl -k https://localhost:443/index.html

# Test with dure client
dure wss client https://example.com --mode get --path /index.html
```

**Server Logs:**
```
New connection from: 127.0.0.1:54321
GET /index.html from 127.0.0.1:54321
Served /index.html - 4532 bytes
Connection closed: 127.0.0.1:54321
```

### 4. Test Webhook POST

```bash
# Using curl
curl -k -X POST https://localhost:443/webhook/test \
    -H "Content-Type: application/json" \
    -d '{"event":"test","data":"hello"}'

# Using dure client
dure wss client https://example.com \
    --mode post \
    --path /webhook/test \
    --body '{"event":"test","data":"hello"}'
```

**Server Logs (pattern matched):**
```
New connection from: 127.0.0.1:54322
POST /webhook/test from 127.0.0.1:54322
Webhook matched pattern: /webhook/*
Webhook request logged
Webhook POST /webhook/test - 200 OK
```

**Server Logs (pattern NOT matched):**
```
POST /other/path from 127.0.0.1:54323
Webhook path not in allow list: /other/path
Webhook POST /other/path - 404 ERROR
```

### 5. Test WebSocket

```bash
dure wss client wss://example.com --mode ws
```

**Interactive Session:**
```
Connecting to example.com:443...
TLS handshake completed
WebSocket connection established

✓ WebSocket Connected!
Type messages to send (Ctrl+D to quit):
hello
< [dure-server-12345] hello
test message
< [dure-server-12345] test message
^D
```

**Server Logs:**
```
New connection from: 127.0.0.1:54324
WebSocket connection established with: 127.0.0.1:54324 (session: session-1234567890-12345)
WebSocket connection closed: 127.0.0.1:54324 (session: session-1234567890-12345)
```

### 6. Monitor Webhook Requests

```bash
# List recent webhook requests
dure webhook list-requests --limit 10 --db dure-server.db

# Filter by pattern
dure webhook list-requests \
    --pattern "/webhook/*" \
    --limit 20 \
    --db dure-server.db
```

**Output:**
```
Recent Webhook Requests:
ID     Pattern              Path                           Remote          Received
-------------------------------------------------------------------------------------------
5      /webhook/*           /webhook/test                  127.0.0.1:54322 2m ago
4      /webhook/*           /webhook/github                127.0.0.1:54320 5m ago
3      /api/v1/callback     /api/v1/callback               192.168.1.100   10m ago
```

### 7. Monitor Sessions

```bash
# List all active sessions (last hour)
dure webhook list-sessions --db dure-server.db

# Filter by type
dure webhook list-sessions --session-type http --db dure-server.db
dure webhook list-sessions --session-type wss --db dure-server.db
```

**Output:**
```
Sessions:
Session ID                     Domain          Type   Requests   Last Seen
-------------------------------------------------------------------------------------
session-1234567890-12345       example.com     wss    15         30s ago
session-1234567891-12346       example.com     http   3          2m ago
session-1234567892-12347       example.com     http   1          5m ago
```

### 8. Cleanup Old Data

```bash
# Delete data older than 24 hours (default)
dure webhook cleanup --db dure-server.db

# Delete data older than 1 hour (3600 seconds)
dure webhook cleanup 3600 --db dure-server.db

# Delete data older than 7 days
dure webhook cleanup 604800 --db dure-server.db
```

**Output:**
```
✓ Cleanup complete:
  Deleted 127 old webhook requests
  Deleted 45 old sessions
```

### 9. Check Status

```bash
dure webhook status --db dure-server.db
```

**Output:**
```
Webhook Configuration:
  Logging enabled: true
  Allow patterns: 3
  Total requests logged: 127
```

## Pattern Matching

Webhook patterns support simple wildcard matching:

| Pattern | Matches | Example |
|---------|---------|---------|
| `*` | Everything | Any POST request |
| `/webhook/*` | Prefix match | `/webhook/test`, `/webhook/github` |
| `/api/v1/*` | Prefix match | `/api/v1/callback`, `/api/v1/notify` |
| `/exact/path` | Exact match | Only `/exact/path` |

## Security Notes

1. **Pattern Matching**: Only POST requests matching allow patterns are logged
2. **Path Traversal**: Static file serving validates paths to prevent `../` attacks
3. **TLS Required**: Server requires valid ACME certificates
4. **Session Tracking**: All connections tracked with unique session IDs
5. **Connection Limits**: Default max 10,000 concurrent connections

## Statistics

The server logs statistics every 60 seconds (configurable):

```
Stats: active=5, total=127, http=89, wss=38, webhooks=12
```

- **active**: Current open connections
- **total**: Total connections since start
- **http**: Total HTTPS requests (GET + POST)
- **wss**: Total WebSocket messages
- **webhooks**: Total webhook POST requests

## Troubleshooting

### Server won't start - Certificate error
```
Error: Failed to load certificates
```
**Solution**: Ensure ACME certificates exist at `~/.acme.sh/{domain}/`

### Webhook returns 404 but pattern exists
```
Webhook path not in allow list: /webhook/test
```
**Solution**: Check pattern with `list-patterns`, ensure wildcards are correct

### Database locked error
```
Failed to store session: database is locked
```
**Solution**: Only one server instance can use the database at a time

### Static files not found
```
File not found: /index.html
```
**Solution**:
- Run with `--no-download` flag removed
- Or manually place files in `serv/` directory

## Complete Example Workflow

```bash
# 1. Setup database
dure webhook enable-logging
dure webhook add-pattern "/webhook/*"
dure webhook add-pattern "/api/github/*"

# 2. Start server (in background)
dure wss server mysite.com &

# 3. Test all protocols
# HTTP GET (static files)
curl -k https://localhost:443/

# HTTP POST (webhook)
curl -k -X POST https://localhost:443/webhook/test \
    -d '{"test":"data"}'

# WebSocket
dure wss client wss://localhost:443 --mode ws

# 4. Monitor
dure webhook list-requests --limit 5
dure webhook list-sessions
dure webhook status

# 5. Cleanup
dure webhook cleanup 3600
```

## Performance

- **Async I/O**: Built on async-std for high concurrency
- **Database**: SQLite with async_std::sync::Mutex for thread-safe access
- **TLS**: Native TLS via rustls
- **WebSocket**: Manual handshake, no extra parsing overhead

## Next Steps

Potential enhancements:
- [ ] Add regex support for webhook patterns
- [ ] Implement rate limiting per session
- [ ] Add webhook retry mechanism
- [ ] Support multiple domains on same server
- [ ] Add metrics export (Prometheus format)
- [ ] Implement session-based authentication
- [ ] Add WebSocket broadcast channels

## License

Same as parent project (MIT/Apache-2.0 dual license)
