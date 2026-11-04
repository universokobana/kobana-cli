---
id: me
title: "kobana inbox me"
sidebar_label: "me"
description: "Identity behind the token"
---

# kobana inbox me

Identity behind the token

Commands of `kobana inbox me`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identity behind the token |

## me

Identity behind the token

```text
kobana inbox me [OPTIONS]
```

`GET /v1/me`

Who the Bearer token acts as: the person (`null` for a machine token), the organization and the token's own reach. Use it to test whether a configured token is valid. Requires no resource scope: any token that verifies is answered. The token itself is never returned.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Identity. |
