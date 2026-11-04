---
id: environment-variables
title: Variables de entorno
sidebar_label: Variables de entorno
sidebar_position: 2
description: Todas las variables que lee kobana — credenciales, configuración y logs — y las reglas del archivo .env.
---

# Variables de entorno

## Autenticación

| Variable | Descripción |
|---|---|
| `KOBANA_<PRODUTO>_TOKEN` | Token (Bearer) de un producto: `KOBANA_BANKING_TOKEN`, `KOBANA_FINANCE_TOKEN`… Tiene la mayor prioridad. Kadu también lee `KADU_TOKEN` |
| `KOBANA_CREDENTIALS_FILE` | Ruta de un JSON de credenciales (el formato de `auth export`). Necesita el campo `product`; vale solo para ese producto y para el `environment` del archivo (sin él, cualquier entorno) |
| `KOBANA_CLIENT_ID` | Client ID del inicio de sesión (por defecto: `kobana-cli`) |
| `KOBANA_CLIENT_SECRET` | Client secret. Igual que `--client-secret`, elige el flujo Client Credentials (solo en `banking`); el inicio de sesión de una persona (PKCE) no usa secret |
| `KOBANA_ACCOUNT_URL` | Dirección de la Cuenta Kobana usada en el inicio de sesión (por defecto: la del entorno) |
| `KOBANA_WORKSPACE_ID` | Espacio de trabajo enviado en `X-Workspace-Id` por `finance` (obligatorio), `dashboards` y `dunning`. Un valor en `--params` gana |

## Configuración

| Variable | Descripción |
|---|---|
| `KOBANA_CONFIG_DIR` | Carpeta de configuración (por defecto: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `production` (por defecto), `sandbox` o `development`; cualquier otro valor es un error |
| `KOBANA_LANG` | `pt`, `en` o `es`. `--lang` gana |

## Logs

| Variable | Descripción |
|---|---|
| `KOBANA_LOG` | Nivel de log en el stderr, por ejemplo `kobana=debug`. Desactivado por defecto |
| `KOBANA_LOG_FILE` | Carpeta para logs JSON con rotación diaria. Desactivado por defecto |

## Archivo `.env`

Cualquier variable también puede vivir en un `.env`: el de la carpeta actual (nunca el de las carpetas superiores) y el de la carpeta de configuración. Una variable ya definida en el entorno gana sobre los dos.

> **El `.env` de la carpeta actual no define credenciales.** Puede venir de un repositorio de terceros, que podría enviar el inicio de sesión a otro servidor, guardar credenciales dentro de él o hacer que el CLI actúe con el token de otra persona. Por eso `KOBANA_ACCOUNT_URL`, `KOBANA_CONFIG_DIR`, `KOBANA_CREDENTIALS_FILE`, `KOBANA_CLIENT_ID`, `KOBANA_CLIENT_SECRET` y todos los tokens (`KOBANA_*_TOKEN`, `KADU_TOKEN`) se ignoran en él, y el CLI avisa. Define esas variables en el entorno o en el `.env` de la carpeta de configuración.
