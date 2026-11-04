---
id: builder
title: "kobana dashboards builder"
sidebar_label: "builder"
description: "Get the portal builder"
---

# kobana dashboards builder

Get the portal builder

Commands of `kobana dashboards builder`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`builder`](#builder) | `GET /v1/builder` | Get the portal builder |
| [`builder draft update`](#builder-draft-update) | `PUT /v1/builder/draft` | Save the portal draft |
| [`builder theme update`](#builder-theme-update) | `PATCH /v1/builder/theme` | Update the portal theme colors |
| [`builder tools create-page`](#builder-tools-create-page) | `POST /v1/builder/tools/create_page` | Create a page |
| [`builder tools delete-page`](#builder-tools-delete-page) | `POST /v1/builder/tools/delete_page` | Delete a page |
| [`builder tools describe-block-type`](#builder-tools-describe-block-type) | `GET /v1/builder/tools/describe_block_type` | Describe a block type |
| [`builder tools duplicate-block`](#builder-tools-duplicate-block) | `POST /v1/builder/tools/duplicate_block` | Duplicate a block |
| [`builder tools get-page`](#builder-tools-get-page) | `GET /v1/builder/tools/get_page` | Get a draft page |
| [`builder tools get-theme`](#builder-tools-get-theme) | `GET /v1/builder/tools/get_theme` | Get the theme colors |
| [`builder tools insert-block`](#builder-tools-insert-block) | `POST /v1/builder/tools/insert_block` | Insert a block |
| [`builder tools list-actions`](#builder-tools-list-actions) | `GET /v1/builder/tools/list_actions` | List button actions |
| [`builder tools list-block-types`](#builder-tools-list-block-types) | `GET /v1/builder/tools/list_block_types` | List block types |
| [`builder tools list-pages`](#builder-tools-list-pages) | `GET /v1/builder/tools/list_pages` | List the draft pages |
| [`builder tools move-block`](#builder-tools-move-block) | `POST /v1/builder/tools/move_block` | Move a block |
| [`builder tools move-page`](#builder-tools-move-page) | `POST /v1/builder/tools/move_page` | Move a page in the menu |
| [`builder tools preview-portal-user-mapping`](#builder-tools-preview-portal-user-mapping) | `GET /v1/builder/tools/preview_portal_user_mapping` | Preview the portal user mapping |
| [`builder tools read-asset`](#builder-tools-read-asset) | `GET /v1/builder/tools/read_asset` | Read the content of a file |
| [`builder tools register-entity`](#builder-tools-register-entity) | `POST /v1/builder/tools/register_entity` | Register an entity from a data source |
| [`builder tools remove-block`](#builder-tools-remove-block) | `POST /v1/builder/tools/remove_block` | Remove a block |
| [`builder tools rename-entity-fields`](#builder-tools-rename-entity-fields) | `POST /v1/builder/tools/rename_entity_fields` | Rename the fields of an entity |
| [`builder tools save-computed-field`](#builder-tools-save-computed-field) | `POST /v1/builder/tools/save_computed_field` | Save a computed field |
| [`builder tools update-block`](#builder-tools-update-block) | `POST /v1/builder/tools/update_block` | Update a block |
| [`builder tools update-page`](#builder-tools-update-page) | `POST /v1/builder/tools/update_page` | Update a page |
| [`builder versions get`](#builder-versions-get) | `GET /v1/builder/versions/{number}` | Get a published version |
| [`builder versions restore`](#builder-versions-restore) | `POST /v1/builder/versions/{number}/restore` | Restore a version into the draft |
| [`builder versions rollback`](#builder-versions-rollback) | `POST /v1/builder/versions/{number}/rollback` | Roll back to a version |
| [`builder encrypt-secret`](#builder-encrypt-secret) | `POST /v1/builder/encrypt_secret` | Encrypt a block secret |
| [`builder publish`](#builder-publish) | `POST /v1/builder/publish` | Publish the portal |
| [`builder test-action`](#builder-test-action) | `POST /v1/builder/test_action` | Test a draft action button |
| [`builder test-form`](#builder-test-form) | `POST /v1/builder/test_form` | Test a draft form |
| [`builder upload-og-image`](#builder-upload-og-image) | `POST /v1/builder/upload_og_image` | Upload the portal og:image |

## builder

Get the portal builder

```text
kobana dashboards builder [OPTIONS] [COMMAND]
```

`GET /v1/builder`

Returns the workspace's portal as the builder sees it: portal identity, the draft (pages in menu order, each badged against the live version, `revision` and `dirty`), the live version number, the published versions, the theme and the SEO defaults. Read `revision` here before saving the draft.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The portal builder. |
| `404` | Resource not found in the current organization. |

## builder draft update

Save the portal draft

```text
kobana dashboards builder draft update [OPTIONS]
```

`PUT /v1/builder/draft`

Replaces the whole draft — pages in menu order, editor state and, optionally, theme and SEO defaults — guarded by the `revision` you last read. A stale `revision` answers 409 and changes nothing. Nothing goes live: to publish, use `POST /builder/publish`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `revision` | `integer` | yes | The draft `revision` you last read (`0` when there is no draft yet). |
| `pages` | `array<object>` | yes | Every page of the draft, in menu order (up to 200). Pages left out are removed from the draft. |
| `editor_state` | `object` | no | Editor state; send back what you read. Omitted = `{}`. |
| `theme` | `object` | no |  |
| `seo_defaults` | `object` | no |  |

More fields and nested objects: `kobana schema dashboards.builder.draft.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Draft saved: the new `revision`, whether it differs from the live version and each page badge. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |

## builder theme update

Update the portal theme colors

```text
kobana dashboards builder theme update [OPTIONS]
```

`PATCH /v1/builder/theme`

Changes colors of the portal pages' theme (the builder's Visual tab) on the draft, per mode, in hex. Only the colors sent change; the portal on the air changes when the draft is published. To read the current colors, and what each is for, use `GET /builder/tools/get_theme`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `light` | `object` | no |  |
| `dark` | `object` | no |  |

More fields and nested objects: `kobana schema dashboards.builder.theme.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Draft theme: the new `revision`, whether the theme changed and how many colors were sent. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## builder tools create-page

Create a page

```text
kobana dashboards builder tools create-page [OPTIONS]
```

`POST /v1/builder/tools/create_page`

Creates a page in the draft and adds it to the portal menu. template: blank (title only), list (title + KPIs + table of `entity`), detail (title + record detail + documents). The title is a PageHeader block: the answer lists the blocks it created — change them with update_block instead of inserting them again.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Menu label. |
| `path` | `string` | no | Route, e.g. /contratos. Omitted = derived from the name. |
| `title` | `string` | no | Browser title; omitted = the name. |
| `template` | `string` | no | Values: `blank`, `list`, `detail`. |
| `entity` | `string` | no | Entity the list/detail template binds to. |
| `show_in_menu` | `boolean` | no | Listed in the portal menu (default true); false for a detail page reached by links. |
| `position` | `integer` | no | Place in the menu; omitted = last. |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.create-page`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools delete-page

Delete a page

```text
kobana dashboards builder tools delete-page [OPTIONS]
```

`POST /v1/builder/tools/delete_page`

Removes a page (and its blocks) from the draft and from the menu. Only when the operator asks for it.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `page_id` | `string` | yes | Page id (list_pages). |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.delete-page`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools describe-block-type

Describe a block type

```text
kobana dashboards builder tools describe-block-type [OPTIONS]
```

`GET /v1/builder/tools/describe_block_type`

The props of one block type: JSON Schema, defaults and slots. Read it before inserting or changing a block you have not used yet.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `type` | `string` | yes | Block type, e.g. DataTable. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool read, in `data`. |
| `404` | Resource not found in the current organization. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools duplicate-block

Duplicate a block

```text
kobana dashboards builder tools duplicate-block [OPTIONS]
```

`POST /v1/builder/tools/duplicate_block`

Copies a block (with the blocks inside it) right after itself, with new ids.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `block_id` | `string` | yes | Block id (get_page). |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.duplicate-block`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools get-page

Get a draft page

```text
kobana dashboards builder tools get-page [OPTIONS]
```

`GET /v1/builder/tools/get_page`

One page of the draft: its settings (name, path, title, SEO) and its block tree, each block with id, type, props and slots.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page_id` | `string` | yes | Page id (list_pages). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool read, in `data`. |
| `404` | Resource not found in the current organization. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools get-theme

Get the theme colors

```text
kobana dashboards builder tools get-theme [OPTIONS]
```

`GET /v1/builder/tools/get_theme`

The colors of the portal pages (the Visual tab of the draft), in hex, for light and dark mode: what each color is for and its current value.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool read, in `data`. |
| `404` | Resource not found in the current organization. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools insert-block

Insert a block

```text
kobana dashboards builder tools insert-block [OPTIONS]
```

`POST /v1/builder/tools/insert_block`

Inserts a block of the catalog in a page: at the page root, or inside a slot of a Columns/Tabs block (parent_block_id + slot). Props not sent take the defaults.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `page_id` | `string` | yes | Page id (list_pages). |
| `type` | `string` | yes | Block type (list_block_types). |
| `props` | `object` | no | Block props (describe_block_type). Never include id or slot props. |
| `parent_block_id` | `string` | no | Columns or Tabs block that receives the new block. |
| `slot` | `string` | no | Slot of the parent: col1..col3 (Columns) or content (Tabs). Omitted = the first. |
| `position` | `integer` | no | 0-based index; omitted = at the end. |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.insert-block`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools list-actions

List button actions

```text
kobana dashboards builder tools list-actions [OPTIONS]
```

`GET /v1/builder/tools/list_actions`

What an ActionButton can do (its actionType): open a link (a portal page or a URL) or fire a webhook, and the \{\{variables\}\} its webhook URL, headers and body may use.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool read, in `data`. |
| `404` | Resource not found in the current organization. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools list-block-types

List block types

```text
kobana dashboards builder tools list-block-types [OPTIONS]
```

`GET /v1/builder/tools/list_block_types`

Lists the block catalog: every block type the portal can render, with what it is for. Only these types exist.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool read, in `data`. |
| `404` | Resource not found in the current organization. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools list-pages

List the draft pages

```text
kobana dashboards builder tools list-pages [OPTIONS]
```

`GET /v1/builder/tools/list_pages`

Lists the pages of the draft in menu order: id, name, path, title and block count.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool read, in `data`. |
| `404` | Resource not found in the current organization. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools move-block

Move a block

```text
kobana dashboards builder tools move-block [OPTIONS]
```

`POST /v1/builder/tools/move_block`

Moves a block inside its page: to another position at the root (parent_block_id null) or into a slot of a Columns/Tabs block.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `block_id` | `string` | yes | Block id (get_page). |
| `position` | `integer` | yes | 0-based index in the target. |
| `parent_block_id` | `string` | no | Columns/Tabs block to move into; null or omitted = the page root. |
| `slot` | `string` | no |  |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.move-block`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools move-page

Move a page in the menu

```text
kobana dashboards builder tools move-page [OPTIONS]
```

`POST /v1/builder/tools/move_page`

Moves a page to another place in the portal menu.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `page_id` | `string` | yes | Page id (list_pages). |
| `position` | `integer` | yes | 0-based index in the menu. |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.move-page`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools preview-portal-user-mapping

Preview the portal user mapping

```text
kobana dashboards builder tools preview-portal-user-mapping [OPTIONS]
```

`GET /v1/builder/tools/preview_portal_user_mapping`

Checks a portal user mapping before saving it with `dashboards_portal_user_mappings_update`: the entity whose records would become the portal users, its tenant ID, e-mail and name fields, how many users that is, and how the tenant IDs match the owners of the other entities' records. Saves nothing.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `entity` | `string` | yes | Entity key. |
| `tenant_id_field` | `string` | yes | The value the other entities' owners hold (the owner field of the entity, or its identifier). |
| `email_field` | `string` | yes | The field with the customer's e-mail. |
| `name_field` | `string` | no | The field with the customer's name, if any. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool read, in `data`. |
| `404` | Resource not found in the current organization. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools read-asset

Read the content of a file

```text
kobana dashboards builder tools read-asset [OPTIONS]
```

`GET /v1/builder/tools/read_asset`

Opens a file of the Assets library to look at its content: an image or a PDF is shown to you, a text file (txt, csv, json, md, xml) or an .xlsx spreadsheet comes as text. Word/PowerPoint/zip cannot be read (only their details).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `asset_id` | `string` | yes | Asset id (list_assets, or the asset_id of a file sent in the chat). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool read, in `data`. |
| `404` | Resource not found in the current organization. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools register-entity

Register an entity from a data source

```text
kobana dashboards builder tools register-entity [OPTIONS]
```

`POST /v1/builder/tools/register_entity`

Registers an entity (data blocks bind to it) from an operation of a connected data source (describe_data_source), with the fields inferred from a sample of its records, and starts its first sync. Use it when the operator wants data that no entity has yet. It takes effect in the workspace right away (it is not part of the draft); the new entity can be used by blocks in this same answer. The workspace caps how many fields a new entity keeps: when the source has more, the refusal lists every field with a sample value — call again with `fields` set to the ones the portal needs (what it shows, filters or sums, and whatever identifies the customer the record belongs to).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `data_source_id` | `string (uuid)` | yes |  |
| `operation_key` | `string` | yes | Operation key (describe_data_source); prefer a "list" operation. |
| `params` | `object` | no | Fixed parameters the operation needs, if any. |
| `key` | `string` | no | Entity key; omitted = inferred from the operation. |
| `name` | `string` | no | Singular name in Portuguese, e.g. "Contrato". |
| `name_plural` | `string` | no | Plural name, e.g. "Contratos". |
| `fields` | `array<string>` | no | Field keys to keep, from the list a refusal gives; omitted = every visible field the sample shows. |
| `field_names` | `object` | no | Labels in Brazilian Portuguese for the kept fields, by key (\{ "amount": "Valor", "expire_at": "Vencimento" \}): they are the column headers the customers read. |
| `owner_field` | `string` | no | The field saying which customer the record belongs to (the titular), when the inferred one is wrong or missing. On a CUSTOMERS entity it is its own id (each customer owns their own record); on a data entity, the customer id it carries (e.g. customer_id). When the portal is keyed by CPF/CNPJ, the document field instead. |
| `owner_match` | `string` | no | How the owner is compared to the users' tenant ID: external_id = as text (an id); document_number = as a CPF/CNPJ, digits only on both sides, so "32.013.964/0001-60" matches "32013964000160". Use document_number whenever the owner field holds a CPF/CNPJ. Values: `external_id`, `document_number`. |

More fields and nested objects: `kobana schema dashboards.builder.tools.register-entity`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools remove-block

Remove a block

```text
kobana dashboards builder tools remove-block [OPTIONS]
```

`POST /v1/builder/tools/remove_block`

Removes a block (and the blocks inside it) from its page.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `block_id` | `string` | yes | Block id (get_page). |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.remove-block`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools rename-entity-fields

Rename the fields of an entity

```text
kobana dashboards builder tools rename-entity-fields [OPTIONS]
```

`POST /v1/builder/tools/rename_entity_fields`

Changes an existing entity: renames its fields — the labels the blocks show (table headers, card labels) — and, optionally, the entity itself, and can set its owner field (the titular: whose record it is). Use it when the labels came from the source in English or as raw keys ("Expire at", "customer_cnpj_cpf"): customers read them, so they go in Brazilian Portuguese; and when the owner is wrong or missing (a customers entity owns its records by its own id). Keys stay; a new owner maps the stored records again. It takes effect in the workspace right away, not in the draft.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `entity` | `string` | yes | Entity key (list_entities). |
| `names` | `object` | no | New label by field key, e.g. \{ "amount": "Valor", "expire_at": "Vencimento" \}. |
| `name` | `string` | no | New singular name of the entity. |
| `name_plural` | `string` | no | New plural name of the entity. |
| `owner_field` | `string` | no | New owner field (a field key of the entity, a computed one included). |
| `owner_match` | `string` | no | How the owner is compared to the users' tenant ID: external_id = as text (an id); document_number = as a CPF/CNPJ, digits only on both sides, so "32.013.964/0001-60" matches "32013964000160". Use document_number whenever the owner field holds a CPF/CNPJ. Values: `external_id`, `document_number`. |

More fields and nested objects: `kobana schema dashboards.builder.tools.rename-entity-fields`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools save-computed-field

Save a computed field

```text
kobana dashboards builder tools save-computed-field [OPTIONS]
```

`POST /v1/builder/tools/save_computed_field`

Adds a computed field to an entity — a value derived from the other fields by an expression (n8n style) — or changes the expression of a computed field it already has (same key). Use it when the operator wants a value the source does not give as is: part of a text, a join of fields, a formatted date or amount, a label from a condition. It is tried on stored records first (an expression that fails on all of them is refused, nothing saved), then saved to the entity in the workspace right away (not part of the draft) and computed on every stored record from the data it already has (no new fetch from the source). Blocks and sample_records see it in this same answer.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `entity` | `string` | yes | Entity key (list_entities). |
| `key` | `string` | yes | Field key, snake_case. Must not be a field the source maps. |
| `name` | `string` | yes | Field label in Portuguese, e.g. "Descrição curta". |
| `expression` | `string` | yes | Template with \{\{ \}\}, e.g. "\{\{ $resource.description.split('\\r\\n')[0] \}\}". |
| `data_type` | `string` | no | What the expression gives: string, int, decimal, money, bool, date, datetime… Values: `string`, `int`, `decimal`, `money`, `bool`, `date`, `datetime`, `email`, `enum`, `document`, `phone`, `url`, `file`, `array`, `json`. |

More fields and nested objects: `kobana schema dashboards.builder.tools.save-computed-field`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools update-block

Update a block

```text
kobana dashboards builder tools update-block [OPTIONS]
```

`POST /v1/builder/tools/update_block`

Changes props of a block. Only the props sent change; the result is validated against the block type.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `block_id` | `string` | yes | Block id (get_page). |
| `props` | `object` | yes | Block props (describe_block_type). Never include id or slot props. |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.update-block`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder tools update-page

Update a page

```text
kobana dashboards builder tools update-page [OPTIONS]
```

`POST /v1/builder/tools/update_page`

Changes the settings of a page: menu name, route, browser title, whether it is listed in the portal menu, and SEO / Open Graph tags. Only the fields sent change.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `page_id` | `string` | yes | Page id (list_pages). |
| `name` | `string` | no |  |
| `path` | `string` | no |  |
| `title` | `string` | no |  |
| `show_in_menu` | `boolean` | no | false = the page stays reachable by links (e.g. a record detail page) but is not in the header menu. |
| `seo` | `object` | no |  |
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.tools.update-page`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | What the tool did, in `data`, and the draft's new `revision` (the same as before when nothing changed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | The tool refused the step (`BUILDER_OPERATION_REJECTED`): unknown block type, props that fail the block schema, a path in use, an unknown entity… The message says why and nothing is saved. |

## builder versions get

Get a published version

```text
kobana dashboards builder versions get [NUMBER] [OPTIONS]
```

`GET /v1/builder/versions/{number}`

Returns a published version of the portal, by number, with the pages it froze.

**Path arguments:** `number`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The version and its pages. |
| `404` | Resource not found in the current organization. |

## builder versions restore

Restore a version into the draft

```text
kobana dashboards builder versions restore [NUMBER] [OPTIONS]
```

`POST /v1/builder/versions/{number}/restore`

Copies a published version into the draft, replacing its pages — to keep editing from an older version. Nothing goes live: the portal changes only when the draft is published. (Putting an older version on the air is `.../rollback`.) `revision` is optional: sent and stale, it answers 409.

**Path arguments:** `number`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `revision` | `integer` | no | The draft `revision` you last read. Optional: when sent and stale, answers 409 and saves nothing. |

More fields and nested objects: `kobana schema dashboards.builder.versions.restore`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Draft holding the version’s pages: the new `revision`, the restored number and how many pages. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |

## builder versions rollback

Roll back to a version

```text
kobana dashboards builder versions rollback [NUMBER] [OPTIONS]
```

`POST /v1/builder/versions/{number}/rollback`

Puts that version back on the air as the next number — portal visitors see it right away. Returns the new version and the builder as it now stands.

**Path arguments:** `number`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `201` | The new live version and the updated builder. |
| `404` | Resource not found in the current organization. |
| `422` | That version is already live (`ALREADY_LIVE`). |

## builder encrypt-secret

Encrypt a block secret

```text
kobana dashboards builder encrypt-secret [OPTIONS]
```

`POST /v1/builder/encrypt_secret`

Encrypts a value a block keeps in the page JSON (a webhook header such as `Authorization`). Returns `secret:<ciphertext>`, bound to this workspace and to the origin of `url` (required), which only the server opens to send the request to that origin. Nothing is stored: save the returned value in the block with the draft. Replaces `POST /builder/secrets`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `value` | `string` | yes | The plaintext to seal. Never echoed back. |
| `url` | `string` | yes | The webhook URL the value is for (required): the secret opens only for a request to the same origin (scheme, host and port). The host must be fixed — no `{{variables}}`. |

More fields and nested objects: `kobana schema dashboards.builder.encrypt-secret`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The sealed value. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | The `url` has no fixed http(s) origin — e.g. a `{{variable}}` in the host (`ACTION_SECRET_URL_REQUIRED`). |

## builder publish

Publish the portal

```text
kobana dashboards builder publish [OPTIONS]
```

`POST /v1/builder/publish`

Saves the draft as sent (same body as the draft save, plus an optional `note`) and freezes it into the next version, in one transaction: portal visitors see it right away. A form field whose regular expression could hang validation answers 422 `UNSAFE_FIELD_PATTERN`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `revision` | `integer` | yes | The draft `revision` you last read (`0` when there is no draft yet). |
| `pages` | `array<object>` | yes | Every page of the draft, in menu order (up to 200). Pages left out are removed from the draft. |
| `editor_state` | `object` | no | Editor state; send back what you read. Omitted = `{}`. |
| `theme` | `object` | no |  |
| `seo_defaults` | `object` | no |  |
| `note` | `string` | no | What changed, shown in the version history. |

More fields and nested objects: `kobana schema dashboards.builder.publish`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Version published, with the draft’s new `revision`. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The `revision` sent is stale: someone (or another tab) saved the draft in between. Read it again and resend. |
| `422` | A form field regular expression could hang validation (`UNSAFE_FIELD_PATTERN`); `details` names the page, block and field. |

## builder test-action

Test a draft action button

```text
kobana dashboards builder test-action [OPTIONS]
```

`POST /v1/builder/test_action`

Runs an Action button's webhook the way the builder's preview does: the request really goes out, marked `preview: true`, and the answer carries the HTTP status the target replied with. Headers may carry `secret:` values sealed by `POST /builder/encrypt_secret`, which open only for the origin they were sealed for. Capped at 20 runs a minute per operator, 60 a minute and 500 a day per organization. Replaces `POST /builder/action_runs`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `block_id` | `string` | yes | Id of the Action button block. |
| `request` | `object` | yes |  |
| `label` | `string` | no |  |
| `page_path` | `string` | no | Path of the page holding the block. Default `/`. |
| `submitter` | `object` | no | Customer the preview acts as, for `{{customer_name}}` / `{{customer_email}}`. |

More fields and nested objects: `kobana schema dashboards.builder.test-action`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The run result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Misconfigured request (URL without https, invalid header, a secret from another workspace…). |

## builder test-form

Test a draft form

```text
kobana dashboards builder test-form [OPTIONS]
```

`POST /v1/builder/test_form`

Runs a Form block's action the way the builder's preview does: the configured e-mail really goes out (or the chosen webhook really fires), marked `preview: true`. The configuration comes in the body (the caller's draft). The test e-mail goes only to the operator the token names. Capped at 20 runs a minute per operator, 60 a minute and 500 a day per organization. Replaces `POST /builder/form_submissions`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `form_id` | `string` | yes | Id of the Form block. |
| `fields` | `array<object>` | yes |  |
| `action` | `object` | yes |  |
| `form_title` | `string` | no |  |
| `page_path` | `string` | no | Path of the page holding the block. Default `/`. |
| `values` | `object` | no | Filled-in values, by field name. |
| `submitter` | `object` | no | Customer the preview acts as, for `{{customer_name}}` / `{{customer_email}}`. |

More fields and nested objects: `kobana schema dashboards.builder.test-form`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Submission recorded: recipients, notifications or the webhook delivery. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Values failing the form rules, an unsafe regular expression or a misconfigured action; `field_errors` lists the fields. |

## builder upload-og-image

Upload the portal og:image

```text
kobana dashboards builder upload-og-image [OPTIONS]
```

`POST /v1/builder/upload_og_image`

Uploads a page's (or the portal's default) og:image — the picture link previews show — as base64 (PNG, JPEG or WebP, at most ~1.5 MB) and answers its public URL, to store in `seo.og_image_url` (or `seo_defaults.og_image_url`) with the next draft save. Replaces `POST /builder/og_image`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `filename` | `string` | yes | Original file name. |
| `content_type` | `string` | yes | Values: `image/png`, `image/jpeg`, `image/webp`. |
| `content_base64` | `string` | yes | The image bytes, base64-encoded (a `data:…;base64,` prefix is accepted). |

More fields and nested objects: `kobana schema dashboards.builder.upload-og-image`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Image uploaded, with its public URL. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `413` | The request body exceeds 2 MB. |
| `422` | Type not accepted, invalid base64, image over the limit, or bytes that do not match `content_type`. |
