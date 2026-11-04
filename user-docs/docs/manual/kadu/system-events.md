---
id: system-events
title: "kobana kadu system-events"
sidebar_label: "system-events"
description: "Eventos de sistema (100 mais recentes)"
---

# kobana kadu system-events

Eventos de sistema (100 mais recentes)

Comandos de `kobana kadu system-events`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`system-events`](#system-events) | `GET /v1/system_events` | Eventos de sistema (100 mais recentes) |

## system-events

Eventos de sistema (100 mais recentes)

```text
kobana kadu system-events [OPÇÕES]
```

`GET /v1/system_events`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `slug` | `string` | não |  |
| `workspace_id` | `string` | não |  |
| `task_id` | `string` | não |  |
| `execution_id` | `string` | não |  |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — filtro inválido |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
