---
id: notification-preferences
title: "kobana dunning notification-preferences"
sidebar_label: "notification-preferences"
---

# kobana dunning notification-preferences


Comandos de `kobana dunning notification-preferences`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`notification-preferences list`](#notification-preferences-list) | `GET /v1/notification-preferences` | Consultar preferências de notificação |
| [`notification-preferences update`](#notification-preferences-update) | `PUT /v1/notification-preferences` | Atualizar preferências de notificação |

## notification-preferences list

Consultar preferências de notificação

```text
kobana dunning notification-preferences list [OPÇÕES]
```

`GET /v1/notification-preferences`

Retorna o gate efetivo por tipo de mensagem — tipos sem registro salvo aparecem com todos os canais ligados.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Preferências efetivas por tipo. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## notification-preferences update

Atualizar preferências de notificação

```text
kobana dunning notification-preferences update [OPÇÕES]
```

`PUT /v1/notification-preferences`

Faz upsert das preferências enviadas (parcial: só os tipos incluídos são alterados).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `preferences` | `array<object>` | sim |  |

Outros campos e objetos aninhados: `kobana schema dunning.notification-preferences.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Preferências gravadas. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
