---
id: nfes
title: "kobana billing nfes"
sidebar_label: "nfes"
---

# kobana billing nfes


Comandos de `kobana billing nfes`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`nfes bulk create`](#nfes-bulk-create) | `POST /v1/nfes/bulk` | Atualizar NFes em lote |
| [`nfes bulk delete`](#nfes-bulk-delete) | `DELETE /v1/nfes/bulk` | Excluir NFes em lote |
| [`nfes request-tries list`](#nfes-request-tries-list) | `GET /v1/nfes/request-tries` | Listar tentativas de requisição (organização) |
| [`nfes request-tries get`](#nfes-request-tries-get) | `GET /v1/nfes/{id}/request-tries` | Listar tentativas de requisição da NFe |
| [`nfes requests get`](#nfes-requests-get) | `GET /v1/nfes/{id}/requests/{request_id}` | Obter requisição externa |
| [`nfes requests list`](#nfes-requests-list) | `GET /v1/nfes/{id}/requests` | Listar requisições externas da NFe |
| [`nfes list`](#nfes-list) | `GET /v1/nfes` | Listar NFes |
| [`nfes create`](#nfes-create) | `POST /v1/nfes` | Criar NFe (rascunho) |
| [`nfes update`](#nfes-update) | `PUT /v1/nfes` | Criar ou atualizar NF-e |
| [`nfes fetch-barueri`](#nfes-fetch-barueri) | `POST /v1/nfes/fetch-barueri` | Buscar NFS-e em Barueri |
| [`nfes fetch-from-sefaz`](#nfes-fetch-from-sefaz) | `POST /v1/nfes/fetch-from-sefaz` | Buscar NFS-e no ADN (SEFAZ) |
| [`nfes import-barueri`](#nfes-import-barueri) | `POST /v1/nfes/import-barueri` | Importar NFS-e Barueri |
| [`nfes sync`](#nfes-sync) | `POST /v1/nfes/sync` | Sincronizar NFes da organização |
| [`nfes get`](#nfes-get) | `GET /v1/nfes/{id}` | Obter NFe |
| [`nfes delete`](#nfes-delete) | `DELETE /v1/nfes/{id}` | Excluir NFe |
| [`nfes cancel`](#nfes-cancel) | `POST /v1/nfes/{id}/cancel` | Cancelar NFe |
| [`nfes issue`](#nfes-issue) | `POST /v1/nfes/{id}/issue` | Emitir NFe |
| [`nfes pdf`](#nfes-pdf) | `GET /v1/nfes/{id}/pdf` | Baixar PDF da NFe |
| [`nfes retry-cancel`](#nfes-retry-cancel) | `POST /v1/nfes/{id}/retry-cancel` | Retentar cancelamento da NFe |
| [`nfes validate-barueri`](#nfes-validate-barueri) | `POST /v1/nfes/{id}/validate-barueri` | Validar NFS-e Barueri |
| [`nfes xml`](#nfes-xml) | `GET /v1/nfes/{id}/xml` | Baixar XML da NFe |

## nfes bulk create

Atualizar NFes em lote

```text
kobana billing nfes bulk create [OPÇÕES]
```

`POST /v1/nfes/bulk`

Atualiza várias NFes de uma vez. Hoje suporta vincular ou desvincular uma fatura (`invoice_id`) em até 100 NFes por requisição.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `nfe_ids` | `array<string (uuid)>` | sim | UUIDs das NFes a atualizar. Mínimo 1. |
| `invoice_id` | `string (uuid)` | sim | UUID da fatura a vincular. Envie `null` para desvincular. |
| `invoice_withholding` | `string` | não | O que fazer se a retenção das NFes for diferente da retenção da fatura: `block` (padrão) recusa com 409 `NFE_WITHHOLDING_MISMATCH`; `update_invoice` atualiza a fatura com a retenção da NF-e. Valores: `block`, `update_invoice`. |

Outros campos e objetos aninhados: `kobana schema billing.nfes.bulk.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Quantidade de NFes atualizadas. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A retenção da NF-e é diferente da retenção da fatura vinculada (`NFE_WITHHOLDING_MISMATCH`). `details` traz os dois lados; repita com `invoice_withholding: "update_invoice"` para atualizar a fatura. |
| `422` | Corpo da requisição falhou na validação. |

## nfes bulk delete

Excluir NFes em lote

```text
kobana billing nfes bulk delete [OPÇÕES]
```

`DELETE /v1/nfes/bulk`

Exclui até 100 NFes de uma vez. Todas devem estar em status `draft` e pertencer à organização atual. Exige o nível de acesso `delete`. Substitui `POST /v1/nfes/bulk-delete`, removido.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | UUIDs das NFes a excluir. Mínimo 1, máximo 100. |

Outros campos e objetos aninhados: `kobana schema billing.nfes.bulk.delete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Quantidade de NFes excluídas. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes request-tries list

Listar tentativas de requisição (organização)

```text
kobana billing nfes request-tries list [OPÇÕES]
```

`GET /v1/nfes/request-tries`

Lista todas as tentativas de requisição ao provedor de NFe da organização, paginadas e ordenadas por data decrescente.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de tentativas. |
| `404` | Recurso não encontrado na organização atual. |

## nfes request-tries get

Listar tentativas de requisição da NFe

```text
kobana billing nfes request-tries get [ID] [OPÇÕES]
```

`GET /v1/nfes/{id}/request-tries`

Lista as tentativas (`NfeRequestTry`) feitas ao provedor para esta NFe — uma entrada por chamada, com endpoint, status HTTP, duração e mensagens de erro.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de tentativas. |
| `404` | Recurso não encontrado na organização atual. |

## nfes requests get

Obter requisição externa

```text
kobana billing nfes requests get [ID] [REQUEST_ID] [OPÇÕES]
```

`GET /v1/nfes/{id}/requests/{request_id}`

**Argumentos do caminho:** `id`, `request_id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da requisição externa, incluindo cabeçalhos e corpos. |
| `404` | Recurso não encontrado na organização atual. |

## nfes requests list

Listar requisições externas da NFe

```text
kobana billing nfes requests list [ID] [OPÇÕES]
```

`GET /v1/nfes/{id}/requests`

Lista as requisições HTTP capturadas para esta NFe (chamadas ao provedor e webhooks recebidos), com paginação. Use `direction` para filtrar entre `outbound` e `inbound`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `direction` | `string` | não | Filtra requisições por direção: `outbound` (chamadas saindo) ou `inbound` (webhooks recebidos). Valores: `outbound`, `inbound`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de requisições. |
| `404` | Recurso não encontrado na organização atual. |

## nfes list

Listar NFes

```text
kobana billing nfes list [OPÇÕES]
```

`GET /v1/nfes`

Retorna uma lista paginada de NFes (Notas Fiscais Eletrônicas) da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `person_id` | `string` | não | Filtra por UUID da pessoa associada. |
| `person_document_number` | `string` | não | Filtra por CPF/CNPJ da pessoa (exato). |
| `person_name_contains` | `string` | não | Filtra por nome da pessoa (busca parcial, sem distinção de maiúsculas). |
| `billing_account_id` | `string` | não | Filtra por `billing_account_id`. |
| `company_id` | `string` | não | Filtra por `company_id` emissor. |
| `invoice_id` | `string` | não | Filtra por `invoice_id` vinculado. |
| `payment_id` | `string` | não | Filtra por `payment_id` vinculado. |
| `transaction_id` | `string` | não | Filtra por `transaction_id` vinculado. |
| `status` | `string` | não | Filtra por status: `draft`, `pending`, `processing`, `issued`, `pending_cancel`, `canceling`, `canceled` ou `error`. Valores: `draft`, `pending`, `processing`, `issued`, `pending_cancel`, `canceling`, `canceled`, `error`. |
| `invoice_filter` | `string` | não | Filtra pela presença de fatura: `with` (apenas com fatura) ou `without` (apenas avulsas). Valores: `with`, `without`. |
| `nfe_number` | `integer,null` | não | Filtra pelo número da NFe. |
| `nfe_series` | `string` | não | Filtra pela série da NFe. |
| `nfe_key` | `string` | não | Filtra pela chave de acesso da NFe. |
| `external_id` | `string` | não | Filtra pelo identificador externo da NFe. |
| `lc116_code` | `string` | não | Filtra pelo código LC 116. |
| `service_code` | `string` | não | Filtra pelo código do serviço municipal. |
| `amount_min` | `number,null` | não | Valor mínimo da NFe em reais (filtro `amount`). |
| `amount_max` | `number,null` | não | Valor máximo da NFe em reais (filtro `amount`). |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `issue_date_from` | `string` | não | Filtra por `issue_date` inicial (inclusivo, formato YYYY-MM-DD). |
| `issue_date_to` | `string` | não | Filtra por `issue_date` final (inclusivo, formato YYYY-MM-DD). |
| `cancelled_after` | `string` | não | Filtra por `cancelled_at` inicial (inclusivo, formato YYYY-MM-DD). |
| `cancelled_before` | `string` | não | Filtra por `cancelled_at` final (inclusivo, formato YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de NFes. |
| `404` | Recurso não encontrado na organização atual. |

## nfes create

Criar NFe (rascunho)

```text
kobana billing nfes create [OPÇÕES]
```

`POST /v1/nfes`

Cria uma NFe em rascunho (`draft`) para emissão posterior via `POST /nfes/{id}/issue`. Aceita dados do tomador (`customer`) e alíquotas opcionais.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sim | UUID da conta de cobrança (tomador). **Obrigatório**. |
| `amount` | `number` | sim | Valor da NFe em reais (float, até 4 casas decimais). **Obrigatório**, deve ser positivo. |
| `customer` | `object` | sim | Dados do tomador no momento da emissão. **Obrigatório**. |
| `company_id` | `string (uuid)` | não | UUID da empresa emissora (prestador). Padrão: empresa padrão da organização. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `invoice_id` | `string (uuid)` | não | UUID da fatura vinculada (opcional). |
| `transaction_id` | `string (uuid)` | não | UUID da transação de origem (opcional). |
| `service_code` | `string` | não | Código do serviço conforme a tabela municipal. |
| `service_description` | `string` | não | Descrição livre do serviço prestado. |
| `lc116_code` | `string` | não | Código LC 116. |
| `cnae_code` | `string` | não | Código CNAE. |
| `nbs_code` | `string` | não | Código NBS. |
| `c_class_trib` | `string` | não | Classificação Tributária (cClassTrib) do IBS/CBS. |
| `ind_op` | `string` | não | Indicador de Operação (indOp) do IBS/CBS. |
| `rps_number` | `integer` | não | Número do RPS (quando a numeração é gerenciada pela aplicação). |
| `rps_serial_number` | `string` | não | Série do RPS. |
| `taxes` | `object` | não | Alíquotas de impostos (em decimal, ex.: `0.05` para 5%). Aplica os defaults da empresa quando omitido. |
| `custom_metadata` | `object` | não | Campos personalizados (validados contra o schema da organização). |

Outros campos e objetos aninhados: `kobana schema billing.nfes.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | NFe criada em rascunho. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes update

Criar ou atualizar NF-e

```text
kobana billing nfes update [OPÇÕES]
```

`PUT /v1/nfes`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança (tomador). **Obrigatório**. |
| `company_id` | `string (uuid)` | não | UUID da empresa emissora (prestador). Padrão: empresa padrão da organização. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `invoice_id` | `string (uuid)` | não | UUID da fatura vinculada (opcional). |
| `transaction_id` | `string (uuid)` | não | UUID da transação de origem (opcional). |
| `amount` | `number` | não | Valor da NFe em reais (float, até 4 casas decimais). **Obrigatório**, deve ser positivo. |
| `service_code` | `string` | não | Código do serviço conforme a tabela municipal. |
| `service_description` | `string` | não | Descrição livre do serviço prestado. |
| `lc116_code` | `string` | não | Código LC 116. |
| `cnae_code` | `string` | não | Código CNAE. |
| `nbs_code` | `string` | não | Código NBS. |
| `c_class_trib` | `string` | não | Classificação Tributária (cClassTrib) do IBS/CBS. |
| `ind_op` | `string` | não | Indicador de Operação (indOp) do IBS/CBS. |
| `rps_number` | `integer` | não | Número do RPS (quando a numeração é gerenciada pela aplicação). |
| `rps_serial_number` | `string` | não | Série do RPS. |
| `customer` | `object` | não | Dados do tomador no momento da emissão. **Obrigatório**. |
| `taxes` | `object` | não | Alíquotas de impostos (em decimal, ex.: `0.05` para 5%). Aplica os defaults da empresa quando omitido. |
| `custom_metadata` | `object` | não | Campos personalizados (validados contra o schema da organização). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.nfes.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes fetch-barueri

Buscar NFS-e em Barueri

```text
kobana billing nfes fetch-barueri [OPÇÕES]
```

`POST /v1/nfes/fetch-barueri`

Busca uma NFS-e no endpoint municipal de Barueri usando o `codigo_autenticidade` impresso no DANFSE e o CPF/CNPJ do tomador. Retorna o XML cru e, quando encontrada, a NFe já existente na organização.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `codigo_autenticidade` | `string` | sim | Código de autenticidade impresso no DANFSE da NFS-e. |
| `cnpj_tomador` | `string` | sim | CPF ou CNPJ do tomador da NFS-e (somente dígitos ou com formatação). |
| `ambiente` | `string` | não | Ambiente municipal: `production` ou `homologation`. Padrão: `production`. Valores: `production`, `homologation`. |

Outros campos e objetos aninhados: `kobana schema billing.nfes.fetch-barueri`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da busca com XML e referência à NFe existente, quando houver. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes fetch-from-sefaz

Buscar NFS-e no ADN (SEFAZ)

```text
kobana billing nfes fetch-from-sefaz [OPÇÕES]
```

`POST /v1/nfes/fetch-from-sefaz`

Busca uma NFS-e no Ambiente de Dados Nacional (ADN) usando uma chave de acesso de 50 dígitos e um certificado digital previamente cadastrado (autenticação mTLS).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `access_key` | `string` | sim | Chave de acesso da NFS-e (50 dígitos). |
| `certificate_id` | `string` | sim | UUID do certificado digital cadastrado na organização (usado para autenticação mTLS). |
| `ambiente` | `anyOf` | não | Ambiente SEFAZ: `1` (produção) ou `2` (homologação). Padrão: `2`. |

Outros campos e objetos aninhados: `kobana schema billing.nfes.fetch-from-sefaz`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da busca no ADN. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes import-barueri

Importar NFS-e Barueri

```text
kobana billing nfes import-barueri [OPÇÕES]
```

`POST /v1/nfes/import-barueri`

Importa uma NFS-e de Barueri para o banco. Aceita um XML cru (retornado por `POST /nfes/fetch-barueri`) ou o `codigo_autenticidade` + `cnpj_tomador` para re-buscar antes de importar. Cria a `BillingAccount` do tomador quando ainda não existir. Exige o nível de acesso `import` — `write` não basta.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `codigo_autenticidade` | `string` | sim | Código de autenticidade impresso no DANFSE da NFS-e. |
| `cnpj_tomador` | `string` | sim | CPF ou CNPJ do tomador da NFS-e (somente dígitos ou com formatação). |
| `xml` | `string` | não | XML cru retornado por `POST /nfes/fetch-barueri`. Opcional — quando ausente, a NFS-e é re-buscada com `codigo_autenticidade` + `cnpj_tomador`. |
| `ambiente` | `string` | não | Ambiente municipal: `production` ou `homologation`. Padrão: `production`. Valores: `production`, `homologation`. |

Outros campos e objetos aninhados: `kobana schema billing.nfes.import-barueri`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da importação com flags `created`/`updated` e NFe persistida. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes sync

Sincronizar NFes da organização

```text
kobana billing nfes sync [OPÇÕES]
```

`POST /v1/nfes/sync`

Dispara um job em background para sincronizar todas as NFes da organização com o provedor (atualiza status, baixa PDFs/XMLs pendentes). Retorna o `job_id` para acompanhamento. Recusa se já existir uma sync ativa.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Sync enfileirada. |
| `404` | Recurso não encontrado na organização atual. |

## nfes get

Obter NFe

```text
kobana billing nfes get [ID] [OPÇÕES]
```

`GET /v1/nfes/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da NFe. |
| `404` | Recurso não encontrado na organização atual. |

## nfes delete

Excluir NFe

```text
kobana billing nfes delete [ID] [OPÇÕES]
```

`DELETE /v1/nfes/{id}`

Exclui uma NFe. Permitido apenas para os status `draft`, `pending`, `processing`, `error` ou `canceled`. NFes `issued` devem ser canceladas via `POST /nfes/{id}/cancel`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | NFe excluída. |
| `404` | Recurso não encontrado na organização atual. |

## nfes cancel

Cancelar NFe

```text
kobana billing nfes cancel [ID] [OPÇÕES]
```

`POST /v1/nfes/{id}/cancel`

Cancela uma NFe emitida. Enfileira o cancelamento de forma assíncrona junto ao provedor. O motivo (`reason`) deve ter no mínimo 15 caracteres e é registrado no audit log.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `reason` | `string` | sim | Motivo do cancelamento. **Mínimo 15 caracteres**. Registrado no audit log e enviado ao provedor. |

Outros campos e objetos aninhados: `kobana schema billing.nfes.cancel`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cancelamento enfileirado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes issue

Emitir NFe

```text
kobana billing nfes issue [ID] [OPÇÕES]
```

`POST /v1/nfes/{id}/issue`

Emite uma NFe em rascunho (`draft`) ou retenta uma NFe em `pending`/`error`. Envia a requisição para o provedor configurado na organização.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | NFe enfileirada/emitida. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes pdf

Baixar PDF da NFe

```text
kobana billing nfes pdf [ID] [OPÇÕES]
```

`GET /v1/nfes/{id}/pdf`

Baixa o PDF da NFe (DANFE) como anexo binário (`application/pdf`). Serve do banco quando disponível; caso contrário, busca no provedor.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | PDF da NFe. |
| `404` | Recurso não encontrado na organização atual. |

## nfes retry-cancel

Retentar cancelamento da NFe

```text
kobana billing nfes retry-cancel [ID] [OPÇÕES]
```

`POST /v1/nfes/{id}/retry-cancel`

Retenta o cancelamento de uma NFe travada em `pending_cancel`, `canceling` ou `error`. Reenfileira o job de cancelamento.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Retentativa de cancelamento enfileirada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## nfes validate-barueri

Validar NFS-e Barueri

```text
kobana billing nfes validate-barueri [ID] [OPÇÕES]
```

`POST /v1/nfes/{id}/validate-barueri`

Re-busca a NFS-e na prefeitura de Barueri e compara com o registro local, devolvendo a lista de diferenças e o XML atualizado para reaplicação via `POST /nfes/import-barueri`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da validação com diferenças e XML atual. |
| `404` | Recurso não encontrado na organização atual. |

## nfes xml

Baixar XML da NFe

```text
kobana billing nfes xml [ID] [OPÇÕES]
```

`GET /v1/nfes/{id}/xml`

Baixa o XML da NFe como anexo binário (`application/xml`). Serve do banco quando disponível; caso contrário, busca no provedor.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | XML da NFe. |
| `404` | Recurso não encontrado na organização atual. |
