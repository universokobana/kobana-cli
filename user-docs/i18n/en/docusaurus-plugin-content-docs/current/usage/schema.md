---
id: schema
title: Schema
sidebar_label: Schema
sidebar_position: 5
description: Look up parameters, body and responses of any endpoint without leaving the terminal.
---

# Schema

`kobana schema` shows, as JSON, everything the specification says about an endpoint: HTTP method and path, path, query and header parameters, body and responses.

```bash
# All products and resources
kobana schema --list

# One endpoint — the path has the shape of the command
kobana schema banking.charge.pix.create
kobana schema finance.payables.list
kobana schema kadu.tasks.run
```

The path is `<product>.<resource>.<method>`. For commands that run by name, the method can be omitted: `kobana schema banking.me`.

## What comes out

| Field | Content |
|---|---|
| `method`, `path` | HTTP verb and path, with the API version |
| `summary`, `description` | Text from the specification, in the CLI's language |
| `path_params` | Path identifiers, in the order of the arguments |
| `parameters` | Query parameters, with type, whether required, and accepted values |
| `header_params` | Headers, such as `X-Workspace-Id` |
| `request_body` | Body schema |
| `responses` | Response codes and their schemas |
| `deprecated` | `true` when the endpoint is retired |

## For agents

The schema is the shortest way for an agent to find out how to call an endpoint without reading the whole documentation. See [Scripts and AI agents](/usage/scripting). The same information, already formatted, is in the [Manual](/manual).
