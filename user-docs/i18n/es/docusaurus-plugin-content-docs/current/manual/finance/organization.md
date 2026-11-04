---
id: organization
title: "kobana finance organization"
sidebar_label: "organization"
description: "Obtener la organización del token"
---

# kobana finance organization

Obtener la organización del token

Comandos de `kobana finance organization`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Obtener la organización del token |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Actualizar el perfil de la organización |

## organization

Obtener la organización del token

```text
kobana finance organization [OPCIONES] [COMANDO]
```

`GET /v1/organization`

Returns the organization the Bearer token belongs to: its public id, registration (name, trade name, CNPJ/CPF), contacts, addresses and `custom_metadata`. Requires scope `finance/settings` with access level `read`. The organization comes only from the token — there is no `{id}` and no listing — and the route is organization-level, so no `X-Workspace-Id` header is needed. Commercial settings and secrets are never returned.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | The organization of the token. |

## organization update

Actualizar el perfil de la organización

```text
kobana finance organization update [OPCIONES]
```

`PATCH /v1/organization`

Partial update of the profile the customer owns: `trade_name`, `website`, `addresses` and `custom_metadata`. Requires scope `finance/settings` with access level `write`, the `settings.update` permission and, since the profile applies to every workspace, the organization owner or access to all of its workspaces. Identity (`id`, `public_id`, `external_id`), the fiscal document (`document_type`, `document_number`) and `name`, `email` and `phone` — mirrors of the Kobana Account, overwritten by the account sync — are not writable (422, like any unknown key); a commercial setting (`courtesy`, `entitlements`, `partner_slug`…) is a 403. `custom_metadata: null` keeps the stored value.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `trade_name` | `string` | no | Trade name. `null` clears it. |
| `website` | `string (uri)` | no | Website URL. `null` clears it. |
| `addresses` | `array<object>` | no | Replaces the whole address list. `[]` clears it. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. `null` mantém o valor atual. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | no | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema finance.organization.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | The updated organization. |
| `400` | Malformed JSON body. |
| `422` | Schema validation failure, including a read-only or unknown field. |
