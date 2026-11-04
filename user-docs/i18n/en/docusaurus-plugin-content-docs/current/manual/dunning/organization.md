---
id: organization
title: "kobana dunning organization"
sidebar_label: "organization"
description: "Get the organization"
---

# kobana dunning organization

Get the organization

Commands of `kobana dunning organization`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Get the organization |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Update the organization |

## organization

Get the organization

```text
kobana dunning organization [OPTIONS] [COMMAND]
```

`GET /v1/organization`

Returns the token’s organization. The organization comes from the credential alone: there is no list and no `{id}`, and no header, query or body picks another one. Organization-level: it does not take the `X-Workspace-Id` header.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The token’s organization. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## organization update

Update the organization

```text
kobana dunning organization update [OPTIONS]
```

`PATCH /v1/organization`

Updates the profile of the token’s organization: trade name, website, time zone and `custom_metadata`. Name, email, phone, document and ids mirror the Kobana account and do not change here (a field sent is ignored); commercial fields (`bypass_subscription`, `courtesy`…) answer `403`. The profile applies to every workspace, so it takes an organization owner or access to every workspace. It does not take the `X-Workspace-Id` header.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `trade_name` | `string` | no | Trade name; `null` clears it. |
| `website` | `string (uri)` | no | Website (URL); `null` clears it. |
| `timezone` | `string` | no | IANA time zone name (e.g. `America/Sao_Paulo`). Offsets and abbreviations are refused. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object). `null` keeps the stored value. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.organization.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The updated organization. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
