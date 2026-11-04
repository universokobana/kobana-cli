---
id: troubleshooting
title: Solución de problemas
sidebar_label: Solución de problemas
sidebar_position: 3
description: Los errores más comunes de kobana — autenticación, permisos, entorno, espacio de trabajo — y cómo resolverlos.
---

# Solución de problemas

Los errores de API, de autenticación y de validación salen como JSON por stdout, con `error.code` y `error.message`; el [código de salida](/reference/exit-codes) del proceso indica la categoría. Ejecuta con `--verbose` para ver la solicitud y la respuesta (sin el token).

## Autenticación (código de salida 2)

### `No authentication configured for <product>`

No hay token para ese producto. Cada producto tiene el suyo:

```bash
kobana billing auth login            # o
export KOBANA_BILLING_TOKEN=...
kobana billing auth status           # confirma qué está vigente
```

`auth status` termina con código `0` incluso sin credencial: revisa el campo `authenticated` de la salida. Un token de otro producto no sirve — consulta [Autenticación](/getting-started/authentication).

### `401` de la API

El token existe, pero está vencido, es de otro producto o de otro entorno. Los tokens de sandbox y de producción son distintos. Vuelve a iniciar sesión (`kobana <producto> auth login`) en el entorno correcto (`--env`).

### `403` de la API

El token es válido, pero no tiene permiso. Causas comunes:

- **Falta el nivel de acceso.** `DELETE` exige `delete`; la importación exige `import`; la escritura exige `write`. Vuelve a iniciar sesión eligiendo los niveles, o usa `--scopes`.
- **Falta el scope del recurso.** El inicio de sesión lista los scopes del producto (`billing/customers`, `inbox/emails`…); un recurso fuera de ellos es rechazado.

### El inicio de sesión abre el navegador en el lugar equivocado

El inicio de sesión usa la Cuenta Kobana del entorno (`account.kobana.com.br`, `account.sandbox.kobana.com.br`). `KOBANA_ACCOUNT_URL` cambia la dirección; verifica que no haya una variable antigua exportada.

## Entorno

### `<product> has no sandbox environment`

`inbox`, `dashboards` y `kadu` no tienen sandbox, y el CLI lo rechaza en lugar de enviar la prueba a producción. Ejecuta sin `--env sandbox`, con cuidado, o usa `--dry-run`.

## Espacio de trabajo

### `400 WORKSPACE_REQUIRED` en `finance`

Toda ruta de `finance` necesita `X-Workspace-Id`:

```bash
export KOBANA_WORKSPACE_ID=uuid-del-espacio
# o, en una sola llamada:
kobana finance payables list --params '{"X-Workspace-Id": "uuid"}'
```

`kobana dunning workspaces list` muestra los espacios a los que llega el token.

## Línea de comandos (código de salida 3)

### `unrecognized subcommand` o `unexpected argument`

El comando o la flag no existe. Mira qué hay en ese nivel:

```bash
kobana billing --help
kobana billing plans --help
```

Los nombres de los comandos cambiaron en versiones antiguas, pero los nombres anteriores siguen valiendo como alias. Si el comando "desapareció", búscalo en el [Manual](/manual).

### `Validation error: id cannot contain …`

El CLI rechaza identificadores con `/`, `\`, `..`, `?`, `#`, `%` o caracteres de control antes de llamar a la API. Es intencional: protege contra la inyección de rutas. Pasa solo el id.

### `invalid --json JSON` en `--params` o `--json`

Usa comillas simples por fuera y dobles por dentro (`--json '{"amount": 99.90}'`). En PowerShell, escapa las comillas dobles internas o usa un archivo.

## Variables y `.env`

### Un token en el `.env` fue ignorado

El `.env` de la carpeta actual no puede definir tokens ni credenciales, porque puede venir de un repositorio de terceros. El CLI lo avisa. Defínelos en el entorno o en el `.env` de la carpeta de configuración (`~/.config/kobana`). Consulta [Variables de entorno](/reference/environment-variables).

## Salida

### Respuesta enorme

Usa `--fields` para limitar las columnas y `--page-limit` para limitar las páginas. Para sumas, usa los endpoints `aggregates`.

### `--output` rechazado

La ruta no puede contener `..`, ser un enlace simbólico, una carpeta ni un dispositivo.

### El aviso "Nueva versión disponible"

Es un aviso por stderr, una vez al día. `kobana update` la instala. No afecta el stdout ni el código de salida.

## ¿Todavía no se resolvió?

Ejecuta el comando con `--verbose` y `KOBANA_LOG=kobana=debug`, y abre un [issue](https://github.com/universokobana/kobana-cli/issues) con el comando (sin tokens), la salida y `kobana --version`.
