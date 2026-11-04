---
id: portal-users
title: "kobana billing portal-users"
sidebar_label: "portal-users"
description: "Lista usuários do portal"
---

# kobana billing portal-users

Lista usuários do portal

Comandos de `kobana billing portal-users`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`portal-users`](#portal-users) | `GET /v1/portal_users` | Lista usuários do portal |

## portal-users

Lista usuários do portal

```text
kobana billing portal-users [OPÇÕES]
```

`GET /v1/portal_users`

Retorna os usuários do portal do cliente na organização autenticada, com paginação. Campos sensíveis (senha, tokens) nunca são expostos.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | não | Restringe a uma conta de cobrança específica. |
| `customer_id` | `string` | não | Restringe a todas as contas de cobrança de um cliente. Aceita o `id` do cliente ou o `external_id`. |
| `status` | `string` | não | Filtra por status: `active`, `pending_invitation`, `pending_verification`, `suspended` ou `locked`. Valores: `active`, `pending_invitation`, `pending_verification`, `suspended`, `locked`. |
| `email` | `string` | não | Busca por trecho do e-mail. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de usuários do portal. |
| `404` | Recurso não encontrado na organização atual. |
