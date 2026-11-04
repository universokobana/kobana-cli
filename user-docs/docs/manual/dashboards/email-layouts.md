---
id: email-layouts
title: "kobana dashboards email-layouts"
sidebar_label: "email-layouts"
---

# kobana dashboards email-layouts


Comandos de `kobana dashboards email-layouts`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`email-layouts list`](#email-layouts-list) | `GET /v1/email_layouts` | Listar layouts de e-mail |
| [`email-layouts create`](#email-layouts-create) | `POST /v1/email_layouts` | Criar layout de e-mail |
| [`email-layouts init-defaults`](#email-layouts-init-defaults) | `POST /v1/email_layouts/init_defaults` | Criar layout padrão |
| [`email-layouts get`](#email-layouts-get) | `GET /v1/email_layouts/{id}` | Consultar layout de e-mail |
| [`email-layouts update`](#email-layouts-update) | `PATCH /v1/email_layouts/{id}` | Atualizar layout de e-mail |
| [`email-layouts delete`](#email-layouts-delete) | `DELETE /v1/email_layouts/{id}` | Excluir layout de e-mail |

## email-layouts list

Listar layouts de e-mail

```text
kobana dashboards email-layouts list [OPÇÕES]
```

`GET /v1/email_layouts`

Lista os layouts de e-mail da organização, o padrão primeiro. Retorna o conjunto inteiro, sem paginação.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Layouts da organização. |
| `404` | Recurso não encontrado na organização atual. |

## email-layouts create

Criar layout de e-mail

```text
kobana dashboards email-layouts create [OPÇÕES]
```

`POST /v1/email_layouts`

Cria um layout. O HTML precisa conter o marcador `{{content}}`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do layout (obrigatório na criação). |
| `html_body` | `string` | sim | Documento HTML com o marcador `{{content}}` (obrigatório na criação; sem o marcador, 422). |
| `is_default` | `boolean` | não | Torna este o layout padrão; o padrão anterior deixa de ser. |

Outros campos e objetos aninhados: `kobana schema dashboards.email-layouts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Layout criado. |
| `400` | Requisição inválida (campo obrigatório ausente, tipo ou variáveis inválidos). |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## email-layouts init-defaults

Criar layout padrão

```text
kobana dashboards email-layouts init-defaults [OPÇÕES]
```

`POST /v1/email_layouts/init_defaults`

Garante que a organização tenha um layout padrão: retorna o atual ou cria o embutido. Nunca sobrescreve um layout.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O layout padrão. |
| `404` | Recurso não encontrado na organização atual. |

## email-layouts get

Consultar layout de e-mail

```text
kobana dashboards email-layouts get [ID] [OPÇÕES]
```

`GET /v1/email_layouts/{id}`

Retorna um layout da organização.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O layout. |
| `404` | Recurso não encontrado na organização atual. |

## email-layouts update

Atualizar layout de e-mail

```text
kobana dashboards email-layouts update [ID] [OPÇÕES]
```

`PATCH /v1/email_layouts/{id}`

Atualiza nome, HTML (que deve manter `{{content}}`), padrão ou ativo. Só os campos enviados mudam.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do layout (obrigatório na criação). |
| `html_body` | `string` | não | Documento HTML com o marcador `{{content}}` (obrigatório na criação; sem o marcador, 422). |
| `is_default` | `boolean` | não | Torna este o layout padrão; o padrão anterior deixa de ser. |
| `is_active` | `boolean` | não | Ativa ou desativa o layout. |

Outros campos e objetos aninhados: `kobana schema dashboards.email-layouts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Layout atualizado. |
| `400` | Requisição inválida (campo obrigatório ausente, tipo ou variáveis inválidos). |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## email-layouts delete

Excluir layout de e-mail

```text
kobana dashboards email-layouts delete [ID] [OPÇÕES]
```

`DELETE /v1/email_layouts/{id}`

Exclui definitivamente um layout. Os e-mails passam a usar o layout padrão (ou o embutido).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Layout excluído. |
| `404` | Recurso não encontrado na organização atual. |
