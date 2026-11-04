---
id: bank-billet-registrations
title: "kobana banking bank-billet-registrations"
sidebar_label: "bank-billet-registrations"
---

# kobana banking bank-billet-registrations


Comandos de `kobana banking bank-billet-registrations`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`bank-billet-registrations list`](#bank-billet-registrations-list) | `GET /v1/bank_billet_registrations` | Listar Registros de Boleto. |
| [`bank-billet-registrations get`](#bank-billet-registrations-get) | `GET /v1/bank_billet_registrations/{id}` | Informações do Registro de Boleto |

## bank-billet-registrations list

Listar Registros de Boleto.

```text
kobana banking bank-billet-registrations list [OPÇÕES]
```

`GET /v1/bank_billet_registrations`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `registered_from` | `string` | não | A partir da Data de registro. |
| `registered_to` | `string` | não | Até a Data de registro. |
| `bank_billet_id` | `string` | não | ID do Boleto. |
| `bank_billet_account_id` | `string` | não | ID da Carteira. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de registros de boleto |

## bank-billet-registrations get

Informações do Registro de Boleto

```text
kobana banking bank-billet-registrations get [ID] [OPÇÕES]
```

`GET /v1/bank_billet_registrations/{id}`

Consulta informações específicas de um Registro de Boleto.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro de Boleto encontrado |
| `404` | Registro de retorno não encontrado |
