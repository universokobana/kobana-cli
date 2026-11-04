---
id: portal-users
title: "kobana dashboards portal-users"
sidebar_label: "portal-users"
---

# kobana dashboards portal-users


Comandos de `kobana dashboards portal-users`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`portal-users activity`](#portal-users-activity) | `GET /v1/portal_users/{id}/activity` | Consultar a atividade do usuário do portal |
| [`portal-users activity timeline`](#portal-users-activity-timeline) | `GET /v1/portal_users/{id}/activity/timeline` | Listar a linha do tempo do usuário do portal |
| [`portal-users list`](#portal-users-list) | `GET /v1/portal_users` | Listar usuários do portal |
| [`portal-users create`](#portal-users-create) | `POST /v1/portal_users` | Criar (convidar) usuário do portal |
| [`portal-users invite`](#portal-users-invite) | `POST /v1/portal_users/invite` | Convidar usuário do portal |
| [`portal-users get`](#portal-users-get) | `GET /v1/portal_users/{id}` | Obter usuário do portal |
| [`portal-users update`](#portal-users-update) | `PATCH /v1/portal_users/{id}` | Atualizar usuário do portal |
| [`portal-users delete`](#portal-users-delete) | `DELETE /v1/portal_users/{id}` | Excluir usuário do portal |
| [`portal-users email-preview`](#portal-users-email-preview) | `GET /v1/portal_users/{id}/email-preview` | Pré-visualizar e-mail do usuário do portal |
| [`portal-users resend`](#portal-users-resend) | `POST /v1/portal_users/{id}/resend` | Reenviar e-mail de convite ou verificação |

## portal-users activity

Consultar a atividade do usuário do portal

```text
kobana dashboards portal-users activity [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/portal_users/{id}/activity`

Analytics de uso de um usuário do portal: logins e páginas visitadas (desde sempre), páginas diferentes, a atividade mais recente, uma série diária de visitas e logins nos últimos `days` dias (dias de São Paulo) e as páginas mais abertas. As entradas individuais ficam em `GET /portal_users/{id}/activity/timeline`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `days` | `integer` | não | Dias cobertos pela `series`, incluindo hoje (1 a 365). Padrão: 30. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resumo da atividade do usuário. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## portal-users activity timeline

Listar a linha do tempo do usuário do portal

```text
kobana dashboards portal-users activity timeline [ID] [OPÇÕES]
```

`GET /v1/portal_users/{id}/activity/timeline`

Logins (`kind: login`, com o `method` usado: `credentials`, `magic-link` ou `sso`) e páginas abertas (`kind: page_view`, com `path` e `page_name`), da mais recente à mais antiga. Paginação por cursor: envie `meta.next_before` como `before` para a página seguinte; `null` indica que não há mais. Acessos de um operador visualizando o portal como o usuário não entram.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `before` | `string` | não | Instante ISO 8601: só entradas mais antigas que ele. |
| `limit` | `integer` | não | Entradas por página (1 a 100). Padrão: 20. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Uma página da linha do tempo. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## portal-users list

Listar usuários do portal

```text
kobana dashboards portal-users list [OPÇÕES]
```

`GET /v1/portal_users`

Retorna os usuários do portal do workspace, com paginação. Campos sensíveis (senha, tokens) nunca são expostos.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `tenant_id` | `string` | não | Restringe aos usuários de um ID do titular (comparação exata). |
| `status` | `string` | não | Filtra por status: `active`, `pending_invitation`, `pending_verification`, `suspended` ou `locked`. Valores: `active`, `pending_verification`, `pending_invitation`, `suspended`, `locked`. |
| `email` | `string` | não | Busca por trecho do e-mail. |
| `external_id` | `string` | não | Filtra pelo `external_id` exato. |
| `sso_only` | `string` | não | Filtra por `sso_only` (`true` ou `false`). Valores: `true`, `false`. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `last_login_after` | `string` | não | Último acesso a partir desta data (ISO 8601). |
| `last_login_before` | `string` | não | Último acesso até esta data (ISO 8601). |
| `person_id` | `string` | não | Filtra por UUID da pessoa associada. |
| `person_document_number` | `string` | não | Filtra por CPF/CNPJ da pessoa (exato). |
| `person_name_contains` | `string` | não | Filtra por nome da pessoa (busca parcial, sem distinção de maiúsculas). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de usuários do portal. |
| `404` | Recurso não encontrado na organização atual. |

## portal-users create

Criar (convidar) usuário do portal

```text
kobana dashboards portal-users create [OPÇÕES]
```

`POST /v1/portal_users`

Cria o usuário no workspace com status `pending_invitation` e envia o e-mail de convite. O convite expira em 7 dias.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string` | sim | E-mail do usuário (obrigatório). Único por workspace, sem diferenciar maiúsculas. |
| `tenant_id` | `string` | não | ID do titular dos registros que o usuário verá no portal. Envie `null` para limpar. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `sso_only` | `boolean` | não | Quando `true`, o usuário só entra pelo SSO. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pela organização (chave/valor). |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-users.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Convite criado e e-mail enfileirado para envio. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Já existe um usuário ou convite para este e-mail no workspace. |

## portal-users invite

Convidar usuário do portal

```text
kobana dashboards portal-users invite [OPÇÕES]
```

`POST /v1/portal_users/invite`

Equivalente ao `POST /portal_users`. Disponível como rota de verbo para SDKs que preferem URLs descritivas.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string` | sim | E-mail do usuário (obrigatório). Único por workspace, sem diferenciar maiúsculas. |
| `tenant_id` | `string` | não | ID do titular dos registros que o usuário verá no portal. Envie `null` para limpar. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `sso_only` | `boolean` | não | Quando `true`, o usuário só entra pelo SSO. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pela organização (chave/valor). |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-users.invite`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Convite criado e e-mail enfileirado para envio. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Já existe um usuário ou convite para este e-mail no workspace. |

## portal-users get

Obter usuário do portal

```text
kobana dashboards portal-users get [ID] [OPÇÕES]
```

`GET /v1/portal_users/{id}`

O parâmetro `{id}` aceita o UUID ou o `external_id` do usuário.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do usuário do portal. |
| `404` | Recurso não encontrado na organização atual. |

## portal-users update

Atualizar usuário do portal

```text
kobana dashboards portal-users update [ID] [OPÇÕES]
```

`PATCH /v1/portal_users/{id}`

Atualiza `email`, `tenant_id`, `external_id`, `sso_only`, `status` e `custom_metadata`. Transições de status permitidas: `active` ↔ `suspended`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string` | não | E-mail do usuário (obrigatório). Único por workspace, sem diferenciar maiúsculas. |
| `tenant_id` | `string` | não | ID do titular dos registros que o usuário verá no portal. Envie `null` para limpar. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `sso_only` | `boolean` | não | Quando `true`, o usuário só entra pelo SSO. |
| `status` | `string` | não | Transições permitidas: `active` ↔ `suspended`. Ao reativar, o usuário volta ao status que tinha antes da suspensão (quem ainda não tinha confirmado o e-mail continua pendente). Valores: `active`, `suspended`. |
| `unlock` | `boolean` | não | `true` encerra o bloqueio de login (muitas senhas erradas): zera as tentativas e libera o login. 400 quando o usuário não está bloqueado. Valores: `true`. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pela organização (chave/valor). |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-users.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Usuário atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Já existe um usuário ou convite para este e-mail no workspace. |

## portal-users delete

Excluir usuário do portal

```text
kobana dashboards portal-users delete [ID] [OPÇÕES]
```

`DELETE /v1/portal_users/{id}`

Convites pendentes são excluídos definitivamente; os demais usuários são suspensos e suas sessões encerradas. Um usuário já suspenso não muda. `outcome` diz o que aconteceu.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Confirmação da exclusão. |
| `404` | Recurso não encontrado na organização atual. |

## portal-users email-preview

Pré-visualizar e-mail do usuário do portal

```text
kobana dashboards portal-users email-preview [ID] [OPÇÕES]
```

`GET /v1/portal_users/{id}/email-preview`

Retorna o assunto e a mensagem do e-mail de convite ou verificação que seria enviado ao usuário, sem disparar o envio. Disponível apenas para usuários `pending_invitation` ou `pending_verification`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pré-visualização do e-mail. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## portal-users resend

Reenviar e-mail de convite ou verificação

```text
kobana dashboards portal-users resend [ID] [OPÇÕES]
```

`POST /v1/portal_users/{id}/resend`

Reenvia o e-mail de convite (para `pending_invitation`, com rotação de token) ou de verificação (para `pending_verification`). Demais status retornam 400.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | E-mail reenviado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
