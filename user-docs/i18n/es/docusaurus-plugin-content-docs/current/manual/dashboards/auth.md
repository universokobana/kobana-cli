---
id: auth
title: "kobana dashboards auth"
sidebar_label: "auth"
description: "Inicia sesión, cierra sesión y muestra las credenciales de este producto"
---

# kobana dashboards auth

Inicia sesión, cierra sesión y muestra las credenciales de este producto

Comandos de `kobana dashboards auth`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`auth login`](#auth-login) |  | Inicia sesión con la Conta Kobana (OAuth + PKCE) |
| [`auth logout`](#auth-logout) |  | Elimina las credenciales guardadas |
| [`auth status`](#auth-status) |  | Muestra el estado de la autenticación |
| [`auth export`](#auth-export) |  | Exporta las credenciales en JSON |

## auth login

Inicia sesión con la Conta Kobana (OAuth + PKCE)

```text
kobana dashboards auth login [OPCIONES]
```

| Opciones | Descripción |
|---|---|
| `--client-id <ID>` | Client ID OAuth (predeterminado: kobana-cli) |
| `--client-secret <SECRET>` | Client secret OAuth: usa el flujo Client Credentials, solo en banking y con --client-id (también: KOBANA_CLIENT_SECRET) |
| `--scopes <ALCANCES>` | Alcances OAuth separados por comas, p. ej. billing/customers,write (predeterminado: los que publica el producto) |

## auth logout

Elimina las credenciales guardadas

```text
kobana dashboards auth logout [OPCIONES]
```

## auth status

Muestra el estado de la autenticación

```text
kobana dashboards auth status [OPCIONES]
```

## auth export

Exporta las credenciales en JSON

```text
kobana dashboards auth export [OPCIONES]
```
