---
id: backups
title: "kobana dashboards backups"
sidebar_label: "backups"
---

# kobana dashboards backups


Comandos de `kobana dashboards backups`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`backups list`](#backups-list) | `GET /v1/backups` | Listar backups |
| [`backups create`](#backups-create) | `POST /v1/backups` | Solicitar backup |
| [`backups restore`](#backups-restore) | `POST /v1/backups/restore` | Restaurar backup |
| [`backups get`](#backups-get) | `GET /v1/backups/{id}` | Consultar backup |
| [`backups delete`](#backups-delete) | `DELETE /v1/backups/{id}` | Excluir backup |
| [`backups cancel`](#backups-cancel) | `POST /v1/backups/{id}/cancel` | Cancelar restauração |
| [`backups download`](#backups-download) | `GET /v1/backups/{id}/download` | Baixar backup |

## backups list

Listar backups

```text
kobana dashboards backups list [OPÇÕES]
```

`GET /v1/backups`

Lista backups e restaurações da organização (só metadados) e informa se um novo backup pode ser pedido agora (`backup_allowed`, `next_backup_at` — um a cada 12 horas em produção). Recurso da organização.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `type` | `string` | não | `backup` ou `restore`. Valores: `backup`, `restore`. |
| `status` | `string` | não | Filtra pelo status. Valores: `pending`, `processing`, `completed`, `failed`, `canceled`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de backups. |
| `404` | Recurso não encontrado na organização atual. |

## backups create

Solicitar backup

```text
kobana dashboards backups create [OPÇÕES]
```

`POST /v1/backups`

Agenda um backup completo (ZIP) da organização, com todos os workspaces. Assíncrono: 202; consulte `GET /backups/{id}`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Backup agendado. |
| `404` | Recurso não encontrado na organização atual. |

## backups restore

Restaurar backup

```text
kobana dashboards backups restore [OPÇÕES]
```

`POST /v1/backups/restore`

DESTRUTIVO: restaura um backup no workspace da chamada (`X-Workspace-Id`), regravando usuários do portal casados por e-mail/ID público. Informe `backup_id` (um backup concluído desta organização — o arquivo é copiado) OU `file_name` (`.zip`) + `content_base64` (limitado aos 2 MB da requisição). Exige `settings.update` e acesso a todos os workspaces. Assíncrono: 202; acompanhe em `GET /backups/{id}` e cancele com `/cancel`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `backup_id` | `string (uuid)` | não | Backup concluído desta organização a restaurar. |
| `file_name` | `string` | não | Nome do arquivo `.zip`, junto com `content_base64`. |
| `content_base64` | `string` | não | O ZIP do backup em base64. |

Outros campos e objetos aninhados: `kobana schema dashboards.backups.restore`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Restauração agendada (registro do tipo `restore`). |
| `400` | Corpo ausente, JSON inválido ou campos incompatíveis. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## backups get

Consultar backup

```text
kobana dashboards backups get [ID] [OPÇÕES]
```

`GET /v1/backups/{id}`

Status e progresso de um backup ou restauração. Uma restauração em andamento cujo job sumiu da fila é marcada como `failed`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O backup. |
| `404` | Recurso não encontrado na organização atual. |

## backups delete

Excluir backup

```text
kobana dashboards backups delete [ID] [OPÇÕES]
```

`DELETE /v1/backups/{id}`

Exclui o registro e o arquivo do armazenamento. Definitivo.

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

## backups cancel

Cancelar restauração

```text
kobana dashboards backups cancel [ID] [OPÇÕES]
```

`POST /v1/backups/{id}/cancel`

Interrompe uma restauração pendente ou em andamento. O que já foi restaurado permanece. 422 para um backup ou uma restauração já encerrada.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O backup. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## backups download

Baixar backup

```text
kobana dashboards backups download [ID] [OPÇÕES]
```

`GET /v1/backups/{id}/download`

Devolve o ZIP do backup. Exige `backups.export` e, para token de uma pessoa, ser dona da organização ou ter acesso a todos os workspaces — o backup contém todos eles. Fica registrado na auditoria.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O arquivo ZIP. |
| `404` | Recurso não encontrado na organização atual. |
