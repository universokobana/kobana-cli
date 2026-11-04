---
id: events
title: "kobana dashboards events"
sidebar_label: "events"
---

# kobana dashboards events


Comandos de `kobana dashboards events`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`events list`](#events-list) | `GET /v1/events` | Listar eventos |
| [`events get`](#events-get) | `GET /v1/events/{id}` | Consultar evento |

## events list

Listar eventos

```text
kobana dashboards events list [OPÇÕES]
```

`GET /v1/events`

Log de eventos do workspace e da organização, do mais recente ao mais antigo. Eventos são evidência: não podem ser alterados nem excluídos.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `type` | `string` | não | Tipo exato do evento (ex.: `api_key.rotated`). |
| `resource_type` | `string` | não | Tipo exato do recurso (ex.: `api_key`). |
| `resource_id` | `string` | não | Somente eventos deste registro. |
| `date_from` | `string` | não | Limite inferior de `created_at` (data ou data-hora ISO 8601). |
| `date_to` | `string` | não | Limite superior de `created_at` (data ou data-hora ISO 8601). Uma data sem hora cobre o dia inteiro (UTC). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de eventos. |
| `400` | Requisição inválida: corpo malformado, campo obrigatório ausente ou filtro com valor inválido. |
| `404` | Recurso não encontrado na organização atual. |

## events get

Consultar evento

```text
kobana dashboards events get [ID] [OPÇÕES]
```

`GET /v1/events/{id}`

Retorna um evento com o payload completo.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O evento. |
| `404` | Recurso não encontrado na organização atual. |
