---
id: mailbox
title: "kobana banking mailbox"
sidebar_label: "mailbox"
description: "Buzón (EDI, archivos)"
---

# kobana banking mailbox

Buzón (EDI, archivos)

Comandos de `kobana banking mailbox`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`mailbox entries email`](#mailbox-entries-email) | `GET /v2/mailbox/entries/{entry_uid}/email` | Mostrar Canal de Correo Electrónico |
| [`mailbox entries email activate`](#mailbox-entries-email-activate) | `PUT /v2/mailbox/entries/{entry_uid}/email/activate` | Activar Canal de Correo Electrónico |
| [`mailbox entries email deactivate`](#mailbox-entries-email-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/email/deactivate` | Desactivar Canal de Correo Electrónico |
| [`mailbox entries email create`](#mailbox-entries-email-create) | `POST /v2/mailbox/entries/{entry_uid}/email` | Crear Canal de Correo Electrónico |
| [`mailbox entries email update`](#mailbox-entries-email-update) | `PATCH /v2/mailbox/entries/{entry_uid}/email` | Actualizar Canal de Correo Electrónico |
| [`mailbox entries email delete`](#mailbox-entries-email-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/email` | Eliminar Canal de Correo Electrónico |
| [`mailbox entries s3`](#mailbox-entries-s3) | `GET /v2/mailbox/entries/{entry_uid}/s3` | Mostrar Canal S3 |
| [`mailbox entries s3 activate`](#mailbox-entries-s3-activate) | `PUT /v2/mailbox/entries/{entry_uid}/s3/activate` | Activar Canal S3 |
| [`mailbox entries s3 deactivate`](#mailbox-entries-s3-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/s3/deactivate` | Desactivar Canal S3 |
| [`mailbox entries s3 fetch`](#mailbox-entries-s3-fetch) | `PUT /v2/mailbox/entries/{entry_uid}/s3/fetch` | Buscar archivos del Canal S3 |
| [`mailbox entries s3 update-password`](#mailbox-entries-s3-update-password) | `PUT /v2/mailbox/entries/{entry_uid}/s3/update_password` | Actualizar credenciales AWS del Canal S3 |
| [`mailbox entries s3 create`](#mailbox-entries-s3-create) | `POST /v2/mailbox/entries/{entry_uid}/s3` | Crear Canal S3 |
| [`mailbox entries s3 delete`](#mailbox-entries-s3-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/s3` | Eliminar Canal S3 |
| [`mailbox entries sftp`](#mailbox-entries-sftp) | `GET /v2/mailbox/entries/{entry_uid}/sftp` | Mostrar Canal SFTP |
| [`mailbox entries sftp activate`](#mailbox-entries-sftp-activate) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/activate` | Activar Canal SFTP |
| [`mailbox entries sftp deactivate`](#mailbox-entries-sftp-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/deactivate` | Desactivar Canal SFTP |
| [`mailbox entries sftp fetch`](#mailbox-entries-sftp-fetch) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/fetch` | Buscar archivos del Canal SFTP |
| [`mailbox entries sftp update-password`](#mailbox-entries-sftp-update-password) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/update_password` | Actualizar credenciales SSH del Canal SFTP |
| [`mailbox entries sftp create`](#mailbox-entries-sftp-create) | `POST /v2/mailbox/entries/{entry_uid}/sftp` | Crear Canal SFTP |
| [`mailbox entries sftp update`](#mailbox-entries-sftp-update) | `PATCH /v2/mailbox/entries/{entry_uid}/sftp` | Actualizar Canal SFTP |
| [`mailbox entries sftp delete`](#mailbox-entries-sftp-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/sftp` | Eliminar Canal SFTP |
| [`mailbox entries syncthing`](#mailbox-entries-syncthing) | `GET /v2/mailbox/entries/{entry_uid}/syncthing` | Mostrar Canal Syncthing |
| [`mailbox entries syncthing activate`](#mailbox-entries-syncthing-activate) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/activate` | Activar Canal Syncthing |
| [`mailbox entries syncthing deactivate`](#mailbox-entries-syncthing-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/deactivate` | Desactivar Canal Syncthing |
| [`mailbox entries syncthing resend-invites`](#mailbox-entries-syncthing-resend-invites) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/resend_invites` | Reenviar Invitaciones de Syncthing |
| [`mailbox entries syncthing update-status`](#mailbox-entries-syncthing-update-status) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/update_status` | Actualizar Estado del Servidor Syncthing |
| [`mailbox entries syncthing create`](#mailbox-entries-syncthing-create) | `POST /v2/mailbox/entries/{entry_uid}/syncthing` | Crear Canal Syncthing |
| [`mailbox entries syncthing update`](#mailbox-entries-syncthing-update) | `PATCH /v2/mailbox/entries/{entry_uid}/syncthing` | Actualizar Canal Syncthing |
| [`mailbox entries syncthing delete`](#mailbox-entries-syncthing-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/syncthing` | Eliminar Canal Syncthing |
| [`mailbox entries whatsapp`](#mailbox-entries-whatsapp) | `GET /v2/mailbox/entries/{entry_uid}/whatsapp` | Mostrar Canal WhatsApp |
| [`mailbox entries whatsapp activate`](#mailbox-entries-whatsapp-activate) | `PUT /v2/mailbox/entries/{entry_uid}/whatsapp/activate` | Activar Canal WhatsApp |
| [`mailbox entries whatsapp deactivate`](#mailbox-entries-whatsapp-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/whatsapp/deactivate` | Desactivar Canal WhatsApp |
| [`mailbox entries whatsapp create`](#mailbox-entries-whatsapp-create) | `POST /v2/mailbox/entries/{entry_uid}/whatsapp` | Crear Canal WhatsApp |
| [`mailbox entries whatsapp update`](#mailbox-entries-whatsapp-update) | `PATCH /v2/mailbox/entries/{entry_uid}/whatsapp` | Actualizar Canal WhatsApp |
| [`mailbox entries whatsapp delete`](#mailbox-entries-whatsapp-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/whatsapp` | Eliminar Canal WhatsApp |
| [`mailbox entries list`](#mailbox-entries-list) | `GET /v2/mailbox/entries` | Listar Buzones de Correo |
| [`mailbox entries create`](#mailbox-entries-create) | `POST /v2/mailbox/entries` | Crear Buzón de Correo |
| [`mailbox entries files`](#mailbox-entries-files) | `POST /v2/mailbox/entries/{entry_uid}/files` | Crear Archivo de Buzón de Correo |
| [`mailbox entries get`](#mailbox-entries-get) | `GET /v2/mailbox/entries/{uid}` | Ver Buzón de Correo |
| [`mailbox entries update`](#mailbox-entries-update) | `PATCH /v2/mailbox/entries/{uid}` | Actualizar Buzón de Correo |
| [`mailbox entries delete`](#mailbox-entries-delete) | `DELETE /v2/mailbox/entries/{uid}` | Eliminar Buzón de Correo |
| [`mailbox files list`](#mailbox-files-list) | `GET /v2/mailbox/files` | Listar Archivos del Buzón de Correo |
| [`mailbox files get`](#mailbox-files-get) | `GET /v2/mailbox/files/{uid}` | Ver Archivo de Buzón de Correo |
| [`mailbox files update`](#mailbox-files-update) | `PATCH /v2/mailbox/files/{uid}` | Actualizar Archivo de Buzón de Correo |
| [`mailbox files delete`](#mailbox-files-delete) | `DELETE /v2/mailbox/files/{uid}` | Eliminar Archivo de Buzón de Correo |

## mailbox entries email

Mostrar Canal de Correo Electrónico

```text
kobana banking mailbox entries email [ENTRY_UID] [OPCIONES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/email`

Devuelve el canal de correo electrónico configurado en el Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal de correo electrónico encontrado. |
| `404` | Canal no encontrado. |

## mailbox entries email activate

Activar Canal de Correo Electrónico

```text
kobana banking mailbox entries email activate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/email/activate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Activación encolada. |

## mailbox entries email deactivate

Desactivar Canal de Correo Electrónico

```text
kobana banking mailbox entries email deactivate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/email/deactivate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Desactivación encolada. |

## mailbox entries email create

Crear Canal de Correo Electrónico

```text
kobana banking mailbox entries email create [ENTRY_UID] [OPCIONES]
```

`POST /v2/mailbox/entries/{entry_uid}/email`

Crea un canal de correo electrónico para el Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `inbox_emails` | `array<object>` | no |  |
| `outbox_emails` | `array<object>` | no |  |

Otros campos y objetos anidados: `kobana schema banking.mailbox.entries.email.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Canal creado. |

## mailbox entries email update

Actualizar Canal de Correo Electrónico

```text
kobana banking mailbox entries email update [ENTRY_UID] [OPCIONES]
```

`PATCH /v2/mailbox/entries/{entry_uid}/email`

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `inbox_emails` | `array<object>` | no |  |
| `outbox_emails` | `array<object>` | no |  |

Otros campos y objetos anidados: `kobana schema banking.mailbox.entries.email.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal actualizado. |
| `404` | Canal no encontrado. |

## mailbox entries email delete

Eliminar Canal de Correo Electrónico

```text
kobana banking mailbox entries email delete [ENTRY_UID] [OPCIONES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/email`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Canal eliminado. |
| `422` | El canal activo no puede ser eliminado. |

## mailbox entries s3

Mostrar Canal S3

```text
kobana banking mailbox entries s3 [ENTRY_UID] [OPCIONES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/s3`

Devuelve el canal S3 configurado en el Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal S3 encontrado. |
| `404` | Canal no encontrado. |

## mailbox entries s3 activate

Activar Canal S3

```text
kobana banking mailbox entries s3 activate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/activate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Activación encolada. |

## mailbox entries s3 deactivate

Desactivar Canal S3

```text
kobana banking mailbox entries s3 deactivate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/deactivate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Desactivación encolada. |

## mailbox entries s3 fetch

Buscar archivos del Canal S3

```text
kobana banking mailbox entries s3 fetch [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/fetch`

Encola la búsqueda de archivos del bucket S3 configurado.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Búsqueda encolada. |

## mailbox entries s3 update-password

Actualizar credenciales AWS del Canal S3

```text
kobana banking mailbox entries s3 update-password [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/update_password`

Encola la actualización de las credenciales AWS del Buzón de Correo y establece el canal como actualizando contraseña.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Actualización encolada. |

## mailbox entries s3 create

Crear Canal S3

```text
kobana banking mailbox entries s3 create [ENTRY_UID] [OPCIONES]
```

`POST /v2/mailbox/entries/{entry_uid}/s3`

Crea un canal S3 para el Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Canal creado. |

## mailbox entries s3 delete

Eliminar Canal S3

```text
kobana banking mailbox entries s3 delete [ENTRY_UID] [OPCIONES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/s3`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Canal eliminado. |
| `422` | El canal activo no puede ser eliminado. |

## mailbox entries sftp

Mostrar Canal SFTP

```text
kobana banking mailbox entries sftp [ENTRY_UID] [OPCIONES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/sftp`

Devuelve el canal SFTP configurado en el Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal SFTP encontrado. |
| `404` | Canal no encontrado. |

## mailbox entries sftp activate

Activar Canal SFTP

```text
kobana banking mailbox entries sftp activate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/activate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Activación encolada. |

## mailbox entries sftp deactivate

Desactivar Canal SFTP

```text
kobana banking mailbox entries sftp deactivate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/deactivate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Desactivación encolada. |

## mailbox entries sftp fetch

Buscar archivos del Canal SFTP

```text
kobana banking mailbox entries sftp fetch [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/fetch`

Encola la búsqueda de archivos del servidor SFTP configurado.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Búsqueda encolada. |

## mailbox entries sftp update-password

Actualizar credenciales SSH del Canal SFTP

```text
kobana banking mailbox entries sftp update-password [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/update_password`

Encola la actualización de las credenciales SSH del Buzón de Correo y establece el canal como actualizando contraseña.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Actualización encolada. |

## mailbox entries sftp create

Crear Canal SFTP

```text
kobana banking mailbox entries sftp create [ENTRY_UID] [OPCIONES]
```

`POST /v2/mailbox/entries/{entry_uid}/sftp`

Crea un canal SFTP para el Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | no | Clave privada SSH (solo escritura) |

Otros campos y objetos anidados: `kobana schema banking.mailbox.entries.sftp.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Canal creado. |

## mailbox entries sftp update

Actualizar Canal SFTP

```text
kobana banking mailbox entries sftp update [ENTRY_UID] [OPCIONES]
```

`PATCH /v2/mailbox/entries/{entry_uid}/sftp`

Actualiza el canal SFTP. El campo de contraseña y los campos administrados por el sistema (host, port, inbox_path, outbox_path) no se aceptan a través de la API.

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | no | Clave privada SSH (solo escritura) |

Otros campos y objetos anidados: `kobana schema banking.mailbox.entries.sftp.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal actualizado. |
| `404` | Canal no encontrado. |

## mailbox entries sftp delete

Eliminar Canal SFTP

```text
kobana banking mailbox entries sftp delete [ENTRY_UID] [OPCIONES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/sftp`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Canal eliminado. |
| `422` | El canal activo no puede ser eliminado. |

## mailbox entries syncthing

Mostrar Canal Syncthing

```text
kobana banking mailbox entries syncthing [ENTRY_UID] [OPCIONES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/syncthing`

Devuelve el canal Syncthing configurado en el Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal Syncthing encontrado. |
| `404` | Canal no encontrado. |

## mailbox entries syncthing activate

Activar Canal Syncthing

```text
kobana banking mailbox entries syncthing activate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/activate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Activación encolada. |

## mailbox entries syncthing deactivate

Desactivar Canal Syncthing

```text
kobana banking mailbox entries syncthing deactivate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/deactivate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Desactivación encolada. |

## mailbox entries syncthing resend-invites

Reenviar Invitaciones de Syncthing

```text
kobana banking mailbox entries syncthing resend-invites [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/resend_invites`

Reenvia las invitaciones de activación del canal Syncthing, forzando el proceso de activación.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Reenvío de invitaciones encolado. |

## mailbox entries syncthing update-status

Actualizar Estado del Servidor Syncthing

```text
kobana banking mailbox entries syncthing update-status [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/update_status`

Solicita la actualización del estado de conexión con el servidor Syncthing.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Actualización de estado encolada. |

## mailbox entries syncthing create

Crear Canal Syncthing

```text
kobana banking mailbox entries syncthing create [ENTRY_UID] [OPCIONES]
```

`POST /v2/mailbox/entries/{entry_uid}/syncthing`

Crea un canal Syncthing para el Buzón de Correo. El device_id debe seguir el formato estándar de Syncthing.

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí |  |
| `device_id` | `string` | sí |  |

Otros campos y objetos anidados: `kobana schema banking.mailbox.entries.syncthing.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Canal creado. |

## mailbox entries syncthing update

Actualizar Canal Syncthing

```text
kobana banking mailbox entries syncthing update [ENTRY_UID] [OPCIONES]
```

`PATCH /v2/mailbox/entries/{entry_uid}/syncthing`

Actualiza el nombre o device_id del canal Syncthing.

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no |  |
| `device_id` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema banking.mailbox.entries.syncthing.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal actualizado. |
| `404` | Canal no encontrado. |

## mailbox entries syncthing delete

Eliminar Canal Syncthing

```text
kobana banking mailbox entries syncthing delete [ENTRY_UID] [OPCIONES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/syncthing`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Canal eliminado. |
| `422` | El canal activo no puede ser eliminado. |

## mailbox entries whatsapp

Mostrar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp [ENTRY_UID] [OPCIONES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/whatsapp`

Devuelve el canal WhatsApp configurado en el Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal WhatsApp encontrado. |
| `404` | Canal no encontrado. |

## mailbox entries whatsapp activate

Activar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp activate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/whatsapp/activate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Activación encolada. |

## mailbox entries whatsapp deactivate

Desactivar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp deactivate [ENTRY_UID] [OPCIONES]
```

`PUT /v2/mailbox/entries/{entry_uid}/whatsapp/deactivate`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Desactivación encolada. |

## mailbox entries whatsapp create

Crear Canal WhatsApp

```text
kobana banking mailbox entries whatsapp create [ENTRY_UID] [OPCIONES]
```

`POST /v2/mailbox/entries/{entry_uid}/whatsapp`

Crea un canal WhatsApp para el Buzón de Correo. El activation_token se genera automáticamente.

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `inbox_phones` | `array<object>` | no |  |
| `outbox_phones` | `array<object>` | no |  |

Otros campos y objetos anidados: `kobana schema banking.mailbox.entries.whatsapp.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Canal creado. |

## mailbox entries whatsapp update

Actualizar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp update [ENTRY_UID] [OPCIONES]
```

`PATCH /v2/mailbox/entries/{entry_uid}/whatsapp`

Actualiza los números de teléfono del canal WhatsApp. El activation_token no se acepta como parámetro.

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `inbox_phones` | `array<object>` | no |  |
| `outbox_phones` | `array<object>` | no |  |

Otros campos y objetos anidados: `kobana schema banking.mailbox.entries.whatsapp.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal actualizado. |
| `404` | Canal no encontrado. |

## mailbox entries whatsapp delete

Eliminar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp delete [ENTRY_UID] [OPCIONES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/whatsapp`

**Argumentos de la ruta:** `entry_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Canal eliminado. |
| `422` | El canal activo no puede ser eliminado. |

## mailbox entries list

Listar Buzones de Correo

```text
kobana banking mailbox entries list [OPCIONES]
```

`GET /v2/mailbox/entries`

Lista los Buzones de Correo de la cuenta.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `per_page` | `integer` | no | Cantidad de registros por página. Máximo de 50. |
| `page` | `integer` | no | Número de página. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Buzones de Correo encontrados. |

## mailbox entries create

Crear Buzón de Correo

```text
kobana banking mailbox entries create [OPCIONES]
```

`POST /v2/mailbox/entries`

Crea un nuevo Buzón de Correo.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.mailbox.entries.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Buzón de Correo creado |
| `422` | Error de validación |

## mailbox entries files

Crear Archivo de Buzón de Correo

```text
kobana banking mailbox entries files [ENTRY_UID] [OPCIONES]
```

`POST /v2/mailbox/entries/{entry_uid}/files`

Crea un nuevo Archivo de Buzón de Correo asociado a un Buzón de Correo.

**Argumentos de la ruta:** `entry_uid`

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.mailbox.entries.files`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Archivo creado |
| `422` | Base64 inválido. |

## mailbox entries get

Ver Buzón de Correo

```text
kobana banking mailbox entries get [UID] [OPCIONES]
```

`GET /v2/mailbox/entries/{uid}`

Consulta información de un Buzón de Correo específico.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Buzón de Correo encontrado |
| `404` | Buzón de Correo no encontrado |

## mailbox entries update

Actualizar Buzón de Correo

```text
kobana banking mailbox entries update [UID] [OPCIONES]
```

`PATCH /v2/mailbox/entries/{uid}`

Actualiza un Buzón de Correo específico.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.mailbox.entries.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Buzón de Correo actualizado |
| `404` | Buzón de Correo no encontrado |

## mailbox entries delete

Eliminar Buzón de Correo

```text
kobana banking mailbox entries delete [UID] [OPCIONES]
```

`DELETE /v2/mailbox/entries/{uid}`

Elimina un Buzón de Correo específico. No es posible eliminar un Buzón de Correo que tiene archivos.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Buzón de Correo eliminado |
| `404` | Buzón de Correo no encontrado |
| `422` | El Buzón de Correo tiene archivos o no puede ser eliminado |

## mailbox files list

Listar Archivos del Buzón de Correo

```text
kobana banking mailbox files list [OPCIONES]
```

`GET /v2/mailbox/files`

Lista los Archivos del Buzón de Correo de la cuenta.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `per_page` | `integer` | no | Cantidad de registros por página. Máximo de 50. |
| `page` | `integer` | no | Número de página. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivos encontrados. |

## mailbox files get

Ver Archivo de Buzón de Correo

```text
kobana banking mailbox files get [UID] [OPCIONES]
```

`GET /v2/mailbox/files/{uid}`

Consulta información de un Archivo de Buzón de Correo específico.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivo encontrado |
| `404` | Archivo no encontrado |

## mailbox files update

Actualizar Archivo de Buzón de Correo

```text
kobana banking mailbox files update [UID] [OPCIONES]
```

`PATCH /v2/mailbox/files/{uid}`

Actualiza un Archivo de Buzón de Correo específico.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.mailbox.files.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivo actualizado |
| `404` | Archivo no encontrado |

## mailbox files delete

Eliminar Archivo de Buzón de Correo

```text
kobana banking mailbox files delete [UID] [OPCIONES]
```

`DELETE /v2/mailbox/files/{uid}`

Elimina un Archivo de Buzón de Correo específico.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Archivo eliminado |
| `404` | Archivo no encontrado |
