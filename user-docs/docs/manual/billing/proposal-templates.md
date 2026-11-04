---
id: proposal-templates
title: "kobana billing proposal-templates"
sidebar_label: "proposal-templates"
---

# kobana billing proposal-templates


Comandos de `kobana billing proposal-templates`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`proposal-templates list`](#proposal-templates-list) | `GET /v1/proposal_templates` | Listar templates de proposta |
| [`proposal-templates create`](#proposal-templates-create) | `POST /v1/proposal_templates` | Criar template de proposta |
| [`proposal-templates update`](#proposal-templates-update) | `PUT /v1/proposal_templates` | Criar ou atualizar modelo de proposta |
| [`proposal-templates get`](#proposal-templates-get) | `GET /v1/proposal_templates/{id}` | Obter template de proposta |
| [`proposal-templates delete`](#proposal-templates-delete) | `DELETE /v1/proposal_templates/{id}` | Excluir template de proposta |
| [`proposal-templates activate`](#proposal-templates-activate) | `POST /v1/proposal_templates/{id}/activate` | Ativar template de proposta |
| [`proposal-templates deactivate`](#proposal-templates-deactivate) | `POST /v1/proposal_templates/{id}/deactivate` | Desativar template de proposta |
| [`proposal-templates duplicate`](#proposal-templates-duplicate) | `POST /v1/proposal_templates/{id}/duplicate` | Duplicar template de proposta |

## proposal-templates list

Listar templates de proposta

```text
kobana billing proposal-templates list [OPÇÕES]
```

`GET /v1/proposal_templates`

Retorna uma lista paginada dos templates de proposta da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca textual por nome ou descrição. |
| `is_active` | `boolean` | não | Filtra por templates ativos (`true`) ou desativados (`false`). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de templates de proposta. |
| `404` | Recurso não encontrado na organização atual. |

## proposal-templates create

Criar template de proposta

```text
kobana billing proposal-templates create [OPÇÕES]
```

`POST /v1/proposal_templates`

Cria um template de proposta vinculado a um Google Doc ou Google Slides. O `google_doc_id` precisa ser único por organização.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do template (único por organização). |
| `google_doc_id` | `string` | sim | ID do Google Doc/Slides usado como template (único por organização). |
| `description` | `string` | não | Descrição opcional do template. |
| `document_type` | `string` | não | Tipo do documento base: `google_doc` ou `google_slides`. Padrão: `google_doc`. Valores: `google_doc`, `google_slides`. |
| `app_script_url` | `string (uri)` | não | URL do Apps Script publicado que renderiza o template, quando aplicável. |
| `google_drive_folder_id` | `string` | não | ID da pasta do Google Drive onde as cópias geradas serão salvas. |
| `is_active` | `boolean` | não | Quando `false`, cria o template já desativado. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados livres definidos pelo cliente para correlação interna. |

Outros campos e objetos aninhados: `kobana schema billing.proposal-templates.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Template de proposta criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposal-templates update

Criar ou atualizar modelo de proposta

```text
kobana billing proposal-templates update [OPÇÕES]
```

`PUT /v1/proposal_templates`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do template (único por organização). |
| `description` | `string` | não | Descrição opcional do template. |
| `document_type` | `string` | não | Tipo do documento base: `google_doc` ou `google_slides`. Padrão: `google_doc`. Valores: `google_doc`, `google_slides`. |
| `google_doc_id` | `string` | não | ID do Google Doc/Slides usado como template (único por organização). |
| `app_script_url` | `string (uri)` | não | URL do Apps Script publicado que renderiza o template, quando aplicável. |
| `google_drive_folder_id` | `string` | não | ID da pasta do Google Drive onde as cópias geradas serão salvas. |
| `is_active` | `boolean` | não | Quando `false`, cria o template já desativado. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados livres definidos pelo cliente para correlação interna. |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.proposal-templates.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposal-templates get

Obter template de proposta

```text
kobana billing proposal-templates get [ID] [OPÇÕES]
```

`GET /v1/proposal_templates/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do template de proposta. |
| `404` | Recurso não encontrado na organização atual. |

## proposal-templates delete

Excluir template de proposta

```text
kobana billing proposal-templates delete [ID] [OPÇÕES]
```

`DELETE /v1/proposal_templates/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Template de proposta removido. |
| `404` | Recurso não encontrado na organização atual. |

## proposal-templates activate

Ativar template de proposta

```text
kobana billing proposal-templates activate [ID] [OPÇÕES]
```

`POST /v1/proposal_templates/{id}/activate`

Ativa um template de proposta previamente desativado, tornando-o disponível para uso em novas propostas.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Template de proposta ativado. |
| `404` | Recurso não encontrado na organização atual. |

## proposal-templates deactivate

Desativar template de proposta

```text
kobana billing proposal-templates deactivate [ID] [OPÇÕES]
```

`POST /v1/proposal_templates/{id}/deactivate`

Desativa um template de proposta. Templates desativados não podem ser usados em novas propostas, mas as propostas já criadas permanecem inalteradas.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Template de proposta desativado. |
| `404` | Recurso não encontrado na organização atual. |

## proposal-templates duplicate

Duplicar template de proposta

```text
kobana billing proposal-templates duplicate [ID] [OPÇÕES]
```

`POST /v1/proposal_templates/{id}/duplicate`

Cria uma cópia do template informado, mantendo configuração e metadados. O novo template recebe um nome derivado.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Template de proposta duplicado. |
| `404` | Recurso não encontrado na organização atual. |
