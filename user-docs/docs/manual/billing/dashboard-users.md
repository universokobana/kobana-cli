---
id: dashboard-users
title: "kobana billing dashboard-users"
sidebar_label: "dashboard-users"
---

# kobana billing dashboard-users


Comandos de `kobana billing dashboard-users`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`dashboard-users list`](#dashboard-users-list) | `GET /v1/dashboard_users` | Listar usuários do dashboard |
| [`dashboard-users create`](#dashboard-users-create) | `POST /v1/dashboard_users` | Convidar usuário do dashboard |
| [`dashboard-users update`](#dashboard-users-update) | `PUT /v1/dashboard_users` | Cria ou atualiza um usuário do painel |
| [`dashboard-users get`](#dashboard-users-get) | `GET /v1/dashboard_users/{id}` | Obter usuário do dashboard |
| [`dashboard-users delete`](#dashboard-users-delete) | `DELETE /v1/dashboard_users/{id}` | Remover usuário do dashboard |

## dashboard-users list

Listar usuários do dashboard

```text
kobana billing dashboard-users list [OPÇÕES]
```

`GET /v1/dashboard_users`

Retorna uma lista paginada dos usuários do dashboard (membros do time) da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca textual por nome ou e-mail. |
| `status` | `string` | não | Filtra por status: `all` (padrão), `active`, `pending_invitation` ou `suspended`. Valores: `all`, `active`, `pending_invitation`, `suspended`. |
| `role` | `string` | não | Filtra pelos usuários que possuem o papel informado (slug do papel). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de usuários do dashboard. |
| `404` | Recurso não encontrado na organização atual. |

## dashboard-users create

Convidar usuário do dashboard

```text
kobana billing dashboard-users create [OPÇÕES]
```

`POST /v1/dashboard_users`

Convida um novo membro para o time da organização. É obrigatório informar `email` e ao menos um papel em `role_ids`. O usuário recebe um convite por e-mail e entra com status `pending_invitation`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string (email)` | sim | E-mail do usuário a ser convidado. |
| `role_ids` | `array<string (uuid)>` | sim | Lista de UUIDs de papéis (`team_roles`) a atribuir ao usuário. Deve conter ao menos um item. |
| `name` | `string` | não | Nome do usuário (opcional — pode ser preenchido pelo usuário no primeiro acesso). |
| `external_id` | `string` | não | Identificador externo do usuário no Kobana OAuth, quando já conhecido. |
| `permissions` | `array<string>` | não | Permissões ad-hoc adicionais (além das herdadas pelos papéis). Aceita os mesmos slugs de permissão. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.dashboard-users.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Usuário convidado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## dashboard-users update

Cria ou atualiza um usuário do painel

```text
kobana billing dashboard-users update [OPÇÕES]
```

`PUT /v1/dashboard_users`

Localiza o usuário por `id`, `public_id`, `external_id` ou `email` — este último é único por organização. Encontrando, atualiza; caso contrário, cria e devolve `201`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string (email)` | não | E-mail do usuário. Além de campo do recurso, serve como chave natural da busca. |
| `name` | `string` | não | Nome do usuário (opcional — pode ser preenchido pelo usuário no primeiro acesso). |
| `external_id` | `string` | não | Identificador externo do usuário no Kobana OAuth, quando já conhecido. |
| `role_ids` | `array<string (uuid)>` | não | Lista de UUIDs de papéis (`team_roles`) a atribuir ao usuário. Deve conter ao menos um item. |
| `permissions` | `array<string>` | não | Permissões ad-hoc adicionais (além das herdadas pelos papéis). Aceita os mesmos slugs de permissão. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.dashboard-users.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## dashboard-users get

Obter usuário do dashboard

```text
kobana billing dashboard-users get [ID] [OPÇÕES]
```

`GET /v1/dashboard_users/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do usuário do dashboard. |
| `404` | Recurso não encontrado na organização atual. |

## dashboard-users delete

Remover usuário do dashboard

```text
kobana billing dashboard-users delete [ID] [OPÇÕES]
```

`DELETE /v1/dashboard_users/{id}`

Remove o usuário do dashboard da organização. Ação irreversível — o usuário precisa ser convidado novamente para recuperar o acesso.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Usuário removido. |
| `404` | Recurso não encontrado na organização atual. |
