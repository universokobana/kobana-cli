---
id: entities
title: "kobana dashboards entities"
sidebar_label: "entities"
---

# kobana dashboards entities


Commands of `kobana dashboards entities`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`entities join preview`](#entities-join-preview) | `POST /v1/entities/join/preview` | Preview a relation |
| [`entities join create`](#entities-join-create) | `POST /v1/entities/join` | Create join entity |
| [`entities join update`](#entities-join-update) | `PUT /v1/entities/{id}/join` | Update relation |
| [`entities logs get`](#entities-logs-get) | `GET /v1/entities/{id}/logs/{log_id}` | Get sync call |
| [`entities logs list`](#entities-logs-list) | `GET /v1/entities/{id}/logs` | Sync log |
| [`entities records list`](#entities-records-list) | `GET /v1/entities/{id}/records` | Query records |
| [`entities records create`](#entities-records-create) | `POST /v1/entities/{id}/records` | Send records |
| [`entities records delete`](#entities-records-delete) | `DELETE /v1/entities/{id}/records` | Clear records |
| [`entities records import`](#entities-records-import) | `POST /v1/entities/{id}/records/import` | Import records from CSV |
| [`entities records get`](#entities-records-get) | `GET /v1/entities/{id}/records/{record_id}` | Get record |
| [`entities sync`](#entities-sync) | `GET /v1/entities/{id}/sync` | Sync state |
| [`entities sync create`](#entities-sync-create) | `POST /v1/entities/{id}/sync` | Sync entity |
| [`entities tenant-syncs list`](#entities-tenant-syncs-list) | `GET /v1/entities/{id}/tenant_syncs` | Tenant syncs |
| [`entities tenant-syncs create`](#entities-tenant-syncs-create) | `POST /v1/entities/{id}/tenant_syncs` | Sync one tenant |
| [`entities list`](#entities-list) | `GET /v1/entities` | List entities |
| [`entities create`](#entities-create) | `POST /v1/entities` | Create entity |
| [`entities preview`](#entities-preview) | `POST /v1/entities/preview` | Preview a source |
| [`entities get`](#entities-get) | `GET /v1/entities/{id}` | Get entity |
| [`entities update`](#entities-update) | `PATCH /v1/entities/{id}` | Update entity |
| [`entities delete`](#entities-delete) | `DELETE /v1/entities/{id}` | Delete entity |
| [`entities aggregate`](#entities-aggregate) | `GET /v1/entities/{id}/aggregate` | Aggregate records |
| [`entities usage`](#entities-usage) | `GET /v1/entities/{id}/usage` | Entity usage |

## entities join preview

Preview a relation

```text
kobana dashboards entities join preview [OPTIONS]
```

`POST /v1/entities/join/preview`

How two entities match on the given fields (counts) and the first combined rows, before the relation is saved. Reads stored records only.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `base_entity_id` | `string (uuid)` | yes | Base entity: every one of its records enters the relation. |
| `related_entity_id` | `string (uuid)` | yes | Related entity, matched with the base on the given fields. |
| `base_field` | `string` | yes | Key of the base entity field used to match. |
| `related_field` | `string` | yes | Key of the related entity field used to match. |
| `type` | `string` | no | `left` (default) keeps base records without a match; `inner` only the matching ones. Values: `inner`, `left`. |
| `match` | `string` | no | `exact` (default) compares the text; `digits` compares only the digits (a CPF with and without punctuation match). Values: `exact`, `digits`. |

More fields and nested objects: `kobana schema dashboards.entities.join.preview`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Relation preview. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## entities join create

Create join entity

```text
kobana dashboards entities join create [OPTIONS]
```

`POST /v1/entities/join`

Registers an entity that relates two others (a join) with the fields picked from each, and queues its first sync.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | yes | Machine name the portal blocks use; unique in the workspace (up to 63 characters). |
| `name` | `string` | yes |  |
| `name_plural` | `string` | yes |  |
| `join` | `object` | yes |  |
| `fields` | `array<object>` | yes | The fields brought from each side: `{ from: base\|related, field_key, key?, name? }`. |
| `description` | `string` | no |  |
| `label_field_key` | `string` | no | Label field; absent = the copy of the base entity's label, `null` = none. |
| `owner_field_key` | `string` | no | Owner field; absent = the copy of the base entity's owner, `null` = none. |

More fields and nested objects: `kobana schema dashboards.entities.join.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Entity created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | Conflict: key already used in the workspace, or the entity is read by a relation. |
| `422` | Request body failed validation. |

## entities join update

Update relation

```text
kobana dashboards entities join update [ID] [OPTIONS]
```

`PUT /v1/entities/{id}/join`

Replaces the relation of a join entity and the fields it brings, then re-reads its records. 422 when the entity is not a join.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | yes | Machine name the portal blocks use; unique in the workspace (up to 63 characters). |
| `name` | `string` | yes |  |
| `name_plural` | `string` | yes |  |
| `join` | `object` | yes |  |
| `fields` | `array<object>` | yes | The fields brought from each side: `{ from: base\|related, field_key, key?, name? }`. |
| `description` | `string` | no |  |
| `label_field_key` | `string` | no | Label field; absent = the copy of the base entity's label, `null` = none. |
| `owner_field_key` | `string` | no | Owner field; absent = the copy of the base entity's owner, `null` = none. |

More fields and nested objects: `kobana schema dashboards.entities.join.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Entity updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | Conflict: key already used in the workspace, or the entity is read by a relation. |
| `422` | Request body failed validation. |

## entities logs get

Get sync call

```text
kobana dashboards entities logs get [ID] [LOG_ID] [OPTIONS]
```

`GET /v1/entities/{id}/logs/{log_id}`

One call of the entity's sync, whole (secrets masked).

**Path arguments:** `id`, `log_id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The call. |
| `404` | Resource not found in the current organization. |

## entities logs list

Sync log

```text
kobana dashboards entities logs list [ID] [OPTIONS]
```

`GET /v1/entities/{id}/logs`

The calls the entity's sync made to its connector, newest first: the connector's request log narrowed to the entity.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `method` | `string` | no | HTTP method of the call, such as `GET`. |
| `success` | `string` | no | Only successful (`true`) or failed (`false`) calls. Values: `true`, `false`. |
| `search` | `string` | no | Part of the endpoint called. |
| `date_from` | `string` | no | First day (YYYY-MM-DD). |
| `date_to` | `string` | no | Last day, inclusive (YYYY-MM-DD). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Sync calls. |
| `404` | Resource not found in the current organization. |

## entities records list

Query records

```text
kobana dashboards entities records list [ID] [OPTIONS]
```

`GET /v1/entities/{id}/records`

The synced records of the entity, with field filters, sort and `limit`/`offset` paging (the same query the portal blocks run).

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no | Records per call (default 50, max 200). |
| `offset` | `integer,null` | no | How many records to skip. |
| `sort` | `string` | no | Field keys with an optional direction, comma-separated (`field:desc,other`); `asc` by default. |
| `filter` | `string` | no | JSON object of field conditions: `{"field": value}` or `{"field": {"gte": 10}}`. Operators: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `contains`, `is_null`. |
| `preview_as` | `string` | no | A portal user ID: only that user's records, as the portal shows them (on an `on_access` entity, syncs that tenant first). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Entity records. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## entities records create

Send records

```text
kobana dashboards entities records create [ID] [OPTIONS]
```

`POST /v1/entities/{id}/records`

Sends records to an entity with no connection (`source_kind: "push"`): each is stored by its identifier — sent again, it is updated; unchanged, it is not rewritten. The relations and portal users that depend on the entity follow. 422 on an entity that reads a connection.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `records` | `array<object>` | yes | The records (up to 1000 per call), each an object with the properties the fields read (`source_path`). Each must carry the identifier. |
| `mode` | `string` | no | `upsert` (default) keeps the records not sent; `replace` removes the ones the call does not carry — the batch becomes the whole set. Values: `upsert`, `replace`. |

More fields and nested objects: `kobana schema dashboards.entities.records.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | How many records were created, updated, kept and removed, and the ones refused. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## entities records delete

Clear records

```text
kobana dashboards entities records delete [ID] [OPTIONS]
```

`DELETE /v1/entities/{id}/records`

Removes every synced record of the entity and resets its sync state, so the next sync reads the source from scratch.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Records removed. |
| `404` | Resource not found in the current organization. |

## entities records import

Import records from CSV

```text
kobana dashboards entities records import [ID] [OPTIONS]
```

`POST /v1/entities/{id}/records/import`

Imports a CSV into an entity with no connection: one record per row, each column a field reads converted to its type (numbers, dates, booleans). Rows without the identifier are refused and listed by row number (row 1 is the header).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `content` | `string` | no | The CSV text: header row with the column names (the fields' `source_path`), `,` or `;` as delimiter, up to 20,000 rows. Send this or `content_base64`. |
| `content_base64` | `string` | no | The CSV file in base64 (UTF-8). |
| `mode` | `string` | no | `upsert` (default) keeps the records not sent; `replace` removes the ones the call does not carry — the batch becomes the whole set. Values: `upsert`, `replace`. |

More fields and nested objects: `kobana schema dashboards.entities.records.import`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | How many records were created, updated, kept and removed, and the ones refused. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## entities records get

Get record

```text
kobana dashboards entities records get [ID] [RECORD_ID] [OPTIONS]
```

`GET /v1/entities/{id}/records/{record_id}`

One synced record — values by field and the record as the source sent it — with its entity.

**Path arguments:** `id`, `record_id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The record and its entity. |
| `404` | Resource not found in the current organization. |

## entities sync

Sync state

```text
kobana dashboards entities sync [ID] [OPTIONS] [COMMAND]
```

`GET /v1/entities/{id}/sync`

Where the entity's sync stands: status, last error, last syncs, record count and stats.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Sync state. |
| `404` | Resource not found in the current organization. |

## entities sync create

Sync entity

```text
kobana dashboards entities sync create [ID] [OPTIONS]
```

`POST /v1/entities/{id}/sync`

Queues a sync of the entity and answers 202 with its ID; follow it at `GET /entities/{id}/sync`. An `on_access` entity has no entity-wide sync (422): use `POST /entities/{id}/tenant_syncs`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `full` | `string` | no | `true` (or `1`) re-reads everything and drops what the source no longer has; without it the sync is incremental where the source allows. Values: `true`, `1`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `202` | Sync queued. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## entities tenant-syncs list

Tenant syncs

```text
kobana dashboards entities tenant-syncs list [ID] [OPTIONS]
```

`GET /v1/entities/{id}/tenant_syncs`

On an `on_access` entity: where each tenant's sync stands, most recently accessed first.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `status` | `string` | no | Status of the tenant's sync. Values: `queued`, `running`, `completed`, `error`. |
| `search` | `string` | no | Part of the tenant ID. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Tenant syncs. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## entities tenant-syncs create

Sync one tenant

```text
kobana dashboards entities tenant-syncs create [ID] [OPTIONS]
```

`POST /v1/entities/{id}/tenant_syncs`

Queues one tenant's sync of an `on_access` entity now. 404 when no portal user carries that tenant ID; 422 when the entity is not `on_access`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `tenant_id` | `string` | yes | Tenant ID (owner ID) to sync. In the body because it may hold any character. |
| `full` | `boolean` | no | When `true`, re-reads every record of the tenant. |

More fields and nested objects: `kobana schema dashboards.entities.tenant-syncs.create`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | Sync queued. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## entities list

List entities

```text
kobana dashboards entities list [OPTIONS]
```

`GET /v1/entities`

Every entity of the workspace, with fields, source and sync state, ordered by name. Not paginated.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Entities of the workspace. |
| `404` | Resource not found in the current organization. |

## entities create

Create entity

```text
kobana dashboards entities create [OPTIONS]
```

`POST /v1/entities`

Registers an entity from a read operation (or SQL query) of the data source with the chosen fields, and queues its first full sync.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | yes | Machine name the portal blocks use; unique in the workspace (up to 63 characters). |
| `name` | `string` | yes |  |
| `name_plural` | `string` | yes |  |
| `fields` | `array<object>` | yes | The fields the entity keeps, in order. On update, the whole list (matched by `key`). |
| `integration_id` | `string (uuid)` | no | Data source (connector) to read from. Omitted with `source_kind: "push"`. |
| `operation_id` | `string (uuid)` | no | Read operation (endpoint, tool or table) of the source. Send this field or `sql`, not both. |
| `sql` | `string` | no | SQL query on a SQL database source. Send this field or `operation_id`. |
| `params` | `object` | no | Parameters the operation is called with (default `{}`). |
| `pagination` | `object` | no |  |
| `sync_filters` | `array<object>` | no | What the sync sends to narrow the records (up to 20). Ignored on a SQL query. |
| `source_kind` | `string` | no | `push`: an entity with no connection — no `integration_id`, `operation_id` or `sql`. Each field's `source_path` defaults to the field key (the property each record sent carries) and `identifier_field_key` is required: a record sent again updates the stored one by it. Values: `push`. |
| `description` | `string` | no |  |
| `identifier_field_key` | `string` | no |  |
| `label_field_key` | `string` | no |  |
| `owner_field_key` | `string` | no | Field holding the owner ID, matched against the portal user's `tenant_id`. Without it the entity cannot be shown in the portal. |
| `owner_match` | `string` | no | How the owner field is compared with the `tenant_id`: as text (`external_id`) or as a CPF/CNPJ (`document_number`, digits only). Values: `external_id`, `document_number`. |
| `sync_mode` | `string` | no | When the sync runs; absent: `scheduled` on create, kept on update. Values: `scheduled`, `manual`, `on_access`. |
| `freshness_seconds` | `integer` | no | `on_access`: how many seconds a tenant's synced records count as fresh. |
| `records_path` | `string` | no | Path to the record list inside the response, when not detected. |
| `sample` | `array<object>` | no | The preview records, kept to re-infer fields later. |

More fields and nested objects: `kobana schema dashboards.entities.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Entity created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | Conflict: key already used in the workspace, or the entity is read by a relation. |
| `422` | Request body failed validation. |

## entities preview

Preview a source

```text
kobana dashboards entities preview [OPTIONS]
```

`POST /v1/entities/preview`

Calls the chosen read operation (or runs the SQL query) and returns the sample with the entity inferred from it. Nothing is stored; the call is kept in the connector's request log.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `integration_id` | `string (uuid)` | yes | Data source (connector) to read from. Omitted with `source_kind: "push"`. |
| `operation_id` | `string (uuid)` | no | Read operation (endpoint, tool or table) of the source. Send this field or `sql`, not both. |
| `sql` | `string` | no | SQL query on a SQL database source. Send this field or `operation_id`. |
| `params` | `object` | no | Parameters the operation is called with (default `{}`). |
| `pagination` | `object` | no |  |
| `sync_filters` | `array<object>` | no | What the sync sends to narrow the records (up to 20). Ignored on a SQL query. |
| `limit` | `integer` | no | Sample size (default 10, max 50). |

More fields and nested objects: `kobana schema dashboards.entities.preview`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Sample and inferred entity. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## entities get

Get entity

```text
kobana dashboards entities get [ID] [OPTIONS]
```

`GET /v1/entities/{id}`

One entity with its fields, source, sync mode, owner field and sync state.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The entity. |
| `404` | Resource not found in the current organization. |

## entities update

Update entity

```text
kobana dashboards entities update [ID] [OPTIONS]
```

`PATCH /v1/entities/{id}`

Name, key, special fields, the sync's pagination, filters and mode, and the field list (matched by key). When the mapping changes, the records are re-read.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | yes | Machine name the portal blocks use; unique in the workspace (up to 63 characters). |
| `name` | `string` | yes |  |
| `name_plural` | `string` | yes |  |
| `fields` | `array<object>` | yes | The fields the entity keeps, in order. On update, the whole list (matched by `key`). |
| `description` | `string` | no |  |
| `identifier_field_key` | `string` | no |  |
| `label_field_key` | `string` | no |  |
| `owner_field_key` | `string` | no | Field holding the owner ID, matched against the portal user's `tenant_id`. Without it the entity cannot be shown in the portal. |
| `owner_match` | `string` | no | How the owner field is compared with the `tenant_id`: as text (`external_id`) or as a CPF/CNPJ (`document_number`, digits only). Values: `external_id`, `document_number`. |
| `sync_mode` | `string` | no | When the sync runs; absent: `scheduled` on create, kept on update. Values: `scheduled`, `manual`, `on_access`. |
| `freshness_seconds` | `integer` | no | `on_access`: how many seconds a tenant's synced records count as fresh. |
| `pagination` | `object` | no |  |
| `sync_filters` | `array<object>` | no | Sync filters (up to 20); absent keeps the current ones. |

More fields and nested objects: `kobana schema dashboards.entities.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Entity updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | Conflict: key already used in the workspace, or the entity is read by a relation. |
| `422` | Request body failed validation. |

## entities delete

Delete entity

```text
kobana dashboards entities delete [ID] [OPTIONS]
```

`DELETE /v1/entities/{id}`

Soft-deletes the entity and its fields. Refused (409) while a join entity reads it.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Entity deleted. |
| `404` | Resource not found in the current organization. |
| `409` | Conflict: key already used in the workspace, or the entity is read by a relation. |

## entities aggregate

Aggregate records

```text
kobana dashboards entities aggregate [ID] [OPTIONS]
```

`GET /v1/entities/{id}/aggregate`

Count, sum, average, min or max over the synced records, optionally grouped by a field — what KPI and chart blocks show.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `op` | `string` | no | Aggregate: `count` (default), `sum`, `avg`, `min` or `max`. Values: `count`, `sum`, `avg`, `min`, `max`. |
| `field` | `string` | no | Key of the aggregated field (required except for `count`; numeric for `sum`/`avg`). |
| `group_by` | `string` | no | Key of the field to group by. |
| `filter` | `string` | no | JSON object of field conditions: `{"field": value}` or `{"field": {"gte": 10}}`. Operators: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `contains`, `is_null`. |
| `preview_as` | `string` | no | A portal user ID: only that user's records, as the portal shows them (on an `on_access` entity, syncs that tenant first). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Aggregate result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## entities usage

Entity usage

```text
kobana dashboards entities usage [ID] [OPTIONS]
```

`GET /v1/entities/{id}/usage`

The portal pages (draft and published) whose blocks read the entity — what breaks if it is removed or its key changes.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pages using the entity. |
| `404` | Resource not found in the current organization. |
