---
id: team-roles
title: "kobana dashboards team-roles"
sidebar_label: "team-roles"
---

# kobana dashboards team-roles


Comandos de `kobana dashboards team-roles`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`team-roles list`](#team-roles-list) | `GET /v1/team_roles` | Listar papéis do time |
| [`team-roles create`](#team-roles-create) | `POST /v1/team_roles` | Criar papel customizado |
| [`team-roles update`](#team-roles-update) | `PUT /v1/team_roles` | Criar ou atualizar um papel de equipe |
| [`team-roles get`](#team-roles-get) | `GET /v1/team_roles/{id}` | Obter papel |
| [`team-roles delete`](#team-roles-delete) | `DELETE /v1/team_roles/{id}` | Remover papel |

## team-roles list

Listar papéis do time

```text
kobana dashboards team-roles list [OPÇÕES]
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
| `is_active` | `string` | não | Filtra por funções ativas (`true`, o padrão), inativas (`false`) ou todas (`all`). Valores: `true`, `false`, `all`. |
| `is_system` | `boolean,null` | não | Filtra por papéis de sistema (`true`) ou customizados (`false`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de papéis. |
| `404` | Recurso não encontrado na organização atual. |

## team-roles create

Criar papel customizado

```text
kobana dashboards team-roles create [OPÇÕES]
```

`POST /v1/team_roles`

Cria um papel customizado. O `slug` deve começar com letra minúscula e conter apenas letras minúsculas, números e `_`. Com um token de pessoa, ela precisa ter acesso a todos os workspaces da organização (ou ser proprietária): um papel vale em todos eles. Caso contrário, 403.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do papel (exibido na interface). |
| `slug` | `string` | sim | Slug estável usado para referenciar o papel (ex.: `support_manager`). |
| `permissions` | `array<string>` | sim | Lista de slugs de permissão concedidas pelo papel. |
| `description` | `string` | não | Descrição opcional do que o papel concede. |
| `is_active` | `boolean` | não | Indica se o papel está ativo e pode ser atribuído. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema dashboards.team-roles.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Papel criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## team-roles update

Criar ou atualizar um papel de equipe

```text
kobana dashboards team-roles update [OPÇÕES]
```

`PUT /v1/team_roles`

Localiza o papel por `id`, `public_id` ou `external_id`. O `slug` não tem restrição de unicidade e por isso **não** identifica um registro. Encontrando, atualiza; caso contrário, cria e devolve `201`. Com um token de pessoa, ela precisa ter acesso a todos os workspaces da organização (ou ser proprietária): um papel vale em todos eles. Caso contrário, 403.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do papel (exibido na interface). |
| `slug` | `string` | não | Slug estável usado para referenciar o papel (ex.: `support_manager`). |
| `description` | `string` | não | Descrição opcional do que o papel concede. |
| `permissions` | `array<string>` | não | Lista de slugs de permissão concedidas pelo papel. |
| `is_active` | `boolean` | não | Indica se o papel está ativo e pode ser atribuído. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema dashboards.team-roles.update`.

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
kobana dashboards team-roles get [ID] [OPÇÕES]
```

`GET /v1/team_roles/{id}`

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do papel. |
| `404` | Recurso não encontrado na organização atual. |

## team-roles delete

Remover papel

```text
kobana dashboards team-roles delete [ID] [OPÇÕES]
```

`DELETE /v1/team_roles/{id}`

Realiza soft delete do papel. Papéis de sistema (`is_system = true`) não podem ser removidos. Com um token de pessoa, ela precisa ter acesso a todos os workspaces da organização (ou ser proprietária): um papel vale em todos eles. Caso contrário, 403.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Papel removido. |
| `404` | Recurso não encontrado na organização atual. |
