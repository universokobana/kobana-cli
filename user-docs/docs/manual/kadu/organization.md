---
id: organization
title: "kobana kadu organization"
sidebar_label: "organization"
description: "Organização do token"
---

# kobana kadu organization

Organização do token

Comandos de `kobana kadu organization`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Organização do token |
| [`organization slack-installations list`](#organization-slack-installations-list) | `GET /v1/organization/slack_installations` | Instalações do bot do Slack da organização |
| [`organization slack-installations update`](#organization-slack-installations-update) | `PATCH /v1/organization/slack_installations/{installationId}` | Configura uma instalação do Slack |
| [`organization slack-installations delete`](#organization-slack-installations-delete) | `DELETE /v1/organization/slack_installations/{installationId}` | Remove uma instalação do Slack |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Atualiza a organização do token |

## organization

Organização do token

```text
kobana kadu organization [OPÇÕES] [COMANDO]
```

`GET /v1/organization`

Devolve **apenas a organização do token** (a da claim `org`). Não recebe id e não lista: não há como ler outra organização por esta API.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## organization slack-installations list

Instalações do bot do Slack da organização

```text
kobana kadu organization slack-installations list [OPÇÕES]
```

`GET /v1/organization/slack_installations`

Workspaces do Slack ligados à organização do token (sempre a do token; sem id).

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## organization slack-installations update

Configura uma instalação do Slack

```text
kobana kadu organization slack-installations update [INSTALLATIONID] [OPÇÕES]
```

`PATCH /v1/organization/slack_installations/{installationId}`

Onde o bot responde (`allowed_channel_types`), arquivos de saída como link ou anexo (`files_mode`, `max_upload_bytes` até 100 MB) e detalhe na thread (`log_detail`).

**Argumentos do caminho:** `installationId`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `allowed_channel_types` | `array<string>` | não |  |
| `files_mode` | `string` | não | Valores: `link`, `upload`. |
| `max_upload_bytes` | `integer` | não |  |
| `log_detail` | `string` | não | Valores: `milestones`, `verbose`. |

Outros campos e objetos aninhados: `kobana schema kadu.organization.slack-installations.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## organization slack-installations delete

Remove uma instalação do Slack

```text
kobana kadu organization slack-installations delete [INSTALLATIONID] [OPÇÕES]
```

`DELETE /v1/organization/slack_installations/{installationId}`

Exige o nível `delete` (ou `all`). Revoga o token do bot no Slack e desfaz as conexões de conta.

**Argumentos do caminho:** `installationId`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## organization update

Atualiza a organização do token

```text
kobana kadu organization update [OPÇÕES]
```

`PATCH /v1/organization`

Altera **apenas a organização do token** (a da claim `org`); não recebe id. `monthly_cost_limit` e `bypass_subscription` são definidos pela Kobana e recusados (403).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não |  |
| `timezone` | `string` | não |  |
| `default_locale` | `string` | não | Valores: `pt-BR`, `en-US`, `es-ES`. |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.organization.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
