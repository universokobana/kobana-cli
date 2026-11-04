---
id: authentication
title: Autenticación
sidebar_label: Autenticación
sidebar_position: 2
description: Inicia sesión en cada producto desde el navegador (OAuth + PKCE) o con un token en una variable de entorno.
---

# Autenticación

> **Un token vale para un solo producto:** los productos no comparten credenciales. Un token del Gateway Bancario no autentica en Facturación Automática, y viceversa. Autentícate en cada producto que vayas a usar.

Los tokens vienen de la **Cuenta Kobana** (`account.kobana.com.br`). El `aud` de cada uno es la URL de la API del producto, por eso el token de uno no sirve para otro.

## Qué método usar

| Situación | Usa |
|---|---|
| Tú, en tu terminal | [Inicio de sesión desde el navegador](#inicio-de-sesión-desde-el-navegador-oauth--pkce) |
| Un script o un agente de IA en tu máquina | El inicio de sesión ya hecho (las credenciales guardadas sirven) o `KOBANA_<PRODUCTO>_TOKEN` |
| Un servidor, contenedor o CI, sin navegador | [Credenciales exportadas](#servidores-y-ci) o `KOBANA_<PRODUCTO>_TOKEN` |
| Una aplicación, sin persona (solo en `banking`) | Client credentials: `--client-id` y `--client-secret` |

## Inicio de sesión desde el navegador (OAuth + PKCE)

```bash
kobana billing auth login
kobana kadu auth login
```

El inicio de sesión sigue la forma de todo comando, `kobana <producto> auth login`. El navegador se abre en la Cuenta Kobana, tú apruebas y el CLI guarda el token. Cuando vence, el CLI lo renueva solo con el refresh token.

El comando lista los **scopes que el producto publica** (leídos en el momento del inicio de sesión) y, a continuación, los niveles de acceso, que valen para todos los recursos elegidos:

| Nivel | Permite |
|---|---|
| `read` | Lectura |
| `write` | Creación y modificación |
| `delete` | Eliminación |
| `import` | Importación en volumen |
| `export` | Exportación en volumen |

Un nivel no implica otro: `DELETE` exige `delete`, e importar exige `import`.

```bash
# Scopes sin preguntar (fuera de una terminal, el valor por defecto es todos los recursos + read)
kobana billing auth login --scopes "billing/customers,billing/plans,read,write"

# Client credentials — solo en banking (token de aplicación, sin persona)
kobana banking auth login --client-id <ID> --client-secret <SECRET>

kobana billing auth status      # quién soy, hasta cuándo vale
kobana billing auth export      # imprime el JSON de las credenciales
kobana billing auth logout
```

Las credenciales se guardan una por producto y entorno (`~/.config/kobana/credentials-<producto>-<entorno>.enc`), cifradas con AES-256-GCM. La clave queda en el keyring del sistema (Keychain en macOS, por ejemplo), con un archivo como alternativa.

## Token por variable de entorno

Para scripts y CI, exporta el token del producto:

```bash
export KOBANA_FINANCE_TOKEN=...
kobana finance payables list
```

| Producto | Variable |
|---|---|
| `banking` | `KOBANA_BANKING_TOKEN` |
| `finance` | `KOBANA_FINANCE_TOKEN` |
| `billing` | `KOBANA_BILLING_TOKEN` |
| `inbox` | `KOBANA_INBOX_TOKEN` |
| `dunning` | `KOBANA_DUNNING_TOKEN` |
| `dashboards` | `KOBANA_DASHBOARDS_TOKEN` |
| `kadu` | `KOBANA_KADU_TOKEN` o `KADU_TOKEN` |

En `banking`, también sirve el token de API de *Integraciones › API › Token de API*, en la interfaz de Kobana.

## Servidores y CI

Donde no hay navegador, inicia sesión en una máquina que tenga uno y lleva las credenciales:

```bash
# En tu máquina
kobana billing auth login
kobana billing auth export > credentials.json

# En el servidor
export KOBANA_CREDENTIALS_FILE=/ruta/credentials.json
kobana billing invoices list
```

El archivo es lo que imprime `auth export`, necesita el campo `product` y vale solo para ese producto (y para el `environment` del archivo, si lo hay). Trátalo como un secreto: guárdalo en la bóveda del CI, no en el repositorio. Si el token vence, el CLI lo renueva con el refresh token del archivo.

Más simple aún, cuando ya tienes el token: guárdalo como secreto del CI y exporta `KOBANA_<PRODUCTO>_TOKEN` en el job.

```yaml
# GitHub Actions
- run: kobana billing invoices list --page-all --fields "id,status"
  env:
    KOBANA_BILLING_TOKEN: ${{ secrets.KOBANA_BILLING_TOKEN }}
```

## Orden de resolución

| Prioridad | Fuente | Configuración |
|---|---|---|
| 1 | Token del producto | `KOBANA_<PRODUCTO>_TOKEN` |
| 2 | Archivo de credenciales | `KOBANA_CREDENTIALS_FILE` |
| 3 | Credenciales guardadas | `kobana <producto> auth login` |

## Espacio de trabajo (`X-Workspace-Id`)

En `finance`, toda ruta actúa sobre **un** espacio de trabajo, elegido con el encabezado `X-Workspace-Id`; sin él la API responde `400 WORKSPACE_REQUIRED`. En `dashboards` y `dunning` el encabezado es opcional mientras la organización tenga un único workspace.

El CLI completa el encabezado con `KOBANA_WORKSPACE_ID`. Para cambiar de espacio en una sola llamada, pásalo en `--params`, que tiene precedencia:

```bash
export KOBANA_WORKSPACE_ID=uuid-del-espacio
kobana finance payables list
kobana finance payables list --params '{"X-Workspace-Id": "otro-uuid"}'
```

## Entornos

El valor por defecto es **producción**. Cámbialo con `--env` o `KOBANA_ENVIRONMENT`:

```bash
kobana banking charge pix list --env sandbox
export KOBANA_ENVIRONMENT=sandbox
kobana billing auth login --env sandbox
```

Los tokens son distintos en cada entorno. Los hosts de cada producto están en [Productos](/reference/products). `--env sandbox` en un producto sin sandbox termina con error, para que nunca se envíe una prueba a producción.
