# dure CLI Reference

Comprehensive reference for all `dure` commands.

---

## Table of Contents

- [Global Options](#global-options)
- [Operating Modes](#operating-modes)
- [Hosting Operations](#hosting-operations)
- [Dure Operations (Community)](#dure-operations-community)
  - [Role Management](#role-management)
  - [Member Management](#member-management)
  - [Device Operations](#device-operations)
  - [Channel Management](#channel-management)
  - [Messaging](#messaging)
  - [Reactions](#reactions)
  - [Direct Messages](#direct-messages)
  - [Threads](#threads)
  - [Polls](#polls)
- [Store Operations](#store-operations)
  - [Product Management](#product-management)
  - [Order Management](#order-management)
  - [Payment Operations](#payment-operations)
  - [Review Management](#review-management)
- [Configuration](#configuration)
- [Exit Codes](#exit-codes)
- [Environment Variables](#environment-variables)
- [JSON Output Schemas](#json-output-schemas)

---

## Global Options

These options apply to all commands:

| Option | Description |
|--------|-------------|
| `--json` | Output as JSON (machine-readable) |
| `--token <TOKEN>` | Override bot token |
| `-y, --yes` | Skip confirmations |
| `--profile <PROFILE>` | Switch permission levels |
| `--serv` | Run server mode |
| `--tray` | Run GUI mode (default) |
| `-v, --verbose` | Increase logging verbosity |
| `-q, --quiet` | Quiet mode (errors only) |
| `--no-color` | Disable colored output |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

---

## Operating Modes

Dure supports three operating modes:

### Server Mode

```bash
dure --serv
```

Runs Dure as a WebSocket server for peer-to-peer communication.

### GUI Mode (Default)

```bash
dure
# or explicitly
dure --tray
```

Launches the egui graphical interface with Material3 design.

### CLI Mode

```bash
dure <COMMAND> [OPTIONS]
```

Command-line interface for all operations.

---

## Hosting Operations

### hosting check

Check required configuration and validation.

```bash
dure hosting check
```

**Example:**
```bash
dure hosting check
```

### hosting init

Initialize hosting configuration.

```bash
dure hosting init
```

**Example:**
```bash
dure hosting init
```

### hosting show

Show hosting details and configurations.

```bash
dure hosting show [DOMAIN]
```

**Examples:**
```bash
# Show specific domain
dure hosting show www.asset.com

# Show all
dure hosting show
```

### hosting close

Close hosting.

```bash
dure hosting close <DOMAIN>
```

**Example:**
```bash
dure hosting close www.asset.com
```

### hosting reopen

Reopen closed hosting.

```bash
dure hosting reopen <DOMAIN>
```

**Example:**
```bash
dure hosting reopen www.asset.com
```

### hosting delete

Delete hosting.

```bash
dure hosting delete <DOMAIN>
```

**Example:**
```bash
dure hosting delete www.asset.com
```

### hosting list

Show bot's server memberships.

```bash
dure hosting list
```

### hosting info

Get server metadata.

```bash
dure hosting info <HOST_NAME>
```

**Example:**
```bash
dure hosting info "Host1"
```

---

## Dure Operations (Community)

### Role Management

#### role list

List roles in a server.

```bash
dure role list <SERVER>
```

**Example:**
```bash
dure role list "Host1"
```

#### role create

Create a role.

```bash
dure role create <SERVER> <ROLE_NAME> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--color <COLOR>` | Role color (hex format) |

**Example:**
```bash
dure role create "Host1" "Contributor" --color "00ff00"
```

#### role assign

Assign a role to a member.

```bash
dure role assign <SERVER> <MEMBER> <ROLE>
```

**Example:**
```bash
dure role assign "Host1" "@alice" "Contributor"
```

#### role remove

Remove a role from a member.

```bash
dure role remove <SERVER> <MEMBER> <ROLE>
```

**Example:**
```bash
dure role remove "Host1" "@alice" "Contributor"
```

#### role edit

Edit a role (name, color, hoist, mentionable).

```bash
dure role edit <SERVER> <ROLE> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--name <NAME>` | New role name |
| `--color <COLOR>` | Role color (hex format) |
| `--hoist` | Make role hoisted |
| `--mentionable` | Make role mentionable |

**Example:**
```bash
dure role edit "Host1" "Contributor" --name "Core Contributor" --color 00ff00 --hoist --mentionable
```

#### role delete

Delete a role (confirmation required).

```bash
dure role delete <SERVER> <ROLE>
```

**Example:**
```bash
dure role delete "Host1" "OldRole"
```

---

### Member Management

**Note:** There are two types of members:
1. **Owner members**: IDs without dots (e.g., `user123`) - for owner devices
2. **Store members**: Domain-based IDs with dots (e.g., `store.example.com`) - for other stores

When a store member receives a message, it syncs with the store WSS.

#### member list

List members in a server.

```bash
dure member list <SERVER> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--limit <N>` | Custom member limit |

**Examples:**
```bash
dure member list "Host1"
dure member list "Host1" --limit 100
```

#### member info

Get member details (roles, join date, etc.).

```bash
dure member info <SERVER> <MEMBER>
```

**Examples:**
```bash
# By username
dure member info "Host1" "@alice"

# By ID
dure member info "Host1" 1234567890
```

#### member kick

Kick a member (confirmation required).

```bash
dure member kick <SERVER> <MEMBER> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--reason <REASON>` | Kick reason |

**Example:**
```bash
dure member kick "Host1" "@spammer" --reason "Spam"
```

#### member ban

Ban a member permanently.

```bash
dure member ban <SERVER> <MEMBER> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--reason <REASON>` | Ban reason |

**Example:**
```bash
dure member ban "Host1" "@troll" --reason "Repeated violations"
```

#### member unban

Unban a member.

```bash
dure member unban <SERVER> <MEMBER>
```

**Example:**
```bash
dure member unban "Host1" "@reformed"
```

#### member timeout

Timeout a member (duration in seconds).

```bash
dure -y member timeout <SERVER> <MEMBER> <DURATION> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--reason <REASON>` | Timeout reason |

**Examples:**
```bash
# Timeout for 1 hour (3600 seconds)
dure -y member timeout "Host1" "@spammer" 3600 --reason "Spam"

# Remove timeout (set duration to 0)
dure -y member timeout "Host1" "@spammer" 0
```

---

### Device Operations

#### device list

Show devices in all hostings.

```bash
dure device list <SERVER> <MEMBER>
```

**Example:**
```bash
dure device list "Host1" "Member1"
```

#### device info

Get device metadata.

```bash
dure device info <SERVER> <MEMBER> <DEVICE>
```

**Example:**
```bash
dure device info "Host1" "Member1" "Device1"
```

---

### Channel Management

#### channel list

Display channels.

```bash
dure channel list [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--server <SERVER>` | Filter by specific server |

**Examples:**
```bash
# All channels
dure channel list

# Channels in specific server
dure channel list --server "Host1"
```

#### channel info

Retrieve channel details.

```bash
dure channel info <CHANNEL>
```

**Example:**
```bash
dure channel info "#channel"
```

#### channel create

Add text/voice/forum channels.

```bash
dure channel create <SERVER> <NAME> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--type <TYPE>` | Channel type (text/voice/forum) |

**Example:**
```bash
dure channel create "Server" "name" --type voice
```

#### channel forum-post

Post forum threads.

```bash
dure channel forum-post <CHANNEL> <TITLE> <CONTENT>
```

**Example:**
```bash
dure channel forum-post "#feedback" "title" "content"
```

#### channel edit

Update channel settings.

```bash
dure channel edit <CHANNEL> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--topic <TOPIC>` | Channel topic |
| `--slowmode <SECONDS>` | Slowmode delay |

**Example:**
```bash
dure channel edit "#channel" --topic "new topic" --slowmode 10
```

#### channel set-permissions

Control access permissions.

```bash
dure channel set-permissions <CHANNEL> <ROLE> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--allow <PERM>` | Allow permission |

**Example:**
```bash
dure channel set-permissions "#channel" "Role" --allow send_messages
```

#### channel delete

Remove channels.

```bash
dure channel delete <CHANNEL>
```

**Example:**
```bash
dure channel delete "#channel"
```

---

### Messaging

#### message send

Deliver messages to channels.

```bash
dure message send <CHANNEL> <MESSAGE>
```

**Example:**
```bash
dure message send "#general" "Hello from the terminal!"
```

#### message list

Retrieve recent messages (default: 10).

```bash
dure message list <CHANNEL>
```

**Example:**
```bash
dure message list "#channel"
```

#### message search

Find messages by content or author.

```bash
dure message search <CHANNEL> <QUERY>
```

**Example:**
```bash
dure message search "#channel" "query"
```

#### message get

Fetch specific message by ID.

```bash
dure message get <CHANNEL> <MESSAGE_ID>
```

**Example:**
```bash
dure message get "#channel" ID
```

#### message history

Export historical records.

```bash
dure message history <CHANNEL> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--days <N>` | Number of days to export |

**Example:**
```bash
dure message history "#channel" --days 7
```

#### message reply

Reply to specific messages.

```bash
dure message reply <CHANNEL> <MESSAGE_ID> <RESPONSE>
```

**Example:**
```bash
dure message reply "#channel" ID "response"
```

#### message edit

Modify bot's own messages.

```bash
dure message edit <CHANNEL> <MESSAGE_ID> <NEW_TEXT>
```

**Example:**
```bash
dure message edit "#channel" ID "new text"
```

#### message delete

Remove messages (confirmation required).

```bash
dure message delete <CHANNEL> <MESSAGE_ID>
```

**Example:**
```bash
dure message delete "#channel" ID
```

#### message bulk-delete

Remove multiple messages at once.

```bash
dure message bulk-delete <CHANNEL> <MESSAGE_ID1> <MESSAGE_ID2> <MESSAGE_ID3>
```

**Example:**
```bash
dure message bulk-delete "#channel" ID1 ID2 ID3
```

---

### Reactions

#### reaction add

Add emoji reactions.

```bash
dure reaction add <CHANNEL> <MESSAGE_ID> <EMOJI>
```

**Example:**
```bash
dure reaction add "#channel" MESSAGE_ID "👍"
```

#### reaction remove

Remove reactions.

```bash
dure reaction remove <CHANNEL> <MESSAGE_ID> <EMOJI>
```

**Example:**
```bash
dure reaction remove "#channel" MESSAGE_ID "👍"
```

#### reaction list

View all reactions on a message.

```bash
dure reaction list <CHANNEL> <MESSAGE_ID>
```

**Example:**
```bash
dure reaction list "#channel" MESSAGE_ID
```

#### reaction users

See who reacted with specific emoji.

```bash
dure reaction users <CHANNEL> <MESSAGE_ID> <EMOJI>
```

**Example:**
```bash
dure reaction users "#channel" MESSAGE_ID "👍"
```

---

### Direct Messages

#### dm send

Send private messages.

```bash
dure dm send <USER> <MESSAGE>
```

**Example:**
```bash
dure dm send "@user" "message"
```

#### dm list

View conversation history.

```bash
dure dm list <USER>
```

**Example:**
```bash
dure dm list "@user"
```

---

### Threads

#### thread create

Create a thread from a message.

```bash
dure thread create <CHANNEL> <MESSAGE_ID> <THREAD_NAME>
```

**Example:**
```bash
dure thread create "#general" 1234567890123456 "Discussion Thread"
```

#### thread list

List active threads in a channel.

```bash
dure thread list <CHANNEL>
```

**Example:**
```bash
dure thread list "#general"
```

#### thread send

Send a message to a thread.

```bash
dure thread send <THREAD_ID> <MESSAGE> [OPTIONS]
```

**Options:**
| Option | Description |
|--------|-------------|
| `--file <PATH>` | Attach file |

**Examples:**
```bash
dure thread send 9876543210 "Replying in the thread"
dure thread send 9876543210 "Here's the file" --file ./data.json
```

#### thread archive

Archive a thread.

```bash
dure thread archive <THREAD_ID>
```

**Example:**
```bash
dure thread archive 9876543210
```

#### thread unarchive

Unarchive a thread.

```bash
dure thread unarchive <THREAD_ID>
```

**Example:**
```bash
dure thread unarchive 9876543210
```

#### thread rename

Rename a thread.

```bash
dure thread rename <THREAD_ID> <NEW_NAME>
```

**Example:**
```bash
dure thread rename 9876543210 "New Thread Name"
```

#### thread add-member

Add a member to a thread.

```bash
dure thread add-member <THREAD_ID> <MEMBER>
```

**Example:**
```bash
dure thread add-member 9876543210 "@alice"
```

#### thread remove-member

Remove a member from a thread.

```bash
dure thread remove-member <THREAD_ID> <MEMBER>
```

**Example:**
```bash
dure thread remove-member 9876543210 "@alice"
```

---

### Polls

#### poll create

Create a poll (via serve mode action).

See the Serve Mode guide for poll_send details.

#### poll results

View poll results.

```bash
dure poll results <CHANNEL> <MESSAGE_ID>
```

**Example:**
```bash
dure poll results "#general" 1234567890123456
```

#### poll end

End a poll early.

```bash
dure poll end <CHANNEL> <MESSAGE_ID>
```

**Example:**
```bash
dure poll end "#general" 1234567890123456
```

---

## Store Operations

### Product Management

#### product create

Create a product.

```bash
dure product create <SERVER> <NAME> <CATEGORY> <IMAGE> <CONTENTS>
```

**Example:**
```bash
dure product create "Server" "Product Name" "Product Category" "Image" "Contents"
```

#### product list

List active products.

```bash
dure product list <SERVER>
```

**Example:**
```bash
dure product list "Server"
```

#### product delete

Delete a product.

```bash
dure product delete <SERVER> <PRODUCT_ID>
```

**Example:**
```bash
dure product delete "Server" 9876543210
```

#### product modify

Modify a product.

```bash
dure product modify <SERVER> <PRODUCT_ID> <NAME> <CATEGORY> <IMAGE> <CONTENTS>
```

**Example:**
```bash
dure product modify "Server" 9876543210 "Product Name" "Product Category" "Image" "Contents"
```

---

### Order Management

#### order create

Create an order from known store.

```bash
dure order create <SERVER> <PRODUCT_IDS> <COUNTS>
```

**Example:**
```bash
dure order create "Server" "Product Ids" "Counts"
```

#### order list

List active orders.

```bash
dure order list <SERVER>
```

**Example:**
```bash
dure order list "Server"
```

---

### Payment Operations

#### payment create

Create a payment.

```bash
dure payment create <SERVER> <ORDER_ID> <PAYMENT_METHOD>
```

**Example:**
```bash
dure payment create "Server" 1234567890123456 "Payment Method"
```

#### payment list

List active payments.

```bash
dure payment list <SERVER>
```

**Example:**
```bash
dure payment list "Server"
```

---

### Review Management

#### review create

Post review to server.

```bash
dure review create <SERVER> <ORDER_ID> <CONTENTS>
```

**Example:**
```bash
dure review create "Server" Order_ID "Review Contents"
```

#### review list

List reviews.

```bash
dure review list <SERVER>
```

**Example:**
```bash
dure review list "Server"
```

---

## Configuration

### config list

Show all configuration.

```bash
dure config list
```

### config get

Get specific value.

```bash
dure config get <KEY>
```

**Example:**
```bash
dure config get device.name
```

### config set

Set value.

```bash
dure config set <KEY>=<VALUE>
```

**Example:**
```bash
dure config set device.name=mylaptop
```

### config edit

Open in editor.

```bash
dure config edit
```

---

## Exit Codes

| Code | Category | Description |
|------|----------|-------------|
| 0 | Success | Command completed |
| 1 | Internal | Unexpected error |
| 2 | Database | Not initialized, locked |
| 3 | Entity | Not found, ambiguous ID |
| 4 | Validation | Invalid input |
| 5 | Communication | WebSocket/API error |
| 6 | Network | Connection failure |
| 7 | Config | Missing configuration |
| 8 | I/O | File system error |

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DURE_DB` / `DURE_DATABASE` | - | Override database path |
| `RUST_LOG` | `error` | Logging level (debug, info, warn, error) |

**Recommended default for CLI use:**

```bash
export RUST_LOG=error
```

This keeps successful commands readable by suppressing low-level dependency logging.

---

## JSON Output Schemas

All commands support `--json` flag for structured output:

```bash
dure --json hosting list
dure --json product list "Server"
dure --json message list "#channel"
```

JSON output structure:

```json
{
  "success": true,
  "data": { ... },
  "timestamp": "2026-04-02T12:00:00Z"
}
```

Error output:

```json
{
  "success": false,
  "error": {
    "code": 3,
    "kind": "not_found",
    "message": "Product not found: prod-xyz999",
    "recovery_hints": [
      "Check the product ID spelling",
      "Use 'dure product list' to find valid IDs"
    ]
  }
}
```

---

## See Also

- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Commands, patterns, and common tasks
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [../README.md](../README.md) - Project overview
- [../CLAUDE.md](../CLAUDE.md) - Complete project guide

---

*Updated: 2026-04-02*
