---
id: certificates
title: "kobana billing certificates"
sidebar_label: "certificates"
---

# kobana billing certificates


Commands of `kobana billing certificates`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`certificates list`](#certificates-list) | `GET /v1/certificates` | List digital certificates |
| [`certificates create`](#certificates-create) | `POST /v1/certificates` | Upload digital certificate |
| [`certificates get`](#certificates-get) | `GET /v1/certificates/{id}` | Get certificate |
| [`certificates delete`](#certificates-delete) | `DELETE /v1/certificates/{id}` | Delete certificate |

## certificates list

List digital certificates

```text
kobana billing certificates list [OPTIONS]
```

`GET /v1/certificates`

Returns a paginated list of the current organization’s digital (A1) certificates. For security reasons, the response **never** includes the private key nor the PFX password — only metadata extracted from the certificate.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `person_id` | `string` | no | Filter by associated person UUID. |
| `person_document_number` | `string` | no | Filter by person CPF/CNPJ (exact match). |
| `person_name_contains` | `string` | no | Filter by person name (partial search, case-insensitive). |
| `status` | `string` | no | Filter by certificate status. `expiring_soon` returns active certificates expiring within 30 days. Values: `active`, `expired`, `revoked`, `expiring_soon`. |
| `search` | `string` | no | Free-text search in `name`, `common_name`, `document` or `organization_name`. |
| `format` | `string` | no | Filter by stored certificate format (`pfx`, `pem`). |
| `document` | `string` | no | Filter by CPF or CNPJ extracted from the certificate (digits only). |
| `serial_number` | `string` | no | Filter by certificate serial number. |
| `not_after_from` | `string` | no | Returns certificates whose expiry date (`not_after`) is greater than or equal to the given date (ISO 8601). |
| `not_after_to` | `string` | no | Returns certificates whose expiry date (`not_after`) is less than or equal to the given date (ISO 8601). |
| `not_before_from` | `string` | no | Returns certificates whose validity start date (`not_before`) is greater than or equal to the given date (ISO 8601). |
| `not_before_to` | `string` | no | Returns certificates whose validity start date (`not_before`) is less than or equal to the given date (ISO 8601). |
| `created_after` | `string` | no | Returns certificates created from the given date (ISO 8601). |
| `created_before` | `string` | no | Returns certificates created up to the given date (ISO 8601). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of certificates. |
| `404` | Resource not found in the current organization. |

## certificates create

Upload digital certificate

```text
kobana billing certificates create [OPTIONS]
```

`POST /v1/certificates`

Uploads an A1 digital certificate via `multipart/form-data`. The private key and password are encrypted at rest and are **never** returned by the API. The endpoint detects duplicates by `thumbprint` and automatically associates the certificate with a `person` based on the extracted CNPJ/CPF.

Accepted `type` values:
- `pfx` — send `file` (binary PFX/P12) and `password`.
- `cert_key` — send `cert_file` (certificate PEM) and `key_file` (key PEM).
- `combined_pem` — send `file` (PEM containing certificate + key).
- `cert_key_password` — send `cert_file`, `key_file` and `password` (password-protected key).

**Request body**

Sent as a `multipart/form-data` form: files go in `--file FIELD=PATH`.

| Name | Type | Required | Description |
|---|---|---|---|
| `type` | `string` | yes | Upload format: `pfx`, `cert_key`, `combined_pem` or `cert_key_password`. Values: `pfx`, `cert_key`, `combined_pem`, `cert_key_password`. |
| `name` | `string` | no | Friendly certificate name. When omitted, it is derived from `common_name` or the document. |
| `file` | `string (binary)` | no | Binary file (PFX/P12) when `type=pfx`, or combined PEM when `type=combined_pem`. |
| `cert_file` | `string (binary)` | no | Certificate PEM file (only for `type=cert_key` or `type=cert_key_password`). |
| `key_file` | `string (binary)` | no | Private key PEM file (only for `type=cert_key` or `type=cert_key_password`). |
| `password` | `string` | no | PFX password (`type=pfx`) or private key password (`type=cert_key_password`). Stored encrypted and never returned. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |

More fields and nested objects: `kobana schema billing.certificates.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Certificate uploaded and stored successfully. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## certificates get

Get certificate

```text
kobana billing certificates get [ID] [OPTIONS]
```

`GET /v1/certificates/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Certificate detail (no sensitive data). |
| `404` | Resource not found in the current organization. |

## certificates delete

Delete certificate

```text
kobana billing certificates delete [ID] [OPTIONS]
```

`DELETE /v1/certificates/{id}`

Removes the certificate via **soft delete**. Encrypted data remains in the database for audit purposes only.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Certificate deleted. |
| `404` | Resource not found in the current organization. |
