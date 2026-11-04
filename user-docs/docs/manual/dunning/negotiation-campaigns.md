---
id: negotiation-campaigns
title: "kobana dunning negotiation-campaigns"
sidebar_label: "negotiation-campaigns"
---

# kobana dunning negotiation-campaigns


Comandos de `kobana dunning negotiation-campaigns`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`negotiation-campaigns list`](#negotiation-campaigns-list) | `GET /v1/negotiation-campaigns` | Listar campanhas |
| [`negotiation-campaigns create`](#negotiation-campaigns-create) | `POST /v1/negotiation-campaigns` | Criar campanha |
| [`negotiation-campaigns end`](#negotiation-campaigns-end) | `POST /v1/negotiation-campaigns/{id}/end` | Encerrar campanha |

## negotiation-campaigns list

Listar campanhas

```text
kobana dunning negotiation-campaigns list [OPÇÕES]
```

`GET /v1/negotiation-campaigns`

Lista as campanhas de negociação da organização (ativas e encerradas), da mais recente para a mais antiga. Sem paginação.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Campanhas da organização (envelope `campaigns`). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## negotiation-campaigns create

Criar campanha

```text
kobana dunning negotiation-campaigns create [OPÇÕES]
```

`POST /v1/negotiation-campaigns`

Cria e ativa uma campanha de desconto. O portal do devedor passa a oferecê-la às cobranças na faixa de atraso configurada.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome da campanha. |
| `discount_percentage` | `number` | sim | Percentual de desconto (0-100). Devolvido como string decimal. |
| `max_installments` | `integer` | não | Máximo de parcelas permitido (1 = só à vista). |
| `min_days_overdue` | `integer` | não | Início da faixa de dias de atraso em que a campanha vale (null = sem mínimo). |
| `max_days_overdue` | `integer` | não | Fim da faixa de dias de atraso em que a campanha vale (null = sem máximo). |
| `valid_until` | `string (date)` | não | Data-limite de validade da campanha (null = sem prazo). |
| `terms` | `string` | não | Condições exibidas ao devedor. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |

Outros campos e objetos aninhados: `kobana schema dunning.negotiation-campaigns.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Campanha criada e ativada (envelope `campaign`). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## negotiation-campaigns end

Encerrar campanha

```text
kobana dunning negotiation-campaigns end [ID] [OPÇÕES]
```

`POST /v1/negotiation-campaigns/{id}/end`

Encerra a campanha (desativa; `activated_at` vira null). O registro é mantido e o portal deixa de oferecê-la. É uma escrita (`write`), não uma exclusão — substituiu o `DELETE /negotiation-campaigns/{id}`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Campanha encerrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
