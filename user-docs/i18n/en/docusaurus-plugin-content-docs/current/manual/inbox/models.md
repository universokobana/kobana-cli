---
id: models
title: "kobana inbox models"
sidebar_label: "models"
description: "List available LLM models"
---

# kobana inbox models

List available LLM models

Commands of `kobana inbox models`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`models`](#models) | `GET /v1/models` | List available LLM models |

## models

List available LLM models

```text
kobana inbox models [OPTIONS]
```

`GET /v1/models`

Models accepted in `Agent.llm_model`, with per-token prices: the AI Gateway catalog narrowed to models that have a price, so every listed model is costed and counted against the workspace limit.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Catalog. |
| `502` | `GATEWAY_ERROR`: the AI Gateway catalog could not be fetched. |
