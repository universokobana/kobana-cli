---
id: team-roles
title: "kobana billing team-roles"
sidebar_label: "team-roles"
---

# kobana billing team-roles


Comandos de `kobana billing team-roles`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`team-roles list`](#team-roles-list) | `GET /v1/team_roles` | Listar papéis do time |
| [`team-roles create`](#team-roles-create) | `POST /v1/team_roles` | Criar papel customizado |
| [`team-roles update`](#team-roles-update) | `PUT /v1/team_roles` | Cria ou atualiza um papel de equipe |
| [`team-roles get`](#team-roles-get) | `GET /v1/team_roles/{id}` | Obter papel |
| [`team-roles delete`](#team-roles-delete) | `DELETE /v1/team_roles/{id}` | Remover papel |

## team-roles list

Listar papéis do time

```text
kobana billing team-roles list [OPÇÕES]
```

`GET /v1/team_roles`

Retorna a lista paginada de papéis (roles) configurados para o time da organização. Inclui tanto papéis de sistema (`is_system = true`) quanto papéis customizados.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca textual por nome ou slug. |
| `is_active` | `boolean,null` | não | Filtra por papéis ativos (`true`) ou inativos (`false`). |
| `is_system` | `boolean,null` | não | Filtra por papéis de sistema (`true`) ou customizados (`false`). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de papéis. |
| `404` | Recurso não encontrado na organização atual. |

## team-roles create

Criar papel customizado

```text
kobana billing team-roles create [OPÇÕES]
```

`POST /v1/team_roles`

Cria um papel customizado. O `slug` deve começar com letra minúscula e conter apenas letras minúsculas, números e `_`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do papel (exibido na interface). |
| `slug` | `string` | sim | Slug estável usado para referenciar o papel (ex.: `billing_manager`). |
| `permissions` | `array<string>` | sim | Lista de slugs de permissão concedidas pelo papel. |
| `description` | `string` | não | Descrição opcional do que o papel concede. |
| `is_active` | `boolean` | não | Indica se o papel está ativo e pode ser atribuído. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.team-roles.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Papel criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## team-roles update

Cria ou atualiza um papel de equipe

```text
kobana billing team-roles update [OPÇÕES]
```

`PUT /v1/team_roles`

Localiza o papel por `id`, `public_id` ou `external_id`. O `slug` não tem restrição de unicidade e por isso **não** identifica um registro. Encontrando, atualiza; caso contrário, cria e devolve `201`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do papel (exibido na interface). |
| `slug` | `string` | não | Slug estável usado para referenciar o papel (ex.: `billing_manager`). |
| `description` | `string` | não | Descrição opcional do que o papel concede. |
| `permissions` | `array<string>` | não | Lista de slugs de permissão concedidas pelo papel. |
| `is_active` | `boolean` | não | Indica se o papel está ativo e pode ser atribuído. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.team-roles.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## team-roles get

Obter papel

```text
kobana billing team-roles get [ID] [OPÇÕES]
```

`GET /v1/team_roles/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do papel. |
| `404` | Recurso não encontrado na organização atual. |

## team-roles delete

Remover papel

```text
kobana billing team-roles delete [ID] [OPÇÕES]
```

`DELETE /v1/team_roles/{id}`

Realiza soft delete do papel. Papéis de sistema (`is_system = true`) não podem ser removidos.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Papel removido. |
| `404` | Recurso não encontrado na organização atual. |
