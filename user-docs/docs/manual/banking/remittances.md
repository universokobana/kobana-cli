---
id: remittances
title: "kobana banking remittances"
sidebar_label: "remittances"
---

# kobana banking remittances


Comandos de `kobana banking remittances`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`remittances list`](#remittances-list) | `GET /v1/remittances` | Listar CNABs |
| [`remittances create`](#remittances-create) | `POST /v1/remittances` | Criar CNAB |
| [`remittances bulk`](#remittances-bulk) | `POST /v1/remittances/bulk` | Criar CNABs em Lote |
| [`remittances get`](#remittances-get) | `GET /v1/remittances/{id}` | Informações do CNAB |
| [`remittances delete`](#remittances-delete) | `DELETE /v1/remittances/{id}` | Apagar CNAB |
| [`remittances raw`](#remittances-raw) | `GET /v1/remittances/{id}/raw` | Raw(text/plain) do CNAB |

## remittances list

Listar CNABs

```text
kobana banking remittances list [OPÇÕES]
```

`GET /v1/remittances`

Lista CNABs.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `expire_from` | `string` | não | A partir da Data de vencimento do boleto. |
| `expire_to` | `string` | não | Até a Data de vencimento do boleto. |
| `created_from` | `string` | não | A partir da Data de criação da Remessa. |
| `created_to` | `string` | não | Até a Data de criação da Remessa. |
| `bank_billet_account_id` | `string` | não | Filtro por ID da Carteira. |
| `our_code` | `string` | não | Filtro por Nosso número. |
| `tags` | `string` | não | Filtro por tags (separadas por vírgula). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de CNABs |

## remittances create

Criar CNAB

```text
kobana banking remittances create [OPÇÕES]
```

`POST /v1/remittances`

Criar CNAB.

Será feita uma solicitação de Remessa. Após a criação da Remessa, ela será processada e ao término do processamento serão emitidas notificações através dos Webhooks.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `bank_billet_account_id` | `integer` | não | ID da Carteira de Cobrança. |
| `bank_billet_remittance_ids` | `array<integer>` | não | IDs de Registros de Remessa. |
| `external_id` | `string` | não | ID externo. |
| `custom_data` | `object` | não | Dados customizados. |
| `tags` | `array<string>` | não | Etiquetas. |

Outros campos e objetos aninhados: `kobana schema banking.remittances.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | CNAB criado com custom_data e tags |
| `404` | Remessa não encontrada |

## remittances bulk

Criar CNABs em Lote

```text
kobana banking remittances bulk [OPÇÕES]
```

`POST /v1/remittances/bulk`

Criar CNABs em Lote.

Serão feitas solicitações de Remessa de acordo com IDs das ocorrências. Será criada uma solicitação de Remessa para cada carteira associada às ocorrências. Após a criação das Remessas, elas serão processadas e ao término do processamento serão emitidas notificações através dos Webhooks.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `bank_billet_remittance_ids` | `array<integer>` | não | IDs de Registros de Remessa. |

Outros campos e objetos aninhados: `kobana schema banking.remittances.bulk`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | CNABs criadas |
| `404` | Ocorrência não encontrada |

## remittances get

Informações do CNAB

```text
kobana banking remittances get [ID] [OPÇÕES]
```

`GET /v1/remittances/{id}`

Consulta informações específicas de um CNAB.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | CNAB encontrado |
| `404` | CNAB não encontrado |

## remittances delete

Apagar CNAB

```text
kobana banking remittances delete [ID] [OPÇÕES]
```

`DELETE /v1/remittances/{id}`

Apagar CNAB.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | CNAB excluído |
| `404` | CNAB não encontrado |

## remittances raw

Raw(text/plain) do CNAB

```text
kobana banking remittances raw [ID] [OPÇÕES]
```

`GET /v1/remittances/{id}/raw`

Raw(text/plain) do CNAB.

Será disparado o evento remittance.downloaded.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Raw do CNAB |
| `404` | CNAB não encontrado |
| `422` | Remessa aguardando processamento |
