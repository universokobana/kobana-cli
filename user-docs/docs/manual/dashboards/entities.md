---
id: entities
title: "kobana dashboards entities"
sidebar_label: "entities"
---

# kobana dashboards entities


Comandos de `kobana dashboards entities`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`entities join preview`](#entities-join-preview) | `POST /v1/entities/join/preview` | Prévia de uma relação |
| [`entities join create`](#entities-join-create) | `POST /v1/entities/join` | Criar entidade de relação |
| [`entities join update`](#entities-join-update) | `PUT /v1/entities/{id}/join` | Alterar relação |
| [`entities logs get`](#entities-logs-get) | `GET /v1/entities/{id}/logs/{log_id}` | Consultar chamada da sincronização |
| [`entities logs list`](#entities-logs-list) | `GET /v1/entities/{id}/logs` | Log da sincronização |
| [`entities records list`](#entities-records-list) | `GET /v1/entities/{id}/records` | Consultar registros |
| [`entities records create`](#entities-records-create) | `POST /v1/entities/{id}/records` | Enviar registros |
| [`entities records delete`](#entities-records-delete) | `DELETE /v1/entities/{id}/records` | Limpar registros |
| [`entities records import`](#entities-records-import) | `POST /v1/entities/{id}/records/import` | Importar registros por CSV |
| [`entities records get`](#entities-records-get) | `GET /v1/entities/{id}/records/{record_id}` | Consultar registro |
| [`entities sync`](#entities-sync) | `GET /v1/entities/{id}/sync` | Estado da sincronização |
| [`entities sync create`](#entities-sync-create) | `POST /v1/entities/{id}/sync` | Sincronizar entidade |
| [`entities tenant-syncs list`](#entities-tenant-syncs-list) | `GET /v1/entities/{id}/tenant_syncs` | Sincronizações por titular |
| [`entities tenant-syncs create`](#entities-tenant-syncs-create) | `POST /v1/entities/{id}/tenant_syncs` | Sincronizar um titular |
| [`entities list`](#entities-list) | `GET /v1/entities` | Listar entidades |
| [`entities create`](#entities-create) | `POST /v1/entities` | Criar entidade |
| [`entities preview`](#entities-preview) | `POST /v1/entities/preview` | Prévia de uma origem |
| [`entities get`](#entities-get) | `GET /v1/entities/{id}` | Consultar entidade |
| [`entities update`](#entities-update) | `PATCH /v1/entities/{id}` | Atualizar entidade |
| [`entities delete`](#entities-delete) | `DELETE /v1/entities/{id}` | Remover entidade |
| [`entities aggregate`](#entities-aggregate) | `GET /v1/entities/{id}/aggregate` | Calcular agregados dos registros |
| [`entities usage`](#entities-usage) | `GET /v1/entities/{id}/usage` | Uso da entidade |

## entities join preview

Prévia de uma relação

```text
kobana dashboards entities join preview [OPÇÕES]
```

`POST /v1/entities/join/preview`

Como duas entidades casam nos campos informados (contagens) e as primeiras linhas combinadas, antes de salvar a relação. Lê só os registros já sincronizados.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `base_entity_id` | `string (uuid)` | sim | Entidade base: todos os seus registros entram na relação. |
| `related_entity_id` | `string (uuid)` | sim | Entidade relacionada, casada com a base pelos campos informados. |
| `base_field` | `string` | sim | Chave do campo da entidade base usado no casamento. |
| `related_field` | `string` | sim | Chave do campo da entidade relacionada usado no casamento. |
| `type` | `string` | não | `left` (padrão) mantém os registros da base sem par; `inner` só os que casam. Valores: `inner`, `left`. |
| `match` | `string` | não | `exact` (padrão) compara o texto; `digits` compara só os dígitos (CPF com e sem pontuação casam). Valores: `exact`, `digits`. |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.join.preview`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Prévia da relação. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## entities join create

Criar entidade de relação

```text
kobana dashboards entities join create [OPÇÕES]
```

`POST /v1/entities/join`

Registra uma entidade que relaciona duas outras (join) com os campos escolhidos de cada uma, e enfileira a primeira sincronização.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | sim | Nome de máquina que os blocos do portal usam; único no workspace (até 63 caracteres). |
| `name` | `string` | sim |  |
| `name_plural` | `string` | sim |  |
| `join` | `object` | sim |  |
| `fields` | `array<object>` | sim | Os campos trazidos de cada lado: `{ from: base\|related, field_key, key?, name? }`. |
| `description` | `string` | não |  |
| `label_field_key` | `string` | não | Campo de título; ausente = a cópia do título da entidade base, `null` = nenhum. |
| `owner_field_key` | `string` | não | Campo do titular; ausente = a cópia do titular da entidade base, `null` = nenhum. |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.join.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Entidade criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Conflito: chave já usada no workspace, ou a entidade é lida por uma relação. |
| `422` | Corpo da requisição falhou na validação. |

## entities join update

Alterar relação

```text
kobana dashboards entities join update [ID] [OPÇÕES]
```

`PUT /v1/entities/{id}/join`

Troca a relação de uma entidade join e os campos que ela traz, e relê seus registros. 422 quando a entidade não é uma relação.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | sim | Nome de máquina que os blocos do portal usam; único no workspace (até 63 caracteres). |
| `name` | `string` | sim |  |
| `name_plural` | `string` | sim |  |
| `join` | `object` | sim |  |
| `fields` | `array<object>` | sim | Os campos trazidos de cada lado: `{ from: base\|related, field_key, key?, name? }`. |
| `description` | `string` | não |  |
| `label_field_key` | `string` | não | Campo de título; ausente = a cópia do título da entidade base, `null` = nenhum. |
| `owner_field_key` | `string` | não | Campo do titular; ausente = a cópia do titular da entidade base, `null` = nenhum. |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.join.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Entidade atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Conflito: chave já usada no workspace, ou a entidade é lida por uma relação. |
| `422` | Corpo da requisição falhou na validação. |

## entities logs get

Consultar chamada da sincronização

```text
kobana dashboards entities logs get [ID] [LOG_ID] [OPÇÕES]
```

`GET /v1/entities/{id}/logs/{log_id}`

Uma chamada da sincronização da entidade, inteira (segredos mascarados).

**Argumentos do caminho:** `id`, `log_id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A chamada. |
| `404` | Recurso não encontrado na organização atual. |

## entities logs list

Log da sincronização

```text
kobana dashboards entities logs list [ID] [OPÇÕES]
```

`GET /v1/entities/{id}/logs`

As chamadas que a sincronização da entidade fez ao conector, das mais novas às mais antigas: o log de requisições do conector restrito à entidade.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `method` | `string` | não | Método HTTP da chamada, como `GET`. |
| `success` | `string` | não | Só chamadas bem-sucedidas (`true`) ou com falha (`false`). Valores: `true`, `false`. |
| `search` | `string` | não | Parte do endpoint chamado. |
| `date_from` | `string` | não | Primeiro dia (AAAA-MM-DD). |
| `date_to` | `string` | não | Último dia, inclusive (AAAA-MM-DD). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Chamadas da sincronização. |
| `404` | Recurso não encontrado na organização atual. |

## entities records list

Consultar registros

```text
kobana dashboards entities records list [ID] [OPÇÕES]
```

`GET /v1/entities/{id}/records`

Os registros sincronizados da entidade, com filtro por campo, ordenação e paginação por `limit`/`offset` (a mesma consulta dos blocos do portal).

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não | Registros por chamada (padrão 50, máximo 200). |
| `offset` | `integer,null` | não | Quantos registros pular. |
| `sort` | `string` | não | Chaves de campo com direção opcional, separadas por vírgula (`campo:desc,outro`); `asc` por padrão. |
| `filter` | `string` | não | Objeto JSON de condições por campo: `{"campo": valor}` ou `{"campo": {"gte": 10}}`. Operadores: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `contains`, `is_null`. |
| `preview_as` | `string` | não | ID de um usuário do portal: só os registros dele, como o portal mostra (em uma entidade `on_access`, sincroniza esse titular antes). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registros da entidade. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## entities records create

Enviar registros

```text
kobana dashboards entities records create [ID] [OPÇÕES]
```

`POST /v1/entities/{id}/records`

Envia registros para uma entidade sem conexão (`source_kind: "push"`): cada um é gravado pelo identificador — enviado de novo, é atualizado; sem mudança, não é reescrito. Relações e usuários do portal que dependem da entidade são atualizados em seguida. 422 numa entidade que lê de uma conexão.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `records` | `array<object>` | sim | Os registros (até 1000 por chamada), cada um um objeto com as propriedades que os campos leem (`source_path`). O identificador é obrigatório em cada um. |
| `mode` | `string` | não | `upsert` (padrão) mantém os registros não enviados; `replace` remove os que a chamada não traz — o envio passa a ser o conjunto inteiro. Valores: `upsert`, `replace`. |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.records.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Quantos registros foram criados, alterados, mantidos e removidos, e os recusados. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## entities records delete

Limpar registros

```text
kobana dashboards entities records delete [ID] [OPÇÕES]
```

`DELETE /v1/entities/{id}/records`

Remove todos os registros sincronizados da entidade e zera o estado da sincronização, para a próxima ler a origem do zero.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registros removidos. |
| `404` | Recurso não encontrado na organização atual. |

## entities records import

Importar registros por CSV

```text
kobana dashboards entities records import [ID] [OPÇÕES]
```

`POST /v1/entities/{id}/records/import`

Importa um CSV para uma entidade sem conexão: um registro por linha, cada coluna lida por um campo convertida para o tipo dele (números, datas, booleanos). Linhas sem identificador são recusadas e listadas pelo número da linha (a 1 é o cabeçalho).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `content` | `string` | não | O texto do CSV: primeira linha com os nomes das colunas (o `source_path` dos campos), separador `,` ou `;`, até 20.000 linhas. Envie este ou `content_base64`. |
| `content_base64` | `string` | não | O arquivo CSV em base64 (UTF-8). |
| `mode` | `string` | não | `upsert` (padrão) mantém os registros não enviados; `replace` remove os que a chamada não traz — o envio passa a ser o conjunto inteiro. Valores: `upsert`, `replace`. |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.records.import`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Quantos registros foram criados, alterados, mantidos e removidos, e os recusados. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## entities records get

Consultar registro

```text
kobana dashboards entities records get [ID] [RECORD_ID] [OPÇÕES]
```

`GET /v1/entities/{id}/records/{record_id}`

Um registro sincronizado — valores por campo e o registro como a origem o enviou — com a sua entidade.

**Argumentos do caminho:** `id`, `record_id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O registro e a entidade. |
| `404` | Recurso não encontrado na organização atual. |

## entities sync

Estado da sincronização

```text
kobana dashboards entities sync [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/entities/{id}/sync`

Onde está a sincronização da entidade: status, último erro, últimas sincronizações, total de registros e estatísticas.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Estado da sincronização. |
| `404` | Recurso não encontrado na organização atual. |

## entities sync create

Sincronizar entidade

```text
kobana dashboards entities sync create [ID] [OPÇÕES]
```

`POST /v1/entities/{id}/sync`

Enfileira uma sincronização da entidade e responde 202 com o ID; o andamento fica em `GET /entities/{id}/sync`. Uma entidade `on_access` não tem sincronização geral (422): use `POST /entities/{id}/tenant_syncs`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `full` | `string` | não | `true` (ou `1`) relê tudo e descarta o que a origem não tem mais; sem ele, a sincronização é incremental quando a origem permite. Valores: `true`, `1`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Sincronização enfileirada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## entities tenant-syncs list

Sincronizações por titular

```text
kobana dashboards entities tenant-syncs list [ID] [OPÇÕES]
```

`GET /v1/entities/{id}/tenant_syncs`

Em uma entidade `on_access`: onde está a sincronização de cada titular, dos acessados mais recentemente aos mais antigos.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `status` | `string` | não | Status da sincronização do titular. Valores: `queued`, `running`, `completed`, `error`. |
| `search` | `string` | não | Parte do Tenant ID. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sincronizações dos titulares. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## entities tenant-syncs create

Sincronizar um titular

```text
kobana dashboards entities tenant-syncs create [ID] [OPÇÕES]
```

`POST /v1/entities/{id}/tenant_syncs`

Enfileira agora a sincronização de um titular de uma entidade `on_access`. 404 quando nenhum usuário do portal tem esse Tenant ID; 422 quando a entidade não é `on_access`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `tenant_id` | `string` | sim | Tenant ID (ID do titular) a sincronizar. Vai no corpo porque pode conter qualquer caractere. |
| `full` | `boolean` | não | Quando `true`, relê todos os registros do titular. |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.tenant-syncs.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Sincronização enfileirada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## entities list

Listar entidades

```text
kobana dashboards entities list [OPÇÕES]
```

`GET /v1/entities`

Todas as entidades do workspace, com campos, origem e estado da sincronização, ordenadas pelo nome. Sem paginação.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Entidades do workspace. |
| `404` | Recurso não encontrado na organização atual. |

## entities create

Criar entidade

```text
kobana dashboards entities create [OPÇÕES]
```

`POST /v1/entities`

Registra uma entidade a partir de uma operação de leitura (ou consulta SQL) da fonte de dados com os campos escolhidos, e enfileira a primeira sincronização completa.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | sim | Nome de máquina que os blocos do portal usam; único no workspace (até 63 caracteres). |
| `name` | `string` | sim |  |
| `name_plural` | `string` | sim |  |
| `fields` | `array<object>` | sim | Os campos que a entidade mantém, em ordem. Na atualização, a lista inteira (casada por `key`). |
| `integration_id` | `string (uuid)` | não | Fonte de dados (conector) de onde ler. Omitido com `source_kind: "push"`. |
| `operation_id` | `string (uuid)` | não | Operação de leitura (endpoint, tool ou tabela) da fonte. Envie este campo ou `sql`, não os dois. |
| `sql` | `string` | não | Consulta SQL em uma fonte de banco SQL. Envie este campo ou `operation_id`. |
| `params` | `object` | não | Parâmetros com que a operação é chamada (padrão `{}`). |
| `pagination` | `object` | não |  |
| `sync_filters` | `array<object>` | não | O que a sincronização envia para filtrar os registros (até 20). Ignorado em uma consulta SQL. |
| `source_kind` | `string` | não | `push`: entidade sem conexão — sem `integration_id`, `operation_id` ou `sql`. O `source_path` de cada campo vale a chave do campo quando omitido (a propriedade que cada registro enviado traz) e `identifier_field_key` é obrigatório: é por ele que um registro enviado de novo atualiza o existente. Valores: `push`. |
| `description` | `string` | não |  |
| `identifier_field_key` | `string` | não |  |
| `label_field_key` | `string` | não |  |
| `owner_field_key` | `string` | não | Campo com o ID do titular, casado com o `tenant_id` do usuário do portal. Sem ele a entidade não aparece no portal. |
| `owner_match` | `string` | não | Como o campo do titular é comparado com o `tenant_id`: como texto (`external_id`) ou como CPF/CNPJ (`document_number`, só dígitos). Valores: `external_id`, `document_number`. |
| `sync_mode` | `string` | não | Quando a sincronização roda; ausente: `scheduled` na criação, mantido na atualização. Valores: `scheduled`, `manual`, `on_access`. |
| `freshness_seconds` | `integer` | não | `on_access`: por quantos segundos os registros sincronizados de um titular valem como atuais. |
| `records_path` | `string` | não | Caminho da lista de registros dentro da resposta, quando não detectado. |
| `sample` | `array<object>` | não | Os registros da prévia, guardados para reinferir campos depois. |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Entidade criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Conflito: chave já usada no workspace, ou a entidade é lida por uma relação. |
| `422` | Corpo da requisição falhou na validação. |

## entities preview

Prévia de uma origem

```text
kobana dashboards entities preview [OPÇÕES]
```

`POST /v1/entities/preview`

Chama a operação de leitura escolhida (ou roda a consulta SQL) e devolve a amostra com a entidade inferida dela. Nada é gravado; a chamada fica no log de requisições do conector.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `integration_id` | `string (uuid)` | sim | Fonte de dados (conector) de onde ler. Omitido com `source_kind: "push"`. |
| `operation_id` | `string (uuid)` | não | Operação de leitura (endpoint, tool ou tabela) da fonte. Envie este campo ou `sql`, não os dois. |
| `sql` | `string` | não | Consulta SQL em uma fonte de banco SQL. Envie este campo ou `operation_id`. |
| `params` | `object` | não | Parâmetros com que a operação é chamada (padrão `{}`). |
| `pagination` | `object` | não |  |
| `sync_filters` | `array<object>` | não | O que a sincronização envia para filtrar os registros (até 20). Ignorado em uma consulta SQL. |
| `limit` | `integer` | não | Tamanho da amostra (padrão 10, máximo 50). |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.preview`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Amostra e entidade inferida. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## entities get

Consultar entidade

```text
kobana dashboards entities get [ID] [OPÇÕES]
```

`GET /v1/entities/{id}`

Uma entidade com campos, origem, modo de sincronização, campo do titular e estado da sincronização.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A entidade. |
| `404` | Recurso não encontrado na organização atual. |

## entities update

Atualizar entidade

```text
kobana dashboards entities update [ID] [OPÇÕES]
```

`PATCH /v1/entities/{id}`

Nome, chave, campos especiais, paginação, filtros e modo da sincronização, e a lista de campos (casada por chave). Quando o mapeamento muda, os registros são relidos.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | sim | Nome de máquina que os blocos do portal usam; único no workspace (até 63 caracteres). |
| `name` | `string` | sim |  |
| `name_plural` | `string` | sim |  |
| `fields` | `array<object>` | sim | Os campos que a entidade mantém, em ordem. Na atualização, a lista inteira (casada por `key`). |
| `description` | `string` | não |  |
| `identifier_field_key` | `string` | não |  |
| `label_field_key` | `string` | não |  |
| `owner_field_key` | `string` | não | Campo com o ID do titular, casado com o `tenant_id` do usuário do portal. Sem ele a entidade não aparece no portal. |
| `owner_match` | `string` | não | Como o campo do titular é comparado com o `tenant_id`: como texto (`external_id`) ou como CPF/CNPJ (`document_number`, só dígitos). Valores: `external_id`, `document_number`. |
| `sync_mode` | `string` | não | Quando a sincronização roda; ausente: `scheduled` na criação, mantido na atualização. Valores: `scheduled`, `manual`, `on_access`. |
| `freshness_seconds` | `integer` | não | `on_access`: por quantos segundos os registros sincronizados de um titular valem como atuais. |
| `pagination` | `object` | não |  |
| `sync_filters` | `array<object>` | não | Filtros da sincronização (até 20); ausente mantém os atuais. |

Outros campos e objetos aninhados: `kobana schema dashboards.entities.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Entidade atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Conflito: chave já usada no workspace, ou a entidade é lida por uma relação. |
| `422` | Corpo da requisição falhou na validação. |

## entities delete

Remover entidade

```text
kobana dashboards entities delete [ID] [OPÇÕES]
```

`DELETE /v1/entities/{id}`

Remove (soft delete) a entidade e seus campos. Recusado (409) enquanto uma entidade de relação a lê.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Entidade removida. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Conflito: chave já usada no workspace, ou a entidade é lida por uma relação. |

## entities aggregate

Calcular agregados dos registros

```text
kobana dashboards entities aggregate [ID] [OPÇÕES]
```

`GET /v1/entities/{id}/aggregate`

Contagem, soma, média, mínimo ou máximo sobre os registros sincronizados, opcionalmente agrupados por um campo — o que os blocos de KPI e gráfico mostram.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `op` | `string` | não | Agregação: `count` (padrão), `sum`, `avg`, `min` ou `max`. Valores: `count`, `sum`, `avg`, `min`, `max`. |
| `field` | `string` | não | Chave do campo agregado (obrigatória exceto em `count`; numérico em `sum`/`avg`). |
| `group_by` | `string` | não | Chave do campo pelo qual agrupar. |
| `filter` | `string` | não | Objeto JSON de condições por campo: `{"campo": valor}` ou `{"campo": {"gte": 10}}`. Operadores: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `contains`, `is_null`. |
| `preview_as` | `string` | não | ID de um usuário do portal: só os registros dele, como o portal mostra (em uma entidade `on_access`, sincroniza esse titular antes). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da agregação. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## entities usage

Uso da entidade

```text
kobana dashboards entities usage [ID] [OPÇÕES]
```

`GET /v1/entities/{id}/usage`

As páginas do portal (rascunho e publicada) cujos blocos leem a entidade — o que quebra se ela for removida ou sua chave mudar.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Páginas que usam a entidade. |
| `404` | Recurso não encontrado na organização atual. |
