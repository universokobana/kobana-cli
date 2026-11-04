---
id: banks
title: "kobana billing banks"
sidebar_label: "banks"
description: "List banks"
---

# kobana billing banks

List banks

Commands of `kobana billing banks`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`banks`](#banks) | `GET /v1/banks` | List banks |

## banks

List banks

```text
kobana billing banks [OPTIONS]
```

`GET /v1/banks`

Brazilian banks with a COMPE code (BACEN), used to register bank accounts. The list is the same for every organization and is refreshed weekly from BrasilAPI.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `search` | `string` | no | Search by part of the name or code. |
| `code` | `string` | no | Exact COMPE code (e.g. `341`). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of banks. |
| `404` | Resource not found in the current organization. |
