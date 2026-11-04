---
id: executions
title: "kobana kadu executions"
sidebar_label: "executions"
---

# kobana kadu executions


Comandos de `kobana kadu executions`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`executions search`](#executions-search) | `GET /v1/executions/search` | Búsqueda de texto en las ejecuciones |
| [`executions list`](#executions-list) | `GET /v1/executions` | Lista ejecuciones |
| [`executions get`](#executions-get) | `GET /v1/executions/{id}` | Detalla la ejecución |
| [`executions cancel`](#executions-cancel) | `POST /v1/executions/{id}/cancel` | Cancela la ejecución |
| [`executions consolidate-memories`](#executions-consolidate-memories) | `POST /v1/executions/{id}/consolidate_memories` | Consolida las memorias de la ejecución |
| [`executions continue`](#executions-continue) | `POST /v1/executions/{id}/continue` | Continúa la conversación (próxima ronda) |
| [`executions events`](#executions-events) | `GET /v1/executions/{id}/events` | Eventos de la ejecución |
| [`executions logs`](#executions-logs) | `GET /v1/executions/{id}/logs` | Logs de la ejecución |
| [`executions todos`](#executions-todos) | `GET /v1/executions/{id}/todos` | Pendientes (todos) de la ejecución |

## executions search

Búsqueda de texto en las ejecuciones

```text
kobana kadu executions search [OPCIONES]
```

`GET /v1/executions/search`

Busca en eventos, todos, logs, resultado y error. Hasta 30 ejecuciones, las más recientes primero, cada una con hasta 3 fragmentos. Solo entran las fuentes que quien llama podría leer directamente: los logs exigen la habilitación `executions_logs.view` de la organización, y en un token de persona el rol necesita `agent.logs.list`, `agent.events.list` y `agent.todos.list` para logs, eventos y todos.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `q` | `string` | sí |  |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — `q` con menos de 2 caracteres |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## executions list

Lista ejecuciones

```text
kobana kadu executions list [OPCIONES]
```

`GET /v1/executions`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Campo en snake_case: created_at (predeterminado), updated_at o un campo filtrable. |
| `sort_order` | `string` | no | Predeterminado: desc. Valores: `asc`, `desc`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## executions get

Detalla la ejecución

```text
kobana kadu executions get [ID] [OPCIONES]
```

`GET /v1/executions/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## executions cancel

Cancela la ejecución

```text
kobana kadu executions cancel [ID] [OPCIONES]
```

`POST /v1/executions/{id}/cancel`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cancelled (o el estado terminal actual, si ya terminó) |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## executions consolidate-memories

Consolida las memorias de la ejecución

```text
kobana kadu executions consolidate-memories [ID] [OPCIONES]
```

`POST /v1/executions/{id}/consolidate_memories`

Lee el resultado, los eventos, los pendientes y el pedido del usuario de esta ejecución terminada y guarda los hechos duraderos como memorias, en el alcance de la Task — incluso con memorias desactivadas en ella. La rehace si ya se consolidó (actualiza las mismas memorias). Es una llamada de modelo cobrada a la organización. Asíncrono.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | en cola |
| `404` | NOT_FOUND |
| `409` | CONFLICT — ejecución no terminada / IDEMPOTENCY_KEY_REUSED |
| `503` | SERVICE_UNAVAILABLE — cola no disponible |

## executions continue

Continúa la conversación (próxima ronda)

```text
kobana kadu executions continue [ID] [OPCIONES]
```

`POST /v1/executions/{id}/continue`

Solo para Task conversacional (`conversational`): crea una ejecución hija de esta que retoma la sesión del agente (contexto, archivos de salida y estado) y atiende el pedido en `message`. Usa los parámetros, la fecha de referencia y el entorno de esta ejecución, que debe estar terminada (`success` o `failed`). Continuar una ronda antigua abre una rama. La nueva ejecución trae `parentExecutionId` (esta) y `rootExecutionId` (la primera ronda).

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `message` | `string` | sí |  |
| `locale` | `string` | no |  |
| `input_file_ids` | `array<string (uuid)>` | no | Archivos nuevos para esta ronda (`POST /files`, aún no usados). |

Otros campos y objetos anidados: `kobana schema kadu.executions.continue`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | ronda creada (en la cola de la Task) |
| `404` | NOT_FOUND |
| `409` | NOT_CONVERSATIONAL / EXECUTOR_NOT_SUPPORTED / EXECUTOR_CHANGED / NOT_FINISHED / SESSION_UNAVAILABLE |
| `422` | ENVIRONMENT_REQUIRED / MISSING_SECRETS (details.keys) / INVALID_PARAMETERS / INVALID_INPUT_FILES / COST_LIMIT_REACHED |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## executions events

Eventos de la ejecución

```text
kobana kadu executions events [ID] [OPCIONES]
```

`GET /v1/executions/{id}/events`

Por `sequence`, hasta 500 por llamada. Para seguir, pasa en `after_sequence` el último `sequence` recibido y repite.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `after_sequence` | `integer,null` | no |  |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## executions logs

Logs de la ejecución

```text
kobana kadu executions logs [ID] [OPCIONES]
```

`GET /v1/executions/{id}/logs`

En orden de llegada, hasta 1000 por llamada. Para seguir una ejecución en curso, pasa en `after_id` el último `id` recibido y repite.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `after_id` | `integer,null` | no |  |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## executions todos

Pendientes (todos) de la ejecución

```text
kobana kadu executions todos [ID] [OPCIONES]
```

`GET /v1/executions/{id}/todos`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
