---
id: data-sources
title: "kobana dashboards data-sources"
sidebar_label: "data-sources"
---

# kobana dashboards data-sources


Comandos de `kobana dashboards data-sources`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`data-sources catalog list`](#data-sources-catalog-list) | `GET /v1/data_sources/catalog` | Listar produtos do catálogo Kobana |
| [`data-sources catalog create`](#data-sources-catalog-create) | `POST /v1/data_sources/catalog` | Conectar produto Kobana |
| [`data-sources logs get`](#data-sources-logs-get) | `GET /v1/data_sources/{id}/logs/{log_id}` | Consultar requisição da fonte |
| [`data-sources logs list`](#data-sources-logs-list) | `GET /v1/data_sources/{id}/logs` | Listar log de requisições da fonte |
| [`data-sources openapi analyze`](#data-sources-openapi-analyze) | `POST /v1/data_sources/openapi/analyze` | Analisar spec OpenAPI |
| [`data-sources operations update`](#data-sources-operations-update) | `PATCH /v1/data_sources/{id}/operations/{operation_id}` | Ligar ou desligar operação da fonte |
| [`data-sources list`](#data-sources-list) | `GET /v1/data_sources` | Listar fontes de dados |
| [`data-sources create`](#data-sources-create) | `POST /v1/data_sources` | Criar fonte de dados |
| [`data-sources test`](#data-sources-test) | `POST /v1/data_sources/test` | Testar conexão antes de salvar |
| [`data-sources get`](#data-sources-get) | `GET /v1/data_sources/{id}` | Consultar fonte de dados |
| [`data-sources update`](#data-sources-update) | `PATCH /v1/data_sources/{id}` | Atualizar fonte de dados |
| [`data-sources delete`](#data-sources-delete) | `DELETE /v1/data_sources/{id}` | Excluir fonte de dados |
| [`data-sources authorize`](#data-sources-authorize) | `POST /v1/data_sources/{id}/authorize` | Autorizar fonte MCP |
| [`data-sources schedule`](#data-sources-schedule) | `PUT /v1/data_sources/{id}/schedule` | Agendar sincronização da fonte |
| [`data-sources spec`](#data-sources-spec) | `PUT /v1/data_sources/{id}/spec` | Substituir spec da fonte |
| [`data-sources sync`](#data-sources-sync) | `POST /v1/data_sources/{id}/sync` | Sincronizar fonte de dados |

## data-sources catalog list

Listar produtos do catálogo Kobana

```text
kobana dashboards data-sources catalog list [OPÇÕES]
```

`GET /v1/data_sources/catalog`

Produtos Kobana que podem ser conectados como fonte de dados só com um token. Lista completa, sem paginação.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Produtos do catálogo. |
| `404` | Recurso não encontrado na organização atual. |

## data-sources catalog create

Conectar produto Kobana

```text
kobana dashboards data-sources catalog create [OPÇÕES]
```

`POST /v1/data_sources/catalog`

Salva uma fonte OpenAPI a partir da spec do produto que acompanha a aplicação e testa a conexão.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `product` | `string` | sim | Chave do produto do catálogo Kobana (veja `GET /data_sources/catalog`). |
| `environment` | `string` | não | Valores: `production`, `sandbox`. |
| `token` | `string` | não | Token de API do produto. Guardado criptografado. |
| `workspace_id` | `string` | não | Workspace dentro do produto, quando a organização do token tem mais de um. |
| `name` | `string` | não | Nome de exibição, único por workspace e tipo. Quando omitido, é derivado da spec, do host ou do banco. |

Outros campos e objetos aninhados: `kobana schema dashboards.data-sources.catalog.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Fonte criada, com o resultado do teste de conexão. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Já existe uma fonte de dados com este nome no workspace. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources logs get

Consultar requisição da fonte

```text
kobana dashboards data-sources logs get [ID] [LOG_ID] [OPÇÕES]
```

`GET /v1/data_sources/{id}/logs/{log_id}`

Uma chamada inteira: cabeçalhos e corpos da requisição e da resposta, com segredos mascarados.

**Argumentos do caminho:** `id`, `log_id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Entrada do log de requisições. |
| `404` | Recurso não encontrado na organização atual. |

## data-sources logs list

Listar log de requisições da fonte

```text
kobana dashboards data-sources logs list [ID] [OPÇÕES]
```

`GET /v1/data_sources/{id}/logs`

Chamadas que a fonte fez (requisições HTTP, consultas ao banco), das mais novas para as mais antigas, sem cabeçalhos nem corpos.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `method` | `string` | não | Método HTTP (GET, POST…). |
| `success` | `string` | não | `true` só as chamadas bem-sucedidas, `false` só as que falharam. Valores: `true`, `false`. |
| `search` | `string` | não | Trecho do endpoint, sem diferenciar maiúsculas. |
| `date_from` | `string` | não | A partir deste dia (YYYY-MM-DD, UTC), inclusive. |
| `date_to` | `string` | não | Até este dia (YYYY-MM-DD, UTC), inclusive. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página do log de requisições. |
| `404` | Recurso não encontrado na organização atual. |

## data-sources openapi analyze

Analisar spec OpenAPI

```text
kobana dashboards data-sources openapi analyze [OPÇÕES]
```

`POST /v1/data_sources/openapi/analyze`

Lê a spec e devolve o que seria importado — título, servidores, esquemas de segurança, autenticação sugerida e operações — sem salvar nada.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `spec` | `string` | sim | O documento OpenAPI 3.x ou Swagger 2.0 como texto (JSON ou YAML), até 5 MB — limitado também pelos 2 MB de corpo da API. |
| `spec_file_name` | `string` | não | Nome do arquivo da spec (`openapi.yaml`). Quando omitido, é deduzido do conteúdo. |

Outros campos e objetos aninhados: `kobana schema dashboards.data-sources.openapi.analyze`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Análise da spec. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources operations update

Ligar ou desligar operação da fonte

```text
kobana dashboards data-sources operations update [ID] [OPERATION_ID] [OPÇÕES]
```

`PATCH /v1/data_sources/{id}/operations/{operation_id}`

Liga ou desliga uma operação da fonte (`enabled`). Operações desligadas não alimentam entidades.

**Argumentos do caminho:** `id`, `operation_id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `enabled` | `boolean` | sim | Operações desligadas não alimentam entidades. |

Outros campos e objetos aninhados: `kobana schema dashboards.data-sources.operations.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Operação atualizada. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources list

Listar fontes de dados

```text
kobana dashboards data-sources list [OPÇÕES]
```

`GET /v1/data_sources`

Fontes de dados do workspace — APIs OpenAPI, servidores MCP e bancos somente leitura —, das mais novas para as mais antigas. Os segredos nunca são devolvidos. Para criar uma fonte, use a rota do tipo: `/data_sources/openapi`, `/mcp`, `/database` ou `/catalog`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de fontes de dados. |
| `404` | Recurso não encontrado na organização atual. |

## data-sources create

Criar fonte de dados

```text
kobana dashboards data-sources create [OPÇÕES]
```

`POST /v1/data_sources`

Salva uma fonte de dados e testa a conexão. `kind` diz qual: `openapi` (API descrita por uma spec OpenAPI — a spec vai como texto em `spec`, o certificado mTLS em base64 em `certificate`; importa as operações), `mcp` (servidor MCP — `url`; sem OAuth conecta e descobre as tools na hora, com OAuth devolve `authorization_url`, que uma pessoa logada no dashboard abre no navegador para concluir) ou `database` (banco somente leitura — `engine` e conexão; cataloga as tabelas ou coleções). Envie só os campos do `kind` escolhido. Produtos Kobana entram por `POST /data_sources/catalog`. Substitui `POST /data_sources/openapi`, `/mcp` e `/database`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema dashboards.data-sources.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Fonte criada, com o resultado do teste de conexão. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Já existe uma fonte de dados com este nome no workspace. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources test

Testar conexão antes de salvar

```text
kobana dashboards data-sources test [OPÇÕES]
```

`POST /v1/data_sources/test`

Testa uma conexão ainda não salva. Informe `kind` (`openapi`, `mcp`, `database` ou `catalog`) e o corpo da rota de criação correspondente. Nada é persistido.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema dashboards.data-sources.test`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do teste de conexão. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources get

Consultar fonte de dados

```text
kobana dashboards data-sources get [ID] [OPÇÕES]
```

`GET /v1/data_sources/{id}`

A fonte de dados e suas operações (endpoints, tools ou tabelas que ela pode ler).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Fonte de dados com as operações. |
| `404` | Recurso não encontrado na organização atual. |

## data-sources update

Atualizar fonte de dados

```text
kobana dashboards data-sources update [ID] [OPÇÕES]
```

`PATCH /v1/data_sources/{id}`

Atualiza nome, `is_active` e os campos de conexão do tipo da fonte. Campos secretos em branco mantêm os guardados. Uma fonte OpenAPI recebe um novo certificado em `certificate`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome de exibição, único por workspace e tipo. Quando omitido, é derivado da spec, do host ou do banco. |
| `is_active` | `boolean` | não |  |
| `base_url` | `string` | não | Base URL da API. Quando omitida, usa o primeiro servidor absoluto da spec. |
| `auth` | `object` | não | Como a API autentica: `none`, `header`, `query`, `bearer`, `basic` ou `cert`. Em `header`/`query`, informe os nomes e valores das chaves; um valor em branco numa atualização mantém o guardado. |
| `credentials` | `object` | não | Segredos da autenticação escolhida: `token` (bearer), `username`/`password` (basic), `certificate_password` (cert). Guardados criptografados e nunca devolvidos. |
| `certificate` | `object` | não | Certificado de cliente para mTLS (`auth.kind = "cert"`). |
| `probe_operation_key` | `string` | não | OpenAPI: operação usada como sonda no teste de conexão; `null` escolhe automaticamente. |
| `default_params` | `object` | não | OpenAPI: valores padrão de parâmetros enviados em toda chamada. |
| `response_format` | `string` | não | Valores: `json`, `xml`. |
| `url` | `string` | não | URL do servidor MCP (endpoint Streamable HTTP ou SSE). Um token na query string é guardado criptografado. |
| `transport` | `string` | não | Valores: `http`, `sse`. |
| `headers` | `anyOf` | não | Cabeçalhos extras enviados em toda chamada: um objeto, ou um `Nome: valor` por linha. Guardados criptografados. |
| `oauth` | `object` | não | OAuth do servidor MCP: `none`, `dcr` (registro dinâmico de cliente) ou `client` (seu próprio client id/secret). |
| `engine` | `string` | não | Valores: `postgres`, `mysql`, `mariadb`, `mssql`, `mongodb`. |
| `host` | `string` | não |  |
| `port` | `anyOf` | não |  |
| `database` | `string` | não |  |
| `username` | `string` | não |  |
| `password` | `string` | não |  |
| `connection_string` | `string` | não | Somente MongoDB: a string de conexão completa (no lugar de host, porta, usuário e senha). Autenticação só por usuário e senha (SCRAM); opções aceitas: authSource, authMechanism (SCRAM-SHA-1/SCRAM-SHA-256), replicaSet, tls/ssl, retryWrites, retryReads, w, journal, wtimeoutMS, readPreference, readConcernLevel, appName, directConnection, loadBalanced, compressors, zlibCompressionLevel e os tempos/limites de pool (maxPoolSize, minPoolSize, maxIdleTimeMS, connectTimeoutMS, socketTimeoutMS, serverSelectionTimeoutMS, heartbeatFrequencyMS, localThresholdMS, maxStalenessSeconds). Qualquer outra opção é recusada com 422. |
| `ssl` | `boolean` | não |  |
| `ssl_verify` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.data-sources.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Fonte de dados atualizada. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Já existe uma fonte de dados com este nome no workspace. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources delete

Excluir fonte de dados

```text
kobana dashboards data-sources delete [ID] [OPÇÕES]
```

`DELETE /v1/data_sources/{id}`

Remove a fonte, suas operações e a agenda de sincronização. Recusado (409) enquanto uma entidade lê a fonte: remova as entidades antes. Não pode ser desfeito.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Fonte excluída. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Uma ou mais entidades leem esta fonte de dados. |

## data-sources authorize

Autorizar fonte MCP

```text
kobana dashboards data-sources authorize [ID] [OPÇÕES]
```

`POST /v1/data_sources/{id}/authorize`

Reinicia a autorização OAuth de um servidor MCP. Devolve `authorization_url` para uma pessoa logada no dashboard abrir no navegador, ou `null` e o teste de conexão quando os tokens guardados ainda funcionam.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | URL de autorização ou resultado do teste. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources schedule

Agendar sincronização da fonte

```text
kobana dashboards data-sources schedule [ID] [OPÇÕES]
```

`PUT /v1/data_sources/{id}/schedule`

Define com que frequência a fonte sincroniza: `{ cron, timezone }`, ou `{ cron: null }` para só sob demanda. Devolve as próximas execuções.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cron` | `string` | sim | Cron de 5 campos (no máximo a cada 5 minutos), ou `null` para sincronizar só sob demanda. |
| `timezone` | `string` | não | Fuso horário IANA da agenda. Padrão: o atual da fonte, ou America/Sao_Paulo. |

Outros campos e objetos aninhados: `kobana schema dashboards.data-sources.schedule`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Agenda salva. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources spec

Substituir spec da fonte

```text
kobana dashboards data-sources spec [ID] [OPÇÕES]
```

`PUT /v1/data_sources/{id}/spec`

Troca a spec OpenAPI e ressincroniza as operações: as que mantêm a chave mantêm suas configurações; as que sumiram da spec são removidas.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `spec` | `string` | sim | O documento OpenAPI 3.x ou Swagger 2.0 como texto (JSON ou YAML), até 5 MB — limitado também pelos 2 MB de corpo da API. |
| `spec_file_name` | `string` | não | Nome do arquivo da spec (`openapi.yaml`). Quando omitido, é deduzido do conteúdo. |

Outros campos e objetos aninhados: `kobana schema dashboards.data-sources.spec`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Spec substituída, com os avisos da análise. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## data-sources sync

Sincronizar fonte de dados

```text
kobana dashboards data-sources sync [ID] [OPÇÕES]
```

`POST /v1/data_sources/{id}/sync`

Enfileira a sincronização de todas as entidades alimentadas pela fonte. A execução é assíncrona.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `full` | `boolean` | não | `true` relê tudo em vez de só o que mudou desde a última sincronização. |

Outros campos e objetos aninhados: `kobana schema dashboards.data-sources.sync`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Sincronização enfileirada. |
| `400` | Corpo que não é JSON, ou campo com tipo errado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
