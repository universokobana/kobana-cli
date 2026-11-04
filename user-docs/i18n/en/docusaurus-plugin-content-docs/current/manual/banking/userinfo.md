---
id: userinfo
title: "kobana banking userinfo"
sidebar_label: "userinfo"
description: "User Information (discontinued)"
---

# kobana banking userinfo

User Information (discontinued)

Commands of `kobana banking userinfo`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`userinfo`](#userinfo) | `GET /v1/userinfo` | User Information (discontinued) |

## userinfo

User Information (discontinued)

```text
kobana banking userinfo [OPTIONS]
```

`GET /v1/userinfo`

Query information from the accessed account.

**Responses**

| HTTP | Description |
|---|---|
| `200` | User found |
| `404` | User not found |
