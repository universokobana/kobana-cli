---
id: quickstart
title: Primeros comandos
sidebar_label: Primeros comandos
sidebar_position: 3
description: Del inicio de sesión a la primera llamada en cinco minutos.
---

# Primeros comandos

## 1. Instala e inicia sesión

```bash
curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash
kobana billing auth login
```

Comprueba que funcionó con `me`, que muestra a quién pertenece el token (persona y organización), sus scopes y su vencimiento, sin exponer el token:

```bash
kobana billing me
```

Si falla, mira [Solución de problemas](/guides/troubleshooting).

## 2. Descubre qué hay

Toda la superficie aparece en `--help`, en cualquier nivel:

```bash
kobana --help                     # los productos
kobana billing --help             # los recursos de Facturación Automática
kobana billing invoices --help    # los comandos de facturas
```

Todo lo que acepta el CLI también está en el [Manual](/manual).

## 3. Lee datos

```bash
kobana billing subscriptions list --fields "id,status,plan_id"
kobana billing invoices get INVOICE_ID
```

La salida es JSON por defecto. Para una tabla: `--output-format table`. Consulta [Salida](/usage/output).

## 4. Filtra y pagina

```bash
kobana billing subscriptions list --params '{"status": "active"}'
kobana billing subscriptions list --page-all --fields "id,status"
```

`--page-all` recorre todas las páginas e imprime una línea JSON por registro. Consulta [Paginación](/usage/pagination).

## 5. Modifica con seguridad

Antes de escribir, mira la solicitud que se enviaría:

```bash
kobana billing invoices finalize INVOICE_ID --dry-run
```

Solo después quita el `--dry-run`. Consulta [Modificaciones seguras](/usage/safe-writes).

## 6. Consulta el schema

```bash
kobana schema billing.subscriptions.create
```

Muestra parámetros, campos obligatorios y respuestas, sin salir de la terminal. Consulta [Schema](/usage/schema).

## Próximos pasos

- [Sintaxis de los comandos](/usage/syntax)
- [Scripts y agentes de IA](/usage/scripting)
- [Códigos de salida](/reference/exit-codes)
