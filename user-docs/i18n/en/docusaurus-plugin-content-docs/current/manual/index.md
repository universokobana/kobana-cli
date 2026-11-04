---
id: index
title: Manual
sidebar_label: Overview
sidebar_position: 1
slug: /manual
description: All kobana commands, by product and resource, generated from the binary itself.
---

# Manual

The manual lists **all the commands** of `kobana`: what each one does, the HTTP method and path, the parameters, the request body and the responses. It is generated from the binary itself — `--help`, `kobana schema` and the embedded OpenAPI specifications — so it follows every version and every language.

```bash
kobana <product> <resource> <method> [flags]
```

## CLI commands

| Command | What for |
|---|---|
| [Global options](/manual/commands/global-options) | The flags every command accepts: `--params`, `--json`, `--fields`, `--dry-run`… |
| [`kobana schema`](/manual/commands/schema) | Parameters, body and responses of an endpoint |
| [`kobana completions`](/manual/commands/completions) | Completion for bash, zsh, fish, PowerShell and elvish |
| [`kobana update`](/manual/commands/update) | Checks for a new version and updates the CLI |

## Products

| Product | Command |
|---|---|
| Banking Gateway | [`kobana banking`](/manual/banking) |
| Smart Finance | [`kobana finance`](/manual/finance) |
| Automatic Billing | [`kobana billing`](/manual/billing) |
| Autonomous Inbox | [`kobana inbox`](/manual/inbox) |
| Dunning | [`kobana dunning`](/manual/dunning) |
| Magic Dashboard | [`kobana dashboards`](/manual/dashboards) |
| Kadu Agent | [`kobana kadu`](/manual/kadu) |

Each product has one page per resource, with one section per command.

## How to read a page

- **The `text` block** is the usage line, as `--help` prints it. `[ID]` are the path identifiers, in route order; `[OPTIONS]`, the [global options](/manual/commands/global-options).
- **`GET /v1/…`** is the call the command makes. The API version appears only there — never in the command.
- **Query parameters, headers and body** go in `--params` and `--json`. The *Required* column says what the API demands.
- **Request body** shows the first-level fields. For nested objects, run the `kobana schema` command indicated on the page.

## Errors common to all commands

All endpoints can respond with these codes, which the manual does not repeat on each command:

| HTTP | Meaning |
|---|---|
| `401` | Token missing, invalid or expired — see [Authentication](/getting-started/authentication) |
| `403` | Valid token, but without permission (or without the required [access level](/getting-started/authentication#login-in-the-browser-oauth--pkce)) for the operation |
| `429` | Request limit exceeded |
| `500` | Unexpected server error |

In the CLI, a 4xx or 5xx response ends with [exit code](/reference/exit-codes) `1`.
