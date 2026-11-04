---
id: security
title: "kobana banking security"
sidebar_label: "security"
description: "Security (access tokens)"
---

# kobana banking security

Security (access tokens)

Commands of `kobana banking security`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`security access-tokens create`](#security-access-tokens-create) | `POST /v2/security/access_tokens` | Create an Access Token |
| [`security access-tokens update`](#security-access-tokens-update) | `PUT /v2/security/access_tokens/{uid}` | Update Access Token |
| [`security access-tokens delete`](#security-access-tokens-delete) | `DELETE /v2/security/access_tokens/{uid}` | Delete Access Token |
| [`security access-tokens disable`](#security-access-tokens-disable) | `PUT /v2/security/access_tokens/{uid}/disable` | Disable Access Token |
| [`security access-tokens enable`](#security-access-tokens-enable) | `PUT /v2/security/access_tokens/{uid}/enable` | Enable Access Token |
| [`security access-tokens renew`](#security-access-tokens-renew) | `PUT /v2/security/access_tokens/{uid}/renew` | Renew Access Token |
| [`security access-tokens revoke`](#security-access-tokens-revoke) | `PUT /v2/security/access_tokens/{uid}/revoke` | Revoke Access Token |

## security access-tokens create

Create an Access Token

```text
kobana banking security access-tokens create [OPTIONS]
```

`POST /v2/security/access_tokens`

Creates a new access token for authentication in the API. The created token can have limited scopes and expiration date.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `expires_at` | `string (date)` | yes | Expiration date (maximum 1 year). Required when access_type = write |
| `reminder_emails` | `array<object>` | yes | Emails for expiration notifications. Required when expires_at is set. |
| `external_id` | `string` | no | External ID of the token |
| `description` | `string` | no | Token description |
| `scope_type` | `string` | no | Scope type: All (all) or Limited (limited) Values: `all`, `limited`. |
| `limited_scopes` | `array<string>` | no | Available scopes (when scope_type = limited). Select one or more values: \| Scope \| Description \| \|--------\|-----------\| \| `admin.subaccounts` \| Manage subaccounts of the main account \| \| `admin.users` \| Manage users and permissions \| \| `automation.email_accounts` \| Manage email accounts for automation \| \| `automation.email_deliveries` \| View and manage email deliveries \| \| `automation.sms_accounts` \| Manage SMS accounts \| \| `automation.sms_deliveries` \| View and manage SMS deliveries \| \| `automation.webhook_deliveries` \| View and manage webhook deliveries \| \| `automation.webhooks` \| Manage webhooks and automated notifications \| \| `billing.transactions` \| View billing and charging transactions \| \| `charge.automatic_pix.locations` \| Manage automatic Pix locations \| \| `charge.automatic_pix.pix` \| Manage automatic Pix charges \| \| `charge.automatic_pix.recurrences` \| Manage automatic Pix recurrences \| \| `charge.automatic_pix.requests` \| Manage automatic Pix requests \| \| `charge.bank_billet_accounts` \| Manage bank boleto accounts \| \| `charge.bank_billet_payments` \| View and register boleto payments \| \| `charge.bank_billet_registrations` \| Manage boleto registrations at banks \| \| `charge.bank_billets` \| Manage bank boletos \| \| `charge.customer_subscriptions` \| Manage customer subscriptions \| \| `charge.installments` \| Manage installments \| \| `charge.payments` \| View and manage received payments \| \| `charge.pix` \| Manage Pix charges \| \| `charge.pix_accounts` \| Manage Pix accounts \| \| `core.providers` \| View available providers \| \| `crm.customers` \| Manage customers and business information \| \| `crm.people` \| Manage people and contacts \| \| `data.bank_billet_queries` \| Query bank boleto information \| \| `financial.accounts` \| Manage financial accounts \| \| `financial.balances` \| View balances and financial movements \| \| `financial.providers` \| Manage financial providers and banking integrations \| \| `financial.statement_transactions` \| View transactions and financial statements \| \| `integration.certificates` \| Manage digital certificates for integrations \| \| `integration.connections` \| Manage connections with banks and providers \| \| `integration.discharges` \| Manage banking return files (discharges) \| \| `integration.edi_boxes` \| Manage EDI mailboxes for file exchange \| \| `integration.remittances` \| Manage banking remittance files \| \| `mailbox.entries` \| Manage mailboxes for receiving files \| \| `mailbox.files` \| View and manage files in mailboxes \| \| `partner.bank_contracts` \| Manage banking contracts with partners \| \| `payment.accounts` \| Manage payment accounts (including EDI remittance enablement) \| \| `payment.bank_billets` \| Make bank boleto payments \| \| `payment.batches` \| Manage payment batches \| \| `payment.dda_accounts` \| Manage DDA accounts \| \| `payment.dda.bank_billets` \| Manage DDA boletos \| \| `payment.darfs` \| Make DARF payments (federal taxes) \| \| `payment.payments` \| Manage all types of payments through the API \| \| `payment.pix` \| Make payments via Pix \| \| `payment.taxes` \| Make tax and fee payments \| \| `payment.utilities` \| Make utility bill payments (water, electricity, etc) \| \| `security.access_tokens` \| Manage access tokens and authentication \| \| `system.events` \| View system events and logs \| \| `system.imports` \| Manage data imports \| \| `system.reports` \| Generate and view system reports \| \| `transfer.accounts` \| Manage transfer accounts (including EDI remittance enablement) \| \| `transfer.batches` \| Manage transfer batches \| \| `transfer.internal` \| Perform internal transfers between accounts \| \| `transfer.pix` \| Perform transfers via Pix \| \| `transfer.ted` \| Perform TED transfers \| \| `transfer.transfers` \| Manage all types of transfers (Pix, TED, Internal) via unified endpoint \| |
| `access_type` | `string` | no | Access type: Read (read) or Read and write (write) Values: `read`, `write`. |
| `tags` | `array<string>` | no | Token tags |
| `custom_data` | `object` | no | Custom data |

More fields and nested objects: `kobana schema banking.security.access-tokens.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Token created |
| `422` | Invalid token data |

## security access-tokens update

Update Access Token

```text
kobana banking security access-tokens update [UID] [OPTIONS]
```

`PUT /v2/security/access_tokens/{uid}`

Updates the information of an existing access token, including description, scopes, tags and expiration date. **Note:** The expiration date can only be set if it was not already set. Once set, it cannot be changed.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `external_id` | `string` | no | External ID of the token |
| `description` | `string` | no | Token description |
| `scope_type` | `string` | no | Scope type (all or limited) Values: `all`, `limited`. |
| `limited_scopes` | `array<string>` | no | Available scopes (when scope_type = limited). Select one or more values: \| Scope \| Description \| \|--------\|-----------\| \| `admin.subaccounts` \| Manage subaccounts of the main account \| \| `admin.users` \| Manage users and permissions \| \| `automation.email_accounts` \| Manage email accounts for automation \| \| `automation.email_deliveries` \| View and manage email deliveries \| \| `automation.sms_accounts` \| Manage SMS accounts \| \| `automation.sms_deliveries` \| View and manage SMS deliveries \| \| `automation.webhook_deliveries` \| View and manage webhook deliveries \| \| `automation.webhooks` \| Manage webhooks and automated notifications \| \| `billing.transactions` \| View billing and charging transactions \| \| `charge.automatic_pix.locations` \| Manage automatic Pix locations \| \| `charge.automatic_pix.pix` \| Manage automatic Pix charges \| \| `charge.automatic_pix.recurrences` \| Manage automatic Pix recurrences \| \| `charge.automatic_pix.requests` \| Manage automatic Pix requests \| \| `charge.bank_billet_accounts` \| Manage bank boleto accounts \| \| `charge.bank_billet_payments` \| View and register boleto payments \| \| `charge.bank_billet_registrations` \| Manage boleto registrations at banks \| \| `charge.bank_billets` \| Manage bank boletos \| \| `charge.customer_subscriptions` \| Manage customer subscriptions \| \| `charge.installments` \| Manage installments \| \| `charge.payments` \| View and manage received payments \| \| `charge.pix` \| Manage Pix charges \| \| `charge.pix_accounts` \| Manage Pix accounts \| \| `core.providers` \| View available providers \| \| `crm.customers` \| Manage customers and business information \| \| `crm.people` \| Manage people and contacts \| \| `data.bank_billet_queries` \| Query bank boleto information \| \| `financial.accounts` \| Manage financial accounts \| \| `financial.balances` \| View balances and financial movements \| \| `financial.providers` \| Manage financial providers and banking integrations \| \| `financial.statement_transactions` \| View transactions and financial statements \| \| `integration.certificates` \| Manage digital certificates for integrations \| \| `integration.connections` \| Manage connections with banks and providers \| \| `integration.discharges` \| Manage banking return files (discharges) \| \| `integration.edi_boxes` \| Manage EDI mailboxes for file exchange \| \| `integration.remittances` \| Manage banking remittance files \| \| `mailbox.entries` \| Manage mailboxes for receiving files \| \| `mailbox.files` \| View and manage files in mailboxes \| \| `partner.bank_contracts` \| Manage banking contracts with partners \| \| `payment.accounts` \| Manage payment accounts (including EDI remittance enablement) \| \| `payment.bank_billets` \| Make bank boleto payments \| \| `payment.batches` \| Manage payment batches \| \| `payment.dda_accounts` \| Manage DDA accounts \| \| `payment.dda.bank_billets` \| Manage DDA boletos \| \| `payment.darfs` \| Make DARF payments (federal taxes) \| \| `payment.payments` \| Manage all types of payments through the API \| \| `payment.pix` \| Make payments via Pix \| \| `payment.taxes` \| Make tax and fee payments \| \| `payment.utilities` \| Make utility bill payments (water, electricity, etc) \| \| `security.access_tokens` \| Manage access tokens and authentication \| \| `system.events` \| View system events and logs \| \| `system.imports` \| Manage data imports \| \| `system.reports` \| Generate and view system reports \| \| `transfer.accounts` \| Manage transfer accounts (including EDI remittance enablement) \| \| `transfer.batches` \| Manage transfer batches \| \| `transfer.internal` \| Perform internal transfers between accounts \| \| `transfer.pix` \| Perform transfers via Pix \| \| `transfer.ted` \| Perform TED transfers \| \| `transfer.transfers` \| Manage all types of transfers (Pix, TED, Internal) via unified endpoint \| |
| `access_type` | `string` | no | Access type Values: `read`, `write`. |
| `expires_at` | `string (date)` | no | Expiration date (maximum 1 year). **Attention:** Can only be set if it was not already set. Once set, it cannot be changed again. |
| `tags` | `array<string>` | no | Token tags |
| `custom_data` | `object` | no | Custom data |
| `reminder_emails` | `array<object>` | no | Emails for expiration notifications |

More fields and nested objects: `kobana schema banking.security.access-tokens.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Token updated |
| `404` | Token not found |

## security access-tokens delete

Delete Access Token

```text
kobana banking security access-tokens delete [UID] [OPTIONS]
```

`DELETE /v2/security/access_tokens/{uid}`

Permanently removes an access token from the system. Only revoked or expired tokens can be deleted.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Token deleted |
| `404` | Token not found |

## security access-tokens disable

Disable Access Token

```text
kobana banking security access-tokens disable [UID] [OPTIONS]
```

`PUT /v2/security/access_tokens/{uid}/disable`

Disables an enabled access token, temporarily preventing its use for authentication.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Token disabled |

## security access-tokens enable

Enable Access Token

```text
kobana banking security access-tokens enable [UID] [OPTIONS]
```

`PUT /v2/security/access_tokens/{uid}/enable`

Enables a disabled access token, allowing its use for authentication.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Token enabled |

## security access-tokens renew

Renew Access Token

```text
kobana banking security access-tokens renew [UID] [OPTIONS]
```

`PUT /v2/security/access_tokens/{uid}/renew`

Creates a new access token with the same settings as the current token and revokes the old token. The new token is returned in the response.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Token renewed |

## security access-tokens revoke

Revoke Access Token

```text
kobana banking security access-tokens revoke [UID] [OPTIONS]
```

`PUT /v2/security/access_tokens/{uid}/revoke`

Permanently revokes an access token. V2 tokens must be disabled before being revoked. V1 tokens can be revoked directly.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Token revoked |
