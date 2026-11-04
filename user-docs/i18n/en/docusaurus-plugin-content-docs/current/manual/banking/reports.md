---
id: reports
title: "kobana banking reports"
sidebar_label: "reports"
---

# kobana banking reports


Commands of `kobana banking reports`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`reports bank-billets`](#reports-bank-billets) | `GET /v1/reports/bank_billets` | Billet Count. |

## reports bank-billets

Billet Count.

```text
kobana banking reports bank-billets [OPTIONS]
```

`GET /v1/reports/bank_billets`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `expire_from` | `string` | no | From Due Date. |
| `expire_to` | `string` | no | To Due Date. |
| `paid_from` | `string` | no | From Payment Date. |
| `paid_to` | `string` | no | To Payment Date. |
| `registered_from` | `string` | no | From Registration Date. |
| `registered_to` | `string` | no | To Registration Date. |
| `created_from` | `string` | no | From Creation Date. |
| `created_to` | `string` | no | To Creation Date. |
| `status` | `string` | no | Billet status. Values: `generating`, `opened`, `paid`, `overdue`, `canceled`, `draft`, `generation_failed`, `validation_failed`, `blocked`, `chargeback`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing billets |
