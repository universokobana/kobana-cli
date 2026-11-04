---
id: bank-billet-batch-exports
title: "kobana banking bank-billet-batch-exports"
sidebar_label: "bank-billet-batch-exports"
---

# kobana banking bank-billet-batch-exports


Comandos de `kobana banking bank-billet-batch-exports`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`bank-billet-batch-exports get`](#bank-billet-batch-exports-get) | `GET /v1/bank_billet_batch_exports/{id}` | Archivo de Exportación del Lote |

## bank-billet-batch-exports get

Archivo de Exportación del Lote

```text
kobana banking bank-billet-batch-exports get [ID] [OPCIONES]
```

`GET /v1/bank_billet_batch_exports/{id}`

Archivo de exportación del lote.

Si el archivo aún no ha sido procesado la respuesta será 404. Si el archivo ya está listo para descargar la respuesta será una redirección para descargar el archivo.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `302` | Exportación de lote generada con éxito |
| `404` | Exportación de lote no procesada |
