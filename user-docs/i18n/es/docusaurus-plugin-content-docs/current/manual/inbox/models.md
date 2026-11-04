---
id: models
title: "kobana inbox models"
sidebar_label: "models"
description: "Listar modelos LLM disponibles"
---

# kobana inbox models

Listar modelos LLM disponibles

Comandos de `kobana inbox models`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`models`](#models) | `GET /v1/models` | Listar modelos LLM disponibles |

## models

Listar modelos LLM disponibles

```text
kobana inbox models [OPCIONES]
```

`GET /v1/models`

Modelos aceptados en `Agent.llm_model`, con precio por token: el catálogo del AI Gateway limitado a los modelos que tienen precio, de modo que todo modelo listado se costea y se cuenta contra el límite del workspace.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Catálogo. |
| `502` | `GATEWAY_ERROR`: no se pudo obtener el catálogo del AI Gateway. |
