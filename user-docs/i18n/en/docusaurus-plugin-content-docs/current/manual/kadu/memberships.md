---
id: memberships
title: "kobana kadu memberships"
sidebar_label: "memberships"
description: "List memberships"
---

# kobana kadu memberships

List memberships

Commands of `kobana kadu memberships`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`memberships`](#memberships) | `GET /v1/memberships` | List memberships |

## memberships

List memberships

```text
kobana kadu memberships [OPTIONS]
```

`GET /v1/memberships`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Field in snake_case: created_at (default), updated_at or a filterable field. |
| `sort_order` | `string` | no | Default: desc. Values: `asc`, `desc`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
