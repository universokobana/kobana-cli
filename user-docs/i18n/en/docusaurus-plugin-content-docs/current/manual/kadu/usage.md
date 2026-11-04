---
id: usage
title: "kobana kadu usage"
sidebar_label: "usage"
description: "Usage and cost of executions, conversations and memory consolidation"
---

# kobana kadu usage

Usage and cost of executions, conversations and memory consolidation

Commands of `kobana kadu usage`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`usage`](#usage) | `GET /v1/usage` | Usage and cost of executions, conversations and memory consolidation |
| [`usage performance`](#usage-performance) | `GET /v1/usage/performance` | Performance: duration, success and failure, memory, files and volumes |

## usage

Usage and cost of executions, conversations and memory consolidation

```text
kobana kadu usage [OPTIONS] [COMMAND]
```

`GET /v1/usage`

Aggregated per day (in the organization's time zone) × group, adding up executions, Slack bot conversation turns (source `slack_chat`, no workspace/activity/Task) and memory consolidation (source `memory_consolidation`, in the workspace/activity/Task of the originating execution). `by_kind` separates the three: what is conversation and what is not. Default: last 30 days; maximum range of 366 days. `group` is the id (tier, workspace, activity, task) or the source; null = no value.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `from` | `string` | no |  |
| `to` | `string` | no |  |
| `group_by` | `string` | no | Values: `tier`, `workspace`, `activity`, `task`, `source`. |
| `workspace_id` | `string` | no |  |
| `activity_id` | `string` | no |  |
| `task_id` | `string` | no |  |
| `source` | `string` | no |  |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## usage performance

Performance: duration, success and failure, memory, files and volumes

```text
kobana kadu usage performance [OPTIONS]
```

`GET /v1/usage/performance`

Daily series (in the organization's time zone) with the average execution duration (total, bootstrap and effective run), finished executions by result, the memories existing at the end of the day (count and bytes of content + summary), the accumulated size of input and output files, filterable by extension (`file_type`), and that of the volumes (`volume_id`). Default: last 30 days; maximum range of 366 days. Memories have no task or source: only workspace and activity filter the memory. Requires Analytics › Performance to be enabled.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `from` | `string` | no |  |
| `to` | `string` | no |  |
| `workspace_id` | `string` | no |  |
| `activity_id` | `string` | no |  |
| `task_id` | `string` | no |  |
| `source` | `string` | no |  |
| `file_type` | `string` | no | Extension of the files in the files chart (e.g. `pdf`); `__none__` = no extension. Omitted = all. |
| `volume_id` | `string` | no | Only this volume in the volume series. An id outside the organization's volumes = all. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
