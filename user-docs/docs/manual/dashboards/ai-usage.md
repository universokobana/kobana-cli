---
id: ai-usage
title: "kobana dashboards ai-usage"
sidebar_label: "ai-usage"
description: "Consultar uso de IA"
---

# kobana dashboards ai-usage

Consultar uso de IA

Comandos de `kobana dashboards ai-usage`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`ai-usage`](#ai-usage) | `GET /v1/ai_usage` | Consultar uso de IA |

## ai-usage

Consultar uso de IA

```text
kobana dashboards ai-usage [OPÇÕES]
```

`GET /v1/ai_usage`

Consumo de IA, só preço (custo e margem nunca saem): totais, detalhamento diário e últimos turnos do workspace da chamada, mais o acumulado do mês, franquias, limites, saldo de crédito e se o próximo turno é permitido — esses da organização inteira, pois o limite e o crédito são compartilhados. Período padrão: mês corrente (America/Sao_Paulo), no máximo 366 dias.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `start_date` | `string` | não | Primeiro dia (AAAA-MM-DD). |
| `end_date` | `string` | não | Último dia (AAAA-MM-DD). |
| `days` | `integer` | não | Os últimos N dias, em vez de um intervalo de datas. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O uso de IA. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
