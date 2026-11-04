---
id: negotiation-campaigns
title: "kobana dunning negotiation-campaigns"
sidebar_label: "negotiation-campaigns"
---

# kobana dunning negotiation-campaigns


Commands of `kobana dunning negotiation-campaigns`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`negotiation-campaigns list`](#negotiation-campaigns-list) | `GET /v1/negotiation-campaigns` | List campaigns |
| [`negotiation-campaigns create`](#negotiation-campaigns-create) | `POST /v1/negotiation-campaigns` | Create campaign |
| [`negotiation-campaigns end`](#negotiation-campaigns-end) | `POST /v1/negotiation-campaigns/{id}/end` | End campaign |

## negotiation-campaigns list

List campaigns

```text
kobana dunning negotiation-campaigns list [OPTIONS]
```

`GET /v1/negotiation-campaigns`

Lists the organization’s negotiation campaigns (active and ended), newest first. Not paginated.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Organization campaigns (`campaigns` envelope). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## negotiation-campaigns create

Create campaign

```text
kobana dunning negotiation-campaigns create [OPTIONS]
```

`POST /v1/negotiation-campaigns`

Creates and activates a discount campaign. The debtor portal starts offering it to charges within the configured overdue range.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Campaign name. |
| `discount_percentage` | `number` | yes | Discount percentage (0-100). Returned as a decimal string. |
| `max_installments` | `integer` | no | Maximum number of installments allowed (1 = single payment). |
| `min_days_overdue` | `integer` | no | Start of the overdue-days range where the campaign applies (null = no minimum). |
| `max_days_overdue` | `integer` | no | End of the overdue-days range where the campaign applies (null = no maximum). |
| `valid_until` | `string (date)` | no | Campaign expiration date (null = no deadline). |
| `terms` | `string` | no | Terms shown to the debtor. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |

More fields and nested objects: `kobana schema dunning.negotiation-campaigns.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Campaign created and activated (`campaign` envelope). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## negotiation-campaigns end

End campaign

```text
kobana dunning negotiation-campaigns end [ID] [OPTIONS]
```

`POST /v1/negotiation-campaigns/{id}/end`

Ends the campaign (deactivates; `activated_at` becomes null). The record is kept and the portal stops offering it. A write (`write`), not a deletion — it replaced `DELETE /negotiation-campaigns/{id}`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Campaign ended. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
