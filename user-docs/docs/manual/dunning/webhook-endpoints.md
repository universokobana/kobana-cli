---
id: webhook-endpoints
title: "kobana dunning webhook-endpoints"
sidebar_label: "webhook-endpoints"
---

# kobana dunning webhook-endpoints


Comandos de `kobana dunning webhook-endpoints`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`webhook-endpoints deliveries retry`](#webhook-endpoints-deliveries-retry) | `POST /v1/webhook-endpoints/{id}/deliveries/{deliveryId}/retry` | Reenviar entrega |
| [`webhook-endpoints deliveries list`](#webhook-endpoints-deliveries-list) | `GET /v1/webhook-endpoints/{id}/deliveries` | Listar entregas do endpoint |
| [`webhook-endpoints list`](#webhook-endpoints-list) | `GET /v1/webhook-endpoints` | Listar endpoints de webhook |
| [`webhook-endpoints create`](#webhook-endpoints-create) | `POST /v1/webhook-endpoints` | Criar endpoint de webhook |
| [`webhook-endpoints get`](#webhook-endpoints-get) | `GET /v1/webhook-endpoints/{id}` | Consultar endpoint de webhook |
| [`webhook-endpoints update`](#webhook-endpoints-update) | `PATCH /v1/webhook-endpoints/{id}` | Atualizar endpoint de webhook |
| [`webhook-endpoints delete`](#webhook-endpoints-delete) | `DELETE /v1/webhook-endpoints/{id}` | Excluir endpoint de webhook |
| [`webhook-endpoints rotate-secret`](#webhook-endpoints-rotate-secret) | `POST /v1/webhook-endpoints/{id}/rotate-secret` | Rotacionar secret |
| [`webhook-endpoints test`](#webhook-endpoints-test) | `POST /v1/webhook-endpoints/{id}/test` | Disparar evento de teste |

## webhook-endpoints deliveries retry

Reenviar entrega

```text
kobana dunning webhook-endpoints deliveries retry [ID] [DELIVERYID] [OPÇÕES]
```

`POST /v1/webhook-endpoints/{id}/deliveries/{deliveryId}/retry`

Re-enfileira uma entrega para nova tentativa imediata (status volta a `pending`). Falha com 409 se o endpoint estiver inativo ou removido.

**Argumentos do caminho:** `id`, `deliveryId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Entrega re-enfileirada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## webhook-endpoints deliveries list

Listar entregas do endpoint

```text
kobana dunning webhook-endpoints deliveries list [ID] [OPÇÕES]
```

`GET /v1/webhook-endpoints/{id}/deliveries`

Lista as entregas do endpoint (paginado, mais recentes primeiro) com a captura de request/response para inspeção e debug.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `status` | `string` | não | Filtra pelo status da entrega. Valores: `pending`, `success`, `failed`, `exhausted`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de entregas. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## webhook-endpoints list

Listar endpoints de webhook

```text
kobana dunning webhook-endpoints list [OPÇÕES]
```

`GET /v1/webhook-endpoints`

Lista os endpoints de webhook da organização com contadores de entrega por status. `secret` e `auth_config` nunca aparecem na listagem.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de endpoints. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## webhook-endpoints create

Criar endpoint de webhook

```text
kobana dunning webhook-endpoints create [OPÇÕES]
```

`POST /v1/webhook-endpoints`

Cria um endpoint de webhook. A URL exige HTTPS e é validada contra endereços internos (SSRF). O `secret` é retornado apenas nesta resposta — guarde com segurança.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `url` | `string (uri)` | sim | URL HTTPS de destino das entregas (validada contra endereços internos — SSRF). |
| `description` | `string` | não | Descrição livre do endpoint. |
| `events` | `array<string>` | não | Tipos de evento assinados. Lista vazia = todos os eventos. |
| `auth_type` | `string` | não | Autenticação do request contra o seu endpoint (`none` \| `basic` \| `bearer` \| `header`), além da assinatura HMAC (sempre enviada). Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | não | Credenciais do `auth_type`: `basic` → `{"username","password"}`; `bearer` → `{"token"}`; `header` → `{"key","value"}`. Cifrado em repouso; nunca retornado nas consultas. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |

Outros campos e objetos aninhados: `kobana schema dunning.webhook-endpoints.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Endpoint criado (inclui `secret` — única exibição). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## webhook-endpoints get

Consultar endpoint de webhook

```text
kobana dunning webhook-endpoints get [ID] [OPÇÕES]
```

`GET /v1/webhook-endpoints/{id}`

Retorna o endpoint com as últimas 20 entregas. O `secret` nunca é retornado.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Endpoint com entregas recentes. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## webhook-endpoints update

Atualizar endpoint de webhook

```text
kobana dunning webhook-endpoints update [ID] [OPÇÕES]
```

`PATCH /v1/webhook-endpoints/{id}`

Atualiza o endpoint (parcial). A URL, se enviada, é revalidada (HTTPS + SSRF); `auth_config` é cifrado e nunca retornado.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `url` | `string (uri)` | não | URL HTTPS de destino das entregas (validada contra endereços internos — SSRF). |
| `description` | `string` | não | Descrição livre do endpoint. |
| `events` | `array<string>` | não | Tipos de evento assinados. Lista vazia = todos os eventos. |
| `is_active` | `boolean` | não | Endpoint ativo recebe entregas; inativo é ignorado pelo worker. |
| `auth_type` | `string` | não | Autenticação do request contra o seu endpoint (`none` \| `basic` \| `bearer` \| `header`), além da assinatura HMAC (sempre enviada). Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | não | Credenciais do `auth_type`: `basic` → `{"username","password"}`; `bearer` → `{"token"}`; `header` → `{"key","value"}`. Cifrado em repouso; nunca retornado nas consultas. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.webhook-endpoints.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Endpoint atualizado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## webhook-endpoints delete

Excluir endpoint de webhook

```text
kobana dunning webhook-endpoints delete [ID] [OPÇÕES]
```

`DELETE /v1/webhook-endpoints/{id}`

Exclui (soft delete) o endpoint; entregas pendentes deixam de ser processadas.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Endpoint excluído. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## webhook-endpoints rotate-secret

Rotacionar secret

```text
kobana dunning webhook-endpoints rotate-secret [ID] [OPÇÕES]
```

`POST /v1/webhook-endpoints/{id}/rotate-secret`

Gera um novo secret HMAC; o anterior deixa de validar imediatamente. O novo secret é retornado apenas nesta resposta — guarde com segurança.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Novo secret (única exibição). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## webhook-endpoints test

Disparar evento de teste

```text
kobana dunning webhook-endpoints test [ID] [OPÇÕES]
```

`POST /v1/webhook-endpoints/{id}/test`

Entrega um evento `webhook.test` REAL ao endpoint, pelo mesmo pipeline dos eventos de negócio (fila, assinatura HMAC, captura de request/response, retry) — valide seu consumidor fim a fim sem esperar um evento real. Entregue somente ao endpoint alvo, independente da lista de eventos assinados. Falha com 409 se o endpoint estiver inativo.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Evento de teste enfileirado para entrega. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |
