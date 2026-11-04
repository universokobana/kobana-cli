---
id: me
title: "kobana kadu me"
sidebar_label: "me"
description: "Identity behind the token"
---

# kobana kadu me

Identity behind the token

Commands of `kobana kadu me`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identity behind the token |

## me

Identity behind the token

```text
kobana kadu me [OPTIONS]
```

`GET /v1/me`

Returns the person, the organization and the token (type, scopes, access level, expiration) of the Bearer credential. Use it to test whether the configured token is valid: it is the only route exempt from the scope check, so any valid token gets a response. `user` is the person in `sub` when the token carries an email, and null for a machine token. It never returns the token itself.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
