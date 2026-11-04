---
id: operation-approvals
title: "kobana billing operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana billing operation-approvals


Commands of `kobana billing operation-approvals`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation_approvals` | List your operation approvals |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation_approvals/{id}` | Get an operation approval |

## operation-approvals list

List your operation approvals

```text
kobana billing operation-approvals list [OPTIONS]
```

`GET /v1/operation_approvals`

The approval-link approvals of the person the token belongs to, in the token’s organization, most recent first: each destructive MCP tool that waited for confirmation on the approval screen, with what happened next. Any token with a person reads its own approvals, with no resource scope; a token without a person gets an empty list. What each operation returned (`result`, `error`, an `amount` read from the record) comes back only when the token can read that approval’s resource and the person still holds a read permission on it; otherwise it is `null` with `result_hidden: true`. Read-only: nothing here approves, cancels or reopens an approval. `title` comes in the `Accept-Language` language (pt-BR by default).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `status` | `string` | no | Only approvals in this status. `pending` leaves out a lapsed one, which is `expired`. Values: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | no | Only approvals requested at or after this instant (ISO 8601). |
| `created_before` | `string` | no | Only approvals requested at or before this instant (ISO 8601). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of approvals. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## operation-approvals get

Get an operation approval

```text
kobana billing operation-approvals get [ID] [OPTIONS]
```

`GET /v1/operation_approvals/{id}`

One approval of the person the token belongs to, in the token’s organization — the `approval.id` the MCP tool answered with `approval_required`. Another person’s approval, another organization’s or a nonexistent id answer the same 404. Never returns the link, the link’s token, the arguments or the screen snapshot.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | The approval. |
| `404` | Resource not found in the current organization. |
