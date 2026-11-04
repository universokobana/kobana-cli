---
id: audit-logs
title: "kobana dashboards audit-logs"
sidebar_label: "audit-logs"
---

# kobana dashboards audit-logs


Comandos de `kobana dashboards audit-logs`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`audit-logs summary`](#audit-logs-summary) | `GET /v1/audit_logs/summary` | Resumo da atividade auditada |
| [`audit-logs list`](#audit-logs-list) | `GET /v1/audit_logs` | Listar registros de auditoria |
| [`audit-logs get`](#audit-logs-get) | `GET /v1/audit_logs/{id}` | Consultar registro de auditoria |

## audit-logs summary

Resumo da atividade auditada

```text
kobana dashboards audit-logs summary [OPÇÕES]
```

`GET /v1/audit_logs/summary`

Atividade das últimas `hours` horas: total de registros, as 10 ações mais frequentes e os 20 registros mais recentes. Exige a permissão de exportar auditoria.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `hours` | `number` | não | Tamanho da janela em horas, de 1 a 8760. Padrão 24. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resumo da atividade. |
| `400` | Requisição inválida: corpo malformado, campo obrigatório ausente ou filtro com valor inválido. |
| `404` | Recurso não encontrado na organização atual. |

## audit-logs list

Listar registros de auditoria

```text
kobana dashboards audit-logs list [OPÇÕES]
```

`GET /v1/audit_logs`

Trilha de auditoria do workspace e da organização, do mais recente ao mais antigo. A trilha não pode ser alterada.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `user_id` | `string` | não | Identificador exato do usuário. |
| `action` | `string` | não | Ação exata (ex.: `webhook.created`). |
| `resource_type` | `string` | não | Tipo exato do recurso (ex.: `webhook`). |
| `resource_id` | `string` | não | ID exato do recurso. |
| `search` | `string` | não | Trecho da ação, do tipo de recurso, do ID ou do e-mail do usuário, sem diferenciar maiúsculas. |
| `date_from` | `string` | não | Limite inferior de `created_at` (data ou data-hora ISO 8601). |
| `date_to` | `string` | não | Limite superior de `created_at` (data ou data-hora ISO 8601). Uma data sem hora cobre o dia inteiro (UTC). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de registros de auditoria. |
| `400` | Requisição inválida: corpo malformado, campo obrigatório ausente ou filtro com valor inválido. |
| `404` | Recurso não encontrado na organização atual. |

## audit-logs get

Consultar registro de auditoria

```text
kobana dashboards audit-logs get [ID] [OPÇÕES]
```

`GET /v1/audit_logs/{id}`

Retorna um registro de auditoria com valores anteriores e novos.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O registro de auditoria. |
| `404` | Recurso não encontrado na organização atual. |
