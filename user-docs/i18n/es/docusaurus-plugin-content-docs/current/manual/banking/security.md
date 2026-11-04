---
id: security
title: "kobana banking security"
sidebar_label: "security"
description: "Seguridad (tokens de acceso)"
---

# kobana banking security

Seguridad (tokens de acceso)

Comandos de `kobana banking security`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`security access-tokens create`](#security-access-tokens-create) | `POST /v2/security/access_tokens` | Crear un Token de Acceso |
| [`security access-tokens update`](#security-access-tokens-update) | `PUT /v2/security/access_tokens/{uid}` | Actualizar Token de Acceso |
| [`security access-tokens delete`](#security-access-tokens-delete) | `DELETE /v2/security/access_tokens/{uid}` | Eliminar Token de Acceso |
| [`security access-tokens disable`](#security-access-tokens-disable) | `PUT /v2/security/access_tokens/{uid}/disable` | Deshabilitar Token de Acceso |
| [`security access-tokens enable`](#security-access-tokens-enable) | `PUT /v2/security/access_tokens/{uid}/enable` | Habilitar Token de Acceso |
| [`security access-tokens renew`](#security-access-tokens-renew) | `PUT /v2/security/access_tokens/{uid}/renew` | Renovar Token de Acceso |
| [`security access-tokens revoke`](#security-access-tokens-revoke) | `PUT /v2/security/access_tokens/{uid}/revoke` | Revocar Token de Acceso |

## security access-tokens create

Crear un Token de Acceso

```text
kobana banking security access-tokens create [OPCIONES]
```

`POST /v2/security/access_tokens`

Crea un nuevo token de acceso (access token) para autenticación en la API. El token creado puede tener alcances limitados y fecha de vencimiento.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `expires_at` | `string (date)` | sí | Fecha de vencimiento (máximo 1 año). Obligatorio cuando access_type = write |
| `reminder_emails` | `array<object>` | sí | Correos electrónicos para notificaciones de vencimiento. Obligatorio cuando expires_at está definido. |
| `external_id` | `string` | no | ID externo del token |
| `description` | `string` | no | Descripción del token |
| `scope_type` | `string` | no | Tipo de ámbito: Todos (all) o Limitado (limited) Valores: `all`, `limited`. |
| `limited_scopes` | `array<string>` | no | Ámbitos disponibles (cuando scope_type = limited). Seleccione uno o más valores: \| Escopo \| Descripción \| \|--------\|-----------\| \| `admin.subaccounts` \| Administrar subcuentas de la cuenta principal \| \| `admin.users` \| Administrar usuarios y permisos \| \| `automation.email_accounts` \| Administrar cuentas de correo electrónico para automatización \| \| `automation.email_deliveries` \| Ver y administrar entregas de correo electrónico \| \| `automation.sms_accounts` \| Administrar cuentas de SMS \| \| `automation.sms_deliveries` \| Ver y administrar entregas de SMS \| \| `automation.webhook_deliveries` \| Ver y administrar entregas de webhooks \| \| `automation.webhooks` \| Administrar webhooks y notificaciones automatizadas \| \| `billing.transactions` \| Ver transacciones de cobranza y facturación \| \| `charge.automatic_pix.locations` \| Administrar ubicaciones de Pix automático \| \| `charge.automatic_pix.pix` \| Administrar cobros Pix automáticos \| \| `charge.automatic_pix.recurrences` \| Administrar recurrencias de Pix automático \| \| `charge.automatic_pix.requests` \| Administrar solicitudes de Pix automático \| \| `charge.bank_billet_accounts` \| Administrar cuentas de boletos bancarios \| \| `charge.bank_billet_payments` \| Ver y registrar pagos de boletos \| \| `charge.bank_billet_registrations` \| Administrar registros de boletos en los bancos \| \| `charge.bank_billets` \| Administrar boletos bancarios \| \| `charge.customer_subscriptions` \| Administrar suscripciones de clientes \| \| `charge.installments` \| Administrar cuotas \| \| `charge.payments` \| Ver y administrar pagos recibidos \| \| `charge.pix` \| Administrar cobros Pix \| \| `charge.pix_accounts` \| Administrar cuentas Pix \| \| `core.providers` \| Ver proveedores disponibles \| \| `crm.customers` \| Administrar clientes e información comercial \| \| `crm.people` \| Administrar personas y contactos \| \| `data.bank_billet_queries` \| Consultar información de boletos bancarios \| \| `financial.accounts` \| Administrar cuentas financieras \| \| `financial.balances` \| Ver saldos y movimientos financieros \| \| `financial.providers` \| Administrar proveedores financieros e integraciones bancarias \| \| `financial.statement_transactions` \| Ver transacciones y estados de cuenta financieros \| \| `integration.certificates` \| Administrar certificados digitales para integraciones \| \| `integration.connections` \| Administrar conexiones con bancos y proveedores \| \| `integration.discharges` \| Administrar archivos de retorno bancario (bajas) \| \| `integration.edi_boxes` \| Administrar casillas EDI para intercambio de archivos \| \| `integration.remittances` \| Administrar archivos de remesa bancaria \| \| `mailbox.entries` \| Administrar buzones para recepción de archivos \| \| `mailbox.files` \| Ver y administrar archivos en buzones \| \| `partner.bank_contracts` \| Administrar contratos bancarios con socios \| \| `payment.accounts` \| Administrar cuentas de pago (incluida habilitación de remesa EDI) \| \| `payment.bank_billets` \| Realizar pagos de boletos bancarios \| \| `payment.batches` \| Administrar lotes de pagos \| \| `payment.dda_accounts` \| Administrar cuentas DDA \| \| `payment.dda.bank_billets` \| Administrar boletos DDA \| \| `payment.darfs` \| Realizar pagos de DARF (impuestos federales) \| \| `payment.payments` \| Administrar todos los tipos de pagos a través de la API \| \| `payment.pix` \| Realizar pagos vía Pix \| \| `payment.taxes` \| Realizar pagos de impuestos y tasas \| \| `payment.utilities` \| Realizar pagos de cuentas de consumo (agua, electricidad, etc) \| \| `security.access_tokens` \| Administrar tokens de acceso y autenticación \| \| `system.events` \| Ver eventos y registros del sistema \| \| `system.imports` \| Administrar importaciones de datos \| \| `system.reports` \| Generar y ver reportes del sistema \| \| `transfer.accounts` \| Administrar cuentas de transferencia (incluida habilitación de remesa EDI) \| \| `transfer.batches` \| Administrar lotes de transferencias \| \| `transfer.internal` \| Realizar transferencias internas entre cuentas \| \| `transfer.pix` \| Realizar transferencias vía Pix \| \| `transfer.ted` \| Realizar transferencias TED \| \| `transfer.transfers` \| Administrar todos los tipos de transferencias (Pix, TED, Internas) vía punto de conexión unificado \| |
| `access_type` | `string` | no | Tipo de acceso: Lectura (read) o Lectura y escritura (write) Valores: `read`, `write`. |
| `tags` | `array<string>` | no | Etiquetas del token |
| `custom_data` | `object` | no | Datos personalizados |

Otros campos y objetos anidados: `kobana schema banking.security.access-tokens.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Token creado |
| `422` | Datos del token inválidos |

## security access-tokens update

Actualizar Token de Acceso

```text
kobana banking security access-tokens update [UID] [OPCIONES]
```

`PUT /v2/security/access_tokens/{uid}`

Actualiza la información de un token de acceso existente, incluida descripción, alcances, etiquetas y fecha de vencimiento. **Nota:** La fecha de vencimiento solo se puede establecer si aún no se ha establecido. Una vez establecida, no se puede cambiar.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `external_id` | `string` | no | ID externo del token |
| `description` | `string` | no | Descripción del token |
| `scope_type` | `string` | no | Tipo de ámbito (all o limited) Valores: `all`, `limited`. |
| `limited_scopes` | `array<string>` | no | Ámbitos disponibles (cuando scope_type = limited). Seleccione uno o más valores: \| Escopo \| Descripción \| \|--------\|-----------\| \| `admin.subaccounts` \| Administrar subcuentas de la cuenta principal \| \| `admin.users` \| Administrar usuarios y permisos \| \| `automation.email_accounts` \| Administrar cuentas de correo electrónico para automatización \| \| `automation.email_deliveries` \| Ver y administrar entregas de correo electrónico \| \| `automation.sms_accounts` \| Administrar cuentas de SMS \| \| `automation.sms_deliveries` \| Ver y administrar entregas de SMS \| \| `automation.webhook_deliveries` \| Ver y administrar entregas de webhooks \| \| `automation.webhooks` \| Administrar webhooks y notificaciones automatizadas \| \| `billing.transactions` \| Ver transacciones de cobranza y facturación \| \| `charge.automatic_pix.locations` \| Administrar ubicaciones de Pix automático \| \| `charge.automatic_pix.pix` \| Administrar cobros Pix automáticos \| \| `charge.automatic_pix.recurrences` \| Administrar recurrencias de Pix automático \| \| `charge.automatic_pix.requests` \| Administrar solicitudes de Pix automático \| \| `charge.bank_billet_accounts` \| Administrar cuentas de boletos bancarios \| \| `charge.bank_billet_payments` \| Ver y registrar pagos de boletos \| \| `charge.bank_billet_registrations` \| Administrar registros de boletos en los bancos \| \| `charge.bank_billets` \| Administrar boletos bancarios \| \| `charge.customer_subscriptions` \| Administrar suscripciones de clientes \| \| `charge.installments` \| Administrar cuotas \| \| `charge.payments` \| Ver y administrar pagos recibidos \| \| `charge.pix` \| Administrar cobros Pix \| \| `charge.pix_accounts` \| Administrar cuentas Pix \| \| `core.providers` \| Ver proveedores disponibles \| \| `crm.customers` \| Administrar clientes e información comercial \| \| `crm.people` \| Administrar personas y contactos \| \| `data.bank_billet_queries` \| Consultar información de boletos bancarios \| \| `financial.accounts` \| Administrar cuentas financieras \| \| `financial.balances` \| Ver saldos y movimientos financieros \| \| `financial.providers` \| Administrar proveedores financieros e integraciones bancarias \| \| `financial.statement_transactions` \| Ver transacciones y estados de cuenta financieros \| \| `integration.certificates` \| Administrar certificados digitales para integraciones \| \| `integration.connections` \| Administrar conexiones con bancos y proveedores \| \| `integration.discharges` \| Administrar archivos de retorno bancario (bajas) \| \| `integration.edi_boxes` \| Administrar casillas EDI para intercambio de archivos \| \| `integration.remittances` \| Administrar archivos de remesa bancaria \| \| `mailbox.entries` \| Administrar buzones para recepción de archivos \| \| `mailbox.files` \| Ver y administrar archivos en buzones \| \| `partner.bank_contracts` \| Administrar contratos bancarios con socios \| \| `payment.accounts` \| Administrar cuentas de pago (incluida habilitación de remesa EDI) \| \| `payment.bank_billets` \| Realizar pagos de boletos bancarios \| \| `payment.batches` \| Administrar lotes de pagos \| \| `payment.dda_accounts` \| Administrar cuentas DDA \| \| `payment.dda.bank_billets` \| Administrar boletos DDA \| \| `payment.darfs` \| Realizar pagos de DARF (impuestos federales) \| \| `payment.payments` \| Administrar todos los tipos de pagos a través de la API \| \| `payment.pix` \| Realizar pagos vía Pix \| \| `payment.taxes` \| Realizar pagos de impuestos y tasas \| \| `payment.utilities` \| Realizar pagos de cuentas de consumo (agua, electricidad, etc) \| \| `security.access_tokens` \| Administrar tokens de acceso y autenticación \| \| `system.events` \| Ver eventos y registros del sistema \| \| `system.imports` \| Administrar importaciones de datos \| \| `system.reports` \| Generar y ver reportes del sistema \| \| `transfer.accounts` \| Administrar cuentas de transferencia (incluida habilitación de remesa EDI) \| \| `transfer.batches` \| Administrar lotes de transferencias \| \| `transfer.internal` \| Realizar transferencias internas entre cuentas \| \| `transfer.pix` \| Realizar transferencias vía Pix \| \| `transfer.ted` \| Realizar transferencias TED \| \| `transfer.transfers` \| Administrar todos los tipos de transferencias (Pix, TED, Internas) vía punto de conexión unificado \| |
| `access_type` | `string` | no | Tipo de acceso Valores: `read`, `write`. |
| `expires_at` | `string (date)` | no | Fecha de vencimiento (máximo 1 año). **Atención:** Solo puede ser definida si aún no estaba definida. Una vez definida, no puede ser alterada nuevamente. |
| `tags` | `array<string>` | no | Etiquetas del token |
| `custom_data` | `object` | no | Datos personalizados |
| `reminder_emails` | `array<object>` | no | Correos electrónicos para notificaciones de vencimiento |

Otros campos y objetos anidados: `kobana schema banking.security.access-tokens.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Token actualizado |
| `404` | Token no encontrado |

## security access-tokens delete

Eliminar Token de Acceso

```text
kobana banking security access-tokens delete [UID] [OPCIONES]
```

`DELETE /v2/security/access_tokens/{uid}`

Elimina permanentemente un token de acceso del sistema. Solo se pueden eliminar tokens revocados o vencidos.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Token eliminado |
| `404` | Token no encontrado |

## security access-tokens disable

Deshabilitar Token de Acceso

```text
kobana banking security access-tokens disable [UID] [OPCIONES]
```

`PUT /v2/security/access_tokens/{uid}/disable`

Deshabilita un token de acceso habilitado, impidiendo temporalmente su uso para autenticación.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Token deshabilitado |

## security access-tokens enable

Habilitar Token de Acceso

```text
kobana banking security access-tokens enable [UID] [OPCIONES]
```

`PUT /v2/security/access_tokens/{uid}/enable`

Habilita un token de acceso deshabilitado, permitiendo su uso para autenticación.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Token habilitado |

## security access-tokens renew

Renovar Token de Acceso

```text
kobana banking security access-tokens renew [UID] [OPCIONES]
```

`PUT /v2/security/access_tokens/{uid}/renew`

Crea un nuevo token de acceso con la misma configuración del token actual y revoca el token antiguo. El nuevo token se devuelve en la respuesta.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Token renovado |

## security access-tokens revoke

Revocar Token de Acceso

```text
kobana banking security access-tokens revoke [UID] [OPCIONES]
```

`PUT /v2/security/access_tokens/{uid}/revoke`

Revoca permanentemente un token de acceso. Los tokens v2 deben estar deshabilitados antes de ser revocados. Los tokens v1 se pueden revocar directamente.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Token revocado |
