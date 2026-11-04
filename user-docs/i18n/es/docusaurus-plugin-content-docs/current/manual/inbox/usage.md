---
id: usage
title: "kobana inbox usage"
sidebar_label: "usage"
---

# kobana inbox usage


Comandos de `kobana inbox usage`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`usage cost`](#usage-cost) | `GET /v1/usage/cost` | Costo por modelo |
| [`usage summary`](#usage-summary) | `GET /v1/usage/summary` | Resumen de uso |

## usage cost

Costo por modelo

```text
kobana inbox usage cost [OPCIONES]
```

`GET /v1/usage/cost`

Costo y tokens por modelo LLM en el rango: totales por modelo y serie diaria.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `from` | `string` | no | Primer día UTC, `YYYY-MM-DD`. Por defecto, 29 días antes de `to`. |
| `to` | `string` | no | Último día UTC (inclusive), `YYYY-MM-DD`. Por defecto, hoy. `from`–`to` puede abarcar como máximo 366 días; un rango mayor es 400 `VALIDATION_ERROR`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |

## usage summary

Resumen de uso

```text
kobana inbox usage summary [OPCIONES]
```

`GET /v1/usage/summary`

Correos procesados, tokens y costo por día UTC en el rango (solo ejecuciones de agente exitosas), más los contadores mensuales acumulados frente a los límites del workspace. El costo está en USD, según lo factura el AI Gateway.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `from` | `string` | no | Primer día UTC, `YYYY-MM-DD`. Por defecto, 29 días antes de `to`. |
| `to` | `string` | no | Último día UTC (inclusive), `YYYY-MM-DD`. Por defecto, hoy. `from`–`to` puede abarcar como máximo 366 días; un rango mayor es 400 `VALIDATION_ERROR`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
