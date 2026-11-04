---
id: portal-oauth-providers
title: "kobana dashboards portal-oauth-providers"
sidebar_label: "portal-oauth-providers"
---

# kobana dashboards portal-oauth-providers


Comandos de `kobana dashboards portal-oauth-providers`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`portal-oauth-providers list`](#portal-oauth-providers-list) | `GET /v1/portal_oauth_providers` | Listar provedores OIDC |
| [`portal-oauth-providers create`](#portal-oauth-providers-create) | `POST /v1/portal_oauth_providers` | Adicionar provedor OIDC |
| [`portal-oauth-providers test`](#portal-oauth-providers-test) | `POST /v1/portal_oauth_providers/test` | Testar emissor OIDC |
| [`portal-oauth-providers update`](#portal-oauth-providers-update) | `PATCH /v1/portal_oauth_providers/{id}` | Atualizar provedor OIDC |
| [`portal-oauth-providers delete`](#portal-oauth-providers-delete) | `DELETE /v1/portal_oauth_providers/{id}` | Remover provedor OIDC |

## portal-oauth-providers list

Listar provedores OIDC

```text
kobana dashboards portal-oauth-providers list [OPÇÕES]
```

`GET /v1/portal_oauth_providers`

Lista os provedores OpenID Connect com que os usuários entram no portal (conjunto completo, sem paginação).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Provedores OIDC do portal. |
| `404` | Recurso não encontrado na organização atual. |

## portal-oauth-providers create

Adicionar provedor OIDC

```text
kobana dashboards portal-oauth-providers create [OPÇÕES]
```

`POST /v1/portal_oauth_providers`

Adiciona um provedor OpenID Connect ao login do portal.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim |  |
| `issuer_url` | `string (uri)` | sim | URL do emissor OpenID Connect (https em produção). |
| `client_id` | `string` | sim |  |
| `client_secret` | `string` | sim | Somente escrita: guardado criptografado e nunca retornado. |
| `button_label` | `string` | não | Texto do botão de login; quando vazio, usa o nome. |
| `scopes` | `string` | não | Escopos separados por espaço. Padrão: `openid email profile`. |
| `email_claim` | `string` | não | Claim do token que traz o e-mail. Padrão: `email`. |
| `enabled` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-oauth-providers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | O provedor (sem o secret). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## portal-oauth-providers test

Testar emissor OIDC

```text
kobana dashboards portal-oauth-providers test [OPÇÕES]
```

`POST /v1/portal_oauth_providers/test`

Busca o documento de descoberta do emissor e informa seus endpoints, para validar a URL antes de salvar. Uma falha na descoberta é um 200 com `ok: false`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `issuer_url` | `string (uri)` | sim | URL do emissor OpenID Connect (https em produção). |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-oauth-providers.test`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do teste. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## portal-oauth-providers update

Atualizar provedor OIDC

```text
kobana dashboards portal-oauth-providers update [ID] [OPÇÕES]
```

`PATCH /v1/portal_oauth_providers/{id}`

Atualiza um provedor OpenID Connect. Parcial: só os campos enviados mudam.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `button_label` | `string` | não |  |
| `issuer_url` | `string (uri)` | não |  |
| `client_id` | `string` | não |  |
| `client_secret` | `string` | não | Omitido ou vazio mantém o secret atual; um valor substitui. Nunca retornado. Obrigatório quando `issuer_url` muda (outro host, porta ou caminho): sem ele a alteração é recusada com 422 `CLIENT_SECRET_REQUIRED_FOR_ISSUER_CHANGE`, para o secret guardado não ser enviado a outro provedor. |
| `scopes` | `string` | não |  |
| `email_claim` | `string` | não |  |
| `enabled` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-oauth-providers.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O provedor (sem o secret). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## portal-oauth-providers delete

Remover provedor OIDC

```text
kobana dashboards portal-oauth-providers delete [ID] [OPÇÕES]
```

`DELETE /v1/portal_oauth_providers/{id}`

Remove o provedor; o botão some do login e ninguém mais entra por ele.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Removido. |
| `404` | Recurso não encontrado na organização atual. |
