---
id: billing-accounts
title: "kobana billing billing-accounts"
sidebar_label: "billing-accounts"
---

# kobana billing billing-accounts


Comandos de `kobana billing billing-accounts`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`billing-accounts list`](#billing-accounts-list) | `GET /v1/billing_accounts` | Listar contas de cobrança |
| [`billing-accounts create`](#billing-accounts-create) | `POST /v1/billing_accounts` | Criar conta de cobrança |
| [`billing-accounts update`](#billing-accounts-update) | `PUT /v1/billing_accounts` | Criar ou atualizar conta de cobrança |
| [`billing-accounts get`](#billing-accounts-get) | `GET /v1/billing_accounts/{id}` | Obter conta de cobrança |
| [`billing-accounts delete`](#billing-accounts-delete) | `DELETE /v1/billing_accounts/{id}` | Excluir conta de cobrança |
| [`billing-accounts close`](#billing-accounts-close) | `POST /v1/billing_accounts/{id}/close` | Fechar conta de cobrança |
| [`billing-accounts credits`](#billing-accounts-credits) | `GET /v1/billing_accounts/{id}/credits` | Listar créditos da conta |
| [`billing-accounts entitlements`](#billing-accounts-entitlements) | `GET /v1/billing_accounts/{id}/entitlements` | Listar produtos contratados da conta |
| [`billing-accounts invoices`](#billing-accounts-invoices) | `GET /v1/billing_accounts/{id}/invoices` | Listar faturas da conta |
| [`billing-accounts reactivate`](#billing-accounts-reactivate) | `POST /v1/billing_accounts/{id}/reactivate` | Reativar conta de cobrança |
| [`billing-accounts subscriptions`](#billing-accounts-subscriptions) | `GET /v1/billing_accounts/{id}/subscriptions` | Listar assinaturas da conta |
| [`billing-accounts suspend`](#billing-accounts-suspend) | `POST /v1/billing_accounts/{id}/suspend` | Suspender conta de cobrança |

## billing-accounts list

Listar contas de cobrança

```text
kobana billing billing-accounts list [OPÇÕES]
```

`GET /v1/billing_accounts`

Retorna uma lista paginada das contas de cobrança da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra por status da conta de cobrança. Valores: `active`, `suspended`, `closed`. |
| `search` | `string` | não | Busca textual livre. |
| `external_id` | `string` | não | Filtra pelo identificador externo da conta. |
| `currency` | `string` | não | Filtra por moeda ISO 4217 (ex.: `BRL`). |
| `auto_collection` | `string` | não | Filtra por cobrança automática habilitada (`true`) ou desabilitada (`false`). Valores: `true`, `false`. |
| `tax_exempt` | `string` | não | Filtra contas isentas de impostos (`true`) ou não (`false`). Valores: `true`, `false`. |
| `customer_tax_type` | `string` | não | Filtra por tipo tributário do cliente. |
| `balance_min` | `number,null` | não | Saldo mínimo da conta em reais (float, até 4 casas decimais). |
| `balance_max` | `number,null` | não | Saldo máximo da conta em reais (float, até 4 casas decimais). |
| `mrr_min` | `number,null` | não | MRR mínimo em reais (float, até 4 casas decimais). |
| `mrr_max` | `number,null` | não | MRR máximo em reais (float, até 4 casas decimais). |
| `allowed_negative_balance_min` | `number,null` | não | Saldo negativo permitido mínimo em reais (float, até 4 casas decimais). |
| `allowed_negative_balance_max` | `number,null` | não | Saldo negativo permitido máximo em reais (float, até 4 casas decimais). |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `updated_after` | `string` | não | Data de atualização inicial do intervalo (YYYY-MM-DD). |
| `updated_before` | `string` | não | Data de atualização final do intervalo (YYYY-MM-DD). |
| `suspended_after` | `string` | não | Filtra contas suspensas a partir desta data (formato `YYYY-MM-DD`). |
| `suspended_before` | `string` | não | Filtra contas suspensas até esta data (formato `YYYY-MM-DD`). |
| `person_id` | `string` | não | Filtra por UUID da pessoa associada. |
| `person_document_number` | `string` | não | Filtra por CPF/CNPJ da pessoa (exato). |
| `person_name_contains` | `string` | não | Filtra por nome da pessoa (busca parcial, sem distinção de maiúsculas). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de contas de cobrança. |
| `404` | Recurso não encontrado na organização atual. |

## billing-accounts create

Criar conta de cobrança

```text
kobana billing billing-accounts create [OPÇÕES]
```

`POST /v1/billing_accounts`

Cria uma conta de cobrança associada a um customer. O bloco `customer` localiza um customer existente por `id`, `external_id` ou `document_number`; quando não encontrado, cria um novo (exige `name` e `document_number`).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `customer` | `object` | sim |  |
| `external_id` | `string` | não | Identificador externo da conta no sistema do cliente. |
| `billing_email` | `string (email)` | não | E-mail de cobrança. Padrão: `customer.email`. |
| `payment_terms_days` | `integer` | não | Prazo para vencimento de fatura em dias (1–90). Padrão: prazo padrão configurado na organização (10 quando não configurado). |
| `auto_collection` | `boolean` | não | Habilita cobrança automática. Padrão: `true`. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.billing-accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conta de cobrança criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## billing-accounts update

Criar ou atualizar conta de cobrança

```text
kobana billing billing-accounts update [OPÇÕES]
```

`PUT /v1/billing_accounts`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `customer` | `object` | não |  |
| `external_id` | `string` | não | Identificador externo da conta no sistema do cliente. |
| `billing_email` | `string (email)` | não | E-mail de cobrança. Padrão: `customer.email`. |
| `payment_terms_days` | `integer` | não | Prazo para vencimento de fatura em dias (1–90). Padrão: prazo padrão configurado na organização (10 quando não configurado). |
| `auto_collection` | `boolean` | não | Habilita cobrança automática. Padrão: `true`. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.billing-accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## billing-accounts get

Obter conta de cobrança

```text
kobana billing billing-accounts get [ID] [OPÇÕES]
```

`GET /v1/billing_accounts/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da conta de cobrança. |
| `404` | Recurso não encontrado na organização atual. |

## billing-accounts delete

Excluir conta de cobrança

```text
kobana billing billing-accounts delete [ID] [OPÇÕES]
```

`DELETE /v1/billing_accounts/{id}`

Exclui a conta de cobrança (soft delete: `deleted_at`), junto com seus métodos de pagamento, cartões, usuários do portal e arquivos — tudo some de toda leitura; faturas, pagamentos e NFS-e já emitidos ficam no histórico. Exige o nível de acesso `delete`. Retorna 422 enquanto houver assinatura que não esteja cancelada, fatura em rascunho ou aberta, ou pagamento pendente. Até outubro de 2026 esta rota fechava a conta (e aceitava `?reason=`) — fechar é `POST /v1/billing_accounts/{id}/close`, reversível.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de cobrança excluída. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## billing-accounts close

Fechar conta de cobrança

```text
kobana billing billing-accounts close [ID] [OPÇÕES]
```

`POST /v1/billing_accounts/{id}/close`

Encerra a conta de cobrança. O motivo (`reason`) é registrado no audit log.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `reason` | `string` | não | Motivo da operação (registrado no audit log). |

Outros campos e objetos aninhados: `kobana schema billing.billing-accounts.close`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de cobrança fechada. |
| `404` | Recurso não encontrado na organização atual. |

## billing-accounts credits

Listar créditos da conta

```text
kobana billing billing-accounts credits [ID] [OPÇÕES]
```

`GET /v1/billing_accounts/{id}/credits`

Retorna uma lista paginada dos créditos da conta de cobrança.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `include_expired` | `string` | não | Inclui créditos expirados quando `true`. Valores: `true`, `false`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de créditos. |
| `404` | Recurso não encontrado na organização atual. |

## billing-accounts entitlements

Listar produtos contratados da conta

```text
kobana billing billing-accounts entitlements [ID] [OPÇÕES]
```

`GET /v1/billing_accounts/{id}/entitlements`

Retorna a lista enxuta de produtos que a conta de cobrança tem direito de acesso, com base nas assinaturas em período de teste, ativas ou em atraso (carência de cobrança). Apenas produtos pais são retornados; produtos filhos (adicionais) são agrupados sob o pai e omitidos. Ideal para liberar ou bloquear o uso de um produto em um app externo.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `product_group_id` | `string` | não | Filtra os produtos contratados por família (grupo de produtos), pelo ID. |
| `product_group_slug` | `string` | não | Filtra os produtos contratados por família (grupo de produtos), pelo slug. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de produtos contratados. |
| `404` | Recurso não encontrado na organização atual. |

## billing-accounts invoices

Listar faturas da conta

```text
kobana billing billing-accounts invoices [ID] [OPÇÕES]
```

`GET /v1/billing_accounts/{id}/invoices`

Retorna uma lista paginada das faturas da conta de cobrança.

**Argumentos do caminho:** `id`

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
| `200` | Lista paginada de faturas. |
| `404` | Recurso não encontrado na organização atual. |

## billing-accounts reactivate

Reativar conta de cobrança

```text
kobana billing billing-accounts reactivate [ID] [OPÇÕES]
```

`POST /v1/billing_accounts/{id}/reactivate`

Reativa uma conta de cobrança suspensa.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de cobrança reativada. |
| `404` | Recurso não encontrado na organização atual. |

## billing-accounts subscriptions

Listar assinaturas da conta

```text
kobana billing billing-accounts subscriptions [ID] [OPÇÕES]
```

`GET /v1/billing_accounts/{id}/subscriptions`

Retorna uma lista paginada das assinaturas da conta de cobrança.

**Argumentos do caminho:** `id`

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
| `200` | Lista paginada de assinaturas. |
| `404` | Recurso não encontrado na organização atual. |

## billing-accounts suspend

Suspender conta de cobrança

```text
kobana billing billing-accounts suspend [ID] [OPÇÕES]
```

`POST /v1/billing_accounts/{id}/suspend`

Suspende a conta. O motivo (`reason`) é registrado no audit log.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `reason` | `string` | não | Motivo da operação (registrado no audit log). |

Outros campos e objetos aninhados: `kobana schema billing.billing-accounts.suspend`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de cobrança suspensa. |
| `404` | Recurso não encontrado na organização atual. |
