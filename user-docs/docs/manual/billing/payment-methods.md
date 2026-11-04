---
id: payment-methods
title: "kobana billing payment-methods"
sidebar_label: "payment-methods"
---

# kobana billing payment-methods


Comandos de `kobana billing payment-methods`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`payment-methods list`](#payment-methods-list) | `GET /v1/payment_methods` | Listar métodos de pagamento |
| [`payment-methods create`](#payment-methods-create) | `POST /v1/payment_methods` | Criar método de pagamento |
| [`payment-methods update`](#payment-methods-update) | `PUT /v1/payment_methods` | Criar ou atualizar forma de pagamento |
| [`payment-methods get`](#payment-methods-get) | `GET /v1/payment_methods/{id}` | Obter método de pagamento |
| [`payment-methods delete`](#payment-methods-delete) | `DELETE /v1/payment_methods/{id}` | Remover método de pagamento |
| [`payment-methods set-default`](#payment-methods-set-default) | `POST /v1/payment_methods/{id}/set-default` | Definir método como padrão |

## payment-methods list

Listar métodos de pagamento

```text
kobana billing payment-methods list [OPÇÕES]
```

`GET /v1/payment_methods`

Retorna uma lista paginada dos métodos de pagamento da organização atual. Filtros opcionais: `billing_account_id`, `type`, `status`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | não | Filtra por UUID da conta de cobrança. |
| `type` | `string` | não | Filtra por tipo: `card`, `bank_slip`, `pix` ou `bank_transfer`. Valores: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `status` | `string` | não | Filtra por status: `active`, `expired` ou `failed`. Valores: `active`, `expired`, `failed`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de métodos de pagamento. |
| `404` | Recurso não encontrado na organização atual. |

## payment-methods create

Criar método de pagamento

```text
kobana billing payment-methods create [OPÇÕES]
```

`POST /v1/payment_methods`

Cria um método de pagamento associado a uma conta de cobrança. Para `type=card`, envie o bloco `card` com o `card_token` gerado pela tokenização client-side. Para `type=bank_transfer`, envie o bloco `bank_account`. Para `bank_slip` e `pix`, basta o `type` — o gateway padrão é resolvido pela organização.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sim | UUID da conta de cobrança proprietária do método de pagamento. |
| `type` | `string` | sim | Tipo do método: `card`, `bank_slip`, `pix` ou `bank_transfer`. Valores: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `is_default` | `boolean` | não | Quando `true`, marca este método como padrão da conta (desmarca os anteriores). O primeiro método criado para uma conta também vira padrão automaticamente. |
| `card` | `object` | não |  |
| `bank_account` | `object` | não |  |
| `billing_details` | `object` | não |  |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados customizados em formato livre (`object`). |

Outros campos e objetos aninhados: `kobana schema billing.payment-methods.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Método de pagamento criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payment-methods update

Criar ou atualizar forma de pagamento

```text
kobana billing payment-methods update [OPÇÕES]
```

`PUT /v1/payment_methods`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança proprietária do método de pagamento. |
| `type` | `string` | não | Tipo do método: `card`, `bank_slip`, `pix` ou `bank_transfer`. Valores: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `is_default` | `boolean` | não | Quando `true`, marca este método como padrão da conta (desmarca os anteriores). O primeiro método criado para uma conta também vira padrão automaticamente. |
| `card` | `object` | não |  |
| `bank_account` | `object` | não |  |
| `billing_details` | `object` | não |  |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados customizados em formato livre (`object`). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.payment-methods.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payment-methods get

Obter método de pagamento

```text
kobana billing payment-methods get [ID] [OPÇÕES]
```

`GET /v1/payment_methods/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do método de pagamento. |
| `404` | Recurso não encontrado na organização atual. |

## payment-methods delete

Remover método de pagamento

```text
kobana billing payment-methods delete [ID] [OPÇÕES]
```

`DELETE /v1/payment_methods/{id}`

Exclui o método de pagamento (soft delete: `deleted_at`, e o status vira `removed` onde ele ainda aparece por relação — nos pagamentos que pagou). Exige o nível de acesso `delete`. Falha quando há pagamentos pendentes ou em processamento. Se era o padrão, outro método ativo é promovido automaticamente.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Método de pagamento removido. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payment-methods set-default

Definir método como padrão

```text
kobana billing payment-methods set-default [ID] [OPÇÕES]
```

`POST /v1/payment_methods/{id}/set-default`

Marca o método como padrão da conta de cobrança e desmarca os anteriores. Falha quando o método não está `active`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Método de pagamento definido como padrão. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
