---
id: global-options
title: "Global options"
sidebar_label: "Global options"
sidebar_position: 1
---

# Global options

They apply to every command, before or after the resource name.

| Options | Description |
|---|---|
| `--params <JSON>` | Query/URL parameters as JSON |
| `--json <JSON>` | Request body as JSON |
| `--file <FIELD=PATH>` | File sent as a part of a multipart form, field=path (repeatable; --json fields become text parts) |
| `--fields <FIELDS>` | Comma-separated list of fields to include in response |
| `--dry-run` | Show the request without executing |
| `--env <ENV>` | API environment: production, sandbox or development (each product has its own hosts). Can also be set via KOBANA_ENVIRONMENT. |
| `--verbose` | Show request/response details on stderr |
| `--output <PATH>` | Save response to file |
| `--output-format <FORMAT>` | Output format: json (default), table, csv |
| `--idempotency-key <KEY>` | Custom idempotency key for mutations |
| `--page-all` | Auto-paginate and output NDJSON |
| `--page-limit <N>` | Maximum pages to fetch (default: 10) |
| `--page-delay <MS>` | Delay between pages in ms (default: 100) |
| `--jq <EXPRESSION>` | Filter the JSON output with a jq expression, e.g. '.data[].id' or, with --page-all (one item at a time), '.id'; no jq install needed |
| `--no-color` | Turn colors off (also: NO_COLOR or CLICOLOR=0); see `kobana accessibility` |
| `--lang <LANG>` | Language of the messages: pt, en or es (default: KOBANA_LANG or the system's) |
| `-h, --help` | Print help |
| `-V, --version` | Print version |
