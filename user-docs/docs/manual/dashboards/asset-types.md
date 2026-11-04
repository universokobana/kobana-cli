---
id: asset-types
title: "kobana dashboards asset-types"
sidebar_label: "asset-types"
---

# kobana dashboards asset-types


Comandos de `kobana dashboards asset-types`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`asset-types list`](#asset-types-list) | `GET /v1/asset_types` | Listar tipos de arquivo |
| [`asset-types create`](#asset-types-create) | `POST /v1/asset_types` | Criar tipo de arquivo |
| [`asset-types get`](#asset-types-get) | `GET /v1/asset_types/{id}` | Buscar tipo de arquivo |
| [`asset-types update`](#asset-types-update) | `PATCH /v1/asset_types/{id}` | Renomear tipo de arquivo |
| [`asset-types delete`](#asset-types-delete) | `DELETE /v1/asset_types/{id}` | Remover tipo de arquivo |

## asset-types list

Listar tipos de arquivo

```text
kobana dashboards asset-types list [OPÇÕES]
```

`GET /v1/asset_types`

Os tipos de arquivo do workspace, por nome. Um bloco Arquivos do Usuário do portal lista os arquivos dos tipos que escolhe.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Os tipos de arquivo. |
| `404` | Recurso não encontrado na organização atual. |

## asset-types create

Criar tipo de arquivo

```text
kobana dashboards asset-types create [OPÇÕES]
```

`POST /v1/asset_types`

Cria um tipo de arquivo. Depois, use o `id` em `type_id` dos arquivos.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do tipo (Propostas, Contratos…), único no workspace sem diferenciar maiúsculas; até 100 caracteres. |

Outros campos e objetos aninhados: `kobana schema dashboards.asset-types.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | O tipo criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## asset-types get

Buscar tipo de arquivo

```text
kobana dashboards asset-types get [ID] [OPÇÕES]
```

`GET /v1/asset_types/{id}`

Um tipo de arquivo.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O tipo de arquivo. |
| `404` | Recurso não encontrado na organização atual. |

## asset-types update

Renomear tipo de arquivo

```text
kobana dashboards asset-types update [ID] [OPÇÕES]
```

`PATCH /v1/asset_types/{id}`

Muda o nome de um tipo de arquivo.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do tipo (Propostas, Contratos…), único no workspace sem diferenciar maiúsculas; até 100 caracteres. |

Outros campos e objetos aninhados: `kobana schema dashboards.asset-types.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O tipo atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## asset-types delete

Remover tipo de arquivo

```text
kobana dashboards asset-types delete [ID] [OPÇÕES]
```

`DELETE /v1/asset_types/{id}`

Remove o tipo. Os arquivos dele continuam, sem tipo — e saem dos blocos Arquivos do Usuário que listavam esse tipo.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O tipo foi removido. |
| `404` | Recurso não encontrado na organização atual. |
