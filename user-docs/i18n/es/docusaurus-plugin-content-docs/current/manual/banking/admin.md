---
id: admin
title: "kobana banking admin"
sidebar_label: "admin"
description: "Administración (subcuentas, usuarios, conexiones)"
---

# kobana banking admin

Administración (subcuentas, usuarios, conexiones)

Comandos de `kobana banking admin`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`admin certificates list`](#admin-certificates-list) | `GET /v2/admin/certificates` | Listar Certificados |
| [`admin certificates create`](#admin-certificates-create) | `POST /v2/admin/certificates` | Incluir un Certificado |
| [`admin certificates delete`](#admin-certificates-delete) | `DELETE /v2/admin/certificates/{uid}` | Excluir un Certificado |
| [`admin commands list`](#admin-commands-list) | `GET /v2/admin/commands` | Listar Comandos |
| [`admin commands get`](#admin-commands-get) | `GET /v2/admin/commands/{uid}` | Ver Comando |
| [`admin connections associations create`](#admin-connections-associations-create) | `POST /v2/admin/connections/{uid}/associations` | Conectar Cuenta de Servicio |
| [`admin connections associations delete`](#admin-connections-associations-delete) | `DELETE /v2/admin/connections/{uid}/associations` | Desconectar Cuenta de Servicio |
| [`admin connections list`](#admin-connections-list) | `GET /v2/admin/connections` | Listar Conexiones |
| [`admin connections create`](#admin-connections-create) | `POST /v2/admin/connections` | Incluir una Conexión |
| [`admin connections get`](#admin-connections-get) | `GET /v2/admin/connections/{uid}` | Información de la conexión |
| [`admin connections update`](#admin-connections-update) | `PUT /v2/admin/connections/{uid}` | Actualizar Conexión |
| [`admin connections delete`](#admin-connections-delete) | `DELETE /v2/admin/connections/{uid}` | Excluir una Conexión |
| [`admin subaccounts list`](#admin-subaccounts-list) | `GET /v2/admin/subaccounts` | Listar Subcuentas |
| [`admin subaccounts create`](#admin-subaccounts-create) | `POST /v2/admin/subaccounts` | Crear una Subcuenta |
| [`admin subaccounts get`](#admin-subaccounts-get) | `GET /v2/admin/subaccounts/{uid}` | Visualizar una Subcuenta |
| [`admin subaccounts update`](#admin-subaccounts-update) | `PUT /v2/admin/subaccounts/{uid}` | Alterar Subcuenta |
| [`admin users list`](#admin-users-list) | `GET /v2/admin/users` | Listar Usuarios |
| [`admin users create`](#admin-users-create) | `POST /v2/admin/users` | Incluir un Usuario |
| [`admin users update`](#admin-users-update) | `PUT /v2/admin/users/{uid}` | Alterar Datos De Un Usuario |
| [`admin users delete`](#admin-users-delete) | `DELETE /v2/admin/users/{uid}` | Excluir un Usuario |

## admin certificates list

Listar Certificados

```text
kobana banking admin certificates list [OPCIONES]
```

`GET /v2/admin/certificates`

Lista todos los certificados registrados en la cuenta con filtros opcionales de paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Certificados encontrados |

## admin certificates create

Incluir un Certificado

```text
kobana banking admin certificates create [OPCIONES]
```

`POST /v2/admin/certificates`

Incluir un certificado.

> **Tipo del Certificado:** Conforme al tipo del certificado informado es necesario enviar el objeto files con contenidos diferentes. Si el tipo del certificado es `CRT` es obligatorio enviar el `crt_file` y el `crt_private_key_file`, si el tipo es `PFX` es obligatorio `pfx_file` y `pfx_password`.

**Cuerpo de la solicitud**

Se envía como formulario `multipart/form-data`: los archivos van en `--file CAMPO=RUTA`.

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí | Nombre dado al certificado |
| `cnpj_cpf` | `string` | sí | CPF/CNPJ |
| `type` | `string` | sí | Tipo de certificado: Si el tipo de certificado es `CRT` es obligatorio enviar `crt_file` y `crt_private_key_file`, si el tipo es `PFX` es obligatorio `pfx_file` y `pfx_password`. * `crt` CRT * `pfx` PFX Valores: `crt`, `pfx`. |
| `files` | `object` | sí | Archivo del certificado y contraseña. Si el tipo de certificado es `CRT` es obligatorio enviar `crt_file` y `crt_private_key_file`, si el tipo es `PFX` es obligatorio `pfx_file` y `pfx_password`. |
| `subaccounts` | `boolean` | no | Indica si el certificado está liberado para usar en subcuentas |

Otros campos y objetos anidados: `kobana schema banking.admin.certificates.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Certificado incluido |
| `422` | Datos del certificado inválido |

## admin certificates delete

Excluir un Certificado

```text
kobana banking admin certificates delete [UID] [OPCIONES]
```

`DELETE /v2/admin/certificates/{uid}`

Elimina permanentemente un certificado. Solo es posible eliminar certificados que no estén vinculados a ninguna conexión activa y que no hayan sido creados por el sistema.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Certificado eliminado |
| `404` | Certificado no encontrado |
| `422` | Certificado creado por el sistema no puede ser eliminado |

## admin commands list

Listar Comandos

```text
kobana banking admin commands list [OPCIONES]
```

`GET /v2/admin/commands`

Lista todos los Comandos de la cuenta, con soporte a filtros por estado, operación y origen de confirmación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Filtrar por estado del Comando. |
| `operation` | `string` | no | Filtrar por tipo de operación. |
| `confirmation_via` | `string` | no | Filtrar por origen de la confirmación. Valores: `api`, `edi`. |
| `created_from` | `string` | no | Filtrar por fecha de creación (inicio). Formato: YYYY-MM-DD. |
| `created_to` | `string` | no | Filtrar por fecha de creación (fin). Formato: YYYY-MM-DD. |
| `resource_type` | `string` | no | Filtrar por tipo de recurso (ej.: `BankBillet`, `Pix`, `Financial::Transfer`). |
| `resource_uid` | `string` | no | Filtrar Comandos de un recurso específico (boleto, Pix o cuenta financiera) por su uid. |
| `financial_provider_slug` | `string` | no | Filtrar por el slug del proveedor financiero. |
| `bank_billet_account_uid` | `string` | no | Filtrar Comandos de Boletos de una cartera (BankBilletAccount) específica, por su uid. |
| `pix_account_uid` | `string` | no | Filtrar Comandos de Pix de una Cuenta Pix (PixAccount) específica, por su uid. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de Comandos |

## admin commands get

Ver Comando

```text
kobana banking admin commands get [UID] [OPCIONES]
```

`GET /v2/admin/commands/{uid}`

Devuelve los detalles de un Comando específico por su UID.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando encontrado |
| `404` | Comando no encontrado |

## admin connections associations create

Conectar Cuenta de Servicio

```text
kobana banking admin connections associations create [UID] [OPCIONES]
```

`POST /v2/admin/connections/{uid}/associations`

Este endpoint se utiliza para asociar una Cuenta de Servicio a una Conexión.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `resource` | `object` | sí |  |

Otros campos y objetos anidados: `kobana schema banking.admin.connections.associations.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Asociación creada |
| `422` | Datos de la asociación inválidos |

## admin connections associations delete

Desconectar Cuenta de Servicio

```text
kobana banking admin connections associations delete [UID] [OPCIONES]
```

`DELETE /v2/admin/connections/{uid}/associations`

Este endpoint se utiliza para remover asociación de una Conexión con Cuenta de Servicio.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `resource` | `object` | sí |  |

Otros campos y objetos anidados: `kobana schema banking.admin.connections.associations.delete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Conexión |
| `404` | Conexión no encontrada |

## admin connections list

Listar Conexiones

```text
kobana banking admin connections list [OPCIONES]
```

`GET /v2/admin/connections`

Lista todas las conexiones registradas con filtros opcionales por proveedor, certificado, estado de habilitación y validación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `provider_slug` | `string` | no | Proveedor financiero |
| `certificate_uid` | `string` | no | UUID del Certificado |
| `enabled` | `boolean` | no | Conexiones habilitadas |
| `validated` | `boolean` | no | Conexiones validadas |
| `created_from` | `string` | no | Fecha inicial del filtro de fecha de creación de la conexión. |
| `created_to` | `string` | no | Fecha final del filtro de fecha de creación de la conexión. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Conexiones encontradas |

## admin connections create

Incluir una Conexión

```text
kobana banking admin connections create [OPCIONES]
```

`POST /v2/admin/connections`

Registra una nueva conexión con un proveedor financiero.

> **Procesamiento Asíncrono:** la respuesta `201` se devuelve antes de que las credenciales sean validadas ante el proveedor. La conexión se crea con `validated_at` y `validate_failed_at` nulos, y la validación termina en segundo plano.

**Ciclo de validación**

| Estado | `validated_at` | `validate_failed_at` |
| --- | --- | --- |
| Validando | `null` | `null` |
| Validada | completado | `null` |
| Falló | `null` | completado |

Cuando la validación es exitosa, `enabled_at` también se completa, aunque la conexión haya sido creada con `enabled: false`.

**Eventos de Webhook**

- `integration.connection.db.created`: se dispara en esta operación, con la conexión aún no validada.
- `integration.connection.db.updated`: se dispara cuando la validación termina, con éxito o falla. Lea el resultado en `object.validated_at` y `object.validate_failed_at`: la falla no aparece en `changes`.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.admin.connections.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Conexión creada |
| `422` | Datos de la conexión inválidos |

## admin connections get

Información de la conexión

```text
kobana banking admin connections get [UID] [OPCIONES]
```

`GET /v2/admin/connections/{uid}`

Consulta una conexión. Justo después de la creación, `validated_at` y `validate_failed_at` nulos indican que la validación de las credenciales aún está en curso (ver Incluir una Conexión).

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Conexión encontrada |
| `404` | Conexión no encontrada |

## admin connections update

Actualizar Conexión

```text
kobana banking admin connections update [UID] [OPCIONES]
```

`PUT /v2/admin/connections/{uid}`

Actualiza una conexión. Con `revalidate: true`, la revalidación de las credenciales puede terminar en segundo plano: en ese caso la respuesta trae `validation_status: pending` y el resultado aparece en `validated_at` y `validate_failed_at`, como en la creación.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no | Nombre de la Conexión |
| `environment` | `string` | no | Entorno de conexión con el proveedor financiero. `production`: Producción. `homologation`: Homologación * Este campo será ignorado en nuestro entorno de Sandbox Valores: `production`, `homologation`. |
| `apis` | `array<string>` | no | APIs permitidas para el proveedor financiero. `charge/bank_billet`: Boleto. `charge/pix`: Pix. `payment`: Pago. `transfer`: Transferencia |
| `credentials` | `object` | no | Credenciales. |
| `certificate_uid` | `string (uuid)` | no | UUID del Certificado. |
| `keypair_uid` | `string (uuid)` | no | UUID del Par de Claves. |
| `enabled` | `boolean` | no | Habilita conexión |
| `revalidate` | `boolean` | no | Revalida las credenciales en el proveedor financiero. Si se pasa true, la conexión solo se alterará si las credenciales y permisos son válidos. |

Otros campos y objetos anidados: `kobana schema banking.admin.connections.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Conexión actualizada |
| `404` | Conexión no encontrada |
| `422` | Credenciales ya utilizadas por otra conexión de la cuenta |

## admin connections delete

Excluir una Conexión

```text
kobana banking admin connections delete [UID] [OPCIONES]
```

`DELETE /v2/admin/connections/{uid}`

Excluye una conexión.
        La exclusión de una conexión resultará en la remoción de todas sus asociaciones, imposibilitando el uso de la API previamente conectada.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Conexión |
| `404` | Conexión no encontrada |

## admin subaccounts list

Listar Subcuentas

```text
kobana banking admin subaccounts list [OPCIONES]
```

`GET /v2/admin/subaccounts`

Lista todas las subcuentas de la cuenta principal con filtros opcionales por email, CNPJ y fecha de creación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `email` | `string` | no | Filtro por email. Busca subcuenta por email |
| `business_cnpj` | `string` | no | Filtro por CNPJ. Busca subcuenta por CNPJ |
| `created_from` | `string` | no | Fecha inicial del filtro de fecha de creación de la subcuenta |
| `created_to` | `string` | no | Fecha final del filtro de fecha de creación de la subcuenta |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Subcuentas encontradas |

## admin subaccounts create

Crear una Subcuenta

```text
kobana banking admin subaccounts create [OPCIONES]
```

`POST /v2/admin/subaccounts`

Crear una subcuenta

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `nickname` | `string` | sí | Apodo |
| `id` | `integer` | no | ID de la subcuenta (deprecated, use uid) |
| `uid` | `string` | no | UID de la subcuenta |
| `parent_id` | `integer` | no | ID de la cuenta principal (deprecated, use parent_uid) |
| `parent_uid` | `string (uuid)` | no | UID de la cuenta principal |
| `email` | `string (email)` | no | Correo electrónico |
| `business_name` | `string` | no | Nombre Comercial |
| `business_cnpj` | `string` | no | CNPJ de la Empresa |
| `business_legal_name` | `string` | no | Razón Social |
| `configuration` | `string (json)` | no | Configuración de datos estándar de la cuenta |
| `api_access_token` | `string` | no | Token de acceso de la subcuenta |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas a Subcuenta |
| `created_via_api` | `boolean` | no | Indica si el registro fue creado por la API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.admin.subaccounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Subcuenta creada |
| `422` | Datos de la subcuenta inválidos |

## admin subaccounts get

Visualizar una Subcuenta

```text
kobana banking admin subaccounts get [UID] [OPCIONES]
```

`GET /v2/admin/subaccounts/{uid}`

Visualizar una subcuenta

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Subcuenta encontrada |
| `404` | Subcuenta no encontrada |

## admin subaccounts update

Alterar Subcuenta

```text
kobana banking admin subaccounts update [UID] [OPCIONES]
```

`PUT /v2/admin/subaccounts/{uid}`

Alterar Subcuenta. No es posible alterar el CNPJ de la subcuenta.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `enabled` | `integer` | no | Indica si la subcuenta está activa o inactiva. Utilice `1` para activar y `0` para desactivar. Valores: `0`, `1`. |
| `id` | `integer` | no | ID de la subcuenta (deprecated, use uid) |
| `uid` | `string` | no | UID de la subcuenta |
| `parent_id` | `integer` | no | ID de la cuenta principal (deprecated, use parent_uid) |
| `parent_uid` | `string (uuid)` | no | UID de la cuenta principal |
| `email` | `string (email)` | no | Correo electrónico |
| `business_cnpj` | `string` | no | CNPJ de la Empresa |
| `nickname` | `string` | no | Apodo |
| `business_legal_name` | `string` | no | Razón Social |
| `account_type` | `string` | no | Tipo de Cuenta: * `individual` Persona Física * `juridical` Persona Jurídica Valores: `individual`, `juridical`. |
| `first_name` | `string` | no | Primer Nombre del Responsable |
| `middle_name` | `string` | no | Nombre del Medio del Responsable |
| `last_name` | `string` | no | Último Nombre del Responsable |
| `full_name` | `string` | no | Nombre completo del Responsable |
| `gender` | `string` | no | Género |
| `cpf` | `string` | no | CPF |
| `address_street_name` | `string` | no | Dirección |
| `address_state` | `string` | no | Estado |
| `address_neighborhood` | `string` | no | Barrio |
| `address_postal_code` | `string` | no | Código Postal |
| `address_number` | `string` | no | Número |
| `address_complement` | `string` | no | Complemento |
| `address_city_name` | `string` | no | Ciudad |
| `phone_number` | `string` | no | Teléfono |
| `date_of_birth` | `string (date)` | no | Fecha de Nacimiento |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas a Subcuenta |
| `business_category` | `integer` | no | Categoría: \| Código de Categoría \| Descripción \| \|---\|---\| \| 1000 \| Arte y objetos de coleccionistas \| \| 1001 \| Bebés \| \| 1002 \| Belleza y fragancias \| \| 1022 \| Juguetes y pasatiempos \| \| 1015 \| Casa y jardín \| \| 1006 \| Computadoras \| \| 1004 \| De empresa a empresa (B2B) \| \| 1007 \| Educación \| \| 1008 \| Electrónica y telecomunicaciones \| \| 1009 \| Entretenimiento y medios \| \| 1021 \| Deportes y actividades al aire libre \| \| 1013 \| Gobierno \| \| 1003 \| Libros y revistas \| \| 1012 \| Regalos y flores \| \| 1018 \| Religión y espiritualidad (con fines de lucro) \| \| 1014 \| Salud y cuidados personales \| \| 1016 \| Organizaciones sin fines de lucro \| \| 1025 \| Servicio \| \| 1020 \| Servicios - Otro \| \| 1010 \| Servicios y productos financieros \| \| 1019 \| Venta minorista (no clasificado en ningún otro lugar) \| \| 1011 \| Venta minorista y servicios de alimentos \| \| 1024 \| Venta de vehículos \| \| 1005 \| Ropa \| \| 1023 \| Viaje \| Valores: `null`, `1000`, `1001`, `1002`, `1022`, `1015`, `1006`, `1004`, `1007`, `1008`, `1009`, `1021`, `1013`, `1003`, `1012`, `1018`, `1014`, `1016`, `1025`, `1020`, `1010`, `1019`, `1011`, `1024`, `1005`, `1023`. |
| `business_subcategory` | `integer` | no | Subcategoría: \| Código de Categoría \| Código de Subcategoría \| Descripción \| \|---\|---\|---\| \| 1017 \| 2173 \| Animales de compañía especiales o raros \| \| 1017 \| 2171 \| Medicamentos y suplementos \| \| 1017 \| 2172 \| Tiendas de mascotas, alimentos y suministros para animales de compañía \| \| 1017 \| 2174 \| Servicios veterinarios \| \| 1000 \| 2000 \| Antigüedades \| \| 1000 \| 2004 \| Arte digital \| \| 1000 \| 2007 \| Costura, bordado y telas \| \| 1000 \| 2003 \| Cámaras y suministros fotográficos \| \| 1000 \| 2005 \| Recuerdos \| \| 1000 \| 2006 \| Tienda de música - instrumentos y partituras \| \| 1000 \| 2002 \| Comerciantes y galerías \| \| 1000 \| 2009 \| Papel de carta, impresión y escritura \| \| 1000 \| 2008 \| Sellos y monedas \| \| 1000 \| 2001 \| Suministros de artesanía \| \| 1000 \| 2010 \| Artículos vintage y de coleccionistas \| \| 1001 \| 2012 \| Muebles \| \| 1001 \| 2013 \| Productos para bebés - Otro \| \| 1001 \| 2014 \| Seguridad y salud \| \| 1001 \| 2011 \| Prendas de vestir \| \| 1002 \| 2015 \| Baño y cuerpo \| \| 1002 \| 2016 \| Fragancias y perfumes \| \| 1002 \| 2017 \| Maquillaje y cosméticos \| \| 1022 \| 2252 \| Artesanía \| \| 1022 \| 2253 \| Cámaras y suministros fotográficos \| \| 1022 \| 2255 \| Recuerdos \| \| 1022 \| 2256 \| Tienda de música - instrumentos y partituras \| \| 1022 \| 2254 \| Tiendas de aficiones, juguetes y juegos \| \| 1022 \| 2258 \| Papel de carta, impresión y escritura \| \| 1022 \| 2257 \| Sellos y monedas \| \| 1022 \| 2260 \| Videojuegos y sistemas \| \| 1022 \| 2259 \| Artículos vintage y de coleccionistas \| \| 1015 \| 2145 \| Antigüedades \| \| 1015 \| 2146 \| Aparatos \| \| 1015 \| 2148 \| Ropa de cama y baño \| \| 1015 \| 2150 \| Cortinas y tapicería \| \| 1015 \| 2157 \| Decoración residencial \| \| 1015 \| 2162 \| Equipos de seguridad y vigilancia \| \| 1015 \| 2152 \| Chimeneas y pantallas de chimenea \| \| 1015 \| 2147 \| Comerciantes y galerías \| \| 1015 \| 2149 \| Materiales de construcción \| \| 1015 \| 2156 \| Máquinas y herramientas \| \| 1015 \| 2153 \| Muebles \| \| 1015 \| 2160 \| Paisajismo \| \| 1015 \| 2163 \| Piscinas y spas \| \| 1015 \| 2151 \| Servicios de control de plagas y desinfección \| \| 1015 \| 2154 \| Suministros de jardinería \| \| 1015 \| 2161 \| Alfombras y tapetes \| \| 1015 \| 2159 \| Utensilios de cocina \| \| 1015 \| 2158 \| Artículos para el hogar \| \| 1015 \| 2155 \| Vidrio, pintura y papel pintado \| \| 1006 \| 2057 \| Computadoras y servicios de procesamiento de datos \| \| 1006 \| 2059 \| Contenido digital \| \| 1006 \| 2058 \| Desktops, portátiles y netbooks \| \| 1006 \| 2069 \| Hospedaje y diseño web \| \| 1006 \| 2064 \| Juegos en línea \| \| 1006 \| 2062 \| Monitores y proyectores \| \| 1006 \| 2066 \| Periféricos \| \| 1006 \| 2065 \| Partes y accesorios \| \| 1006 \| 2063 \| Red de comunicación \| \| 1006 \| 2060 \| Servicios de comercio electrónico \| \| 1006 \| 2061 \| Servicios de reparación y mantenimiento \| \| 1006 \| 2068 \| Servicios de capacitación \| \| 1006 \| 2067 \| Software \| \| 1004 \| 2027 \| Agrícola \| \| 1004 \| 2047 \| Mayorista \| \| 1004 \| 2031 \| Construcción \| \| 1004 \| 2025 \| Contabilidad \| \| 1004 \| 2043 \| Publicación e impresión \| \| 1004 \| 2045 \| Embalaje y envío \| \| 1004 \| 2030 \| Fotografía comercial, arte y gráficos \| \| 1004 \| 2038 \| Correo directo \| \| 1004 \| 2039 \| Marketing \| \| 1004 \| 2040 \| Marketing de varios niveles \| \| 1004 \| 2041 \| Muebles de oficina y comerciales \| \| 1004 \| 2029 \| Productos químicos y similares \| \| 1004 \| 2026 \| Publicidad \| \| 1004 \| 2028 \| Servicios de arquitectura, ingeniería y prospección \| \| 1004 \| 2032 \| Servicios de consultoría \| \| 1004 \| 2044 \| Servicios de copia rápida y reproducción \| \| 1004 \| 2046 \| Servicios de taquigrafía y secretarial \| \| 1004 \| 2034 \| Servicios de arrendamiento y alquiler de equipos \| \| 1004 \| 2036 \| Servicios de reclutamiento y selección \| \| 1004 \| 2035 \| Servicios de reparación de equipos \| \| 1004 \| 2033 \| Servicios educativos \| \| 1004 \| 2042 \| Suministros y equipos de oficina \| \| 1004 \| 2037 \| Suministros industriales y de producción \| \| 1007 \| 2073 \| Academias, estudios y escuelas de danza \| \| 1007 \| 2070 \| Escuelas comerciales y secretariales \| \| 1007 \| 2074 \| Escuelas primarias y secundarias \| \| 1007 \| 2075 \| Escuelas vocacionales y comerciales \| \| 1007 \| 2072 \| Colegios y universidades \| \| 1007 \| 2071 \| Servicios de cuidado infantil diario \| \| 1008 \| 2079 \| Accesorios electrónicos generales \| \| 1008 \| 2085 \| Tarjetas telefónicas \| \| 1008 \| 2076 \| Cámaras, videocámaras y equipos \| \| 1008 \| 2081 \| Electrónica residencial \| \| 1008 \| 2083 \| Equipos de telecomunicación y ventas \| \| 1008 \| 2082 \| Seguridad y vigilancia \| \| 1008 \| 2084 \| Servicios de telecomunicación \| \| 1008 \| 2077 \| Audio de coche y electrónica \| \| 1008 \| 2078 \| Teléfonos celulares, PDA y buscapersonas \| \| 1008 \| 2080 \| Audio residencial \| \| 1009 \| 2097 \| Animadores de programas \| \| 1009 \| 2094 \| Juguetes y juegos \| \| 1009 \| 2095 \| Máquinas tragamonedas \| \| 1009 \| 2096 \| Contenido digital \| \| 1009 \| 2091 \| Contenido digital para adultos \| \| 1009 \| 2088 \| Películas - DVDs, videocasetes \| \| 1009 \| 2087 \| Entradas de cine \| \| 1009 \| 2093 \| Entradas de teatro \| \| 1009 \| 2092 \| Entradas para conciertos \| \| 1009 \| 2098 \| Juego \| \| 1009 \| 2099 \| Juegos en línea \| \| 1009 \| 2086 \| Recuerdos \| \| 1009 \| 2089 \| Música - CD, casetes y álbumes \| \| 1009 \| 2090 \| TV y radio por cable, satélite y otros tipos pagados \| \| 1009 \| 2100 \| Videojuegos y sistemas \| \| 1021 \| 2241 \| Academias, estudios y escuelas de danza \| \| 1021 \| 2244 \| Accesorios de armas de fuego \| \| 1021 \| 2248 \| Armas de artes marciales \| \| 1021 \| 2245 \| Armas de fuego \| \| 1021 \| 2237 \| Calzado deportivo \| \| 1021 \| 2240 \| Camping y actividades al aire libre \| \| 1021 \| 2246 \| Caza \| \| 1021 \| 2243 \| Engranajes de reloj \| \| 1021 \| 2250 \| Equipo deportivo \| \| 1021 \| 2242 \| Ejercicio y acondicionamiento físico \| \| 1021 \| 2247 \| Cuchillos \| \| 1021 \| 2249 \| Juegos y juguetes deportivos \| \| 1021 \| 2238 \| Tienda, servicio y reparación de bicicletas \| \| 1021 \| 2251 \| Piscinas y spas \| \| 1021 \| 2239 \| Remo, vela y accesorios \| \| 1013 \| 2137 \| Servicios gubernamentales (no clasificado en ningún otro lugar) \| \| 1003 \| 2019 \| Contenido digital \| \| 1003 \| 2023 \| Publicación e impresión \| \| 1003 \| 2021 \| Ficción y no ficción \| \| 1003 \| 2020 \| Libros de texto y educativos \| \| 1003 \| 2018 \| Libros de audio \| \| 1003 \| 2024 \| Libros raros y usados \| \| 1003 \| 2022 \| Revistas \| \| 1012 \| 2134 \| Alimentos gourmet \| \| 1012 \| 2132 \| Floristerías \| \| 1012 \| 2133 \| Tiendas de regalos, tarjetas, novedades y recuerdos \| \| 1012 \| 2136 \| Suministros para fiestas \| \| 1012 \| 2135 \| Vivero de plantas y flores \| \| 1018 \| 2176 \| Mercancía \| \| 1018 \| 2177 \| Servicios (no clasificados en ningún otro lugar) \| \| 1018 \| 2175 \| Servicios de asociación \| \| 1014 \| 2141 \| Asistencia médica \| \| 1014 \| 2140 \| Cuidado dental \| \| 1014 \| 2142 \| Equipos y suministros médicos \| \| 1014 \| 2138 \| Farmacia (excepto medicamentos recetados) \| \| 1014 \| 2139 \| Farmacia (incluyendo medicamentos recetados) \| \| 1014 \| 2143 \| Oftalmología \| \| 1014 \| 2144 \| Vitaminas y suplementos \| \| 1016 \| 2169 \| Educacional \| \| 1016 \| 2164 \| Entidad de asistencia \| \| 1016 \| 2167 \| Otro \| \| 1016 \| 2168 \| Personal \| \| 1016 \| 2165 \| Político \| \| 1016 \| 2166 \| Religioso \| \| 1025 \| 2297 \| Accesorios \| \| 1025 \| 2296 \| Alquiler de camiones y remolques utilitarios \| \| 1025 \| 2293 \| Alquiler de remolques y vehículos recreativos \| \| 1025 \| 2291 \| Alquiler y arrendamiento de barcos \| \| 1025 \| 2294 \| Herramientas y equipos \| \| 1025 \| 2292 \| Lavado de autos \| \| 1025 \| 2288 \| Alquiler de autos \| \| 1025 \| 2284 \| Partes y suministros nuevos - vehículo motor \| \| 1025 \| 2285 \| Partes usadas - vehículo motor \| \| 1025 \| 2287 \| Reparación y pintura de autos \| \| 1025 \| 2295 \| Servicio de grúa \| \| 1025 \| 2289 \| Servicios de autos \| \| 1025 \| 2290 \| Suministro y servicios de neumáticos automotrices \| \| 1025 \| 2286 \| Audio y video \| \| 1020 \| 2232 \| Ajustes y alteraciones \| \| 1020 \| 2216 \| Asistencia médica \| \| 1020 \| 2188 \| Carpintería \| \| 1020 \| 2217 \| Clubes y organizaciones asociadas \| \| 1020 \| 2192 \| Computadoras y servicios de procesamiento de datos \| \| 1020 \| 2225 \| Reparación de radios, televisores y estéreos \| \| 1020 \| 2226 \| Agente de bienes raíces \| \| 1020 \| 2197 \| Cuidado dental \| \| 1020 \| 2205 \| Diseño gráfico y comercial \| \| 1020 \| 2230 \| Embalaje y envío \| \| 1020 \| 2204 \| Contratistas generales \| \| 1020 \| 2220 \| Citas en línea \| \| 1020 \| 2200 \| Entretenimiento \| \| 1020 \| 2222 \| Estudios fotográficos - retratos \| \| 1020 \| 2221 \| Acabado de fotos \| \| 1020 \| 2191 \| Fotografía comercial \| \| 1020 \| 2227 \| Gestión de alquiler de bienes raíces \| \| 1020 \| 2207 \| Identidades, certificados y pasaportes \| \| 1020 \| 2208 \| Importación y exportación \| \| 1020 \| 2203 \| Juego \| \| 1020 \| 2190 \| Limpieza y mantenimiento \| \| 1020 \| 2215 \| Loterías y sorteos \| \| 1020 \| 2218 \| Publicación e impresión variada \| \| 1020 \| 2219 \| Mudanza y almacenamiento \| \| 1020 \| 2235 \| Oftalmología \| \| 1020 \| 2212 \| Paisajismo y horticultura \| \| 1020 \| 2202 \| Planificación de eventos y bodas \| \| 1020 \| 2185 \| Publicidad \| \| 1020 \| 2199 \| Reparación de dispositivos pequeños y eléctricos \| \| 1020 \| 2228 \| Reparaciones de muebles y tapicería \| \| 1020 \| 2236 \| Reparaciones de relojes y joyas \| \| 1020 \| 2210 \| Seguros - auto y casa \| \| 1020 \| 2211 \| Seguros - vida y anualidad \| \| 1020 \| 2214 \| Servicio de entrega local \| \| 1020 \| 2233 \| Servicio de telecomunicación \| \| 1020 \| 2229 \| Servicios (no clasificados en ningún otro lugar) \| \| 1020 \| 2198 \| Servicios de comercio electrónico \| \| 1020 \| 2195 \| Servicios de asesoramiento \| \| 1020 \| 2187 \| Servicios de carrera \| \| 1020 \| 2194 \| Servicios de consultoría \| \| 1020 \| 2189 \| Servicios de cuidado infantil \| \| 1020 \| 2224 \| Servicios de copia rápida y reproducción \| \| 1020 \| 2196 \| Servicios de entrega \| \| 1020 \| 2201 \| Servicios de arrendamiento y alquiler de equipos \| \| 1020 \| 2231 \| Servicios de piscina \| \| 1020 \| 2223 \| Servicios de protección y seguridad \| \| 1020 \| 2209 \| Servicios de recuperación de información \| \| 1020 \| 2193 \| Servicios de red informática \| \| 1020 \| 2186 \| Servicios y clubes de compras \| \| 1020 \| 2213 \| Servicios legales y abogados \| \| 1020 \| 2206 \| Spas de salud y belleza \| \| 1020 \| 2234 \| Servicios públicos \| \| 1010 \| 2122 \| Agentes y corredores de valores \| \| 1010 \| 2115 \| Agentes o corredores hipotecarios \| \| 1010 \| 2103 \| Bolsa de commodities y futuros \| \| 1010 \| 2118 \| Tarjetas de valores prepagadas y almacenadas \| \| 1010 \| 2109 \| Compañía financiera \| \| 1010 \| 2110 \| Consultoría financiera y de inversiones \| \| 1010 \| 2101 \| Contabilidad \| \| 1010 \| 2106 \| Cooperativa de crédito \| \| 1010 \| 2119 \| Agente de bienes raíces \| \| 1010 \| 2107 \| Corredores y cambio \| \| 1010 \| 2108 \| Custodia de valores \| \| 1010 \| 2114 \| Empresa de servicios financieros \| \| 1010 \| 2104 \| Empresas de información crediticia del consumidor \| \| 1010 \| 2120 \| Entrega \| \| 1010 \| 2102 \| Oficina de cobranza \| \| 1010 \| 2121 \| Gestión de alquiler de bienes raíces \| \| 1010 \| 2113 \| Inversiones - general \| \| 1010 \| 2116 \| Moneda de juego en línea \| \| 1010 \| 2117 \| Anticipo de pago o préstamo de nómina \| \| 1010 \| 2111 \| Seguros - auto y casa \| \| 1010 \| 2112 \| Seguros - vida y anualidad \| \| 1010 \| 2105 \| Servicio de orientación sobre deudas \| \| 1010 \| 2123 \| Transferencia electrónica y orden de pago \| \| 1019 \| 2181 \| Bienes duraderos \| \| 1019 \| 2182 \| Bienes no duraderos \| \| 1019 \| 2179 \| Tienda departamental \| \| 1019 \| 2180 \| Tienda de descuentos \| \| 1019 \| 2183 \| Tienda de usados y segunda mano \| \| 1019 \| 2184 \| Tienda de variedades \| \| 1019 \| 2178 \| Productos químicos y similares \| \| 1011 \| 2124 \| Bebidas alcohólicas \| \| 1011 \| 2126 \| Café y té \| \| 1011 \| 2127 \| Alimentos gourmet \| \| 1011 \| 2130 \| Tabaco \| \| 1011 \| 2128 \| Tiendas de alimentos variados y especialidades \| \| 1011 \| 2129 \| Restaurante \| \| 1011 \| 2125 \| Servicios de buffé \| \| 1011 \| 2131 \| Vitaminas y suplementos \| \| 1024 \| 2277 \| Aviación \| \| 1024 \| 2283 \| Artículos vintage y de coleccionistas \| \| 1024 \| 2275 \| Vendedor de autos - nuevos y usados \| \| 1024 \| 2276 \| Vendedor de autos - solo usados \| \| 1024 \| 2278 \| Vendedor de barcos \| \| 1024 \| 2280 \| Vendedor de motocicletas \| \| 1024 \| 2279 \| Vendedor de remolques \| \| 1024 \| 2281 \| Vendedor de remolques utilitarios y recreativos \| \| 1024 \| 2282 \| Vendedor de vehículos recreativos \| \| 1005 \| 2053 \| Accesorios \| \| 1005 \| 2055 \| Mayorista - piedras y metales preciosos \| \| 1005 \| 2056 \| Joyería de moda \| \| 1005 \| 2050 \| Ropa para mujer \| \| 1005 \| 2048 \| Ropa infantil \| \| 1005 \| 2049 \| Ropa para hombre \| \| 1005 \| 2051 \| Zapatos \| \| 1005 \| 2052 \| Uniformes militares y civiles \| \| 1005 \| 2054 \| Venta minorista - joyería fina y relojes \| \| 1023 \| 2268 \| Campamento de deportes y recreación \| \| 1023 \| 2274 \| Agencia de viajes \| \| 1023 \| 2264 \| Cruceros \| \| 1023 \| 2265 \| Hospedaje y alojamiento \| \| 1023 \| 2261 \| Línea aérea \| \| 1023 \| 2263 \| Línea de autobús \| \| 1023 \| 2262 \| Alquiler de autos \| \| 1023 \| 2266 \| Maletas y mercancía de cuero \| \| 1023 \| 2272 \| Parques o campamentos de remolques \| \| 1023 \| 2270 \| Propiedades de tiempo compartido recreativo \| \| 1023 \| 2267 \| Servicios de recreación \| \| 1023 \| 2273 \| Servicios de transporte - otros \| \| 1023 \| 2269 \| Taxis y limusinas \| \| 1023 \| 2271 \| Viajes \| Valores: `null`, `2173`, `2171`, `2172`, `2174`, `2000`, `2004`, `2007`, `2003`, `2005`, `2006`, `2002`, `2009`, `2008`, `2001`, `2010`, `2012`, `2013`, `2014`, `2011`, `2015`, `2016`, `2017`, `2252`, `2253`, `2255`, `2256`, `2254`, `2258`, `2257`, `2260`, `2259`, `2145`, `2146`, `2148`, `2150`, `2157`, `2162`, `2152`, `2147`, `2149`, `2156`, `2153`, `2160`, `2163`, `2151`, `2154`, `2161`, `2159`, `2158`, `2155`, `2057`, `2059`, `2058`, `2069`, `2064`, `2062`, `2066`, `2065`, `2063`, `2060`, `2061`, `2068`, `2067`, `2027`, `2047`, `2031`, `2025`, `2043`, `2045`, `2030`, `2038`, `2039`, `2040`, `2041`, `2029`, `2026`, `2028`, `2032`, `2044`, `2046`, `2034`, `2036`, `2035`, `2033`, `2042`, `2037`, `2073`, `2070`, `2074`, `2075`, `2072`, `2071`, `2079`, `2085`, `2076`, `2081`, `2083`, `2082`, `2084`, `2077`, `2078`, `2080`, `2097`, `2094`, `2095`, `2096`, `2091`, `2088`, `2087`, `2093`, `2092`, `2098`, `2099`, `2086`, `2089`, `2090`, `2100`, `2241`, `2244`, `2248`, `2245`, `2237`, `2240`, `2246`, `2243`, `2250`, `2242`, `2247`, `2249`, `2238`, `2251`, `2239`, `2137`, `2019`, `2023`, `2021`, `2020`, `2018`, `2024`, `2022`, `2134`, `2132`, `2133`, `2136`, `2135`, `2176`, `2177`, `2175`, `2141`, `2140`, `2142`, `2138`, `2139`, `2143`, `2144`, `2169`, `2164`, `2167`, `2168`, `2165`, `2166`, `2297`, `2296`, `2293`, `2291`, `2294`, `2292`, `2288`, `2284`, `2285`, `2287`, `2295`, `2289`, `2290`, `2286`, `2232`, `2216`, `2188`, `2217`, `2192`, `2225`, `2226`, `2197`, `2205`, `2230`, `2204`, `2220`, `2200`, `2222`, `2221`, `2191`, `2227`, `2207`, `2208`, `2203`, `2190`, `2215`, `2218`, `2219`, `2235`, `2212`, `2202`, `2185`, `2199`, `2228`, `2236`, `2210`, `2211`, `2214`, `2233`, `2229`, `2198`, `2195`, `2187`, `2194`, `2189`, `2224`, `2196`, `2201`, `2231`, `2223`, `2209`, `2193`, `2186`, `2213`, `2206`, `2234`, `2122`, `2115`, `2103`, `2118`, `2109`, `2110`, `2101`, `2106`, `2119`, `2107`, `2108`, `2114`, `2104`, `2120`, `2102`, `2121`, `2113`, `2116`, `2117`, `2111`, `2112`, `2105`, `2123`, `2181`, `2182`, `2179`, `2180`, `2183`, `2184`, `2178`, `2124`, `2126`, `2127`, `2130`, `2128`, `2129`, `2125`, `2131`, `2277`, `2283`, `2275`, `2276`, `2278`, `2280`, `2279`, `2281`, `2282`, `2053`, `2055`, `2056`, `2050`, `2048`, `2049`, `2051`, `2052`, `2054`, `2268`, `2274`, `2264`, `2265`, `2261`, `2263`, `2262`, `2266`, `2272`, `2270`, `2267`, `2273`, `2269`, `2271`. |
| `business_website` | `string` | no | Sitio web |
| `business_name` | `string` | no | Nombre Comercial |
| `business_type` | `string` | no | Tipo de Empresa: * `mei` Microempresario Individual * `individual` Microempresa/Empresa de pequeño porte * `proprietorship` Sociedad Empresarial Limitada * `partnership` Sociedad Anónima de Capital Cerrado * `corporation` Sociedad Anónima de Capital Abierto * `nonprofit` Sin fines de lucro * `government` Gubernamental Valores: `null`, `mei`, `individual`, `proprietorship`, `partnership`, `corporation`, `nonprofit`, `government`. |
| `mother_name` | `string` | no | Nombre de la Madre |
| `father_name` | `string` | no | Nombre del Padre |
| `configuration` | `string (json)` | no | Configuración de datos predeterminados para boleto |
| `api_access_token` | `string` | no | Token de acceso de la subcuenta |
| `created_via_api` | `boolean` | no | Indica si el registro fue creado por la API |
| `billing_email` | `string` | no | Correo de facturación |
| `simple_opting` | `boolean` | no | Optante por el sistema simple |
| `rate_limits` | `object` | no | Límites de solicitudes - Hash con clave y valor en formato JSON. |

Otros campos y objetos anidados: `kobana schema banking.admin.subaccounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Subcuenta alterada |
| `404` | Subcuenta no encontrado |
| `422` | Parámetros inválidos |

## admin users list

Listar Usuarios

```text
kobana banking admin users list [OPCIONES]
```

`GET /v2/admin/users`

Lista todos los usuarios registrados en la cuenta, con opción de filtrar por email. Soporta paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `email` | `string` | no | Filtro por email. Busca usuario por email |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Usuarios encontrados |

## admin users create

Incluir un Usuario

```text
kobana banking admin users create [OPCIONES]
```

`POST /v2/admin/users`

Incluir un usuario

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `id` | `integer` | no | ID del usuario (deprecated, use uid) |
| `uid` | `string` | no | UID público del usuario |
| `intercom_id` | `string` | no | ID del usuario usado para identificación en Intercom |
| `email` | `string (email)` | no | Correo electrónico del usuario |
| `email_verified` | `boolean` | no | Indica si el correo electrónico del usuario ya fue confirmado (alcance email) |
| `phone` | `string` | no | Teléfono registrado (alcance teléfono) |
| `first_name` | `string` | no | Primer nombre |
| `middle_name` | `string` | no | Nombre del medio |
| `last_name` | `string` | no | Último nombre |
| `full_name` | `string` | no | Nombre completo |
| `created_via_api` | `boolean` | no | Indica si fue creado mediante API |
| `permissions` | `array<string>` | no | Permisos del usuario |
| `created_at` | `string (date-time)` | no | Fecha de creación |
| `updated_at` | `string (date-time)` | no | Fecha de actualización |
| `enabled_at` | `string (date-time)` | no | Fecha de habilitación |
| `legal_data` | `object` | no | Datos legales (nombre civil y CPF) |
| `profile_photo_url` | `string (uri)` | no | URL de la foto de perfil (ActiveStorage) o Gravatar como alternativa |
| `gender` | `string` | no | Identidad de género (undisclosed, woman, man, non_binary, other_identity o valor libre — alcance perfil.identidad) |
| `birthday` | `string (date)` | no | Fecha de nacimiento |
| `languages` | `array<string>` | no | Idiomas preferidos de comunicación del usuario (alcance perfil.comunicación) |
| `email_unsubscribed_at` | `string (date-time)` | no | Fecha en que el usuario canceló todas las comunicaciones por correo electrónico (opt-out general), null si nunca canceló (alcance profile.communication) |
| `timezone` | `string` | no | Zona horaria del usuario (alcance perfil.preferencias) |
| `accessibility_resources` | `array<string>` | no | Recursos de accesibilidad utilizados en la plataforma (alcance perfil.accesibilidad) |
| `disability_disclosure` | `array<string>` | no | Deficiencias informadas por el usuario, opcional (alcance perfil.accesibilidad) |

Otros campos y objetos anidados: `kobana schema banking.admin.users.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Usuario incluido |
| `422` | Datos del usuario inválido |

## admin users update

Alterar Datos De Un Usuario

```text
kobana banking admin users update [UID] [OPCIONES]
```

`PUT /v2/admin/users/{uid}`

Actualiza la información de un usuario existente, incluyendo nombre, email y permisos.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `id` | `integer` | no | ID del usuario (deprecated, use uid) |
| `uid` | `string` | no | UID público del usuario |
| `intercom_id` | `string` | no | ID del usuario usado para identificación en Intercom |
| `email` | `string (email)` | no | Correo electrónico del usuario |
| `email_verified` | `boolean` | no | Indica si el correo electrónico del usuario ya fue confirmado (alcance email) |
| `phone` | `string` | no | Teléfono registrado (alcance teléfono) |
| `first_name` | `string` | no | Primer nombre |
| `middle_name` | `string` | no | Nombre del medio |
| `last_name` | `string` | no | Último nombre |
| `full_name` | `string` | no | Nombre completo |
| `created_via_api` | `boolean` | no | Indica si fue creado mediante API |
| `permissions` | `array<string>` | no | Permisos del usuario |
| `created_at` | `string (date-time)` | no | Fecha de creación |
| `updated_at` | `string (date-time)` | no | Fecha de actualización |
| `enabled_at` | `string (date-time)` | no | Fecha de habilitación |
| `legal_data` | `object` | no | Datos legales (nombre civil y CPF) |
| `profile_photo_url` | `string (uri)` | no | URL de la foto de perfil (ActiveStorage) o Gravatar como alternativa |
| `gender` | `string` | no | Identidad de género (undisclosed, woman, man, non_binary, other_identity o valor libre — alcance perfil.identidad) |
| `birthday` | `string (date)` | no | Fecha de nacimiento |
| `languages` | `array<string>` | no | Idiomas preferidos de comunicación del usuario (alcance perfil.comunicación) |
| `email_unsubscribed_at` | `string (date-time)` | no | Fecha en que el usuario canceló todas las comunicaciones por correo electrónico (opt-out general), null si nunca canceló (alcance profile.communication) |
| `timezone` | `string` | no | Zona horaria del usuario (alcance perfil.preferencias) |
| `accessibility_resources` | `array<string>` | no | Recursos de accesibilidad utilizados en la plataforma (alcance perfil.accesibilidad) |
| `disability_disclosure` | `array<string>` | no | Deficiencias informadas por el usuario, opcional (alcance perfil.accesibilidad) |

Otros campos y objetos anidados: `kobana schema banking.admin.users.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Usuario Alterado |
| `404` | Usuario no encontrado |
| `422` | Parámetros inválidos |

## admin users delete

Excluir un Usuario

```text
kobana banking admin users delete [UID] [OPCIONES]
```

`DELETE /v2/admin/users/{uid}`

Elimina permanentemente un usuario del sistema.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Usuario eliminado |
| `404` | Usuario no encontrado |
