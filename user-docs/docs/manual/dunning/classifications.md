---
id: classifications
title: "kobana dunning classifications"
sidebar_label: "classifications"
---

# kobana dunning classifications


Comandos de `kobana dunning classifications`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`classifications list`](#classifications-list) | `GET /v1/classifications` | Listar classificações |
| [`classifications create`](#classifications-create) | `POST /v1/classifications` | Criar classificação |
| [`classifications get`](#classifications-get) | `GET /v1/classifications/{id}` | Consultar classificação |
| [`classifications update`](#classifications-update) | `PUT /v1/classifications/{id}` | Atualizar classificação |
| [`classifications delete`](#classifications-delete) | `DELETE /v1/classifications/{id}` | Excluir classificação |

## classifications list

Listar classificações

```text
kobana dunning classifications list [OPÇÕES]
```

`GET /v1/classifications`

Lista as classificações da organização, por padrão ordenadas por `priority` e `name`. Use `include=_count` para os contadores.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `is_default` | `string` | não | Filtra pela classificação padrão (`true` \| `false`). Valores: `true`, `false`. |
| `auto_assign` | `string` | não | Filtra por atribuição automática (`true` \| `false`). Valores: `true`, `false`. |
| `activated_at` | `string` | não | Filtra por ativação: `true` = ativas, `false` ou `null` = inativas. Valores: `true`, `false`, `null`. |
| `include` | `string` | não | Relações a embutir. Suportado: `_count` (clientes e réguas). |
| `sort` | `string` | não | Campo de ordenação (`name` \| `code` \| `priority` \| `created_at` \| `updated_at`). Valores: `name`, `code`, `priority`, `createdAt`, `updatedAt`. |
| `order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de classificações. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## classifications create

Criar classificação

```text
kobana dunning classifications create [OPÇÕES]
```

`POST /v1/classifications`

Cria uma classificação. Marcar `is_default` desmarca a padrão anterior; `code` duplicado falha com 409.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome da classificação. |
| `code` | `string` | sim | Código estável (minúsculas, números e underscore; único na organização). |
| `description` | `string` | não | Descrição da classificação. |
| `color` | `string` | não | Cor hexadecimal exibida na interface (ex.: `#22c55e`). |
| `priority` | `integer` | não | Prioridade de avaliação (menor avalia primeiro). |
| `is_default` | `boolean` | não | true quando é a classificação padrão da organização (única). |
| `auto_assign` | `boolean` | não | true quando clientes são atribuídos automaticamente pelos critérios. |
| `criteria` | `object` | não | Critérios de atribuição automática (comportamento de pagamento). |
| `activated_at` | `string (date-time)` | não | Momento da ativação (null = inativa). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.classifications.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Classificação criada (com contadores). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## classifications get

Consultar classificação

```text
kobana dunning classifications get [ID] [OPÇÕES]
```

`GET /v1/classifications/{id}`

Retorna uma classificação pelo ID. Use `include=_count` para os contadores.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `include` | `string` | não | Relações a embutir. Suportado: `_count` (clientes e réguas). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Classificação encontrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## classifications update

Atualizar classificação

```text
kobana dunning classifications update [ID] [OPÇÕES]
```

`PUT /v1/classifications/{id}`

Atualiza uma classificação (parcial). `code` duplicado falha com 409.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome da classificação. |
| `code` | `string` | não | Código estável (minúsculas, números e underscore; único na organização). |
| `description` | `string` | não | Descrição da classificação. |
| `color` | `string` | não | Cor hexadecimal exibida na interface (ex.: `#22c55e`). |
| `priority` | `integer` | não | Prioridade de avaliação (menor avalia primeiro). |
| `is_default` | `boolean` | não | true quando é a classificação padrão da organização (única). |
| `auto_assign` | `boolean` | não | true quando clientes são atribuídos automaticamente pelos critérios. |
| `criteria` | `object` | não | Critérios de atribuição automática (comportamento de pagamento). |
| `activated_at` | `string (date-time)` | não | Momento da ativação (null = inativa). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.classifications.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Classificação atualizada (com contadores). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## classifications delete

Excluir classificação

```text
kobana dunning classifications delete [ID] [OPÇÕES]
```

`DELETE /v1/classifications/{id}`

Exclui (soft delete) uma classificação. Falha com 409 se houver clientes ou réguas associados. O `code` continua reservado.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Classificação excluída. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |
