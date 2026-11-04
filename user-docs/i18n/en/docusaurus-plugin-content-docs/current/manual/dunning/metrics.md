---
id: metrics
title: "kobana dunning metrics"
sidebar_label: "metrics"
---

# kobana dunning metrics


Commands of `kobana dunning metrics`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`metrics recovery`](#metrics-recovery) | `GET /v1/metrics/recovery` | Recovery metrics |

## metrics recovery

Recovery metrics

```text
kobana dunning metrics recovery [OPTIONS]
```

`GET /v1/metrics/recovery`

Returns the period’s recovery metrics (default: last 6 months, counted from the first day of the starting month).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `months` | `integer` | no | Window in months (1–24; default 6). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Computed metrics. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
