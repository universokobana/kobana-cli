---
id: assets
title: "kobana dashboards assets"
sidebar_label: "assets"
---

# kobana dashboards assets


Comandos de `kobana dashboards assets`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`assets list`](#assets-list) | `GET /v1/assets` | Listar arquivos |
| [`assets create`](#assets-create) | `POST /v1/assets` | Adicionar arquivo |
| [`assets presign`](#assets-presign) | `POST /v1/assets/presign` | Gerar URL de envio |
| [`assets get`](#assets-get) | `GET /v1/assets/{id}` | Consultar arquivo |
| [`assets update`](#assets-update) | `PATCH /v1/assets/{id}` | Atualizar arquivo |
| [`assets delete`](#assets-delete) | `DELETE /v1/assets/{id}` | Remover arquivo |
| [`assets download`](#assets-download) | `GET /v1/assets/{id}/download` | Baixar arquivo |

## assets list

Listar arquivos

```text
kobana dashboards assets list [OPÇÕES]
```

`GET /v1/assets`

A biblioteca de arquivos do workspace — imagens e documentos que o portal mostra ou oferece para download. Ordenável por `created_at` (padrão), `name` ou `size_bytes`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `kind` | `string` | não | Filtra pelo tipo: `image`, `document` ou `other`. Valores: `image`, `document`, `other`. |
| `visibility` | `string` | não | Quem pode baixar: `public` (qualquer pessoa com o link), `restricted` (usuários logados no portal do workspace) ou `private` (só membros do dashboard). Arquivos novos começam `restricted`. Valores: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | não | Só os arquivos deste tenant. |
| `type_id` | `string` | não | Só os arquivos deste tipo. |
| `tag` | `string` | não | Só os arquivos com esta tag. |
| `search` | `string` | não | Trecho do nome do arquivo (sem diferenciar maiúsculas). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de arquivos. |
| `404` | Recurso não encontrado na organização atual. |

## assets create

Adicionar arquivo

```text
kobana dashboards assets create [OPÇÕES]
```

`POST /v1/assets`

Registra um arquivo na biblioteca, de um de dois jeitos: o `key` de um arquivo já enviado pela URL de `POST /assets/presign`, ou o próprio conteúdo em `content_base64`. O arquivo começa `restricted`, a menos que `visibility` diga outra coisa. Dispara o webhook `asset.created`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome de exibição, com a extensão (obrigatório). A extensão decide o tipo aceito quando `content_type` não vem. |
| `key` | `string` | não | O `key` devolvido por `POST /assets/presign`, depois de enviar o arquivo para a URL assinada. Envie `key` ou `content_base64`, nunca os dois. |
| `content_base64` | `string` | não | O conteúdo do arquivo em base64, para arquivos pequenos: o corpo da requisição tem limite de 2 MB (cerca de 1,5 MB de arquivo). Para arquivos maiores, use `POST /assets/presign`. |
| `content_type` | `string` | não | Tipo MIME. Quando omitido, é deduzido da extensão. Aceitos: imagens PNG, JPEG, GIF, WebP e AVIF; PDF, texto, CSV, Markdown, JSON, XML, Office e ZIP. SVG e HTML não são aceitos. |
| `alt_text` | `string` | não | Texto alternativo de uma imagem (acessibilidade). |
| `visibility` | `string` | não | Quem pode baixar: `public` (qualquer pessoa com o link), `restricted` (usuários logados no portal do workspace) ou `private` (só membros do dashboard). Arquivos novos começam `restricted`. Valores: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal que veem o arquivo — todos os usuários do mesmo tenant veem os mesmos arquivos, nos blocos Arquivos do Usuário. `null` = arquivo do workspace, sem tenant. |
| `type_id` | `string (uuid)` | não | Tipo do arquivo (`GET /asset_types`: Propostas, Contratos…). Um bloco Arquivos do Usuário lista os tipos que escolhe; um arquivo com tipo e sem tenant aparece para todos os usuários nos blocos desse tipo. `null` = sem tipo. |
| `tags` | `array<string>` | não | Tags do arquivo (até 20, cada uma com até 50 caracteres; repetidas são ignoradas). Substitui as atuais. |

Outros campos e objetos aninhados: `kobana schema dashboards.assets.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Arquivo registrado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## assets presign

Gerar URL de envio

```text
kobana dashboards assets presign [OPÇÕES]
```

`POST /v1/assets/presign`

Uma URL assinada e de curta duração para enviar o arquivo direto ao storage com `PUT` — o caminho para arquivos acima do limite de 2 MB do corpo. Depois do `PUT`, registre o arquivo em `POST /assets` com o `key` devolvido.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `filename` | `string` | sim | Nome do arquivo com a extensão. |
| `size` | `integer` | sim | Tamanho exato, em bytes, do arquivo que será enviado (máximo 25 MB). |
| `content_type` | `string` | não | Tipo MIME. Quando omitido, é deduzido da extensão. Aceitos: imagens PNG, JPEG, GIF, WebP e AVIF; PDF, texto, CSV, Markdown, JSON, XML, Office e ZIP. SVG e HTML não são aceitos. |

Outros campos e objetos aninhados: `kobana schema dashboards.assets.presign`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | URL de envio gerada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## assets get

Consultar arquivo

```text
kobana dashboards assets get [ID] [OPÇÕES]
```

`GET /v1/assets/{id}`

Um arquivo da biblioteca, com o link, a visibilidade, o tipo e o tamanho.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O arquivo. |
| `404` | Recurso não encontrado na organização atual. |

## assets update

Atualizar arquivo

```text
kobana dashboards assets update [ID] [OPÇÕES]
```

`PATCH /v1/assets/{id}`

Renomeia o arquivo, troca o texto alternativo ou a visibilidade. Tornar público abre a URL do storage para qualquer pessoa; `restricted` ou `private` a fecha.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome de exibição do arquivo (por padrão, o nome original, com a extensão). |
| `alt_text` | `string` | não | Texto alternativo de uma imagem. Envie `null` para limpar. |
| `visibility` | `string` | não | Quem pode baixar: `public` (qualquer pessoa com o link), `restricted` (usuários logados no portal do workspace) ou `private` (só membros do dashboard). Arquivos novos começam `restricted`. Valores: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal que veem o arquivo — todos os usuários do mesmo tenant veem os mesmos arquivos, nos blocos Arquivos do Usuário. `null` = arquivo do workspace, sem tenant. |
| `type_id` | `string (uuid)` | não | Tipo do arquivo (`GET /asset_types`: Propostas, Contratos…). Um bloco Arquivos do Usuário lista os tipos que escolhe; um arquivo com tipo e sem tenant aparece para todos os usuários nos blocos desse tipo. `null` = sem tipo. |
| `tags` | `array<string>` | não | Tags do arquivo (até 20, cada uma com até 50 caracteres; repetidas são ignoradas). Substitui as atuais. |

Outros campos e objetos aninhados: `kobana schema dashboards.assets.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivo atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## assets delete

Remover arquivo

```text
kobana dashboards assets delete [ID] [OPÇÕES]
```

`DELETE /v1/assets/{id}`

Tira o arquivo da biblioteca. O arquivo guardado continua no storage, para que links já publicados não quebrem. Dispara o webhook `asset.deleted`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivo removido. |
| `404` | Recurso não encontrado na organização atual. |

## assets download

Baixar arquivo

```text
kobana dashboards assets download [ID] [OPÇÕES]
```

`GET /v1/assets/{id}/download`

O conteúdo do arquivo, qualquer que seja a visibilidade, com o `Content-Type` dele.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `inline` | `string` | não | Com `true`, imagens e PDF vêm para exibição no lugar (`Content-Disposition: inline`); os demais tipos vêm sempre como anexo. Valores: `true`, `false`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O conteúdo do arquivo. |
| `404` | Recurso não encontrado na organização atual. |
