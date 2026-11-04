---
id: organization
title: "kobana billing organization"
sidebar_label: "organization"
description: "Obter organização atual"
---

# kobana billing organization

Obter organização atual

Comandos de `kobana billing organization`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Obter organização atual |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Atualizar organização atual |

## organization

Obter organização atual

```text
kobana billing organization [OPÇÕES] [COMANDO]
```

`GET /v1/organization`

Retorna os dados da organização à qual o token autenticado pertence. Não há parâmetro `:id` — a organização é sempre resolvida a partir da chave da API.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da organização atual. |
| `404` | Recurso não encontrado na organização atual. |

## organization update

Atualizar organização atual

```text
kobana billing organization update [OPÇÕES]
```

`PATCH /v1/organization`

Atualiza o perfil que a organização autenticada controla: `addresses` e `custom_metadata` (`null` mantém o valor atual). Identidade (`id`, `external_id`, `status`, documento) e os campos espelhados da Conta Kobana (`name`, `email`, `phone`) não são alteráveis por este endpoint: enviados, são ignorados — para corrigi-los, altere a Conta Kobana. Campos comerciais (como `courtesy` e `bypass_subscription`) são recusados com `403`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `addresses` | `array<object>` | não | Lista de endereços da organização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). `null` é ignorado e mantém o valor atual. |

Outros campos e objetos aninhados: `kobana schema billing.organization.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Organização atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
