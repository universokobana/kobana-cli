---
id: withholdings
title: "kobana billing withholdings"
sidebar_label: "withholdings"
---

# kobana billing withholdings


Comandos de `kobana billing withholdings`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`withholdings list`](#withholdings-list) | `GET /v1/withholdings` | Listar retenções |
| [`withholdings create`](#withholdings-create) | `POST /v1/withholdings` | Cria uma retenção |
| [`withholdings update`](#withholdings-update) | `PUT /v1/withholdings` | Cria ou atualiza uma retenção |
| [`withholdings convert-to-credits`](#withholdings-convert-to-credits) | `POST /v1/withholdings/convert-to-credits` | Converter retenções pendentes em créditos (em lote) |
| [`withholdings get`](#withholdings-get) | `GET /v1/withholdings/{id}` | Obter retenção |
| [`withholdings delete`](#withholdings-delete) | `DELETE /v1/withholdings/{id}` | Remove uma retenção |
| [`withholdings convert-to-credit`](#withholdings-convert-to-credit) | `POST /v1/withholdings/{id}/convert-to-credit` | Converter retenção em crédito |

## withholdings list

Listar retenções

```text
kobana billing withholdings list [OPÇÕES]
```

`GET /v1/withholdings`

Retorna uma lista paginada das retenções tributárias da organização atual. Suporta filtros por tipo de tributo, status, retentor, fatura, NF-e e competência.

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
| `tax_type` | `string` | não | Filtra por tipo de tributo: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs` ou `ibs`. Valores: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `status` | `string` | não | Filtra por status: `retained`, `compensated` ou `partially_compensated`. Valores: `retained`, `compensated`, `partially_compensated`. |
| `withholder_id` | `string` | não | Filtra pelas retenções de um retentor (conta de cobrança) específico. |
| `invoice_id` | `string` | não | Filtra pelas retenções associadas a uma fatura. |
| `nfe_id` | `string` | não | Filtra pelas retenções associadas a uma NF-e. |
| `competence_year` | `integer` | não | Filtra pelo ano de competência (2000–3000). |
| `competence_month` | `integer` | não | Filtra pelo mês de competência (1–12). |
| `amount_min` | `number,null` | não | Valor mínimo retido em reais (ex.: `10.5` para R$ 10,50). |
| `amount_max` | `number,null` | não | Valor máximo retido em reais (ex.: `500.0` para R$ 500,00). |
| `base_min` | `number,null` | não | Base de cálculo mínima em reais. |
| `base_max` | `number,null` | não | Base de cálculo máxima em reais. |
| `compensated_min` | `number,null` | não | Valor mínimo já compensado em reais. |
| `compensated_max` | `number,null` | não | Valor máximo já compensado em reais. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de retenções. |
| `404` | Recurso não encontrado na organização atual. |

## withholdings create

Cria uma retenção

```text
kobana billing withholdings create [OPÇÕES]
```

`POST /v1/withholdings`

Registra uma retenção na fonte para o retentor informado. `base` e `amount` são valores em reais (float, até 4 casas decimais).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `tax_type` | `string` | sim | Tributo retido. Valores: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `base` | `number` | sim | Base de cálculo, em reais (float, até 4 casas decimais). |
| `rate` | `number` | sim | Alíquota aplicada, em percentual. |
| `amount` | `number` | sim | Montante retido, em reais (float, até 4 casas decimais). |
| `withholder_id` | `string (uuid)` | sim | Conta de cobrança que efetuou a retenção. |
| `competence_month` | `integer` | sim | Mês de competência (1–12). |
| `competence_year` | `integer` | sim | Ano de competência. |
| `invoice_id` | `string (uuid)` | não | Fatura de origem, quando houver. |
| `nfe_id` | `string (uuid)` | não | Nota fiscal relacionada, quando houver. |
| `metadata` | `object` | não |  |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema billing.withholdings.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Retenção criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## withholdings update

Cria ou atualiza uma retenção

```text
kobana billing withholdings update [OPÇÕES]
```

`PUT /v1/withholdings`

Localiza a retenção por `id`, `public_id` ou `external_id`. Não encontrando, cria com o corpo completo e devolve `201`. **Encontrando, aplica apenas `competence_month`, `competence_year`, `metadata`, `external_id` e `custom_metadata`**: `base`, `rate`, `amount` e `withholder_id` são imutáveis após o registro e vêm ignorados no corpo de atualização.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `tax_type` | `string` | não | Tributo retido. Valores: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `base` | `number` | não | Base de cálculo, em reais (float, até 4 casas decimais). |
| `rate` | `number` | não | Alíquota aplicada, em percentual. |
| `amount` | `number` | não | Montante retido, em reais (float, até 4 casas decimais). |
| `withholder_id` | `string (uuid)` | não | Conta de cobrança que efetuou a retenção. |
| `competence_month` | `integer` | não | Mês de competência (1–12). |
| `competence_year` | `integer` | não | Ano de competência. |
| `invoice_id` | `string (uuid)` | não | Fatura de origem, quando houver. |
| `nfe_id` | `string (uuid)` | não | Nota fiscal relacionada, quando houver. |
| `metadata` | `object` | não |  |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não |  |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.withholdings.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## withholdings convert-to-credits

Converter retenções pendentes em créditos (em lote)

```text
kobana billing withholdings convert-to-credits [OPÇÕES]
```

`POST /v1/withholdings/convert-to-credits`

Converte todas as retenções pendentes da organização — opcionalmente restritas por `tax_type` e/ou competência — em créditos. Retorna a lista de créditos gerados, a quantidade total convertida e o valor total em reais.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `tax_type` | `string` | não | Restringe a conversão a um tipo de tributo (ex.: `irrf`, `pis`, `cofins`). Valores: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `competence_year` | `integer` | não | Restringe a conversão ao ano de competência informado. |
| `competence_month` | `integer` | não | Restringe a conversão ao mês de competência informado (1–12). |

Outros campos e objetos aninhados: `kobana schema billing.withholdings.convert-to-credits`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lote de retenções convertido em créditos. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## withholdings get

Obter retenção

```text
kobana billing withholdings get [ID] [OPÇÕES]
```

`GET /v1/withholdings/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da retenção. |
| `404` | Recurso não encontrado na organização atual. |

## withholdings delete

Remove uma retenção

```text
kobana billing withholdings delete [ID] [OPÇÕES]
```

`DELETE /v1/withholdings/{id}`

Recusa a remoção quando a retenção já foi compensada, total ou parcialmente.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Retenção removida. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## withholdings convert-to-credit

Converter retenção em crédito

```text
kobana billing withholdings convert-to-credit [ID] [OPÇÕES]
```

`POST /v1/withholdings/{id}/convert-to-credit`

Converte uma retenção (total ou parcialmente) em um crédito na conta de cobrança derivada da fatura associada — ou, na ausência dela, do próprio retentor. Quando `amount` é omitido, converte o saldo pendente da retenção.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | não | Valor a converter em reais (float, até 4 casas decimais). Padrão: saldo pendente da retenção. |

Outros campos e objetos aninhados: `kobana schema billing.withholdings.convert-to-credit`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Crédito gerado a partir da retenção. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
