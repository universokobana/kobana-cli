---
id: users
title: "kobana kadu users"
sidebar_label: "users"
description: "Organization members (up to 100)"
---

# kobana kadu users

Organization members (up to 100)

Commands of `kobana kadu users`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`users`](#users) | `GET /v1/users` | Organization members (up to 100) |

## users

Organization members (up to 100)

```text
kobana kadu users [OPTIONS]
```

`GET /v1/users`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
