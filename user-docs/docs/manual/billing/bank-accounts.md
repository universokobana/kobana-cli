---
id: bank-accounts
title: "kobana billing bank-accounts"
sidebar_label: "bank-accounts"
---

# kobana billing bank-accounts


Comandos de `kobana billing bank-accounts`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`bank-accounts list`](#bank-accounts-list) | `GET /v1/bank_accounts` | Listar contas bancárias |
| [`bank-accounts create`](#bank-accounts-create) | `POST /v1/bank_accounts` | Cadastrar conta bancária |
| [`bank-accounts get`](#bank-accounts-get) | `GET /v1/bank_accounts/{id}` | Consultar conta bancária |
| [`bank-accounts update`](#bank-accounts-update) | `PATCH /v1/bank_accounts/{id}` | Atualizar conta bancária |
| [`bank-accounts delete`](#bank-accounts-delete) | `DELETE /v1/bank_accounts/{id}` | Excluir conta bancária |

## bank-accounts list

Listar contas bancárias

```text
kobana billing bank-accounts list [OPÇÕES]
```

`GET /v1/bank_accounts`

Contas bancárias da organização para onde os clientes transferem numa cobrança `bank_transfer` (`POST /invoices/{id}/pay`).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `active` | `string` | não | Filtra por contas ativas (`true`) ou inativas (`false`). Valores: `true`, `false`. |
| `bank_id` | `string` | não | Filtra pelo banco (UUID de `/banks`). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de contas bancárias. |
| `404` | Recurso não encontrado na organização atual. |

## bank-accounts create

Cadastrar conta bancária

```text
kobana billing bank-accounts create [OPÇÕES]
```

`POST /v1/bank_accounts`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Apelido da conta, para o operador. |
| `bank_id` | `string (uuid)` | sim | UUID do banco (veja `/banks`). |
| `agency` | `string` | sim | Agência, só dígitos. |
| `account_number` | `string` | sim | Número da conta, só dígitos. |
| `type` | `string` | não | `checking` (corrente) ou `savings` (poupança). Valores: `checking`, `savings`. |
| `agency_digit` | `string` | não | Dígito da agência, quando houver. |
| `account_digit` | `string` | não | Dígito da conta, quando houver. |
| `holder_name` | `string` | não | Titular mostrado ao cliente. Vazio usa a razão social da empresa emissora da fatura (ou da organização). |
| `holder_document` | `string` | não | CPF/CNPJ do titular, só dígitos. Vazio usa o documento da empresa emissora (ou da organização). |
| `company_id` | `string (uuid)` | não | Empresa emissora a que pertence (opcional). |
| `active` | `boolean` | não | Só destinos ativos podem ser escolhidos numa nova cobrança. |

Outros campos e objetos aninhados: `kobana schema billing.bank-accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conta bancária criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## bank-accounts get

Consultar conta bancária

```text
kobana billing bank-accounts get [ID] [OPÇÕES]
```

`GET /v1/bank_accounts/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta bancária encontrada. |
| `404` | Recurso não encontrado na organização atual. |

## bank-accounts update

Atualizar conta bancária

```text
kobana billing bank-accounts update [ID] [OPÇÕES]
```

`PATCH /v1/bank_accounts/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Apelido da conta, para o operador. |
| `bank_id` | `string (uuid)` | não | UUID do banco (veja `/banks`). |
| `type` | `string` | não | `checking` (corrente) ou `savings` (poupança). Valores: `checking`, `savings`. |
| `agency` | `string` | não | Agência, só dígitos. |
| `agency_digit` | `string` | não | Dígito da agência, quando houver. |
| `account_number` | `string` | não | Número da conta, só dígitos. |
| `account_digit` | `string` | não | Dígito da conta, quando houver. |
| `holder_name` | `string` | não | Titular mostrado ao cliente. Vazio usa a razão social da empresa emissora da fatura (ou da organização). |
| `holder_document` | `string` | não | CPF/CNPJ do titular, só dígitos. Vazio usa o documento da empresa emissora (ou da organização). |
| `company_id` | `string (uuid)` | não | Empresa emissora a que pertence (opcional). |
| `active` | `boolean` | não | Só destinos ativos podem ser escolhidos numa nova cobrança. |

Outros campos e objetos aninhados: `kobana schema billing.bank-accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta bancária atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## bank-accounts delete

Excluir conta bancária

```text
kobana billing bank-accounts delete [ID] [OPÇÕES]
```

`DELETE /v1/bank_accounts/{id}`

Exclusão lógica. Cobranças já enviadas continuam mostrando os dados que o cliente recebeu.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta bancária excluída. |
| `404` | Recurso não encontrado na organização atual. |
