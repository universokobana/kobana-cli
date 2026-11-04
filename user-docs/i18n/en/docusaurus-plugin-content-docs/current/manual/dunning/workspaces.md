---
id: workspaces
title: "kobana dunning workspaces"
sidebar_label: "workspaces"
description: "List workspaces"
---

# kobana dunning workspaces

List workspaces

Commands of `kobana dunning workspaces`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`workspaces`](#workspaces) | `GET /v1/workspaces` | List workspaces |

## workspaces

List workspaces

```text
kobana dunning workspaces [OPTIONS]
```

`GET /v1/workspaces`

Lists the workspaces the token reaches — every workspace of the organization for an integration token. Send the `id` (or `public_id`) in the `X-Workspace-Id` header of every other call; it is optional while the organization has a single workspace. This route does not take the header.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Reachable workspaces. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
