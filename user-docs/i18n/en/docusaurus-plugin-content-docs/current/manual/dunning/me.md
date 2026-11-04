---
id: me
title: "kobana dunning me"
sidebar_label: "me"
description: "Token identity"
---

# kobana dunning me

Token identity

Commands of `kobana dunning me`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Token identity |

## me

Token identity

```text
kobana dunning me [OPTIONS]
```

`GET /v1/me`

Returns who owns the Bearer token sent. Use it to check that the configured token is valid. It never returns the token or any secret.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Token identity. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
