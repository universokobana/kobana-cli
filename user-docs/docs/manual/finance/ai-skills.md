---
id: ai-skills
title: "kobana finance ai-skills"
sidebar_label: "ai-skills"
---

# kobana finance ai-skills


Comandos de `kobana finance ai-skills`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`ai-skills list`](#ai-skills-list) | `GET /v1/ai-skills` | Listar habilidades de IA |
| [`ai-skills create`](#ai-skills-create) | `POST /v1/ai-skills` | Criar habilidade de IA |
| [`ai-skills get`](#ai-skills-get) | `GET /v1/ai-skills/{id}` | Obter habilidade de IA por ID |
| [`ai-skills update`](#ai-skills-update) | `PATCH /v1/ai-skills/{id}` | Atualizar habilidade de IA |
| [`ai-skills delete`](#ai-skills-delete) | `DELETE /v1/ai-skills/{id}` | Excluir habilidade de IA |

## ai-skills list

Listar habilidades de IA

```text
kobana finance ai-skills list [OPÇÕES]
```

`GET /v1/ai-skills`

Returns a cursor-paginated page of the AI skills the token's person can see in the workspace: their own and the ones other members shared with them (`is_owner` tells them apart). A skill is an instruction set the assistant loads when a task matches its `description`. Requires scope `finance/ai_skills` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `active` | `string` | não | Filter by the skill's on/off flag. `false` returns the skills the assistant does not see; omit to return both. Valores: `true`, `false`. |
| `mine` | `string` | não | `true` returns only the token person's own skills, `false` only the ones shared with them; omit to return both. Valores: `true`, `false`. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `name`, `created_at`, `updated_at`. |
| `sort_order` | `string` | não | Order direction. Defaults to `asc`; ignored without `sort_by`. Valores: `asc`, `desc`. |
| `limit` | `integer` | não | Page size. Default 25, max 100. |
| `cursor` | `string` | não | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Page of AI skills plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## ai-skills create

Criar habilidade de IA

```text
kobana finance ai-skills create [OPÇÕES]
```

`POST /v1/ai-skills`

Creates an AI skill owned by the token's person, in the Agent Skills format: a `name`, a `description` saying what the skill does and when to use it — all the assistant reads to decide to load it —, Markdown `instructions` and optional text `files`. It is born active and `private`; `visibility` and `member_ids` share it right away, within the workspace's AI settings. A person has at most 100 skills per workspace. Requires scope `finance/ai_skills` with access level `write`. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Free text, as people read it. |
| `description` | `string` | sim | What the skill does AND when to use it: it is all the assistant reads to decide to load the skill. |
| `instructions` | `string` | sim | The SKILL.md body, in Markdown: the steps, the tools to call by name, the format of the answer. |
| `slug` | `string` | não | Optional: without it, it is made from `name` ("Month-End Close" → `month-end-close`). Unique among the person's skills (409 `slug_taken`). |
| `files` | `array<object>` | não | Text files besides SKILL.md, replaced as a whole. Paths are unique (case-insensitive); the contents add up to at most 200000 characters. |
| `visibility` | `string` | não | Default `private`. Anything else needs the person's `ai-skills.share` (403 `missing_share_permission`); above the workspace's allowed maximum: 403 `above_max_visibility`. Valores: `private`, `restricted`, `workspace`. |
| `member_ids` | `array<string (uuid)>` | não | The whole list of members for `restricted` (the owner is ignored). Each must be an active member who uses the assistant in the workspace (422 `member_not_eligible`). Omit to keep the stored list. |

Outros campos e objetos aninhados: `kobana schema finance.ai-skills.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Skill created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | The person already has a skill with this `slug` (`details.reason: slug_taken`), or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, the person's skill limit (`details.reason: too_many_skills`), or `member_ids` naming someone who cannot receive it (`details.reason: member_not_eligible`, with the refused `member_ids`). |

## ai-skills get

Obter habilidade de IA por ID

```text
kobana finance ai-skills get [ID] [OPÇÕES]
```

`GET /v1/ai-skills/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/ai_skills` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Get an AI skill by id |
| `404` | AI skill not found among the skills the token person can see in the workspace. |

## ai-skills update

Atualizar habilidade de IA

```text
kobana finance ai-skills update [ID] [OPÇÕES]
```

`PATCH /v1/ai-skills/{id}`

Partial update of one of the token person's skills — someone else's is read-only (403, `details.reason: not_owner`). `files` and `member_ids` replace the whole list. `active: false` hides the skill from the assistant. Requires scope `finance/ai_skills` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Free text, as people read it. |
| `slug` | `string` | não | Lowercase letters, digits and hyphens, no hyphen at either end. Unique among the person's skills (409 `slug_taken`). |
| `description` | `string` | não | What the skill does AND when to use it: it is all the assistant reads to decide to load the skill. |
| `instructions` | `string` | não | The SKILL.md body, in Markdown: the steps, the tools to call by name, the format of the answer. |
| `files` | `array<object>` | não | Text files besides SKILL.md, replaced as a whole. Paths are unique (case-insensitive); the contents add up to at most 200000 characters. |
| `active` | `boolean` | não | `false` hides the skill from the assistant; `true` brings it back. |
| `visibility` | `string` | não | `private` is always allowed to the owner; anything else needs the person's `ai-skills.share` (403 `missing_share_permission`). Above the workspace's allowed maximum: 403 `above_max_visibility`. Omitted with `member_ids`, the current one stays. Valores: `private`, `restricted`, `workspace`. |
| `member_ids` | `array<string (uuid)>` | não | The whole list of members for `restricted` (the owner is ignored). Each must be an active member who uses the assistant in the workspace (422 `member_not_eligible`). Omit to keep the stored list. |

Outros campos e objetos aninhados: `kobana schema finance.ai-skills.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated skill. |
| `400` | Malformed JSON body. |
| `404` | AI skill not found among the skills the token person can see in the workspace. |
| `409` | The person already has another skill with this `slug` (`details.reason: slug_taken`), or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, or `member_ids` naming someone who cannot receive it (`details.reason: member_not_eligible`). |

## ai-skills delete

Excluir habilidade de IA

```text
kobana finance ai-skills delete [ID] [OPÇÕES]
```

`DELETE /v1/ai-skills/{id}`

Removes one of the token person's skills, for everyone it was shared with; copies other people made of it stay. Someone else's skill is a 403 (`details.reason: not_owner`). Requires scope `finance/ai_skills` with access level `delete`. Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | AI skill deleted. |
| `404` | AI skill not found among the skills the token person can see in the workspace. |
