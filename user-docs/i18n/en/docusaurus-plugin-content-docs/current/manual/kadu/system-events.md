---
id: system-events
title: "kobana kadu system-events"
sidebar_label: "system-events"
description: "System events (100 most recent)"
---

# kobana kadu system-events

System events (100 most recent)

Commands of `kobana kadu system-events`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`system-events`](#system-events) | `GET /v1/system_events` | System events (100 most recent) |

## system-events

System events (100 most recent)

```text
kobana kadu system-events [OPTIONS]
```

`GET /v1/system_events`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `slug` | `string` | no |  |
| `workspace_id` | `string` | no |  |
| `task_id` | `string` | no |  |
| `execution_id` | `string` | no |  |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — invalid filter |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
