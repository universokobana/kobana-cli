---
id: organization
title: "kobana dunning organization"
sidebar_label: "organization"
description: "Consultar la organización"
---

# kobana dunning organization

Consultar la organización

Comandos de `kobana dunning organization`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Consultar la organización |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Actualizar la organización |

## organization

Consultar la organización

```text
kobana dunning organization [OPCIONES] [COMANDO]
```

`GET /v1/organization`

Devuelve la organización del token. La organización viene solo de la credencial: no hay lista ni `{id}`, y ningún header, query o cuerpo elige otra. Es de nivel de organización: no exige el header `X-Workspace-Id`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La organización del token. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## organization update

Actualizar la organización

```text
kobana dunning organization update [OPCIONES]
```

`PATCH /v1/organization`

Actualiza el perfil de la organización del token: nombre comercial, sitio web, zona horaria y `custom_metadata`. Nombre, correo, teléfono, documento e ids reflejan la cuenta Kobana y no cambian por aquí (el campo enviado se ignora); los campos comerciales (`bypass_subscription`, `courtesy`…) responden `403`. El perfil vale para todos los workspaces, así que exige ser dueño de la organización o tener acceso a todos los workspaces. No exige el header `X-Workspace-Id`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `trade_name` | `string` | no | Nombre comercial; `null` lo limpia. |
| `website` | `string (uri)` | no | Sitio web (URL); `null` lo limpia. |
| `timezone` | `string` | no | Nombre de zona IANA (ej.: `America/Sao_Paulo`). Offsets y abreviaturas se rechazan. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre). `null` mantiene el valor guardado. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.organization.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La organización actualizada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
