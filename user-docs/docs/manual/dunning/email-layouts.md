---
id: email-layouts
title: "kobana dunning email-layouts"
sidebar_label: "email-layouts"
description: "Listar layouts de e-mail"
---

# kobana dunning email-layouts

Listar layouts de e-mail

Comandos de `kobana dunning email-layouts`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`email-layouts`](#email-layouts) | `GET /v1/email-layouts` | Listar layouts de e-mail |
| [`email-layouts create`](#email-layouts-create) | `POST /v1/email-layouts` | Criar layout de e-mail |

## email-layouts

Listar layouts de e-mail

```text
kobana dunning email-layouts [OPÇÕES] [COMANDO]
```

`GET /v1/email-layouts`

Lista os layouts de e-mail da organização (padrão primeiro) e o layout embutido de referência.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de layouts. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## email-layouts create

Criar layout de e-mail

```text
kobana dunning email-layouts create [OPÇÕES]
```

`POST /v1/email-layouts`

Cria um layout de e-mail. O `html_body` deve conter o placeholder `{{content}}` (senão, 400 `MISSING_CONTENT_PLACEHOLDER`). `isDefault: true` desmarca o padrão anterior.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do layout. |
| `html_body` | `string` | sim | HTML completo do layout; deve conter o placeholder `{{content}}`. |
| `is_default` | `boolean` | não | Layout padrão da organização (apenas um; definir um novo padrão desmarca o anterior). |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |

Outros campos e objetos aninhados: `kobana schema dunning.email-layouts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Layout criado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
