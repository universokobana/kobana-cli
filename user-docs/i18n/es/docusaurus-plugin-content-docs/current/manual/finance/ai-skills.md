---
id: ai-skills
title: "kobana finance ai-skills"
sidebar_label: "ai-skills"
---

# kobana finance ai-skills


Comandos de `kobana finance ai-skills`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`ai-skills list`](#ai-skills-list) | `GET /v1/ai-skills` | Listar habilidades de IA |
| [`ai-skills create`](#ai-skills-create) | `POST /v1/ai-skills` | Crear habilidad de IA |
| [`ai-skills get`](#ai-skills-get) | `GET /v1/ai-skills/{id}` | Obtener habilidad de IA por ID |
| [`ai-skills update`](#ai-skills-update) | `PATCH /v1/ai-skills/{id}` | Actualizar habilidad de IA |
| [`ai-skills delete`](#ai-skills-delete) | `DELETE /v1/ai-skills/{id}` | Eliminar habilidad de IA |

## ai-skills list

Listar habilidades de IA

```text
kobana finance ai-skills list [OPCIONES]
```

`GET /v1/ai-skills`

Returns a cursor-paginated page of the AI skills the token's person can see in the workspace: their own and the ones other members shared with them (`is_owner` tells them apart). A skill is an instruction set the assistant loads when a task matches its `description`. Requires scope `finance/ai_skills` with access level `read`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `active` | `string` | no | Filter by the skill's on/off flag. `false` returns the skills the assistant does not see; omit to return both. Valores: `true`, `false`. |
| `mine` | `string` | no | `true` returns only the token person's own skills, `false` only the ones shared with them; omit to return both. Valores: `true`, `false`. |
| `sort_by` | `string` | no | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `name`, `created_at`, `updated_at`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`; ignored without `sort_by`. Valores: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Page of AI skills plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## ai-skills create

Crear habilidad de IA

```text
kobana finance ai-skills create [OPCIONES]
```

`POST /v1/ai-skills`

Creates an AI skill owned by the token's person, in the Agent Skills format: a `name`, a `description` saying what the skill does and when to use it — all the assistant reads to decide to load it —, Markdown `instructions` and optional text `files`. It is born active and `private`; `visibility` and `member_ids` share it right away, within the workspace's AI settings. A person has at most 100 skills per workspace. Requires scope `finance/ai_skills` with access level `write`. Pass an `X-Idempotency-Key` header on retries.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Free text, as people read it. |
| `description` | `string` | sí | What the skill does AND when to use it: it is all the assistant reads to decide to load the skill. |
| `instructions` | `string` | sí | The SKILL.md body, in Markdown: the steps, the tools to call by name, the format of the answer. |
| `slug` | `string` | no | Optional: without it, it is made from `name` ("Month-End Close" → `month-end-close`). Unique among the person's skills (409 `slug_taken`). |
| `files` | `array<object>` | no | Text files besides SKILL.md, replaced as a whole. Paths are unique (case-insensitive); the contents add up to at most 200000 characters. |
| `visibility` | `string` | no | Default `private`. Anything else needs the person's `ai-skills.share` (403 `missing_share_permission`); above the workspace's allowed maximum: 403 `above_max_visibility`. Valores: `private`, `restricted`, `workspace`. |
| `member_ids` | `array<string (uuid)>` | no | The whole list of members for `restricted` (the owner is ignored). Each must be an active member who uses the assistant in the workspace (422 `member_not_eligible`). Omit to keep the stored list. |

Otros campos y objetos anidados: `kobana schema finance.ai-skills.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Skill created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | The person already has a skill with this `slug` (`details.reason: slug_taken`), or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, the person's skill limit (`details.reason: too_many_skills`), or `member_ids` naming someone who cannot receive it (`details.reason: member_not_eligible`, with the refused `member_ids`). |

## ai-skills get

Obtener habilidad de IA por ID

```text
kobana finance ai-skills get [ID] [OPCIONES]
```

`GET /v1/ai-skills/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/ai_skills` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Get an AI skill by id |
| `404` | AI skill not found among the skills the token person can see in the workspace. |

## ai-skills update

Actualizar habilidad de IA

```text
kobana finance ai-skills update [ID] [OPCIONES]
```

`PATCH /v1/ai-skills/{id}`

Partial update of one of the token person's skills — someone else's is read-only (403, `details.reason: not_owner`). `files` and `member_ids` replace the whole list. `active: false` hides the skill from the assistant. Requires scope `finance/ai_skills` with access level `write`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Free text, as people read it. |
| `slug` | `string` | no | Lowercase letters, digits and hyphens, no hyphen at either end. Unique among the person's skills (409 `slug_taken`). |
| `description` | `string` | no | What the skill does AND when to use it: it is all the assistant reads to decide to load the skill. |
| `instructions` | `string` | no | The SKILL.md body, in Markdown: the steps, the tools to call by name, the format of the answer. |
| `files` | `array<object>` | no | Text files besides SKILL.md, replaced as a whole. Paths are unique (case-insensitive); the contents add up to at most 200000 characters. |
| `active` | `boolean` | no | `false` hides the skill from the assistant; `true` brings it back. |
| `visibility` | `string` | no | `private` is always allowed to the owner; anything else needs the person's `ai-skills.share` (403 `missing_share_permission`). Above the workspace's allowed maximum: 403 `above_max_visibility`. Omitted with `member_ids`, the current one stays. Valores: `private`, `restricted`, `workspace`. |
| `member_ids` | `array<string (uuid)>` | no | The whole list of members for `restricted` (the owner is ignored). Each must be an active member who uses the assistant in the workspace (422 `member_not_eligible`). Omit to keep the stored list. |

Otros campos y objetos anidados: `kobana schema finance.ai-skills.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Updated skill. |
| `400` | Malformed JSON body. |
| `404` | AI skill not found among the skills the token person can see in the workspace. |
| `409` | The person already has another skill with this `slug` (`details.reason: slug_taken`), or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, or `member_ids` naming someone who cannot receive it (`details.reason: member_not_eligible`). |

## ai-skills delete

Eliminar habilidad de IA

```text
kobana finance ai-skills delete [ID] [OPCIONES]
```

`DELETE /v1/ai-skills/{id}`

Removes one of the token person's skills, for everyone it was shared with; copies other people made of it stay. Someone else's skill is a 403 (`details.reason: not_owner`). Requires scope `finance/ai_skills` with access level `delete`. Calling DELETE twice returns 200 then 404.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | AI skill deleted. |
| `404` | AI skill not found among the skills the token person can see in the workspace. |
