---
id: me
title: "kobana billing me"
sidebar_label: "me"
description: "Current token identity"
---

# kobana billing me

Current token identity

Commands of `kobana billing me`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Current token identity |

## me

Current token identity

```text
kobana billing me [OPTIONS]
```

`GET /v1/me`

Returns who the Bearer token belongs to (person and organization) plus the token’s own scopes and expiry. Use it to test whether the configured token is valid. It never returns the token or any secret. `user` is `null` when the token is not a person’s.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Token identity. |
| `404` | Resource not found in the current organization. |
