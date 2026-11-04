---
id: data-sources
title: "kobana dashboards data-sources"
sidebar_label: "data-sources"
---

# kobana dashboards data-sources


Commands of `kobana dashboards data-sources`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`data-sources catalog list`](#data-sources-catalog-list) | `GET /v1/data_sources/catalog` | List Kobana catalog products |
| [`data-sources catalog create`](#data-sources-catalog-create) | `POST /v1/data_sources/catalog` | Connect a Kobana product |
| [`data-sources logs get`](#data-sources-logs-get) | `GET /v1/data_sources/{id}/logs/{log_id}` | Get a request of a data source |
| [`data-sources logs list`](#data-sources-logs-list) | `GET /v1/data_sources/{id}/logs` | List the request log of a data source |
| [`data-sources openapi analyze`](#data-sources-openapi-analyze) | `POST /v1/data_sources/openapi/analyze` | Analyze an OpenAPI spec |
| [`data-sources operations update`](#data-sources-operations-update) | `PATCH /v1/data_sources/{id}/operations/{operation_id}` | Enable or disable an operation of a data source |
| [`data-sources list`](#data-sources-list) | `GET /v1/data_sources` | List data sources |
| [`data-sources create`](#data-sources-create) | `POST /v1/data_sources` | Create a data source |
| [`data-sources test`](#data-sources-test) | `POST /v1/data_sources/test` | Test a connection before saving |
| [`data-sources get`](#data-sources-get) | `GET /v1/data_sources/{id}` | Get a data source |
| [`data-sources update`](#data-sources-update) | `PATCH /v1/data_sources/{id}` | Update a data source |
| [`data-sources delete`](#data-sources-delete) | `DELETE /v1/data_sources/{id}` | Delete a data source |
| [`data-sources authorize`](#data-sources-authorize) | `POST /v1/data_sources/{id}/authorize` | Authorize an MCP data source |
| [`data-sources schedule`](#data-sources-schedule) | `PUT /v1/data_sources/{id}/schedule` | Schedule the sync of a data source |
| [`data-sources spec`](#data-sources-spec) | `PUT /v1/data_sources/{id}/spec` | Replace the spec of a data source |
| [`data-sources sync`](#data-sources-sync) | `POST /v1/data_sources/{id}/sync` | Sync a data source |

## data-sources catalog list

List Kobana catalog products

```text
kobana dashboards data-sources catalog list [OPTIONS]
```

`GET /v1/data_sources/catalog`

Kobana products that can be connected as a data source with just a token. Whole list, unpaginated.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Catalog products. |
| `404` | Resource not found in the current organization. |

## data-sources catalog create

Connect a Kobana product

```text
kobana dashboards data-sources catalog create [OPTIONS]
```

`POST /v1/data_sources/catalog`

Saves an OpenAPI source from the product spec shipped with the app and tests the connection.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `product` | `string` | yes | Kobana catalog product key (see `GET /data_sources/catalog`). |
| `environment` | `string` | no | Values: `production`, `sandbox`. |
| `token` | `string` | no | API token of the product. Stored encrypted. |
| `workspace_id` | `string` | no | Workspace inside the product, when the token's organization has more than one. |
| `name` | `string` | no | Display name, unique per workspace and kind. Derived from the spec, the host or the database when omitted. |

More fields and nested objects: `kobana schema dashboards.data-sources.catalog.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Source created, with the outcome of the connection test. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `409` | A data source with this name already exists in the workspace. |
| `422` | Request body failed validation. |

## data-sources logs get

Get a request of a data source

```text
kobana dashboards data-sources logs get [ID] [LOG_ID] [OPTIONS]
```

`GET /v1/data_sources/{id}/logs/{log_id}`

One call, whole: request and response headers and bodies, with secrets masked.

**Path arguments:** `id`, `log_id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Request log entry. |
| `404` | Resource not found in the current organization. |

## data-sources logs list

List the request log of a data source

```text
kobana dashboards data-sources logs list [ID] [OPTIONS]
```

`GET /v1/data_sources/{id}/logs`

Calls the source made (HTTP requests, database queries), newest first, without headers nor bodies.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `method` | `string` | no | HTTP method (GET, POST…). |
| `success` | `string` | no | `true` for successful calls only, `false` for failed ones only. Values: `true`, `false`. |
| `search` | `string` | no | Substring of the endpoint, case-insensitive. |
| `date_from` | `string` | no | From this day (YYYY-MM-DD, UTC), inclusive. |
| `date_to` | `string` | no | Up to this day (YYYY-MM-DD, UTC), inclusive. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of the request log. |
| `404` | Resource not found in the current organization. |

## data-sources openapi analyze

Analyze an OpenAPI spec

```text
kobana dashboards data-sources openapi analyze [OPTIONS]
```

`POST /v1/data_sources/openapi/analyze`

Reads the spec and returns what would be imported — title, servers, security schemes, suggested auth and operations — without saving anything.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `spec` | `string` | yes | The OpenAPI 3.x or Swagger 2.0 document as text (JSON or YAML), up to 5 MB — also bounded by the API's 2 MB body limit. |
| `spec_file_name` | `string` | no | File name of the spec (`openapi.yaml`). Guessed from the content when omitted. |

More fields and nested objects: `kobana schema dashboards.data-sources.openapi.analyze`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Spec analysis. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## data-sources operations update

Enable or disable an operation of a data source

```text
kobana dashboards data-sources operations update [ID] [OPERATION_ID] [OPTIONS]
```

`PATCH /v1/data_sources/{id}/operations/{operation_id}`

Turns one operation of the source on or off (`enabled`). Disabled operations do not feed entities.

**Path arguments:** `id`, `operation_id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `enabled` | `boolean` | yes | Disabled operations do not feed entities. |

More fields and nested objects: `kobana schema dashboards.data-sources.operations.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Updated operation. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## data-sources list

List data sources

```text
kobana dashboards data-sources list [OPTIONS]
```

`GET /v1/data_sources`

Data sources of the workspace — OpenAPI APIs, MCP servers and read-only databases — newest first. Secrets are never returned. To create a source, use the route of its kind: `/data_sources/openapi`, `/mcp`, `/database` or `/catalog`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of data sources. |
| `404` | Resource not found in the current organization. |

## data-sources create

Create a data source

```text
kobana dashboards data-sources create [OPTIONS]
```

`POST /v1/data_sources`

Saves a data source and tests the connection. `kind` says which: `openapi` (an API described by an OpenAPI spec — the spec goes as text in `spec`, the mTLS certificate base64-encoded in `certificate`; imports its operations), `mcp` (an MCP server — `url`; without OAuth it connects and discovers the tools right away, with OAuth it returns `authorization_url`, which a person signed in to the dashboard opens in a browser to finish) or `database` (a read-only database — `engine` and connection; catalogs its tables or collections). Send only the fields of the chosen `kind`. Kobana products go through `POST /data_sources/catalog`. Replaces `POST /data_sources/openapi`, `/mcp` and `/database`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

The body is not a plain object (or has variants). See `kobana schema dashboards.data-sources.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Source created, with the outcome of the connection test. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `409` | A data source with this name already exists in the workspace. |
| `422` | Request body failed validation. |

## data-sources test

Test a connection before saving

```text
kobana dashboards data-sources test [OPTIONS]
```

`POST /v1/data_sources/test`

Tests a connection that is not saved yet. Send `kind` (`openapi`, `mcp`, `database` or `catalog`) and the body of the matching create route. Nothing is persisted.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

The body is not a plain object (or has variants). See `kobana schema dashboards.data-sources.test`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Outcome of the connection test. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## data-sources get

Get a data source

```text
kobana dashboards data-sources get [ID] [OPTIONS]
```

`GET /v1/data_sources/{id}`

The data source and its operations (endpoints, tools or tables it can read).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Data source with its operations. |
| `404` | Resource not found in the current organization. |

## data-sources update

Update a data source

```text
kobana dashboards data-sources update [ID] [OPTIONS]
```

`PATCH /v1/data_sources/{id}`

Updates the name, `is_active` and the connection fields of the kind of the source. Blank secret fields keep the stored ones. An OpenAPI source takes a new certificate in `certificate`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Display name, unique per workspace and kind. Derived from the spec, the host or the database when omitted. |
| `is_active` | `boolean` | no |  |
| `base_url` | `string` | no | Base URL of the API. Defaults to the first absolute server of the spec. |
| `auth` | `object` | no | How the API authenticates: `none`, `header`, `query`, `bearer`, `basic` or `cert`. For `header`/`query`, send the key names and values; a blank value on an update keeps the stored one. |
| `credentials` | `object` | no | Secrets of the chosen auth: `token` (bearer), `username`/`password` (basic), `certificate_password` (cert). Stored encrypted and never returned. |
| `certificate` | `object` | no | Client certificate for mTLS (`auth.kind = "cert"`). |
| `probe_operation_key` | `string` | no | OpenAPI: operation used as the probe of the connection test; `null` picks one automatically. |
| `default_params` | `object` | no | OpenAPI: default parameter values sent on every call. |
| `response_format` | `string` | no | Values: `json`, `xml`. |
| `url` | `string` | no | MCP server URL (Streamable HTTP or SSE endpoint). A token in the query string is stored encrypted. |
| `transport` | `string` | no | Values: `http`, `sse`. |
| `headers` | `anyOf` | no | Extra headers sent on every call: an object, or one `Name: value` per line. Stored encrypted. |
| `oauth` | `object` | no | OAuth of the MCP server: `none`, `dcr` (dynamic client registration) or `client` (your own client id/secret). |
| `engine` | `string` | no | Values: `postgres`, `mysql`, `mariadb`, `mssql`, `mongodb`. |
| `host` | `string` | no |  |
| `port` | `anyOf` | no |  |
| `database` | `string` | no |  |
| `username` | `string` | no |  |
| `password` | `string` | no |  |
| `connection_string` | `string` | no | MongoDB only: the full connection string (instead of host, port, username and password). Username/password (SCRAM) authentication only; accepted options: authSource, authMechanism (SCRAM-SHA-1/SCRAM-SHA-256), replicaSet, tls/ssl, retryWrites, retryReads, w, journal, wtimeoutMS, readPreference, readConcernLevel, appName, directConnection, loadBalanced, compressors, zlibCompressionLevel and the pool timeouts/limits (maxPoolSize, minPoolSize, maxIdleTimeMS, connectTimeoutMS, socketTimeoutMS, serverSelectionTimeoutMS, heartbeatFrequencyMS, localThresholdMS, maxStalenessSeconds). Any other option is refused with 422. |
| `ssl` | `boolean` | no |  |
| `ssl_verify` | `boolean` | no |  |

More fields and nested objects: `kobana schema dashboards.data-sources.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Updated data source. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `409` | A data source with this name already exists in the workspace. |
| `422` | Request body failed validation. |

## data-sources delete

Delete a data source

```text
kobana dashboards data-sources delete [ID] [OPTIONS]
```

`DELETE /v1/data_sources/{id}`

Removes the source, its operations and its sync schedule. Refused (409) while an entity reads the source: remove the entities first. Cannot be undone.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Source deleted. |
| `404` | Resource not found in the current organization. |
| `409` | One or more entities read this data source. |

## data-sources authorize

Authorize an MCP data source

```text
kobana dashboards data-sources authorize [ID] [OPTIONS]
```

`POST /v1/data_sources/{id}/authorize`

Restarts the OAuth authorization of an MCP server. Returns `authorization_url` for a person signed in to the dashboard to open in a browser, or `null` plus the connection test when the stored tokens still work.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Authorization URL or test outcome. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## data-sources schedule

Schedule the sync of a data source

```text
kobana dashboards data-sources schedule [ID] [OPTIONS]
```

`PUT /v1/data_sources/{id}/schedule`

Sets how often the source syncs: `{ cron, timezone }`, or `{ cron: null }` for on demand only. Returns the next runs.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `cron` | `string` | yes | 5-field cron (at most every 5 minutes), or `null` to sync on demand only. |
| `timezone` | `string` | no | IANA timezone of the schedule. Default: the source's current one, or America/Sao_Paulo. |

More fields and nested objects: `kobana schema dashboards.data-sources.schedule`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Schedule saved. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## data-sources spec

Replace the spec of a data source

```text
kobana dashboards data-sources spec [ID] [OPTIONS]
```

`PUT /v1/data_sources/{id}/spec`

Replaces the OpenAPI spec and re-syncs the operations: those that keep their key keep their settings; those gone from the spec are removed.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `spec` | `string` | yes | The OpenAPI 3.x or Swagger 2.0 document as text (JSON or YAML), up to 5 MB — also bounded by the API's 2 MB body limit. |
| `spec_file_name` | `string` | no | File name of the spec (`openapi.yaml`). Guessed from the content when omitted. |

More fields and nested objects: `kobana schema dashboards.data-sources.spec`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Spec replaced, with the warnings of the analysis. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## data-sources sync

Sync a data source

```text
kobana dashboards data-sources sync [ID] [OPTIONS]
```

`POST /v1/data_sources/{id}/sync`

Queues a sync of every entity fed by the source. The run is asynchronous.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `full` | `boolean` | no | `true` re-reads everything instead of only what changed since the last sync. |

More fields and nested objects: `kobana schema dashboards.data-sources.sync`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | Sync queued. |
| `400` | Body that is not JSON, or a field of the wrong type. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
