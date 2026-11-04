---
id: exports
title: "kobana dunning exports"
sidebar_label: "exports"
---

# kobana dunning exports


Comandos de `kobana dunning exports`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`exports charges`](#exports-charges) | `GET /v1/exports/charges` | Exportar cobranças (CSV síncrono) |
| [`exports jobs list`](#exports-jobs-list) | `GET /v1/exports/jobs` | Listar exportações assíncronas |
| [`exports jobs create`](#exports-jobs-create) | `POST /v1/exports/jobs` | Criar exportação assíncrona |
| [`exports jobs download`](#exports-jobs-download) | `GET /v1/exports/jobs/{id}/download` | Baixar CSV da exportação |
| [`exports people`](#exports-people) | `GET /v1/exports/people` | Exportar clientes (CSV síncrono) |

## exports charges

Exportar cobranças (CSV síncrono)

```text
kobana dunning exports charges [OPÇÕES]
```

`GET /v1/exports/charges`

Gera e responde na hora o CSV de cobranças (`text/csv; charset=utf-8` com BOM, `Content-Disposition: attachment`), com filtros opcionais de status e período de vencimento. Colunas: cliente, documento_cliente, numero_documento, descricao, valor_original, valor_atual, vencimento, dias_atraso, status, origem, external_id. Teto de 10 mil linhas; acima disso use a exportação assíncrona (POST /exports/jobs). Exige o nível de acesso `export` (ou `all`) — `read` e `write` não bastam.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Filtra as cobranças pelo status (ex.: `pending`, `overdue`, `paid`). |
| `from` | `string` | não | Vencimento a partir de (data ISO `YYYY-MM-DD`). |
| `to` | `string` | não | Vencimento até (data ISO `YYYY-MM-DD`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivo CSV (`text/csv; charset=utf-8` com BOM para o Excel abrir acentos corretamente; `Content-Disposition: attachment`). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## exports jobs list

Listar exportações assíncronas

```text
kobana dunning exports jobs list [OPÇÕES]
```

`GET /v1/exports/jobs`

Histórico das 50 exportações assíncronas mais recentes da organização (sem paginação). Use esta rota para acompanhar o status dos jobs; o CSV em si não vem na listagem. É uma leitura (`read`): o arquivo em si exige `export`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista das exportações mais recentes. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## exports jobs create

Criar exportação assíncrona

```text
kobana dunning exports jobs create [OPÇÕES]
```

`POST /v1/exports/jobs`

Agenda a geração do CSV num worker, sem o teto de 10 mil linhas do modo síncrono e sem travar a request. Acompanhe o status em GET /exports/jobs e baixe o arquivo em GET /exports/jobs/\{id\}/download antes de `expires_at`. Responde 202. Exige o nível de acesso `export` (ou `all`) — `read` e `write` não bastam.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `resource_type` | `string` | sim | Tipo de recurso exportado (`people` = clientes \| `charges` = cobranças). Valores: `people`, `charges`. |
| `file_name` | `string` | não | Nome do arquivo gerado (padrão: `{resourceType}.csv`). |
| `filters` | `object` | não | Filtros aplicados à exportação (somente `charges`); ficam gravados no job para auditoria. |

Outros campos e objetos aninhados: `kobana schema dunning.exports.jobs.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Exportação aceita e enfileirada para processamento. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## exports jobs download

Baixar CSV da exportação

```text
kobana dunning exports jobs download [ID] [OPÇÕES]
```

`GET /v1/exports/jobs/{id}/download`

Serve o CSV já gerado de uma exportação assíncrona (`text/csv; charset=utf-8` com BOM). Só entrega se o job estiver `completed` e dentro da validade (`expires_at`). Exige o nível de acesso `export` (ou `all`) — `read` e `write` não bastam.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivo CSV (`text/csv; charset=utf-8` com BOM para o Excel abrir acentos corretamente; `Content-Disposition: attachment`). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Exportação ainda não concluída (código `NOT_READY`; o campo `status` informa o estado atual). |
| `410` | Exportação expirada — gere uma nova (código `EXPIRED`). |
| `413` | Corpo da requisição acima de 2 MB. |

## exports people

Exportar clientes (CSV síncrono)

```text
kobana dunning exports people [OPÇÕES]
```

`GET /v1/exports/people`

Gera e responde na hora o CSV de clientes (`text/csv; charset=utf-8` com BOM, `Content-Disposition: attachment`). Colunas: nome, documento, tipo_documento, email, telefone, classificacao, external_id, tags, criado_em. Teto de 10 mil linhas; acima disso use a exportação assíncrona (POST /exports/jobs). Exige o nível de acesso `export` (ou `all`) — `read` e `write` não bastam.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivo CSV (`text/csv; charset=utf-8` com BOM para o Excel abrir acentos corretamente; `Content-Disposition: attachment`). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
