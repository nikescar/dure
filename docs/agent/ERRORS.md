# Errors

Most commands return non-zero exit codes on failure and may emit a structured error envelope.

Example (captured with stderr redirection):

```bash
dure product list "NonExistentStore" --json > /dev/null 2>err.json || true
cat err.json | jq .
```

Shape:

```json
{
  "error": {
    "code": "SITE_NOT_FOUND",
    "message": "Site not found: NonExistentStore",
    "hint": "Run 'dure site list' to see available sites.",
    "retryable": false,
    "context": { "searched_site": "NonExistentStore" }
  }
}
```

Machine-readable schema:

```bash
dure schema error --format json
```
