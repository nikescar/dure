# WebSocket Authentication

This document describes the WebSocket authentication system implemented in Dure.

## Overview

The Dure WebSocket server supports device-based authentication using public key cryptography. Clients authenticate by sending their device ID and public key, and receive a session token for subsequent requests.

## Architecture

### Message Flow

```
Client                          Server
  |                               |
  |--- auth.login request ------->|
  |                               | - Validate device_id & public_key
  |                               | - Store device auth in DB
  |                               | - Generate session token
  |<-- auth.response -------------|
  |                               |
  |--- (authenticated messages)-->|
  |                               |
  |--- auth.logout request ------->|
  |                               | - Delete session & device auth
  |<-- auth.logout.response ------|
  |                               |
```

## Components

### 1. Message Types (`mobile/src/site/messages/auth.rs`)

Defines the request and response structures:

- **AuthLoginRequest**: Client sends device credentials
  - `device_id`: Unique device identifier (from machine-id)
  - `public_key`: Device's public key for encryption
  - `session_id`: Optional session ID for reconnection
  - `client_version`: Optional client version string

- **AuthResponse**: Server response to login
  - `success`: Whether authentication succeeded
  - `session_id`: Session token for this connection
  - `server_public_key`: Server's public key for E2E encryption
  - `error`: Error message if failed
  - `device_info`: Authenticated device information
  - `expires_at`: Session expiration timestamp

- **AuthLogoutRequest**: Client requests logout
  - `session_id`: Session to terminate

- **AuthLogoutResponse**: Server confirms logout
  - `success`: Whether logout succeeded
  - `message`: Optional status message

### 2. Message Handlers (`mobile/src/wss/server/handlers/`)

#### Main Handler (`handlers/mod.rs`)

Routes incoming `ClientMessage` to appropriate handler based on message type.

```rust
pub async fn handle_client_message(
    msg: ClientMessage,
    session_id: &str,
    settings: &ServerSettings,
) -> Result<ServerMessage>
```

#### Auth Handler (`handlers/auth.rs`)

Implements authentication logic:

**handle_login()**
1. Validates device_id and public_key
2. Checks for session reconnection
3. Stores authenticated device in database
4. Returns auth response with session details

**handle_logout()**
1. Validates session_id
2. Deletes session from database
3. Removes device authentication
4. Returns logout confirmation

### 3. Storage Layer (`mobile/src/storage/models/device.rs`)

Manages device authentication persistence:

- `store_device_auth()`: Save/update device authentication
- `get_device_auth()`: Retrieve device by device_id
- `get_device_by_session()`: Find device for a session
- `update_device_activity()`: Update last_seen timestamp
- `delete_device_auth()`: Remove device authentication
- `list_authenticated_devices()`: List all authenticated devices
- `cleanup_old_devices()`: Remove stale authentications

### 4. Database Schema

**authenticated_devices table:**
```sql
CREATE TABLE authenticated_devices (
    device_id TEXT PRIMARY KEY NOT NULL,
    public_key TEXT NOT NULL,
    session_id TEXT NOT NULL,
    authenticated_at INTEGER NOT NULL,
    last_seen INTEGER NOT NULL
);
```

Indexes:
- `idx_authenticated_devices_session_id`: Fast session lookups
- `idx_authenticated_devices_last_seen`: Cleanup of stale devices

### 5. WebSocket Integration (`mobile/src/wss/server/ws.rs`)

The WebSocket handler:
1. Parses incoming JSON messages as `ClientMessage`
2. Routes to `handle_client_message()`
3. Serializes `ServerMessage` response
4. Handles parse/handler errors gracefully

## Usage

### Client Authentication Flow

#### 1. Connect to WebSocket

```javascript
const ws = new WebSocket('wss://example.com/ws');
```

#### 2. Send Login Request

```json
{
  "type": "auth.login",
  "device_id": "machine-id-12345",
  "public_key": "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----",
  "session_id": null,
  "client_version": "dure-client/1.0.0"
}
```

#### 3. Receive Auth Response

