---
id: discharges
title: "kobana banking discharges"
sidebar_label: "discharges"
---

# kobana banking discharges


Comandos de `kobana banking discharges`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`discharges list`](#discharges-list) | `GET /v1/discharges` | Listar CNABs |
| [`discharges create`](#discharges-create) | `POST /v1/discharges` | Enviar CNAB |
| [`discharges get`](#discharges-get) | `GET /v1/discharges/{id}` | Informações do CNAB |
| [`discharges download`](#discharges-download) | `GET /v1/discharges/{id}/download` | Download do CNAB |
| [`discharges pay-off`](#discharges-pay-off) | `PUT /v1/discharges/{id}/pay_off` | Quitar Boletos |
| [`discharges reprocess`](#discharges-reprocess) | `PUT /v1/discharges/{id}/reprocess` | Reprocessar CNAB |

## discharges list

Listar CNABs

```text
kobana banking discharges list [OPÇÕES]
```

`GET /v1/discharges`

Lista CNABs.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `created_from` | `string` | não | A partir da Data de criação do Retorno. |
| `created_to` | `string` | não | Até a Data de criação do Retorno. |
| `bank_billet_account_id` | `string` | não | Filtro por ID da Carteira. |
| `tags` | `string` | não | Filtro por tags (separadas por vírgula). |
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de CNABs |

## discharges create

Enviar CNAB

```text
kobana banking discharges create [OPÇÕES]
```

`POST /v1/discharges`

Envia um arquivo CNAB de retorno bancário para processamento de pagamentos e baixas de boletos.

**Corpo da requisição**

Enviado como formulário `multipart/form-data`: os arquivos vão em `--file CAMPO=CAMINHO`.

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `file` | `string (binary)` | não | Arquivo |
| `filename` | `string` | não | Nome do arquivo |
| `bank_billet_account_id` | `integer` | não | ID da Carteira de Cobrança. |
| `external_id` | `string` | não | ID externo. |
| `custom_data` | `object` | não | Dados customizados. |
| `tags` | `array<string>` | não | Etiquetas. |

Outros campos e objetos aninhados: `kobana schema banking.discharges.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | CNAB enviado com custom_data e tags |
| `422` | Dados inválidos |

## discharges get

Informações do CNAB

```text
kobana banking discharges get [ID] [OPÇÕES]
```

`GET /v1/discharges/{id}`

Consulta informações específicas de um CNAB.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | CNAB encontrado |
| `404` | CNAB não encontrado |

## discharges download

Download do CNAB

```text
kobana banking discharges download [ID] [OPÇÕES]
```

`GET /v1/discharges/{id}/download`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Download com sucesso |
| `404` | CNAB não encontrado |

## discharges pay-off

Quitar Boletos

```text
kobana banking discharges pay-off [ID] [OPÇÕES]
```

`PUT /v1/discharges/{id}/pay_off`

Quitar boletos.

É necessário já ter enviado o CNAB. Todos os boletos que forem identificados dentro do CNAB serão marcados como PAGO e os Webhooks serão disparados.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Boletos quitados |
| `404` | CNAB não encontrado |

## discharges reprocess

Reprocessar CNAB

```text
kobana banking discharges reprocess [ID] [OPÇÕES]
```

`PUT /v1/discharges/{id}/reprocess`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | CNAB reprocessada |
| `404` | CNAB não encontrado |
