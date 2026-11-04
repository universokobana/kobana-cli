---
id: audit-logs
title: "kobana dunning audit-logs"
sidebar_label: "audit-logs"
description: "Consultar trilha de auditoria"
---

# kobana dunning audit-logs

Consultar trilha de auditoria

Comandos de `kobana dunning audit-logs`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`audit-logs`](#audit-logs) | `GET /v1/audit-logs` | Consultar trilha de auditoria |

## audit-logs

Consultar trilha de auditoria

```text
kobana dunning audit-logs [OPÇÕES]
```

`GET /v1/audit-logs`

Lista os registros da trilha de auditoria da organização (mais recentes primeiro), com filtros por ação, entidade, ator e período.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `action` | `string` | não | Filtra por ação (busca parcial, `contains`). |
| `entity_type` | `string` | não | Filtra pelo tipo de entidade. |
| `entity_id` | `string` | não | Filtra pela entidade (UUID). |
| `actor_type` | `string` | não | Filtra pelo tipo de ator (`user` \| `system` \| `job`). Valores: `user`, `system`, `job`. |
| `actor_id` | `string` | não | Filtra pelo ator (ID do usuário, nome do job ou do sistema). |
| `from` | `string` | não | Início do período (ISO 8601, inclusive). |
| `to` | `string` | não | Fim do período (ISO 8601, inclusive). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de registros de auditoria. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
