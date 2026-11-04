---
id: templates
title: "kobana dunning templates"
sidebar_label: "templates"
---

# kobana dunning templates


Comandos de `kobana dunning templates`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`templates list`](#templates-list) | `GET /v1/templates` | Listar templates |
| [`templates create`](#templates-create) | `POST /v1/templates` | Criar template |
| [`templates get`](#templates-get) | `GET /v1/templates/{id}` | Consultar template |
| [`templates update`](#templates-update) | `PUT /v1/templates/{id}` | Atualizar template |
| [`templates delete`](#templates-delete) | `DELETE /v1/templates/{id}` | Excluir template |
| [`templates duplicate`](#templates-duplicate) | `POST /v1/templates/{id}/duplicate` | Duplicar template |

## templates list

Listar templates

```text
kobana dunning templates list [OPÇÕES]
```

`GET /v1/templates`

Lista os templates de mensagem da organização com paginação, busca e filtros.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `search` | `string` | não | Busca por nome, assunto ou corpo. |
| `channel` | `string` | não | Filtra pelo canal. Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | não | Filtra pela categoria. Valores: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `tone` | `string` | não | Filtra pelo tom. Valores: `friendly`, `neutral`, `firm`, `urgent`. |
| `is_active` | `string` | não | Filtra por ativação: `true` = ativos, `false` = inativos. Valores: `true`, `false`. |
| `sort_by` | `string` | não | Campo de ordenação (nome público em snake_case, ex.: `due_date`). Valores: `name`, `channel`, `category`, `createdAt`, `updatedAt`. |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de templates. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## templates create

Criar template

```text
kobana dunning templates create [OPÇÕES]
```

`POST /v1/templates`

Cria um template de mensagem. Falha com 409 se já existir template com o mesmo `name` ou `external_id`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do template (único na organização). |
| `channel` | `string` | sim | Canal de envio (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | sim | Categoria (`reminder` \| `overdue` \| `negotiation` \| `negativation_warning` \| `protest_warning` \| `payment_confirmation`). Valores: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `body` | `string` | sim | Corpo da mensagem, com variáveis no formato `{{variavel}}`. |
| `tone` | `string` | não | Tom da mensagem (`friendly` \| `neutral` \| `firm` \| `urgent`). Valores: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `subject` | `string` | não | Assunto (usado em e-mail). |
| `variables` | `array<string>` | não | Variáveis disponíveis no corpo/assunto. |
| `activated_at` | `string (date-time)` | não | Momento da ativação (null = inativo). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações; único na organização). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.templates.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Template criado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## templates get

Consultar template

```text
kobana dunning templates get [ID] [OPÇÕES]
```

`GET /v1/templates/{id}`

Retorna um template pelo ID.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Template encontrado. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## templates update

Atualizar template

```text
kobana dunning templates update [ID] [OPÇÕES]
```

`PUT /v1/templates/{id}`

Atualiza um template (parcial). Falha com 409 se o novo `name` ou `external_id` já estiver em uso.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do template (único na organização). |
| `channel` | `string` | não | Canal de envio (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | não | Categoria (`reminder` \| `overdue` \| `negotiation` \| `negativation_warning` \| `protest_warning` \| `payment_confirmation`). Valores: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `tone` | `string` | não | Tom da mensagem (`friendly` \| `neutral` \| `firm` \| `urgent`). Valores: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `subject` | `string` | não | Assunto (usado em e-mail). |
| `body` | `string` | não | Corpo da mensagem, com variáveis no formato `{{variavel}}`. |
| `variables` | `array<string>` | não | Variáveis disponíveis no corpo/assunto. |
| `activated_at` | `string (date-time)` | não | Momento da ativação (null = inativo). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações; único na organização). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.templates.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Template atualizado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## templates delete

Excluir template

```text
kobana dunning templates delete [ID] [OPÇÕES]
```

`DELETE /v1/templates/{id}`

Exclui (soft delete) um template. Falha com 409 se ele estiver em uso por etapas de régua.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Template excluído. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## templates duplicate

Duplicar template

```text
kobana dunning templates duplicate [ID] [OPÇÕES]
```

`POST /v1/templates/{id}/duplicate`

Cria uma cópia do template com o nome informado. A cópia nasce inativa (`activated_at` null) e sem `external_id`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do template (único na organização). |

Outros campos e objetos aninhados: `kobana schema dunning.templates.duplicate`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Template duplicado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |
