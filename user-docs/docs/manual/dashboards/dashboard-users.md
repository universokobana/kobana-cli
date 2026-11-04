---
id: dashboard-users
title: "kobana dashboards dashboard-users"
sidebar_label: "dashboard-users"
---

# kobana dashboards dashboard-users


Comandos de `kobana dashboards dashboard-users`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`dashboard-users workspaces`](#dashboard-users-workspaces) | `GET /v1/dashboard_users/{id}/workspaces` | Consultar os workspaces de um membro |
| [`dashboard-users workspaces update`](#dashboard-users-workspaces-update) | `PUT /v1/dashboard_users/{id}/workspaces` | Definir os workspaces de um membro |
| [`dashboard-users list`](#dashboard-users-list) | `GET /v1/dashboard_users` | Listar usuários do dashboard |
| [`dashboard-users create`](#dashboard-users-create) | `POST /v1/dashboard_users` | Convidar usuário do dashboard |
| [`dashboard-users update`](#dashboard-users-update) | `PUT /v1/dashboard_users` | Criar ou atualizar um usuário do painel |
| [`dashboard-users get`](#dashboard-users-get) | `GET /v1/dashboard_users/{id}` | Obter usuário do dashboard |
| [`dashboard-users delete`](#dashboard-users-delete) | `DELETE /v1/dashboard_users/{id}` | Remover usuário do dashboard |
| [`dashboard-users resend`](#dashboard-users-resend) | `POST /v1/dashboard_users/{id}/resend` | Reenviar convite de membro |

## dashboard-users workspaces

Consultar os workspaces de um membro

```text
kobana dashboards dashboard-users workspaces [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/dashboard_users/{id}/workspaces`

Lista todos os workspaces da organização, marcando os que o membro acessa — também é a lista do que pode ser concedido.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Workspaces da organização com o acesso do membro. |
| `404` | Recurso não encontrado na organização atual. |

## dashboard-users workspaces update

Definir os workspaces de um membro

```text
kobana dashboards dashboard-users workspaces update [ID] [OPÇÕES]
```

`PUT /v1/dashboard_users/{id}/workspaces`

Substitui o conjunto de workspaces do membro. Ninguém altera o próprio acesso (400), só um dono altera o acesso de um dono, e só um dono concede ou revoga um workspace que ele mesmo não acessa (403). Um token de máquina pode conceder qualquer workspace, mas nunca altera o acesso de um dono. Tirar o último dono de um workspace responde 403.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `workspace_ids` | `array<string (uuid)>` | sim | O conjunto final de workspaces que o membro deve acessar. Os removidos são pausados, não apagados. |

Outros campos e objetos aninhados: `kobana schema dashboards.dashboard-users.workspaces.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Workspaces da organização com o acesso do membro. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## dashboard-users list

Listar usuários do dashboard

```text
kobana dashboards dashboard-users list [OPÇÕES]
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

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de usuários do dashboard. |
| `404` | Recurso não encontrado na organização atual. |

## dashboard-users create

Convidar usuário do dashboard

```text
kobana dashboards dashboard-users create [OPÇÕES]
```

`POST /v1/dashboard_users`

Convida um novo membro para o time da organização. É obrigatório informar `email` e ao menos um papel em `role_ids`. O usuário recebe um convite por e-mail e entra com status `pending_invitation`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string (email)` | sim | E-mail do usuário a ser convidado. |
| `role_ids` | `array<string (uuid)>` | sim | Lista de UUIDs de papéis (`team_roles`) a atribuir ao usuário. Deve conter ao menos um item. |
| `name` | `string` | não | Nome do usuário (opcional — pode ser preenchido pelo usuário no primeiro acesso). |
| `external_id` | `string` | não | Identificador externo do usuário, guardado como atributo do membro. Não define a identidade do usuário: ela é sempre resolvida pelo `email`. |
| `permissions` | `array<string>` | não | Permissões ad-hoc adicionais (além das herdadas pelos papéis). Aceita os mesmos slugs de permissão. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema dashboards.dashboard-users.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Usuário convidado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Dados inválidos. Com o código `IDENTITY_CONFLICT`, o `external_id` informado é o identificador de outra identidade, com outro e-mail. |

## dashboard-users update

Criar ou atualizar um usuário do painel

```text
kobana dashboards dashboard-users update [OPÇÕES]
```

`PUT /v1/dashboard_users`

Localiza o usuário por `id`, `public_id`, `external_id` ou `email` — este último é único por organização. Encontrando, atualiza; caso contrário, cria e devolve `201`. Ao atualizar, com um token de pessoa, ela precisa ter acesso a todos os workspaces em que o membro trabalha (ou ser proprietária da organização); caso contrário, 403.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string (email)` | não | E-mail do usuário. Além de campo do recurso, serve como chave natural da busca. |
| `name` | `string` | não | Nome do usuário (opcional — pode ser preenchido pelo usuário no primeiro acesso). |
| `external_id` | `string` | não | Identificador externo do usuário, guardado como atributo do membro. Não define a identidade do usuário: ela é sempre resolvida pelo `email`. |
| `role_ids` | `array<string (uuid)>` | não | Lista de UUIDs de papéis (`team_roles`) a atribuir ao usuário. Deve conter ao menos um item. |
| `permissions` | `array<string>` | não | Permissões ad-hoc adicionais (além das herdadas pelos papéis). Aceita os mesmos slugs de permissão. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema dashboards.dashboard-users.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Dados inválidos. Com o código `IDENTITY_CONFLICT`, o `external_id` informado é o identificador de outra identidade, com outro e-mail. |

## dashboard-users get

Obter usuário do dashboard

```text
kobana dashboards dashboard-users get [ID] [OPÇÕES]
```

`GET /v1/dashboard_users/{id}`

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do usuário do dashboard. |
| `404` | Recurso não encontrado na organização atual. |

## dashboard-users delete

Remover usuário do dashboard

```text
kobana dashboards dashboard-users delete [ID] [OPÇÕES]
```

`DELETE /v1/dashboard_users/{id}`

Remove o usuário do dashboard da organização. Ação irreversível — o usuário precisa ser convidado novamente para recuperar o acesso. Com um token de pessoa, ela precisa ter acesso a todos os workspaces em que o membro trabalha (ou ser proprietária da organização); caso contrário, 403.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Usuário removido. |
| `404` | Recurso não encontrado na organização atual. |

## dashboard-users resend

Reenviar convite de membro

```text
kobana dashboards dashboard-users resend [ID] [OPÇÕES]
```

`POST /v1/dashboard_users/{id}/resend`

Reenvia o e-mail de convite, com um novo link de acesso, a um membro cujo convite ainda está pendente. Limitado a 5 envios por hora por destinatário (429 acima disso).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Convite reenviado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | O membro não tem convite pendente (`INVALID_STATUS`). |
| `502` | Não foi possível enviar o e-mail (`EMAIL_ERROR`). |
