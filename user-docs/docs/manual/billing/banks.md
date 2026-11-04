---
id: banks
title: "kobana billing banks"
sidebar_label: "banks"
description: "Listar bancos"
---

# kobana billing banks

Listar bancos

Comandos de `kobana billing banks`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`banks`](#banks) | `GET /v1/banks` | Listar bancos |

## banks

Listar bancos

```text
kobana billing banks [OPÇÕES]
```

`GET /v1/banks`

Bancos brasileiros com código COMPE (BACEN), usados no cadastro de contas bancárias. A lista é a mesma para todas as organizações e é atualizada semanalmente a partir da BrasilAPI.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca por parte do nome ou do código. |
| `code` | `string` | não | Código COMPE exato (ex.: `341`). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de bancos. |
| `404` | Recurso não encontrado na organização atual. |
