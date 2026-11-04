---
id: api-keys
title: "kobana dashboards api-keys"
sidebar_label: "api-keys"
---

# kobana dashboards api-keys


Comandos de `kobana dashboards api-keys`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`api-keys list`](#api-keys-list) | `GET /v1/api_keys` | Listar chaves de API |
| [`api-keys create`](#api-keys-create) | `POST /v1/api_keys` | Criar chave de API |
| [`api-keys get`](#api-keys-get) | `GET /v1/api_keys/{id}` | Consultar chave de API |
| [`api-keys update`](#api-keys-update) | `PATCH /v1/api_keys/{id}` | Atualizar chave de API |
| [`api-keys delete`](#api-keys-delete) | `DELETE /v1/api_keys/{id}` | Remover chave revogada |
| [`api-keys revoke`](#api-keys-revoke) | `POST /v1/api_keys/{id}/revoke` | Revogar chave de API |
| [`api-keys rotate`](#api-keys-rotate) | `POST /v1/api_keys/{id}/rotate` | Rotacionar chave de API |

## api-keys list

Listar chaves de API

```text
kobana dashboards api-keys list [OPÇÕES]
```

`GET /v1/api_keys`

Lista as chaves de API (`kb_…`) da organização, sem o segredo. Essas chaves não autenticam mais a API v1 nem o MCP — ambos usam tokens de acesso emitidos pelo servidor de autorização da Kobana; os registros continuam como inventário de chaves da organização (ciclo de vida, avisos de expiração e webhooks `api_key.*`). Recurso da organização: não usa `X-Workspace-Id`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra pelo status: `active`, `disabled` ou `revoked`. Valores: `active`, `disabled`, `revoked`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de chaves de API. |
| `404` | Recurso não encontrado na organização atual. |

## api-keys create

Criar chave de API

```text
kobana dashboards api-keys create [OPÇÕES]
```

`POST /v1/api_keys`

Cria uma chave ativa, válida por um ano salvo `expires_at`. O segredo (`key`) volta UMA única vez nesta resposta — só o hash é guardado. Exige uma pessoa que alcance todos os workspaces (dona da organização ou com acesso a cada um), como no painel.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome da chave (obrigatório). |
| `description` | `string` | não |  |
| `source` | `string` | não | Onde a chave será usada. Padrão `dashboard`. Valores: `api`, `dashboard`, `manage`. |
| `notification_emails` | `array<string>` | não | E-mails que recebem os avisos de expiração. |
| `expires_at` | `string (date-time)` | não | Validade (data-hora ISO 8601 no futuro). Padrão: daqui a um ano. |

Outros campos e objetos aninhados: `kobana schema dashboards.api-keys.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Chave criada, com o segredo em `key` (exibido só agora). |
| `400` | Corpo ausente, JSON inválido ou campos incompatíveis. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## api-keys get

Consultar chave de API

```text
kobana dashboards api-keys get [ID] [OPÇÕES]
```

`GET /v1/api_keys/{id}`

Retorna uma chave da organização — metadados e carimbos do ciclo de vida, nunca o segredo.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A chave de API. |
| `404` | Recurso não encontrado na organização atual. |

## api-keys update

Atualizar chave de API

```text
kobana dashboards api-keys update [ID] [OPÇÕES]
```

`PATCH /v1/api_keys/{id}`

Edita nome, descrição, origem, e-mails de aviso e validade, e/ou ativa ⇄ desativa a chave com `enabled`. Chave revogada é definitiva (409).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `description` | `string` | não |  |
| `source` | `string` | não | Valores: `api`, `dashboard`, `manage`. |
| `notification_emails` | `array<string>` | não | E-mails que recebem os avisos de expiração. |
| `expires_at` | `string (date-time)` | não | Nova validade (ISO 8601); `null` remove a expiração. |
| `enabled` | `boolean` | não | `false` desativa a chave; `true` reativa. |

Outros campos e objetos aninhados: `kobana schema dashboards.api-keys.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A chave de API. |
| `400` | Corpo ausente, JSON inválido ou campos incompatíveis. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | O estado atual do recurso não permite esta operação. |
| `422` | Corpo da requisição falhou na validação. |

## api-keys delete

Remover chave revogada

```text
kobana dashboards api-keys delete [ID] [OPÇÕES]
```

`DELETE /v1/api_keys/{id}`

Tira da lista uma chave JÁ REVOGADA (exclusão lógica; o histórico fica). Uma chave ativa ou desativada precisa ser revogada antes (409).

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
| `409` | O estado atual do recurso não permite esta operação. |

## api-keys revoke

Revogar chave de API

```text
kobana dashboards api-keys revoke [ID] [OPÇÕES]
```

`POST /v1/api_keys/{id}/revoke`

Revoga a chave em definitivo — não pode ser reativada. 409 se já estiver revogada.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A chave de API. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | O estado atual do recurso não permite esta operação. |

## api-keys rotate

Rotacionar chave de API

```text
kobana dashboards api-keys rotate [ID] [OPÇÕES]
```

`POST /v1/api_keys/{id}/rotate`

Emite uma chave sucessora (mesmo nome e configurações) e mantém a anterior funcionando por `grace_days` dias (padrão 7, máximo 30). Só uma chave ativa pode ser rotacionada (409). O segredo novo volta UMA única vez.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `grace_days` | `integer` | não | Dias em que a chave anterior continua valendo (0 a 30, padrão 7). |

Outros campos e objetos aninhados: `kobana schema dashboards.api-keys.rotate`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A chave sucessora com o segredo em `key`, e `previous_key` com o prazo de carência da anterior. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | O estado atual do recurso não permite esta operação. |
| `422` | Corpo da requisição falhou na validação. |
