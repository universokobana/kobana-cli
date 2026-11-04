---
id: me
title: "kobana banking me"
sidebar_label: "me"
description: "Account information"
---

# kobana banking me

Account information

Commands of `kobana banking me`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`me`](#me) | `GET /v2/me` | Get authenticated user information |

## me

Get authenticated user information

```text
kobana banking me [OPTIONS]
```

`GET /v2/me`

Returns information about the authenticated user, including account, user and access token data.

**Responses**

| HTTP | Description |
|---|---|
| `200` | User information without user associated with the token |
