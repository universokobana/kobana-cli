---
id: edi
title: "kobana banking edi"
sidebar_label: "edi"
description: "EDI (caixas EDI)"
---

# kobana banking edi

EDI (caixas EDI)

Comandos de `kobana banking edi`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`edi boxes list`](#edi-boxes-list) | `GET /v2/edi/edi_boxes` | Listar Caixas Postais |
| [`edi boxes create`](#edi-boxes-create) | `POST /v2/edi/edi_boxes` | Criar Caixa Postal |
| [`edi boxes get`](#edi-boxes-get) | `GET /v2/edi/edi_boxes/{uid}` | Visualizar Caixa Postal |
| [`edi boxes update`](#edi-boxes-update) | `PUT /v2/edi/edi_boxes/{uid}` | Atualizar Caixa Postal |

## edi boxes list

Listar Caixas Postais

```text
kobana banking edi boxes list [OPÇÕES]
```

`GET /v2/edi/edi_boxes`

Lista Caixas Postais.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `resource_type` | `string` | não | Filtro por Tipo de Recurso. Carteira de Cobrança (`charge.bank_billet_account`) ou Conta Financeira (`financial.account`). |
| `per_page` | `integer` | não | Quantidade de registros por página. Máximo de 50. |
| `page` | `integer` | não | Número da página. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Caixas Postais encontradas. |

## edi boxes create

Criar Caixa Postal

```text
kobana banking edi boxes create [OPÇÕES]
```

`POST /v2/edi/edi_boxes`

Cria uma nova Caixa Postal.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.edi.boxes.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Caixa Postal criada |
| `422` | Falha na validação |

## edi boxes get

Visualizar Caixa Postal

```text
kobana banking edi boxes get [UID] [OPÇÕES]
```

`GET /v2/edi/edi_boxes/{uid}`

Consulta informações de uma Caixa Postal específica.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Caixa Postal encontrada |
| `404` | Caixa Postal não encontrada |

## edi boxes update

Atualizar Caixa Postal

```text
kobana banking edi boxes update [UID] [OPÇÕES]
```

`PUT /v2/edi/edi_boxes/{uid}`

Atualiza uma Caixa Postal específica.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.edi.boxes.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Caixa Postal atualizada |
| `404` | Caixa Postal não encontrada |
| `422` | operation/resource não podem ser alterados sem editable: true |
