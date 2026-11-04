---
id: rate-limits
title: "kobana dashboards rate-limits"
sidebar_label: "rate-limits"
description: "Consultar limites de requisição"
---

# kobana dashboards rate-limits

Consultar limites de requisição

Comandos de `kobana dashboards rate-limits`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`rate-limits`](#rate-limits) | `GET /v1/rate_limits` | Consultar limites de requisição |

## rate-limits

Consultar limites de requisição

```text
kobana dashboards rate-limits [OPÇÕES]
```

`GET /v1/rate_limits`

Os limites de requisição em vigor para a organização, se são personalizados (definidos pela equipe Kobana) e os padrões da plataforma. Somente leitura.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Os limites. |
| `404` | Recurso não encontrado na organização atual. |
