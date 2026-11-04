---
id: webhooks
title: "kobana dashboards webhooks"
sidebar_label: "webhooks"
---

# kobana dashboards webhooks


Comandos de `kobana dashboards webhooks`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`webhooks deliveries retry`](#webhooks-deliveries-retry) | `POST /v1/webhooks/{id}/deliveries/{delivery_id}/retry` | Reenviar entrega de webhook |
| [`webhooks deliveries list`](#webhooks-deliveries-list) | `GET /v1/webhooks/{id}/deliveries` | Listar entregas de um webhook |
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | Listar endpoints de webhook |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Criar endpoint de webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Consultar endpoint de webhook |
| [`webhooks update`](#webhooks-update) | `PATCH /v1/webhooks/{id}` | Atualizar endpoint de webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Excluir endpoint de webhook |
| [`webhooks rotate-secret`](#webhooks-rotate-secret) | `POST /v1/webhooks/{id}/rotate_secret` | Rotacionar segredo do webhook |
| [`webhooks test`](#webhooks-test) | `POST /v1/webhooks/{id}/test` | Enviar webhook de teste |

## webhooks deliveries retry

Reenviar entrega de webhook

```text
kobana dashboards webhooks deliveries retry [ID] [DELIVERY_ID] [OPÇÕES]
```

`POST /v1/webhooks/{id}/deliveries/{delivery_id}/retry`

Coloca a entrega de volta em `pending` e a enfileira para novo envio. O envio é assíncrono: acompanhe o resultado no log de entregas.

**Argumentos do caminho:** `id`, `delivery_id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Entrega enfileirada para reenvio. |
| `404` | Recurso não encontrado na organização atual. |

## webhooks deliveries list

Listar entregas de um webhook

```text
kobana dashboards webhooks deliveries list [ID] [OPÇÕES]
```

`GET /v1/webhooks/{id}/deliveries`

Log de entregas de um endpoint, da mais recente à mais antiga.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | não | `pending`, `delivered` ou `failed`. Valores: `pending`, `delivered`, `failed`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de entregas. |
| `404` | Recurso não encontrado na organização atual. |

## webhooks list

Listar endpoints de webhook

```text
kobana dashboards webhooks list [OPÇÕES]
```

`GET /v1/webhooks`

Lista os endpoints de webhook do workspace, do mais recente ao mais antigo, com o total de entregas de cada um.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | não | `active` ou `inactive`. Valores: `active`, `inactive`. |
| `search` | `string` | não | Trecho da URL ou da descrição, sem diferenciar maiúsculas. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de endpoints de webhook. |
| `404` | Recurso não encontrado na organização atual. |

## webhooks create

Criar endpoint de webhook

```text
kobana dashboards webhooks create [OPÇÕES]
```

`POST /v1/webhooks`

Cria um endpoint ativo no workspace. A resposta traz o `secret` de assinatura — ele só aparece aqui (e na rotação).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `url` | `string` | sim | URL do endpoint (obrigatória na criação). Precisa ser uma URL válida. |
| `events` | `array<string>` | sim | Tipos de evento a receber. Pelo menos um. |
| `description` | `string` | não | Descrição livre. `null` limpa. |
| `auth_type` | `string` | não | Autenticação das entregas (padrão `none`). Ao alterar, envie também o `auth_config` correspondente. Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | não | Credenciais do `auth_type`: basic → `{ username, password }`, bearer → `{ token }`, header → `{ key, value }`. Guardadas criptografadas e nunca retornadas. |

Outros campos e objetos aninhados: `kobana schema dashboards.webhooks.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Endpoint criado, com o segredo de assinatura. |
| `400` | Requisição inválida: corpo malformado, campo obrigatório ausente ou filtro com valor inválido. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## webhooks get

Consultar endpoint de webhook

```text
kobana dashboards webhooks get [ID] [OPÇÕES]
```

`GET /v1/webhooks/{id}`

Retorna um endpoint de webhook do workspace. As entregas ficam em `GET /webhooks/{id}/deliveries`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O endpoint de webhook. |
| `404` | Recurso não encontrado na organização atual. |

## webhooks update

Atualizar endpoint de webhook

```text
kobana dashboards webhooks update [ID] [OPÇÕES]
```

`PATCH /v1/webhooks/{id}`

Altera URL, descrição, eventos assinados, `active` ou a autenticação. Campos ausentes ficam como estão. Quando a `url` muda de origem, caminho ou query string (a ordem dos parâmetros não conta) e `auth_type` não vem na mesma requisição, as credenciais guardadas são descartadas (`auth_type` passa a `none`): elas foram dadas para o destino anterior. O segredo de assinatura é mantido — use `rotate_secret` se o endpoint mudou de dono.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `url` | `string` | não | URL do endpoint (obrigatória na criação). Precisa ser uma URL válida. |
| `events` | `array<string>` | não | Tipos de evento a receber. Pelo menos um. |
| `description` | `string` | não | Descrição livre. `null` limpa. |
| `active` | `boolean` | não | `false` pausa o endpoint; `true` retoma. |
| `auth_type` | `string` | não | Autenticação das entregas (padrão `none`). Ao alterar, envie também o `auth_config` correspondente. Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | não | Credenciais do `auth_type`: basic → `{ username, password }`, bearer → `{ token }`, header → `{ key, value }`. Guardadas criptografadas e nunca retornadas. |

Outros campos e objetos aninhados: `kobana schema dashboards.webhooks.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Endpoint atualizado. |
| `400` | Requisição inválida: corpo malformado, campo obrigatório ausente ou filtro com valor inválido. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## webhooks delete

Excluir endpoint de webhook

```text
kobana dashboards webhooks delete [ID] [OPÇÕES]
```

`DELETE /v1/webhooks/{id}`

Exclui o endpoint definitivamente, junto com o log de entregas. Os eventos deixam de ser enviados na hora.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Endpoint excluído. |
| `404` | Recurso não encontrado na organização atual. |

## webhooks rotate-secret

Rotacionar segredo do webhook

```text
kobana dashboards webhooks rotate-secret [ID] [OPÇÕES]
```

`POST /v1/webhooks/{id}/rotate_secret`

Gera um novo segredo de assinatura. O anterior deixa de valer na hora; o novo `secret` só aparece nesta resposta.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Endpoint com o novo segredo. |
| `404` | Recurso não encontrado na organização atual. |

## webhooks test

Enviar webhook de teste

```text
kobana dashboards webhooks test [ID] [OPÇÕES]
```

`POST /v1/webhooks/{id}/test`

Envia agora um evento `test.webhook` assinado para a URL do endpoint e o registra no log de entregas. Responde 200 com `success: false` quando o endpoint responde fora de 2xx ou não é alcançado, e 422 quando a URL aponta para um endereço bloqueado (rede privada).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da tentativa. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
