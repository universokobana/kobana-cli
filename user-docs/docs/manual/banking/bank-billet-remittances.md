---
id: bank-billet-remittances
title: "kobana banking bank-billet-remittances"
sidebar_label: "bank-billet-remittances"
---

# kobana banking bank-billet-remittances


Comandos de `kobana banking bank-billet-remittances`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`bank-billet-remittances occurrence create`](#bank-billet-remittances-occurrence-create) | `POST /v1/bank_billet_remittances/{id}/occurrence/{our_code}` | Criar Pendências |
| [`bank-billet-remittances pending`](#bank-billet-remittances-pending) | `GET /v1/bank_billet_remittances/pending` | Listar Registros de Remessa Pendentes |
| [`bank-billet-remittances list`](#bank-billet-remittances-list) | `GET /v1/bank_billet_remittances` | Listar Registros de Remessa |
| [`bank-billet-remittances get`](#bank-billet-remittances-get) | `GET /v1/bank_billet_remittances/{id}` | Informações do Registro de Remessa |
| [`bank-billet-remittances delete`](#bank-billet-remittances-delete) | `DELETE /v1/bank_billet_remittances/{id}` | Excluir Pendências |

## bank-billet-remittances occurrence create

Criar Pendências

```text
kobana banking bank-billet-remittances occurrence create [ID] [OUR_CODE] [OPÇÕES]
```

`POST /v1/bank_billet_remittances/{id}/occurrence/{our_code}`

Criar pendências.

**Argumentos do caminho:** `id`, `our_code`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pendências criada |
| `422` | Não foi possível processar |

## bank-billet-remittances pending

Listar Registros de Remessa Pendentes

```text
kobana banking bank-billet-remittances pending [OPÇÕES]
```

`GET /v1/bank_billet_remittances/pending`

Listar Registros de Remessa pendentes

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `our_code` | `string` | não | Código de operação de registro. Consulte os possíveis valores para cada banco. |
| `expire_from` | `string` | não | A partir de Data de vencimento. |
| `expire_to` | `string` | não | Até Data de vencimento. |
| `bank_billet_id` | `string` | não | ID do Boleto. |
| `bank_billet_account_id` | `string` | não | ID da Carteira. |
| `remittance_id` | `string` | não | ID da Remessa. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de registros de remessa |

## bank-billet-remittances list

Listar Registros de Remessa

```text
kobana banking bank-billet-remittances list [OPÇÕES]
```

`GET /v1/bank_billet_remittances`

Listar Registros de Remessa.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `our_code` | `string` | não | Código de operação de registro. Consulte os possíveis valores para cada banco. |
| `expire_from` | `string` | não | A partir de Data de vencimento. |
| `expire_to` | `string` | não | Até Data de vencimento. |
| `bank_billet_id` | `string` | não | ID do Boleto. |
| `bank_billet_account_id` | `string` | não | ID da Carteira. |
| `remittance_id` | `string` | não | ID da Remessa. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de registros de remessa |

## bank-billet-remittances get

Informações do Registro de Remessa

```text
kobana banking bank-billet-remittances get [ID] [OPÇÕES]
```

`GET /v1/bank_billet_remittances/{id}`

Consulta informações específicas de um Registro de Remessa.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro de Remessa encontrado |
| `404` | Registro de Remessa não encontrado |

## bank-billet-remittances delete

Excluir Pendências

```text
kobana banking bank-billet-remittances delete [ID] [OPÇÕES]
```

`DELETE /v1/bank_billet_remittances/{id}`

Excluir pendências.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Pendências excluida |
| `404` | Retorno de remessa não encontrado |
