---
id: me
title: "kobana finance me"
sidebar_label: "me"
description: "Get the identity behind the token"
---

# kobana finance me

Get the identity behind the token

Commands of `kobana finance me`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Get the identity behind the token |

## me

Get the identity behind the token

```text
kobana finance me [OPTIONS]
```

`GET /v1/me`

Returns the person, organization and token details behind the Bearer token, so an integrator can test that the configured token is valid. Any valid token works: no scope and no `X-Workspace-Id` header are needed. Never returns the token itself.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The identity behind the token. |
