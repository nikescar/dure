# Quick Start - dure-wss

## The Permission Denied Error

When you see:
```
Error: Permission denied (os error 13)
```

This means the server tried to bind to port 443 (HTTPS default), which requires **root privileges** on Linux.

## Solutions

### Option 1: Use Non-Privileged Port (>1024) ✅ Recommended for Testing

```bash
# Port 8443 (common HTTPS alternative)
cargo run --package dure-wss -- server www.dure.app --addr 0.0.0.0:8443

# Port 3000 (common dev port)
cargo run --package dure-wss -- server www.dure.app --addr 0.0.0.0:3000
```

Then access via: `https://www.dure.app:8443`

### Option 2: Run with sudo (Production)

```bash
# Build release binary first
cargo build --package dure-wss --release

# Run with sudo
sudo ./target/release/dure-wss server www.dure.app --addr 0.0.0.0:443
```

**Security Note**: Running as root is not recommended for production. Use Option 3 instead.

### Option 3: Linux Capabilities (Production) ✅ Recommended

Allow the binary to bind to privileged ports without running as root:

```bash
# Build release binary
cargo build --package dure-wss --release

# Grant capability to bind to ports < 1024
sudo setcap 'cap_net_bind_service=+ep' ./target/release/dure-wss

# Now run as regular user
./target/release/dure-wss server www.dure.app --addr 0.0.0.0:443
```

**Advantages**:
- No need for sudo
- More secure (limited privileges)
- Standard approach for production servers

### Option 4: Reverse Proxy (Production) ✅ Recommended

Run `dure-wss` on a high port and use nginx/caddy as a reverse proxy:

```bash
# Run dure-wss on port 8443
./target/release/dure-wss server www.dure.app --addr 127.0.0.1:8443
```

**Nginx configuration** (`/etc/nginx/sites-available/dure`):
```nginx
server {
    listen 443 ssl http2;
    server_name www.dure.app;

    ssl_certificate /home/wj/.config/dure/certs/dure.app.crt;
    ssl_certificate_key /home/wj/.config/dure/certs/dure.app.key;

    # HTTP/HTTPS proxy
    location / {
        proxy_pass https://127.0.0.1:8443;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket upgrade
    location /ws {
        proxy_pass https://127.0.0.1:8443;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
    }
}
```

**Caddy configuration** (`/etc/caddy/Caddyfile`):
```caddy
www.dure.app {
    reverse_proxy localhost:8443 {
        transport http {
            tls_insecure_skip_verify
        }
    }
}
```

## Complete Usage Examples

### Development Testing
```bash
# Run on high port with debug output
RUST_LOG=debug cargo run --package dure-wss -- \
  server www.dure.app \
  --addr 0.0.0.0:8443 \
  --stats-interval 30
```

### Production Deployment
```bash
# Build optimized binary
cargo build --package dure-wss --release --profile release

# Grant port binding capability
sudo setcap 'cap_net_bind_service=+ep' ./target/release/dure-wss

# Run as systemd service (see below)
sudo systemctl start dure-wss@www.dure.app
```

### Docker Deployment
```bash
# Use high port in container, map to 443 on host
docker run -p 443:8443 dure-wss \
  server www.dure.app --addr 0.0.0.0:8443
```

## Systemd Service (Production)

Create `/etc/systemd/system/dure-wss@.service`:

```ini
[Unit]
Description=Dure WSS Server for %i
After=network.target

[Service]
Type=simple
User=wj
Group=wj
WorkingDirectory=/home/wj/work/dure
ExecStart=/home/wj/work/dure/target/release/dure-wss server %i --addr 0.0.0.0:443
Restart=on-failure
RestartSec=5s

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/home/wj/.local/share/dure /home/wj/.config/dure

# Capabilities for port binding
AmbientCapabilities=CAP_NET_BIND_SERVICE
CapabilityBoundingSet=CAP_NET_BIND_SERVICE

[Install]
WantedBy=multi-user.target
```

**Usage**:
```bash
# Enable and start for www.dure.app
sudo systemctl enable dure-wss@www.dure.app
sudo systemctl start dure-wss@www.dure.app

# Check status
sudo systemctl status dure-wss@www.dure.app

# View logs
sudo journalctl -u dure-wss@www.dure.app -f
```

## Testing the Server

### Using the built-in client

```bash
# Test WebSocket connection
cargo run --package dure-wss -- client wss://www.dure.app:8443 --insecure

# Test HTTPS GET
cargo run --package dure-wss -- client https://www.dure.app:8443 \
  --mode get --path /api/status --insecure

# Test HTTPS POST
cargo run --package dure-wss -- client https://www.dure.app:8443 \
  --mode post --path /webhook \
  --body '{"event":"test"}' \
  --insecure
```

**Note**: `--insecure` flag skips TLS verification, useful for self-signed certs in development.

### Using curl

```bash
# HTTPS GET
curl -k https://www.dure.app:8443/

# HTTPS POST
curl -k -X POST https://www.dure.app:8443/webhook \
  -H "Content-Type: application/json" \
  -d '{"event":"test"}'
```

### Using websocat

```bash
# Install websocat
cargo install websocat

# Test WebSocket
websocat -k wss://www.dure.app:8443/ws
```

## Troubleshooting

### Port already in use
```
Error: Address already in use (os error 98)
```

**Solution**: Another service is using the port. Find and stop it:
```bash
sudo lsof -i :8443
# or
sudo ss -tulpn | grep :8443
```

### Certificate not found
```
Error: No such file or directory (os error 2)
```

**Solution**: Generate certificates first using the main dure application:
```bash
# Using main dure CLI
dure acme issue www.dure.app

# Or use the GUI
dure-desktop
```

### Database locked
```
Error: database is locked
```

**Solution**: Another dure process is using the database:
```bash
# Find the process
ps aux | grep dure

# Stop it
pkill dure-desktop
```

## Next Steps

- See [README.md](./README.md) for full feature documentation
- See [ARCHITECTURE.md](./ARCHITECTURE.md) for technical details
- See [CHANGELOG.md](./CHANGELOG.md) for version history
