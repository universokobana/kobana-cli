---
id: ai-agents
title: "kobana finance ai-agents"
sidebar_label: "ai-agents"
---

# kobana finance ai-agents


Comandos de `kobana finance ai-agents`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`ai-agents triggers`](#ai-agents-triggers) | `GET /v1/ai-agents/{id}/triggers` | Listar os acionadores de um agente de IA |
| [`ai-agents triggers create`](#ai-agents-triggers-create) | `POST /v1/ai-agents/{id}/triggers` | Adicionar acionador a um agente de IA |
| [`ai-agents triggers update`](#ai-agents-triggers-update) | `PATCH /v1/ai-agents/{id}/triggers/{triggerId}` | Atualizar acionador de um agente de IA |
| [`ai-agents triggers delete`](#ai-agents-triggers-delete) | `DELETE /v1/ai-agents/{id}/triggers/{triggerId}` | Remover acionador de um agente de IA |
| [`ai-agents triggers token`](#ai-agents-triggers-token) | `POST /v1/ai-agents/{id}/triggers/{triggerId}/token` | Trocar o token de um acionador de API |
| [`ai-agents list`](#ai-agents-list) | `GET /v1/ai-agents` | Listar agentes de IA |
| [`ai-agents create`](#ai-agents-create) | `POST /v1/ai-agents` | Criar agente de IA |
| [`ai-agents get`](#ai-agents-get) | `GET /v1/ai-agents/{id}` | Obter agente de IA por ID |
| [`ai-agents update`](#ai-agents-update) | `PATCH /v1/ai-agents/{id}` | Atualizar agente de IA |
| [`ai-agents delete`](#ai-agents-delete) | `DELETE /v1/ai-agents/{id}` | Excluir agente de IA |

## ai-agents triggers

Listar os acionadores de um agente de IA

```text
kobana finance ai-agents triggers [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/ai-agents/{id}/triggers`

What runs the agent: `manual`, `event`, `api` and `schedule` triggers, oldest first. Not paginated. An `api` trigger shows its endpoint and the last characters of its token, never the token. Requires scope `finance/ai_agents` with access level `read`.

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
| `200` | The agent's triggers. |
| `400` | Unknown name in `fields`. |
| `404` | AI agent (or trigger) not found among the token person's agents in the workspace. |

## ai-agents triggers create

Adicionar acionador a um agente de IA

```text
kobana finance ai-agents triggers create [ID] [OPÇÕES]
```

`POST /v1/ai-agents/{id}/triggers`

`manual` (at most one), `event` with `event_type` (one per event), `api` (answers the endpoint's token — the ONLY time it is returned; store it) or `schedule` with `cron` and optional `timezone`. Requires scope `finance/ai_agents` with access level `write`. Takes no `X-Idempotency-Key`: the response may carry a token, and a recorded response would keep it.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `kind` | `string` | sim | `manual` enables starting the agent by hand (the dashboard's "Run now"); `event` runs it when a system event happens in the workspace (`event_type`); `api` exposes an endpoint with its own token; `schedule` runs it on a cron. Event and API triggers queue at most 20 runs per hour per agent; an agent has at most 20 triggers. Valores: `manual`, `event`, `api`, `schedule`. |
| `event_type` | `string` | não | Required for `event`; not allowed for other kinds. Valores: `receipt.saved`, `fiscal_document.received`, `payable.created`, `receivable.created`. |
| `cron` | `string` | não | Required for `schedule`: 5-field cron, at most every 15 minutes (422, `reason: too_frequent`). |
| `timezone` | `string` | não | `schedule` only. IANA time zone; defaults to `America/Sao_Paulo`. |

Outros campos e objetos aninhados: `kobana schema finance.ai-agents.triggers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Trigger created. `token` is set only for `api`. The `Location` header points at the new trigger. |
| `400` | Malformed JSON body. |
| `404` | AI agent (or trigger) not found among the token person's agents in the workspace. |
| `409` | `details.reason` is `duplicate_manual`, `duplicate_api`, `duplicate_event` or `too_many_triggers`. |
| `422` | Schema validation failure; a field missing or not allowed for the kind (`details.reason` `missing_field` / `field_not_allowed`, `details.field`); or an invalid schedule (`invalid_cron`, `invalid_timezone`, `too_frequent`). |

## ai-agents triggers update

Atualizar acionador de um agente de IA

```text
kobana finance ai-agents triggers update [ID] [TRIGGERID] [OPÇÕES]
```

`PATCH /v1/ai-agents/{id}/triggers/{triggerId}`

Partial. `active` pauses or resumes the trigger; `event_type` applies to `event` triggers; `cron` and `timezone` to `schedule` triggers (the next run is recomputed from now). Requires scope `finance/ai_agents` with access level `write`.

**Argumentos do caminho:** `id`, `triggerId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `active` | `boolean` | não | Pause (`false`) or resume (`true`) this trigger. |
| `event_type` | `string` | não | `event` triggers only. Valores: `receipt.saved`, `fiscal_document.received`, `payable.created`, `receivable.created`. |
| `cron` | `string` | não | `schedule` triggers only. |
| `timezone` | `string` | não | `schedule` triggers only. |

Outros campos e objetos aninhados: `kobana schema finance.ai-agents.triggers.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated trigger. |
| `400` | Malformed JSON body. |
| `404` | AI agent (or trigger) not found among the token person's agents in the workspace. |
| `409` | `X-Idempotency-Key` reuse with a different body, or `duplicate_event` (the agent already listens to it). |
| `422` | Schema validation failure, a field that does not apply to the kind, or an invalid schedule (`details.reason`). |

## ai-agents triggers delete

Remover acionador de um agente de IA

```text
kobana finance ai-agents triggers delete [ID] [TRIGGERID] [OPÇÕES]
```

`DELETE /v1/ai-agents/{id}/triggers/{triggerId}`

Removes the trigger (and, for a schedule, its schedule). Runs it started stay in the assistant. Requires scope `finance/ai_agents` with access level `delete`.

**Argumentos do caminho:** `id`, `triggerId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Trigger removed. |
| `404` | AI agent (or trigger) not found among the token person's agents in the workspace. |

## ai-agents triggers token

Trocar o token de um acionador de API

```text
kobana finance ai-agents triggers token [ID] [TRIGGERID] [OPÇÕES]
```

`POST /v1/ai-agents/{id}/triggers/{triggerId}/token`

Issues a new token for an `api` trigger; the previous one stops working at once. The token is returned only in this response. Requires scope `finance/ai_agents` with access level `write`. Takes no `X-Idempotency-Key`, for the same reason as the create.

**Argumentos do caminho:** `id`, `triggerId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The trigger and its new token. |
| `404` | AI agent (or trigger) not found among the token person's agents in the workspace. |
| `422` | The trigger is not an `api` trigger. |

## ai-agents list

Listar agentes de IA

```text
kobana finance ai-agents list [OPÇÕES]
```

`GET /v1/ai-agents`

Returns a cursor-paginated page of the AI agents that belong to the token's person in the workspace — never another member's. Requires scope `finance/ai_agents` with access level `read`. Each agent carries its triggers and its latest run; every run is a new assistant conversation (`last_run.conversation_id`).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `active` | `string` | não | Filter by the agent's pause flag. `false` returns the paused agents; omit to return both. Valores: `true`, `false`. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=created_at&sort_order=asc`. Valores: `created_at`, `title`. |
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
| `200` | Page of AI agents plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## ai-agents create

Criar agente de IA

```text
kobana finance ai-agents create [OPÇÕES]
```

`POST /v1/ai-agents`

Creates an AI agent owned by the token's person: instructions the assistant runs with that person's permissions. It is born active with a `manual` trigger; add schedule, event or API triggers through `POST /ai-agents/{id}/triggers`. Every run spends AI credit and becomes a new assistant conversation; a destructive action waits for the person's confirmation in the assistant. Requires scope `finance/ai_agents` with access level `write`. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `title` | `string` | sim |  |
| `instructions` | `string` | sim | The prompt the assistant executes on every run. |
| `summary` | `string` | não | Short description shown on the agent's card. |

Outros campos e objetos aninhados: `kobana schema finance.ai-agents.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Agent created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure. |

## ai-agents get

Obter agente de IA por ID

```text
kobana finance ai-agents get [ID] [OPÇÕES]
```

`GET /v1/ai-agents/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/ai_agents` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get an AI agent by id |
| `404` | AI agent not found among the token person's agents in the workspace. |

## ai-agents update

Atualizar agente de IA

```text
kobana finance ai-agents update [ID] [OPÇÕES]
```

`PATCH /v1/ai-agents/{id}`

Partial update. `active: false` pauses every automatic trigger (schedule, event, API) and `active: true` resumes them, recomputing the next scheduled run from now. Triggers are managed under `/ai-agents/{id}/triggers`. Requires scope `finance/ai_agents` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `title` | `string` | não |  |
| `summary` | `string` | não |  |
| `instructions` | `string` | não |  |
| `active` | `boolean` | não | `false` pauses every automatic trigger (schedule, event, API); `true` resumes them. |

Outros campos e objetos aninhados: `kobana schema finance.ai-agents.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated agent. |
| `400` | Malformed JSON body. |
| `404` | AI agent not found among the token person's agents in the workspace. |
| `409` | `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure. |

## ai-agents delete

Excluir agente de IA

```text
kobana finance ai-agents delete [ID] [OPÇÕES]
```

`DELETE /v1/ai-agents/{id}`

Removes the agent, its triggers and their schedules. Its past runs stay in the assistant as conversations. Requires scope `finance/ai_agents` with access level `delete`. Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | AI agent deleted. |
| `404` | AI agent not found among the token person's agents in the workspace. |
