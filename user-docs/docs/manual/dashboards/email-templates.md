---
id: email-templates
title: "kobana dashboards email-templates"
sidebar_label: "email-templates"
---

# kobana dashboards email-templates


Comandos de `kobana dashboards email-templates`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`email-templates preview`](#email-templates-preview) | `GET /v1/email_templates/{id}/preview` | Pré-visualizar modelo de e-mail |
| [`email-templates preview create`](#email-templates-preview-create) | `POST /v1/email_templates/{id}/preview` | Renderizar modelo com variáveis |
| [`email-templates list`](#email-templates-list) | `GET /v1/email_templates` | Listar modelos de e-mail |
| [`email-templates create`](#email-templates-create) | `POST /v1/email_templates` | Criar modelo de e-mail |
| [`email-templates init-defaults`](#email-templates-init-defaults) | `POST /v1/email_templates/init_defaults` | Restaurar modelos padrão |
| [`email-templates get`](#email-templates-get) | `GET /v1/email_templates/{id}` | Consultar modelo de e-mail |
| [`email-templates update`](#email-templates-update) | `PATCH /v1/email_templates/{id}` | Atualizar modelo de e-mail |
| [`email-templates delete`](#email-templates-delete) | `DELETE /v1/email_templates/{id}` | Excluir modelo de e-mail |
| [`email-templates duplicate`](#email-templates-duplicate) | `POST /v1/email_templates/{id}/duplicate` | Duplicar modelo de e-mail |

## email-templates preview

Pré-visualizar modelo de e-mail

```text
kobana dashboards email-templates preview [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/email_templates/{id}/preview`

Renderiza o modelo com dados de exemplo, dentro do layout da organização. Nada é enviado.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O e-mail renderizado. |
| `404` | Recurso não encontrado na organização atual. |

## email-templates preview create

Renderizar modelo com variáveis

```text
kobana dashboards email-templates preview create [ID] [OPÇÕES]
```

`POST /v1/email_templates/{id}/preview`

Renderiza o modelo com as `variables` enviadas, sem layout. Nada é enviado.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `variables` | `object` | não | Valores das variáveis `{{nome}}` do modelo. |

Outros campos e objetos aninhados: `kobana schema dashboards.email-templates.preview.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O e-mail renderizado. |
| `400` | Requisição inválida (campo obrigatório ausente, tipo ou variáveis inválidos). |
| `404` | Recurso não encontrado na organização atual. |

## email-templates list

Listar modelos de e-mail

```text
kobana dashboards email-templates list [OPÇÕES]
```

`GET /v1/email_templates`

Lista os modelos de e-mail da própria organização, ordenados por tipo e nome. Retorna o conjunto inteiro, sem paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `type` | `string` | não | Somente modelos deste tipo de notificação. Valores: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `is_active` | `string` | não | `true` para os modelos ativos, `false` para os rascunhos. Valores: `true`, `false`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Modelos da organização. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## email-templates create

Criar modelo de e-mail

```text
kobana dashboards email-templates create [OPÇÕES]
```

`POST /v1/email_templates`

Cria um modelo para um tipo de notificação. Fica como rascunho, a menos que `is_active` seja `true` — nesse caso o modelo ativo anterior do tipo vira rascunho.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do modelo (obrigatório na criação). |
| `type` | `string` | sim | Tipo de notificação que o modelo renderiza (obrigatório). Valores: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `subject` | `string` | sim | Assunto do e-mail, com variáveis `{{nome}}` (obrigatório na criação). |
| `html_body` | `string` | sim | Corpo HTML, com variáveis `{{nome}}` (obrigatório na criação). |
| `text_body` | `string` | não | Corpo alternativo em texto puro. `null` limpa. |
| `bcc` | `anyOf` | não | Lista (ou um único endereço) de e-mails em cópia oculta. Endereços inválidos geram 422; omitido na atualização, mantém a lista atual. |
| `variables` | `array<string>` | não | Nomes das variáveis que o modelo usa. |
| `is_active` | `boolean` | não | Torna este o modelo ativo do tipo; o ativo anterior vira rascunho. Padrão `false` na criação. |

Outros campos e objetos aninhados: `kobana schema dashboards.email-templates.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Modelo criado. |
| `400` | Requisição inválida (campo obrigatório ausente, tipo ou variáveis inválidos). |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## email-templates init-defaults

Restaurar modelos padrão

```text
kobana dashboards email-templates init-defaults [OPÇÕES]
```

`POST /v1/email_templates/init_defaults`

Cria ou restaura o modelo de cada tipo de notificação com o conteúdo padrão e o ativa. Sobrescreve o conteúdo dos modelos existentes da organização.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Uma linha por tipo de notificação. |
| `404` | Recurso não encontrado na organização atual. |

## email-templates get

Consultar modelo de e-mail

```text
kobana dashboards email-templates get [ID] [OPÇÕES]
```

`GET /v1/email_templates/{id}`

Retorna um modelo da organização ou um modelo padrão da plataforma.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O modelo. |
| `404` | Recurso não encontrado na organização atual. |

## email-templates update

Atualizar modelo de e-mail

```text
kobana dashboards email-templates update [ID] [OPÇÕES]
```

`PATCH /v1/email_templates/{id}`

Atualiza um modelo da organização. Só os campos enviados mudam. Modelos padrão da plataforma não podem ser alterados (404).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do modelo (obrigatório na criação). |
| `subject` | `string` | não | Assunto do e-mail, com variáveis `{{nome}}` (obrigatório na criação). |
| `html_body` | `string` | não | Corpo HTML, com variáveis `{{nome}}` (obrigatório na criação). |
| `text_body` | `string` | não | Corpo alternativo em texto puro. `null` limpa. |
| `bcc` | `anyOf` | não | Lista (ou um único endereço) de e-mails em cópia oculta. Endereços inválidos geram 422; omitido na atualização, mantém a lista atual. |
| `variables` | `array<string>` | não | Nomes das variáveis que o modelo usa. |
| `is_active` | `boolean` | não | Torna este o modelo ativo do tipo; o ativo anterior vira rascunho. Padrão `false` na criação. |
| `public_id` | `string (uuid)` | não | Novo identificador público (UUID), quando a organização permite escolhê-lo. |

Outros campos e objetos aninhados: `kobana schema dashboards.email-templates.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Modelo atualizado. |
| `400` | Requisição inválida (campo obrigatório ausente, tipo ou variáveis inválidos). |
| `404` | Recurso não encontrado na organização atual. |
| `409` | O `public_id` informado já está em uso. |
| `422` | Corpo da requisição falhou na validação. |

## email-templates delete

Excluir modelo de e-mail

```text
kobana dashboards email-templates delete [ID] [OPÇÕES]
```

`DELETE /v1/email_templates/{id}`

Exclui definitivamente um modelo inativo da organização. O modelo ativo de um tipo não pode ser excluído (422): ative outro antes.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Modelo excluído. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## email-templates duplicate

Duplicar modelo de e-mail

```text
kobana dashboards email-templates duplicate [ID] [OPÇÕES]
```

`POST /v1/email_templates/{id}/duplicate`

Copia um modelo como rascunho (inativo).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `new_name` | `string` | não | Nome da cópia. Padrão: "&lt;nome> (Cópia)". |

Outros campos e objetos aninhados: `kobana schema dashboards.email-templates.duplicate`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | A cópia criada. |
| `400` | Requisição inválida (campo obrigatório ausente, tipo ou variáveis inválidos). |
| `404` | Recurso não encontrado na organização atual. |
