---
id: reports
title: "kobana banking reports"
sidebar_label: "reports"
---

# kobana banking reports


Comandos de `kobana banking reports`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`reports bank-billets`](#reports-bank-billets) | `GET /v1/reports/bank_billets` | Cantidad de Boletos. |

## reports bank-billets

Cantidad de Boletos.

```text
kobana banking reports bank-billets [OPCIONES]
```

`GET /v1/reports/bank_billets`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `expire_from` | `string` | no | A partir de la Fecha de vencimiento. |
| `expire_to` | `string` | no | Hasta la Fecha de vencimiento. |
| `paid_from` | `string` | no | A partir de la Fecha de pago. |
| `paid_to` | `string` | no | Hasta la Fecha de pago. |
| `registered_from` | `string` | no | A partir de la Fecha de registro. |
| `registered_to` | `string` | no | Hasta la Fecha de registro. |
| `created_from` | `string` | no | A partir de la Fecha de creación. |
| `created_to` | `string` | no | Hasta la Fecha de creación. |
| `status` | `string` | no | Estado del boleto. Valores: `generating`, `opened`, `paid`, `overdue`, `canceled`, `draft`, `generation_failed`, `validation_failed`, `blocked`, `chargeback`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de boletos |
