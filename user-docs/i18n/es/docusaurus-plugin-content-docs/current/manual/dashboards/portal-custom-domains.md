---
id: portal-custom-domains
title: "kobana dashboards portal-custom-domains"
sidebar_label: "portal-custom-domains"
---

# kobana dashboards portal-custom-domains


Comandos de `kobana dashboards portal-custom-domains`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`portal-custom-domains list`](#portal-custom-domains-list) | `GET /v1/portal_custom_domains` | Listar dominios propios |
| [`portal-custom-domains create`](#portal-custom-domains-create) | `POST /v1/portal_custom_domains` | Agregar dominio propio |
| [`portal-custom-domains delete`](#portal-custom-domains-delete) | `DELETE /v1/portal_custom_domains/{id}` | Eliminar dominio propio |
| [`portal-custom-domains dns-status`](#portal-custom-domains-dns-status) | `GET /v1/portal_custom_domains/{id}/dns_status` | Consultar DNS del dominio propio |
| [`portal-custom-domains verify`](#portal-custom-domains-verify) | `POST /v1/portal_custom_domains/{id}/verify` | Verificar y aprovisionar dominio propio |

## portal-custom-domains list

Listar dominios propios

```text
kobana dashboards portal-custom-domains list [OPCIONES]
```

`GET /v1/portal_custom_domains`

Lista los dominios propios del portal del workspace (conjunto completo, sin paginación). Vacía donde los dominios propios no son compatibles.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Dominios propios del portal. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-custom-domains create

Agregar dominio propio

```text
kobana dashboards portal-custom-domains create [OPCIONES]
```

`POST /v1/portal_custom_domains`

Registra un dominio propio como `pending_verification`, sin tocar AWS, y devuelve los registros DNS a publicar. Luego llame a `POST /portal_custom_domains/{id}/verify`. El registro es solo un reclamo: el dominio queda con el primer portal que publique su TXT de verificación (los demás reclamos no comprobados se liberan), y un reclamo no comprobado vence a los 7 días. Cada portal tiene como máximo 10 dominios. Los dominios de la plataforma (`kobana.com.br`, `kobana.tech` y los dominios compartidos de portal) no se aceptan.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `hostname` | `string` | sí | Nombre completo del dominio, ej.: `portal.ejemplo.com`. |

Otros campos y objetos anidados: `kobana schema dashboards.portal-custom-domains.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | El dominio propio. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Este dominio ya está registrado (comprobado por otro portal, o ya reclamado por este). |
| `422` | Dominio reservado de la plataforma (`hostname_reserved`) o el portal alcanzó su límite de dominios (`domain_limit_reached`). |

## portal-custom-domains delete

Eliminar dominio propio

```text
kobana dashboards portal-custom-domains delete [ID] [OPCIONES]
```

`DELETE /v1/portal_custom_domains/{id}`

Elimina el dominio propio y deshace su tenant de CloudFront y el certificado, cuando están aprovisionados. El portal deja de responder en esa dirección.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Eliminado. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-custom-domains dns-status

Consultar DNS del dominio propio

```text
kobana dashboards portal-custom-domains dns-status [ID] [OPCIONES]
```

`GET /v1/portal_custom_domains/{id}/dns_status`

Consulta de solo lectura de los CNAME requeridos, registro por registro, sin cambiar nada en AWS.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Estado del DNS. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-custom-domains verify

Verificar y aprovisionar dominio propio

```text
kobana dashboards portal-custom-domains verify [ID] [OPCIONES]
```

`POST /v1/portal_custom_domains/{id}/verify`

Mientras está pendiente, verifica el DNS y aprovisiona el dominio en AWS (certificado y tenant de CloudFront); una vez aprovisionado, actualiza su estado. Devuelve el dominio con su nuevo `status`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El dominio propio. |
| `404` | Recurso no encontrado en la organización actual. |
