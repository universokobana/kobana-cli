---
id: proposal-templates
title: "kobana billing proposal-templates"
sidebar_label: "proposal-templates"
---

# kobana billing proposal-templates


Commands of `kobana billing proposal-templates`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`proposal-templates list`](#proposal-templates-list) | `GET /v1/proposal_templates` | List proposal templates |
| [`proposal-templates create`](#proposal-templates-create) | `POST /v1/proposal_templates` | Create proposal template |
| [`proposal-templates update`](#proposal-templates-update) | `PUT /v1/proposal_templates` | Upsert proposal template |
| [`proposal-templates get`](#proposal-templates-get) | `GET /v1/proposal_templates/{id}` | Get proposal template |
| [`proposal-templates delete`](#proposal-templates-delete) | `DELETE /v1/proposal_templates/{id}` | Delete proposal template |
| [`proposal-templates activate`](#proposal-templates-activate) | `POST /v1/proposal_templates/{id}/activate` | Activate proposal template |
| [`proposal-templates deactivate`](#proposal-templates-deactivate) | `POST /v1/proposal_templates/{id}/deactivate` | Deactivate proposal template |
| [`proposal-templates duplicate`](#proposal-templates-duplicate) | `POST /v1/proposal_templates/{id}/duplicate` | Duplicate proposal template |

## proposal-templates list

List proposal templates

```text
kobana billing proposal-templates list [OPTIONS]
```

`GET /v1/proposal_templates`

Returns a paginated list of the current organization's proposal templates.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `search` | `string` | no | Text search by name or description. |
| `is_active` | `boolean` | no | Filter by active (`true`) or deactivated (`false`) templates. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of proposal templates. |
| `404` | Resource not found in the current organization. |

## proposal-templates create

Create proposal template

```text
kobana billing proposal-templates create [OPTIONS]
```

`POST /v1/proposal_templates`

Creates a proposal template linked to a Google Doc or Google Slides. The `google_doc_id` must be unique per organization.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Template name (unique per organization). |
| `google_doc_id` | `string` | yes | Google Doc/Slides ID used as the template (unique per organization). |
| `description` | `string` | no | Optional template description. |
| `document_type` | `string` | no | Base document type: `google_doc` or `google_slides`. Default: `google_doc`. Values: `google_doc`, `google_slides`. |
| `app_script_url` | `string (uri)` | no | URL of the published Apps Script that renders the template, when applicable. |
| `google_drive_folder_id` | `string` | no | Google Drive folder ID where generated copies will be saved. |
| `is_active` | `boolean` | no | When `false`, creates the template already deactivated. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Free-form metadata defined by the client for internal correlation. |

More fields and nested objects: `kobana schema billing.proposal-templates.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Proposal template created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposal-templates update

Upsert proposal template

```text
kobana billing proposal-templates update [OPTIONS]
```

`PUT /v1/proposal_templates`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Template name (unique per organization). |
| `description` | `string` | no | Optional template description. |
| `document_type` | `string` | no | Base document type: `google_doc` or `google_slides`. Default: `google_doc`. Values: `google_doc`, `google_slides`. |
| `google_doc_id` | `string` | no | Google Doc/Slides ID used as the template (unique per organization). |
| `app_script_url` | `string (uri)` | no | URL of the published Apps Script that renders the template, when applicable. |
| `google_drive_folder_id` | `string` | no | Google Drive folder ID where generated copies will be saved. |
| `is_active` | `boolean` | no | When `false`, creates the template already deactivated. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Free-form metadata defined by the client for internal correlation. |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.proposal-templates.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposal-templates get

Get proposal template

```text
kobana billing proposal-templates get [ID] [OPTIONS]
```

`GET /v1/proposal_templates/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal template detail. |
| `404` | Resource not found in the current organization. |

## proposal-templates delete

Delete proposal template

```text
kobana billing proposal-templates delete [ID] [OPTIONS]
```

`DELETE /v1/proposal_templates/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal template removed. |
| `404` | Resource not found in the current organization. |

## proposal-templates activate

Activate proposal template

```text
kobana billing proposal-templates activate [ID] [OPTIONS]
```

`POST /v1/proposal_templates/{id}/activate`

Activates a previously deactivated proposal template, making it available for use in new proposals.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal template activated. |
| `404` | Resource not found in the current organization. |

## proposal-templates deactivate

Deactivate proposal template

```text
kobana billing proposal-templates deactivate [ID] [OPTIONS]
```

`POST /v1/proposal_templates/{id}/deactivate`

Deactivates a proposal template. Deactivated templates cannot be used in new proposals, but proposals already created remain unchanged.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal template deactivated. |
| `404` | Resource not found in the current organization. |

## proposal-templates duplicate

Duplicate proposal template

```text
kobana billing proposal-templates duplicate [ID] [OPTIONS]
```

`POST /v1/proposal_templates/{id}/duplicate`

Creates a copy of the given template, keeping configuration and metadata. The new template gets a derived name.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `201` | Proposal template duplicated. |
| `404` | Resource not found in the current organization. |