**Success:**
```json
{
  "type": "auth.response",
  "success": true,
  "session_id": "session-1234567890-42",
  "server_public_key": "SERVER_KEY_dure-server-12345",
  "error": null,
  "device_info": {
    "device_id": "machine-id-12345",
    "device_name": null,
    "platform": "dure-client/1.0.0",
    "last_seen": "2026-04-12T10:30:00Z"
  },
  "expires_at": "2026-04-13T10:30:00Z"
}
```

**Failure:**
```json
{
  "type": "auth.response",
  "success": false,
  "session_id": null,
  "server_public_key": null,
  "error": "Invalid device_id or public_key",
  "device_info": null,
  "expires_at": null
}
```

#### 4. Send Authenticated Messages

After successful authentication, send other message types:

```json
{
  "type": "product.list",
  "channel_id": "store-123",
  "limit": 10
}
```

#### 5. Logout

```json
{
  "type": "auth.logout",
  "session_id": "session-1234567890-42"
}
```

Response:
```json
{
  "type": "auth.logout.response",
  "success": true,
  "message": "Logged out successfully"
}
```

## Error Handling

### Parse Errors

If the client sends invalid JSON or unknown message type:

```json
{
  "type": "error",
  "code": "PARSE_ERROR",
  "message": "Invalid message format: missing field `device_id`",
  "request_id": null,
  "details": null
}
```

### Handler Errors

If the handler encounters an error:

```json
{
  "type": "error",
  "code": "HANDLER_ERROR",
  "message": "Failed to store authentication",
  "request_id": null,
  "details": null
}
```

### Not Implemented

For unimplemented message types:

```json
{
  "type": "error",
  "code": "NOT_IMPLEMENTED",
  "message": "Handler not implemented for this message type",
  "request_id": null,
  "details": null
}
```

## Security Considerations

### Current Implementation (MVP)

The current implementation provides basic authentication:
- ✅ Device identification via device_id
- ✅ Session tracking in database
- ✅ Session expiration (24 hours)
- ⚠️ Server public key is placeholder (not cryptographically secure)
- ⚠️ No signature verification of device public key
- ⚠️ No end-to-end encryption (yet)

### Future Enhancements (TODO)

1. **Cryptographic Verification**
   - Verify device public key signature
   - Challenge-response authentication
   - Proper server key pair generation

2. **End-to-End Encryption**
   - Use public keys for message encryption
   - Perfect forward secrecy (PFS)
   - Key rotation

3. **Authorization**
   - Permission system (read/write/admin)
   - Channel-based access control
   - Device whitelisting/blacklisting

4. **Rate Limiting**
   - Failed auth attempt tracking
   - Temporary device bans
   - DDoS protection

5. **Audit Logging**
   - Authentication events
   - Failed login attempts
   - Session lifecycle tracking

## Testing

### Run Example Client

```bash
# Start the server
cargo run --bin wss-server -- --domain localhost

# In another terminal, run the example client
cargo run --example websocket_auth_client
```

### Manual Testing with wscat

```bash
# Install wscat
npm install -g wscat

# Connect and authenticate
wscat -c ws://localhost:8443/ws

# Send auth.login
> {"type":"auth.login","device_id":"test-device","public_key":"test-key","session_id":null,"client_version":"wscat"}

# You should receive auth.response
< {"type":"auth.response","success":true,...}
```

### Unit Tests

```bash
# Run device storage tests
cargo test --lib storage::models::device

# Run session tests
cargo test --lib storage::models::session
```

## Troubleshooting

### Authentication fails with empty fields

**Problem:** Client receives error "Invalid device_id or public_key"

**Solution:** Ensure both `device_id` and `public_key` are non-empty strings.

### Session not found after reconnection

**Problem:** Using `session_id` to reconnect but server doesn't recognize it

**Causes:**
1. Session expired (> 24 hours old)
2. Server was restarted (sessions are in memory)
3. Wrong session_id

**Solution:** Always handle auth failure by re-authenticating with new session.

### Database errors

**Problem:** "Failed to store device auth"

**Solution:** Run migrations to ensure `authenticated_devices` table exists:

```bash
diesel migration run --migration-dir mobile/migrations
```

## References

- [Message Types](../src/site/messages/README.md) - Complete message documentation
- [AsyncAPI Docs](../../crates/asyncapi-gen/docs/api-docs/) - Interactive API documentation
- [Session Management](../src/storage/models/session.rs) - Session storage implementation
