---
id: webhook-deliveries
title: "kobana dashboards webhook-deliveries"
sidebar_label: "webhook-deliveries"
---

# kobana dashboards webhook-deliveries


Comandos de `kobana dashboards webhook-deliveries`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`webhook-deliveries list`](#webhook-deliveries-list) | `GET /v1/webhook_deliveries` | Listar entregas de webhook |
| [`webhook-deliveries get`](#webhook-deliveries-get) | `GET /v1/webhook_deliveries/{id}` | Consultar entrega de webhook |

## webhook-deliveries list

Listar entregas de webhook

```text
kobana dashboards webhook-deliveries list [OPÇÕES]
```

`GET /v1/webhook_deliveries`

Entregas de todos os endpoints do workspace, da mais recente à mais antiga, com o tipo do evento e o resumo do endpoint.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | não | `pending`, `delivered` ou `failed`. Valores: `pending`, `delivered`, `failed`. |
| `webhook_id` | `string` | não | Somente entregas deste endpoint de webhook. |
| `date_from` | `string` | não | Limite inferior de `created_at` (data ou data-hora ISO 8601). |
| `date_to` | `string` | não | Limite superior de `created_at` (data ou data-hora ISO 8601). Uma data sem hora cobre o dia inteiro (UTC). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de entregas. |
| `400` | Requisição inválida: corpo malformado, campo obrigatório ausente ou filtro com valor inválido. |
| `404` | Recurso não encontrado na organização atual. |

## webhook-deliveries get

Consultar entrega de webhook

```text
kobana dashboards webhook-deliveries get [ID] [OPÇÕES]
```

`GET /v1/webhook_deliveries/{id}`

Retorna uma entrega: requisição (redigida), resposta, tentativas e o endpoint de destino.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A entrega. |
| `404` | Recurso não encontrado na organização atual. |
