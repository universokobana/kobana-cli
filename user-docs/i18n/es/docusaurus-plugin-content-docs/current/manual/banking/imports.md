---
id: imports
title: "kobana banking imports"
sidebar_label: "imports"
---

# kobana banking imports


Comandos de `kobana banking imports`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports/{collection_name}` | Listar Importaciones |
| [`imports create`](#imports-create) | `POST /v1/imports/{collection_name}` | Importar |
| [`imports get`](#imports-get) | `GET /v1/imports/{collection_name}/{id}` | Ver Importación |

## imports list

Listar Importaciones

```text
kobana banking imports list [COLLECTION_NAME] [OPCIONES]
```

`GET /v1/imports/{collection_name}`

Lista las importaciones realizadas para un tipo específico de recurso (clientes, suscripciones, boletos o carnés).

**Argumentos de la ruta:** `collection_name`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Importaciones encontradas |

## imports create

Importar

```text
kobana banking imports create [COLLECTION_NAME] [OPCIONES]
```

`POST /v1/imports/{collection_name}`

Importa recursos (clientes, suscripciones, boletos o carnés) a través de un archivo CSV. El procesamiento se realiza de forma asíncrona.

> **Procesamiento Asíncrono:** Este endpoint se procesa asincronamente.

La Importación se realiza asincronamente, es decir, al recibir la solicitud,
nuestra API guarda los parámetros de la solicitud y procesa la importación en segundo plano.
Después de que se procese la importación, los registros se asociarán a su Cuenta.
Recibe en la respuesta de esta solicitud el ID de la Importación creada y puede consultarlo en cualquier momento.

**Eventos de Webhook**

Cuando se realiza la solicitud, se dispara un evento `import.created`.
Nuestro servidor luego comienza el procesamiento de la importación.
Después del final del procesamiento, se dispara el evento `import.processed`.

**Argumentos de la ruta:** `collection_name`

**Cuerpo de la solicitud**

Se envía como formulario `multipart/form-data`: los archivos van en `--file CAMPO=RUTA`.

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `source` | `string (binary)` | sí | Archivo para importación. |

Otros campos y objetos anidados: `kobana schema banking.imports.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Importación creada |
| `422` | Ya existe una importación en curso o concluida para este archivo |
| `503` | No fue posible verificar la duplicidad de la importación en este momento — intente nuevamente |

## imports get

Ver Importación

```text
kobana banking imports get [COLLECTION_NAME] [ID] [OPCIONES]
```

`GET /v1/imports/{collection_name}/{id}`

Consulta información específica de una Importación.

**Argumentos de la ruta:** `collection_name`, `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Importación encontrada |
| `404` | Cuenta no encontrada |
