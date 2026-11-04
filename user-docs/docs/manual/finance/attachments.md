---
id: attachments
title: "kobana finance attachments"
sidebar_label: "attachments"
---

# kobana finance attachments


Comandos de `kobana finance attachments`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`attachments list`](#attachments-list) | `GET /v1/attachments` | Listar anexos |
| [`attachments create`](#attachments-create) | `POST /v1/attachments` | Enviar um anexo |
| [`attachments get`](#attachments-get) | `GET /v1/attachments/{id}` | Obter metadados do anexo por ID |
| [`attachments delete`](#attachments-delete) | `DELETE /v1/attachments/{id}` | Excluir anexo |
| [`attachments download`](#attachments-download) | `GET /v1/attachments/{id}/download` | Baixar bytes do anexo |
| [`attachments links`](#attachments-links) | `POST /v1/attachments/{id}/links` | Vincular anexo a um registro |
| [`attachments unlink`](#attachments-unlink) | `POST /v1/attachments/{id}/unlink` | Desvincular anexo de um registro |

## attachments list

Listar anexos

```text
kobana finance attachments list [OPÇÕES]
```

`GET /v1/attachments`

Returns a cursor-paginated page of attachment metadata for the workspace tied to the token's `sub` claim. Each row carries an absolute `download_url` pointing at the download endpoint; fetching the bytes requires the same Bearer JWT (scope `finance/attachments`, access level `read`). Aggregate counts (`linked_payables_count` etc.) expose how many parent rows reference each blob without forcing the client to call every parent endpoint. Default sort is `uploaded_at` descending (newest first). Requires scope `finance/attachments` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `mime_type` | `string` | não | Filter by exact MIME type. Accepts `[in]=application/pdf,image/png`. |
| `file_name` | `string` | não | Filter by exact file name. Accepts `[in]`. |
| `uploaded_by` | `string` | não | Filter by uploader email. Accepts `[in]`. |
| `uploaded_at` | `string` | não | ISO date filter on upload time. Range operators: `[gte]`, `[lte]`, `[gt]`, `[lt]`. |
| `linked_to_payable_id` | `string` | não | Return only attachments linked to this payable. Single UUID via eq. |
| `linked_to_receivable_id` | `string` | não | Return only attachments linked to this receivable. Single UUID via eq. |
| `linked_to_transaction_id` | `string` | não | Return only attachments linked to this transaction. Single UUID via eq. |
| `sort_by` | `string` | não | Field to order by. Valores: `file_name`, `file_size`, `uploaded_at`. |
| `sort_order` | `string` | não | Order direction. Defaults to `asc`; ignored without `sort_by`. Valores: `asc`, `desc`. |
| `limit` | `integer` | não | Page size. Default 25, max 100. |
| `cursor` | `string` | não | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Page of attachments plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## attachments create

Enviar um anexo

```text
kobana finance attachments create [OPÇÕES]
```

`POST /v1/attachments`

Uploads a file to the workspace blob registry. Send as `multipart/form-data` with a `file` part and an optional `title`. Requires scope `finance/attachments` with access level `write`. Allowed types: pdf, png, jpg/jpeg, xml, ofx, csv, xlsx; max 10 MB. For PDF/PNG/JPEG the declared MIME must match the payload's magic bytes (mismatch → 422). The response is the same metadata shape as `GET /attachments/{id}`; link counts start at 0 — associate the file via `POST /attachments/{id}/links`. `custom_metadata` may ride along as a JSON string in the multipart body.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

Enviado como formulário `multipart/form-data`: os arquivos vão em `--file CAMPO=CAMINHO`.

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `file` | `string (binary)` | sim | The file bytes (multipart field `file`). Allowed types: pdf, png, jpg/jpeg, xml, ofx, csv, xlsx. Max 10 MB. For PDF/PNG/JPEG the declared MIME must match the payload's magic bytes (mismatch → 422). |
| `title` | `string` | não | Optional label that overrides `file_name` in UIs (multipart field `title`). |
| `custom_metadata` | `string` | não | Metadados do usuário, como texto JSON (multipart field `custom_metadata`) — multipart carries no types, so the object is sent serialised. Máx. 64 KB, 16 níveis. JSON inválido ou um valor que não seja objeto → 422. |

Outros campos e objetos aninhados: `kobana schema finance.attachments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Attachment uploaded. The `Location` header points at the new row. |
| `400` | Body is not valid multipart/form-data. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Missing `file` part, file too large, disallowed MIME type, or magic bytes that don't match the declared type. |

## attachments get

Obter metadados do anexo por ID

```text
kobana finance attachments get [ID] [OPÇÕES]
```

`GET /v1/attachments/{id}`

Returns the metadata row including `download_url`. Fetching the bytes still requires `GET /attachments/{id}/download` with the same scope. Returns the same shape as the rows from the list endpoint. Requires scope `finance/attachments` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Get attachment metadata by id |
| `404` | Attachment not found within the tenant. |

## attachments delete

Excluir anexo

```text
kobana finance attachments delete [ID] [OPÇÕES]
```

`DELETE /v1/attachments/{id}`

Deletes the attachment row and its stored bytes. Requires scope `finance/attachments` with access level `delete`. Returns 409 (`CONFLICT`, with per-entity link counts in `details`) when the file is still linked to any payable / receivable / transaction — unlink it first via `POST /attachments/{id}/unlink`. Also 409 (`details.receipts`) when the file is a payment receipt's document: there is no unlink for that, delete the receipt instead and the file goes with it. Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Attachment deleted. |
| `404` | Attachment not found within the tenant. |
| `409` | The attachment is still linked to one or more records, or is a payment receipt's document (`details` carries the counts). |

## attachments download

Baixar bytes do anexo

```text
kobana finance attachments download [ID] [OPÇÕES]
```

`GET /v1/attachments/{id}/download`

Streams the file bytes for the attachment back to the caller. Requires scope `finance/attachments` with access level `read` — the same Bearer JWT as the listing endpoint. `Content-Type` reflects the stored MIME; `Content-Disposition` is `inline` for PDF / images and `attachment` for other types. No caching — `Cache-Control: private, no-store`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | File bytes (binary stream). See `Content-Type` and `Content-Disposition` headers. |
| `404` | Attachment not found within the tenant. |

## attachments links

Vincular anexo a um registro

```text
kobana finance attachments links [ID] [OPÇÕES]
```

`POST /v1/attachments/{id}/links`

Associates an existing attachment with a payable / receivable / transaction. The `{id}` path segment is the attachment; the body identifies the parent. Requires scope `finance/attachments` with access level `write`. Returns 409 when the association already exists, 404 when either the attachment or the parent is outside the tenant.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `type` | `string` | sim | Kind of parent record to (un)link the attachment to. Valores: `payable`, `receivable`, `transaction`. |
| `id` | `string (uuid)` | sim | Id of the parent payable / receivable / transaction within the same workspace. |

Outros campos e objetos aninhados: `kobana schema finance.attachments.links`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Association created. Echoes `{ attachment_id, parent_type, parent_id }`. |
| `400` | Malformed JSON body. |
| `404` | Attachment or parent record not found within the tenant. |
| `409` | The attachment is already linked to this record. |
| `422` | Schema validation failure (unknown `type`, non-uuid `id`). |

## attachments unlink

Desvincular anexo de um registro

```text
kobana finance attachments unlink [ID] [OPÇÕES]
```

`POST /v1/attachments/{id}/unlink`

Removes a single association between the attachment (`{id}`) and the parent identified in the body. Requires scope `finance/attachments` with access level `write`. Returns 404 when no such association exists. The attachment row itself is left intact — delete it with `DELETE /attachments/{id}`. Replaces `DELETE /attachments/{id}/links`: unlinking deletes nothing, so it is a write, and `DELETE` is kept for what needs the `delete` access level.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `type` | `string` | sim | Kind of parent record to (un)link the attachment to. Valores: `payable`, `receivable`, `transaction`. |
| `id` | `string (uuid)` | sim | Id of the parent payable / receivable / transaction within the same workspace. |

Outros campos e objetos aninhados: `kobana schema finance.attachments.unlink`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Association removed. Echoes the link that was removed: `{ attachment_id, parent_type, parent_id }`. |
| `400` | Malformed JSON body, or an empty / over-long `X-Idempotency-Key`. |
| `404` | No association between this attachment and the given record. |
| `409` | `X-Idempotency-Key` reused with a different body, or the same key is still in flight. |
| `422` | Schema validation failure (unknown `type`, non-uuid `id`). |
