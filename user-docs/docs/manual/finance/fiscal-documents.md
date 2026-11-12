---
id: fiscal-documents
title: "kobana finance fiscal-documents"
sidebar_label: "fiscal-documents"
---

# kobana finance fiscal-documents


Comandos de `kobana finance fiscal-documents`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`fiscal-documents list`](#fiscal-documents-list) | `GET /v1/fiscal-documents` | Listar notas fiscais |
| [`fiscal-documents get`](#fiscal-documents-get) | `GET /v1/fiscal-documents/{id}` | Obter nota fiscal por ID |

## fiscal-documents list

Listar notas fiscais

```text
kobana finance fiscal-documents list [OPÇÕES]
```

`GET /v1/fiscal-documents`

Returns a cursor-paginated page of the fiscal documents (NF-e / NFS-e) **received** against the workspace's companies, for the workspace tied to the token's `sub` claim. These notes are parked read-only — the app never emits them, so there is no create/cancel here. Default sort is `-issued_at`. Requires scope `finance/fiscal_documents` with access level `read`.

⚠️ `xml_download_url` points at the **attachments** resource, so fetching the XML bytes needs a token that also reaches `finance/attachments` (or `finance/all` / `all`). A note can legitimately keep no XML (a `summary`-completeness note), in which case `xml_attachment_id` and `xml_download_url` are both `null`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by the linked workspace company UUID. Accepts `[in]`. |
| `kind` | `string` | não | `service` (NFS-e) or `product` (NF-e). Accepts `[in]`. Valores: `service`, `product`. |
| `situation` | `string` | não | Fiscal status. Accepts `[in]`. Valores: `authorized`, `cancelled`, `unknown`. |
| `source` | `string` | não | How the note entered the workspace. Accepts `[in]`. Valores: `nfe_io`, `xml_upload`, `sefaz`. |
| `access_key` | `string` | não | Filter by the 44-digit access key (exact). Accepts `[in]`. |
| `number` | `string` | não | Filter by document number (exact). Accepts `[in]`. |
| `issuer_document` | `string` | não | Filter by issuer CNPJ/CPF (digits only) — every note from a given supplier. Accepts `[in]`. |
| `issued_at` | `string` | não | ISO date filter on the issue date. Range operators: `[gte]`, `[lte]`, `[gt]`, `[lt]`. |
| `created_at` | `string` | não | ISO timestamp filter on when the note appeared in the workspace. Range operators supported. |
| `updated_at` | `string` | não | ISO timestamp filter on the last write. Range operators supported. This is the incremental-sync window: poll `?updated_at[gte]=<end of your last poll>&sort_by=updated_at`. Deletions do NOT appear here. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=issued_at&sort_order=desc`. Valores: `created_at`, `issued_at`, `total_amount`, `updated_at`. |
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
| `200` | Page of fiscal documents plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## fiscal-documents get

Obter nota fiscal por ID

```text
kobana finance fiscal-documents get [ID] [OPÇÕES]
```

`GET /v1/fiscal-documents/{id}`

A single received fiscal document. Returns the same shape as the rows from the list endpoint. Requires scope `finance/fiscal_documents` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish. The XML bytes are not served here — follow `xml_download_url`, which lives on the attachments resource and needs `finance/attachments`.

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
| `200` | Get a fiscal document by id |
| `404` | Fiscal document not found within the tenant. |
