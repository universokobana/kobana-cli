---
id: usage
title: "kobana dashboards usage"
sidebar_label: "usage"
description: "Consultar consumo da plataforma"
---

# kobana dashboards usage

Consultar consumo da plataforma

Comandos de `kobana dashboards usage`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`usage`](#usage) | `GET /v1/usage` | Consultar consumo da plataforma |

## usage

Consultar consumo da plataforma

```text
kobana dashboards usage [OPÇÕES]
```

`GET /v1/usage`

Para cada métrica (membros, clientes, usuários do portal, integrações…), a série diária no período, o valor mais recente e o pico. Padrão: mês corrente; no máximo 12 meses para trás. Recurso da organização.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `start_date` | `string` | não | Primeiro dia (AAAA-MM-DD). Padrão: dia 1 do mês corrente. |
| `end_date` | `string` | não | Último dia (AAAA-MM-DD). Padrão: hoje. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O consumo. |
| `404` | Recurso não encontrado na organização atual. |
