---
id: syntax
title: Command syntax
sidebar_label: Syntax
sidebar_position: 1
description: How a command is built — product, resource, method — and how to pass parameters, body and files.
---

# Command syntax

```bash
kobana <product> <resource> <method> [arguments] [flags]
```

- **`<product>`**: `banking`, `finance`, `billing`, `inbox`, `dunning`, `dashboards` or `kadu` ([Products](/reference/products)).
- **`<resource>`**: can be nested — `bank-billets`, `charge pix`, `webhooks deliveries`.
- **`<method>`**: the action — `list`, `get`, `create`, `update`, `delete` or a resource action, such as `cancel`, `pause`, `replay`.

No command is written by hand: the CLI reads the OpenAPI specifications embedded in the binary and builds the tree on every run. That is why the [Manual](/manual) is generated from the binary itself.

## How names are chosen

| API route | Command |
|---|---|
| `GET /v1/bank_billets` | `kobana banking bank-billets list` |
| `GET /v1/bank_billets/{id}` | `kobana banking bank-billets get 123` |
| `POST /v2/charge/pix` | `kobana banking charge pix create` |
| `PUT /v1/bank_billets/{id}/cancel` | `kobana banking bank-billets cancel 123` |

- **The API version never appears in the command.** The CLI takes care of `/v1` and `/v2`.
- **Read-only resources run by name**: `kobana banking me`, `kobana billing organization`.
- **Actions whose name is already a verb** run without the HTTP verb: `kobana banking webhook-deliveries resend`.
- A `delete` is never implicit.
- Old names remain valid as aliases, outside the help.

## Path identifiers

Route ids are passed as **arguments**, in the order they appear:

```bash
kobana banking bank-billets get 123
kobana banking charge pix commands get PIX_UID COMMAND_UID
```

`--params '{"id": "123"}'` still works; when both appear, the argument wins.

## Parameters and body

URL, query and header parameters go in `--params`; the request body goes in `--json`:

```bash
kobana billing subscriptions list --params '{"status": "active", "per_page": 50}'
kobana banking charge pix create --json '{"amount": 99.90, "pix_account_uid": "UID"}'
```

## Files

`--file field=path`, repeatable, sends the request as `multipart/form-data`. Each `--json` field becomes a text part; the file type comes from the extension (`.eml` goes as `message/rfc822`). Only endpoints that accept a form take `--file`.

```bash
kobana inbox emails create --file file=./mensagem.eml --json '{"inbox_id": "INBOX_ID"}'
kobana finance attachments create --file file=./nota.pdf --json '{"title": "NF 123"}'
```

## Help at any level

```bash
kobana --help
kobana finance --help
kobana finance payables --help
kobana finance payables list --help
```

See all the flags in [Global options](/manual/commands/global-options).
