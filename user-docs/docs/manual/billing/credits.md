---
id: credits
title: "kobana billing credits"
sidebar_label: "credits"
---

# kobana billing credits


Comandos de `kobana billing credits`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`credits list`](#credits-list) | `GET /v1/credits` | Listar créditos |
| [`credits create`](#credits-create) | `POST /v1/credits` | Adicionar crédito |
| [`credits update`](#credits-update) | `PUT /v1/credits` | Criar ou atualizar crédito |
| [`credits get`](#credits-get) | `GET /v1/credits/{id}` | Obter crédito |
| [`credits delete`](#credits-delete) | `DELETE /v1/credits/{id}` | Excluir crédito |

## credits list

Listar créditos

```text
kobana billing credits list [OPÇÕES]
```

`GET /v1/credits`

Retorna uma lista paginada dos créditos da organização atual. Por padrão, créditos expirados são omitidos — use `usage` ou `include_expired` para incluí-los.

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
| `billing_account_id` | `string` | não | Filtra créditos pela conta de cobrança. |
| `company_id` | `string` | não | Filtra créditos pela `company` vinculada. |
| `type` | `string` | não | Filtra pelo tipo do crédito: `promotional`, `adjustment`, `refund` ou `manual`. Valores: `promotional`, `adjustment`, `refund`, `manual`. |
| `usage` | `string` | não | Filtra pelo estado de uso — `available` (saldo remanescente e não expirado), `used` (consumido integralmente) ou `expired` (expirado). Valores: `available`, `used`, `expired`. |
| `include_expired` | `string` | não | Inclui créditos expirados quando `true`. Ignorado se `usage` for informado. Valores: `true`, `false`. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `amount_min` | `number,null` | não | Valor mínimo do crédito em BRL (inclusivo). |
| `amount_max` | `number,null` | não | Valor máximo do crédito em BRL (inclusivo). |
| `amount_remaining_min` | `number,null` | não | Saldo remanescente mínimo em BRL (inclusivo). |
| `amount_remaining_max` | `number,null` | não | Saldo remanescente máximo em BRL (inclusivo). |
| `expires_after` | `string` | não | Limite inferior de `expires_at` (YYYY-MM-DD, inclusivo). |
| `expires_before` | `string` | não | Limite superior de `expires_at` (YYYY-MM-DD, inclusivo). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de créditos. |
| `404` | Recurso não encontrado na organização atual. |

## credits create

Adicionar crédito

```text
kobana billing credits create [OPÇÕES]
```

`POST /v1/credits`

Adiciona um crédito a uma conta de cobrança. O valor é creditado no `balance` da `BillingAccount` e fica disponível para abater faturas futuras. Quando `company_id` é informado, o crédito só pode ser aplicado a faturas da mesma `company`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sim | UUID da conta de cobrança que receberá o crédito. |
| `type` | `string` | sim | Tipo do crédito: `promotional`, `adjustment`, `refund` ou `manual`. Valores: `promotional`, `adjustment`, `refund`, `manual`. |
| `amount` | `number` | sim | Valor do crédito em reais (float, até 4 casas decimais). Deve ser inteiro positivo. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não | UUID opcional da `company` à qual o crédito está vinculado. Quando informado, restringe a aplicação do crédito a faturas da mesma `company`. |
| `description` | `string` | não | Descrição livre (até 500 caracteres) — exibida no extrato e no audit log. |
| `expires_at` | `string (date-time)` | não | Data e hora de expiração (ISO8601). Após essa data o saldo remanescente é descartado. |
| `custom_metadata` | `object` | não | Metadados customizados validados contra o schema da organização. |

Outros campos e objetos aninhados: `kobana schema billing.credits.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Crédito criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## credits update

Criar ou atualizar crédito

```text
kobana billing credits update [OPÇÕES]
```

`PUT /v1/credits`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança que receberá o crédito. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não | UUID opcional da `company` à qual o crédito está vinculado. Quando informado, restringe a aplicação do crédito a faturas da mesma `company`. |
| `type` | `string` | não | Tipo do crédito: `promotional`, `adjustment`, `refund` ou `manual`. Valores: `promotional`, `adjustment`, `refund`, `manual`. |
| `amount` | `number` | não | Valor do crédito em reais (float, até 4 casas decimais). Deve ser inteiro positivo. |
| `description` | `string` | não | Descrição livre (até 500 caracteres) — exibida no extrato e no audit log. |
| `expires_at` | `string (date-time)` | não | Data e hora de expiração (ISO8601). Após essa data o saldo remanescente é descartado. |
| `custom_metadata` | `object` | não | Metadados customizados validados contra o schema da organização. |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.credits.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## credits get

Obter crédito

```text
kobana billing credits get [ID] [OPÇÕES]
```

`GET /v1/credits/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do crédito. |
| `404` | Recurso não encontrado na organização atual. |

## credits delete

Excluir crédito

```text
kobana billing credits delete [ID] [OPÇÕES]
```

`DELETE /v1/credits/{id}`

Remove o crédito e estorna o saldo correspondente da conta de cobrança. Só é permitido se nenhum centavo do crédito tiver sido consumido — caso contrário a API responde `400`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Crédito excluído. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
