---
id: financial
title: "kobana banking financial"
sidebar_label: "financial"
description: "Financial (accounts, balances, statements)"
---

# kobana banking financial

Financial (accounts, balances, statements)

Commands of `kobana banking financial`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`financial accounts balances list`](#financial-accounts-balances-list) | `GET /v2/financial/accounts/{financial_account_uid}/balances` | List balances |
| [`financial accounts balances create`](#financial-accounts-balances-create) | `POST /v2/financial/accounts/{financial_account_uid}/balances` | Create a balance |
| [`financial accounts balances get`](#financial-accounts-balances-get) | `GET /v2/financial/accounts/{financial_account_uid}/balances/{balance_uid}` | View a Balance |
| [`financial accounts commands get`](#financial-accounts-commands-get) | `GET /v2/financial/accounts/{financial_account_uid}/commands/{uid}` | View a Command |
| [`financial accounts commands list`](#financial-accounts-commands-list) | `GET /v2/financial/accounts/{financial_account_uid}/commands` | List Account Commands |
| [`financial accounts statement-transactions imports list`](#financial-accounts-statement-transactions-imports-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports` | List Statement Imports |
| [`financial accounts statement-transactions imports create`](#financial-accounts-statement-transactions-imports-create) | `POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports` | Import Statement |
| [`financial accounts statement-transactions imports get`](#financial-accounts-statement-transactions-imports-get) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports/{uid}` | View Statement Import |
| [`financial accounts statement-transactions summary`](#financial-accounts-statement-transactions-summary) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/summary` | Summarize Statement |
| [`financial accounts statement-transactions syncs list`](#financial-accounts-statement-transactions-syncs-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs` | List Statement Synchronizations |
| [`financial accounts statement-transactions syncs get`](#financial-accounts-statement-transactions-syncs-get) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs/{uid}` | Query Statement Synchronization |
| [`financial accounts statement-transactions sync`](#financial-accounts-statement-transactions-sync) | `POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/sync` | Synchronize Statement |
| [`financial accounts statement-transactions list`](#financial-accounts-statement-transactions-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions` | List Transactions |
| [`financial accounts list`](#financial-accounts-list) | `GET /v2/financial/accounts` | List Accounts |
| [`financial accounts create`](#financial-accounts-create) | `POST /v2/financial/accounts` | Create an Account |
| [`financial accounts get`](#financial-accounts-get) | `GET /v2/financial/accounts/{uid}` | View an Account |
| [`financial accounts update`](#financial-accounts-update) | `PUT /v2/financial/accounts/{uid}` | Update Account |
| [`financial accounts delete`](#financial-accounts-delete) | `DELETE /v2/financial/accounts/{uid}` | Delete an Account |
| [`financial companies list`](#financial-companies-list) | `GET /v2/financial/companies` | List Companies |
| [`financial companies create`](#financial-companies-create) | `POST /v2/financial/companies` | Create a Company |
| [`financial companies get`](#financial-companies-get) | `GET /v2/financial/companies/{uid}` | Get a Company |
| [`financial companies update`](#financial-companies-update) | `PUT /v2/financial/companies/{uid}` | Update a Company |
| [`financial companies delete`](#financial-companies-delete) | `DELETE /v2/financial/companies/{uid}` | Delete a Company |
| [`financial providers`](#financial-providers) | `GET /v2/financial/providers` | List Providers |
| [`financial statement-categories`](#financial-statement-categories) | `GET /v2/financial/statement_categories` | List Statement Categories |

## financial accounts balances list

List balances

```text
kobana banking financial accounts balances list [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/balances`

Lists all balances of a specific financial account. Returns information about available, blocked, and automatically invested amounts.

**Path arguments:** `financial_account_uid`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Balances found |

## financial accounts balances create

Create a balance

```text
kobana banking financial accounts balances create [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`POST /v2/financial/accounts/{financial_account_uid}/balances`

Creates a new manual balance for a financial account. Allows recording initial balances or manual reconciliation adjustments.

**Path arguments:** `financial_account_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `uid` | `string (uuid)` | no | UID of the Balance |
| `amount` | `number` | no | Amount of the balance |
| `blocked_amount` | `number` | no | Blocked amount |
| `automatically_invested_amount` | `number` | no | Amount automatically invested |
| `created_at` | `string (date-time)` | no | Date of creation. iso8601 format |
| `custom_data` | `object` | no | Hash with key and value in JSON format. Optional for internal control. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `tags` | `array<string>` | no | Tags associated with the import. Optional for internal control. |

More fields and nested objects: `kobana schema banking.financial.accounts.balances.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Balance created |

## financial accounts balances get

View a Balance

```text
kobana banking financial accounts balances get [FINANCIAL_ACCOUNT_UID] [BALANCE_UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/balances/{balance_uid}`

Query specific information of an Account.

**Path arguments:** `financial_account_uid`, `balance_uid`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `field` | `string` | no | Field to search for ID. Example: `uid` or `external_id` |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Account found |
| `404` | Account not found |

## financial accounts commands get

View a Command

```text
kobana banking financial accounts commands get [FINANCIAL_ACCOUNT_UID] [UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/commands/{uid}`

Query detailed information of a specific command executed on a financial account.

**Path arguments:** `financial_account_uid`, `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command found |
| `404` | Command not found |

## financial accounts commands list

List Account Commands

```text
kobana banking financial accounts commands list [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/commands`

Lists all commands executed on a financial account. Commands represent asynchronous operations such as statement synchronization and other automated actions.

**Path arguments:** `financial_account_uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Commands found |
| `404` | Account not found |

## financial accounts statement-transactions imports list

List Statement Imports

```text
kobana banking financial accounts statement-transactions imports list [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports`

Lists all statement imports performed for a financial account. Allows filtering by status, processing date, tags, and imported file name.

**Path arguments:** `financial_account_uid`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `status` | `string` | no | Filter by import status |
| `source_file_name` | `string` | no | Name of imported file |
| `processed_from` | `string` | no | Start date of import processing date filter |
| `processed_to` | `string` | no | End date of import processing date filter |
| `created_from` | `string` | no | Start date of import creation date filter |
| `created_to` | `string` | no | End date of import creation date filter |
| `tags` | `string` | no | Filter by import tags |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Statement Imports found |

## financial accounts statement-transactions imports create

Import Statement

```text
kobana banking financial accounts statement-transactions imports create [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports`

Imports a bank statement through a CNAB file. Processing is performed asynchronously and transactions are associated with the financial account.
> **Asynchronous Processing:** This endpoint is processed asynchronously and creates an update command for the Financial Account.

Importing a statement for a Financial Account is done asynchronously, that is, when receiving the request,
our API stores the request parameters and processes the import in the background.
After the import is processed, the transactions will be associated with the Financial Account.
You receive in the response of this request the UID of the created Import and can query it anytime.

**Webhook Events**

When the request is made, an `import.created` event is triggered.
Our server then begins processing the import.
After processing is complete, the `import.processed` event is triggered.

**Path arguments:** `financial_account_uid`

**Request body**

Sent as a `multipart/form-data` form: files go in `--file FIELD=PATH`.

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | `string (binary)` | yes | File for import. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |

More fields and nested objects: `kobana schema banking.financial.accounts.statement-transactions.imports.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Import created |

## financial accounts statement-transactions imports get

View Statement Import

```text
kobana banking financial accounts statement-transactions imports get [FINANCIAL_ACCOUNT_UID] [UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports/{uid}`

Query specific information of a Statement Import.

**Path arguments:** `financial_account_uid`, `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Statement Import found |
| `404` | Account not found |

## financial accounts statement-transactions summary

Summarize Statement

```text
kobana banking financial accounts statement-transactions summary [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/summary`

Aggregates statement transactions into totals by group, instead of returning one line per transaction.

Answers questions like "how much did I spend on fees in July?" in a single request, with the
arithmetic done in the database — without paginating the entire statement.

> **The window here can reach 730 days**, against 180 from the listing. The response size is
> limited by the number of groups, not by history, so a long window is safe.

Accepts the same filters as the listing. If the combination of `group_by` and window generates more than
1000 groups, the request is rejected with `422` instead of returning a truncated summary — a total
cut in half looks complete and deceives silently.

The response also brings the `filters` and `meta` blocks, identical to those in the listing, with the addition
of `filters.group_by` informing the grouping applied.

**Path arguments:** `financial_account_uid`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `group_by` | `string` | no | Grouping dimensions separated by comma: `day`, `week`, `month`, `kind`, `category`. Default: `month`. |
| `occurrence_date_from` | `string` | no | Start date of the window. Same derivation rule as the listing, with 730-day limit. |
| `occurrence_date_to` | `string` | no | End date of the window. |
| `kind` | `string` | no | Same filter as the listing. Values: `credit`, `debit`. |
| `category` | `string` | no | Same filter as the listing. |
| `q` | `string` | no | Same filter as the listing. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Calculated summary |

## financial accounts statement-transactions syncs list

List Statement Synchronizations

```text
kobana banking financial accounts statement-transactions syncs list [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs`

Lists the statement synchronizations requested for a Financial Account, from most recent to oldest.

Each item represents a request made in
Synchronize Statement
and aggregates the commands created for the requested window — windows that span more than one month generate one command per month.

The `commands_summary` field brings the count of commands by status, which allows identifying how many months
failed without needing to open the request. To see the commands individually, use the query by `uid`.

> Only synchronizations requested via API appear here. Automatic periodic synchronizations do not generate a record.

**Path arguments:** `financial_account_uid`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `status` | `string` | no | Filters by aggregated synchronization status. Values: `pending`, `processing`, `done`, `partial`, `failed`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Synchronizations found |

## financial accounts statement-transactions syncs get

Query Statement Synchronization

```text
kobana banking financial accounts statement-transactions syncs get [FINANCIAL_ACCOUNT_UID] [UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs/{uid}`

Query the status of a requested statement synchronization.

The status is recalculated with each query from the request commands:

* `pending` - no command has started processing
* `processing` - in progress (part of commands have finished)
* `done` - all commands have been confirmed and transactions are already available in
  List Transactions
* `partial` - all finished, but part failed. The `commands` array shows which window failed and why
* `failed` - all finished and none were confirmed

The `commands` array brings each command with its own window in `params.start_at`/`params.end_at` and the
error message in `error_message`, when applicable. It is through it that **which month** failed in a
multi-month synchronization is identified.

**Path arguments:** `financial_account_uid`, `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Synchronization found |
| `404` | Synchronization not found |

## financial accounts statement-transactions sync

Synchronize Statement

```text
kobana banking financial accounts statement-transactions sync [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/sync`

Synchronizes the bank statement of a financial account. Fetches transactions directly from the financial institution through API. Requires active connection.
> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a statement synchronization command for the Financial Account.

Statement synchronization of a Financial Account is done asynchronously, that is, when receiving the request, our
API stores the request parameters and creates a Command. This command is processed in the background and then confirmed
and the transactions will be imported into the Financial Account. You receive in the response of this request the ID of the created command
and can query it anytime.

See more in List Financial Account Commands and
View a Command.

In order to execute the Statement Synchronization, there must be an active Connection (i.e., validated and enabled)
associated with the Financial Account.

**Synchronization Period**

The fields `start_at` and `end_at` are optional and delimit the window to synchronize.
When omitted, the default window goes from **the first day of the current month until today**.

* informing only `start_at`, the end date assumed is today;
* informing only `end_at`, the start date assumed is the first day of the month of `end_at`;
* the end date cannot be in the future;
* the maximum period is **2 years** back from today.

Windows that span more than one calendar month are **sliced into one command per month**. This keeps each
request to the financial institution within a volume that the bank can respond to, and allows
reprocessing only the month that eventually fails. A window within the same month continues to create
a single command.

**Tracking the Import**

In addition to the command in `data`, the response brings in `sync` the record of the requested import, with its own `uid`.
Use this `uid` to track the entire set — especially in multi-month windows, where
several commands are created — in
Query Statement Synchronization.

**Webhook Events**

When the request is made, a `financial.account.statement_sync.requested` event is triggered.
Our server then begins communication with the financial institution's API to fetch transactions.
Only after it is confirmed at the financial institution, then the statement is updated.
At this moment the `financial.account.statement_sync.confirmed` event is triggered.

**Path arguments:** `financial_account_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `start_at` | `string (date)` | no | Start date of synchronization. Default: first day of the current month. Maximum of 2 years back. |
| `end_at` | `string (date)` | no | End date of synchronization. Default: today. Cannot be in the future. |

More fields and nested objects: `kobana schema banking.financial.accounts.statement-transactions.sync`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Command created per month for multi-month window |
| `422` | Statement Connection not active |

## financial accounts statement-transactions list

List Transactions

```text
kobana banking financial accounts statement-transactions list [FINANCIAL_ACCOUNT_UID] [OPTIONS]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions`

Lists the transactions of the statement of a financial account, filtering by period, type, amount, and description.

**Date window — scheduled change**

Today, a request without dates returns the entire statement of the account. This behavior
**will change on 05/02/2027**: the query will always run within a window of at most
180 days, which is what keeps the endpoint fast on accounts with long history.

You can adopt the new behavior right now by sending `date_window=strict`, and you should do it
before the cutoff date. With it:

| Parameters sent | Window queried |
|---|---|
| none | The last 180 days |
| only `occurrence_date_from` | 180 days from it |
| only `occurrence_date_to` | 180 days before it |
| both | The informed period, as long as it does not exceed 180 days |

An interval greater than 180 days is rejected with `422` instead of being silently reduced — a partial statement that looks complete is worse than an error. For longer periods, make successive calls of up to 180 days each.

As long as `date_window=strict` is not sent, nothing changes. Requests whose result **would be
different** under the strict window respond with the headers `Deprecation: true` and
`Sunset: Fri, 05 Feb 2027 00:00:00 GMT` — use them to discover which of your integrations
need adjustment.

**Path arguments:** `financial_account_uid`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page. Values above the maximum are reduced to the maximum. |
| `date_window` | `string` | no | Send `strict` to adopt the 180-day window starting now that will be the standard on 05/02/2027. Without it, current behavior is preserved. Values: `strict`. |
| `occurrence_date_from` | `string` | no | Start date of the occurrence date filter. With `date_window=strict`, if omitted it is derived from `occurrence_date_to` (180 days before) or, in the absence of both, from today. |
| `occurrence_date_to` | `string` | no | End date of the occurrence date filter. With `date_window=strict`, if omitted it is derived from `occurrence_date_from` (180 days after) or, in the absence of both, is today. |
| `kind` | `string` | no | Filters by transaction type. Values: `credit`, `debit`. |
| `amount_from` | `number` | no | Minimum transaction value, in reais and with sign — debits are negative, as in the `amount` field of the response. Use period as decimal separator. |
| `amount_to` | `number` | no | Maximum transaction value, in reais and with sign — debits are negative, as in the `amount` field of the response. Use period as decimal separator. |
| `amount_abs_from` | `number` | no | Minimum value in modulus, ignoring the sign. Use for "above X, inflow or outflow". |
| `amount_abs_to` | `number` | no | Maximum value in modulus, ignoring the sign. |
| `category` | `string` | no | Category code, or comma-separated list. Consult the catalog at `GET /v2/financial/statement_categories`. Unknown codes are ignored. |
| `updated_since` | `string` | no | Returns only transactions changed from this moment (ISO 8601). Used for incremental synchronization. |
| `q` | `string` | no | Text search in the transaction description sent by the bank (`description`). Matches any part of the text, without distinguishing uppercase, lowercase, or accents — `manutencao` finds `MANUTENÇÃO`. |
| `view` | `string` | no | Use `compact` to receive only the essential fields of each transaction. Values: `compact`. |
| `fields` | `string` | no | Comma-separated list of fields to return in each transaction. Takes precedence over `view`. An unknown field results in `422`. |
| `pagination` | `string` | no | Use `keyset` for cursor pagination, stable when new transactions arrive during the scan. The response brings `next_cursor` instead of URLs. Values: `keyset`. |
| `cursor` | `string` | no | Cursor returned in `pagination.next_cursor`. Only has effect with `pagination=keyset`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Transactions found |
| `422` | Date range larger than 180 days (with date_window=strict) |

## financial accounts list

List Accounts

```text
kobana banking financial accounts list [OPTIONS]
```

`GET /v2/financial/accounts`

Lists all registered financial accounts. Returns information about bank accounts including provider, account type, bank data, and custom metadata.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Accounts found |

## financial accounts create

Create an Account

```text
kobana banking financial accounts create [OPTIONS]
```

`POST /v2/financial/accounts`

Creates a new financial account for managing bank transactions. Supports checking and savings accounts.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `account_number` | `string` | yes | Account Number |
| `account_digit` | `string` | yes | Account digit |
| `agency_number` | `string` | yes | Branch Number |
| `financial_provider_slug` | `string` | yes | Financial Provider |
| `person_info` | `object` | yes | Data of the Account Holder |
| `uid` | `string (uuid)` | no | UID of Financial Account |
| `kind` | `string` | no | Type of Account `checking`: Checking Account `savings`: Savings Account `payment`: Payment Account |
| `agency_digit` | `string` | no | Agency Digit. Optional: will be calculated automatically using Module 11 when the bank requires it and not provided. |
| `bank_number` | `integer` | no | COMPE Number of the Bank. Required if bank_id is not provided. |
| `created_at` | `string (date-time)` | no | Date of creation. iso8601 format |
| `custom_data` | `object` | no | Hash with key and value in JSON format. Optional for internal control. |
| `custom_name` | `string` | no | Name of the Financial Account for identification within Kobana. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `company_uid` | `string (uuid)` | no | UID of the Company (`/v2/financial/companies`) the account belongs to. A nonexistent, malformed or other account's UID is rejected with 422 (`unresolvable_uid`). `null` or empty unlinks the account from the company; without the field, the company does not change. |
| `payment_agreement_code` | `string` | no | Number of the Payment Agreement contracted with the Bank. |
| `tags` | `array<string>` | no | Tags associated with the import. Optional for internal control. |
| `updated_at` | `string (date-time)` | no | Date of last update. iso8601 format |
| `service_accounts` | `array<object>` | no | Service accounts associated with this financial account. |

More fields and nested objects: `kobana schema banking.financial.accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Account created |
| `422` | Validation error |

## financial accounts get

View an Account

```text
kobana banking financial accounts get [UID] [OPTIONS]
```

`GET /v2/financial/accounts/{uid}`

Query specific information of an Account.

**Path arguments:** `uid`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `field` | `string` | no | Field to search for ID. Example: `uid` or `external_id` |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Account found |
| `404` | Account not found |

## financial accounts update

Update Account

```text
kobana banking financial accounts update [UID] [OPTIONS]
```

`PUT /v2/financial/accounts/{uid}`

Updates a specific Financial Account.

**Path arguments:** `uid`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `field` | `string` | no | Field to search for ID. Example: `uid` or `external_id` |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `kind` | `string` | no | Type of Account `checking`: Checking Account `savings`: Savings Account `payment`: Payment Account |
| `account_number` | `string` | no | Account Number |
| `account_digit` | `string` | no | Account digit |
| `agency_number` | `string` | no | Branch Number |
| `agency_digit` | `string` | no | Agency Digit. Optional: will be calculated automatically using Module 11 when the bank requires it and not provided. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. Optional for internal control. |
| `custom_name` | `string` | no | Name of the Financial Account for identification within Kobana. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `company_uid` | `string (uuid)` | no | UID of the Company (`/v2/financial/companies`) the account belongs to. A nonexistent, malformed or other account's UID is rejected with 422 (`unresolvable_uid`). `null` or empty unlinks the account from the company; without the field, the company does not change. |
| `payment_agreement_code` | `string` | no | Number of the Payment Agreement contracted with the Bank. |
| `person_info` | `object` | no | Data of the Account Holder |
| `tags` | `array<string>` | no | Tags associated with the import. Optional for internal control. |

More fields and nested objects: `kobana schema banking.financial.accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Financial Account updated |
| `404` | Financial Account not found |
| `422` | Validation failure |

## financial accounts delete

Delete an Account

```text
kobana banking financial accounts delete [UID] [OPTIONS]
```

`DELETE /v2/financial/accounts/{uid}`

Deletes a Financial Account. The deletion is processed asynchronously.

⚠️ Attention: this operation cascades and removes all records
associated with the Financial Account, including boleto wallets,
transfers, payments, statements, and other related resources.
This action cannot be undone.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Account sent for deletion |
| `404` | Financial Account not found |

## financial companies list

List Companies

```text
kobana banking financial companies list [OPTIONS]
```

`GET /v2/financial/companies`

Lists the account's companies (CNPJs), most recent first.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `cnpj` | `string` | no | Filters by CNPJ (with or without mask). |
| `active` | `boolean` | no | Filters by active or inactive companies. |
| `external_id` | `string` | no | Filters by the ID in your system. |
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Companies found |

## financial companies create

Create a Company

```text
kobana banking financial companies create [OPTIONS]
```

`POST /v2/financial/companies`

Creates a company (CNPJ) in the account. Contacts in the Person format: lists of addresses, emails and phones.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Legal name. |
| `cnpj` | `string` | yes | CNPJ without mask, in uppercase (accepts the alphanumeric CNPJ). |
| `trade_name` | `string` | no | Trade name. |
| `active` | `boolean` | no | Active company. Only one active company per CNPJ in the account. |
| `municipal_registration` | `string` | no | Municipal registration. |
| `tax_regime` | `string` | no | Tax regime. Values: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`, `null`. |
| `addresses` | `array<object>` | no |  |
| `emails` | `array<object>` | no |  |
| `phones` | `array<object>` | no |  |
| `external_id` | `string` | no | ID in your system. Unique per account. |
| `tags` | `array<string>` | no |  |
| `custom_data` | `object` | no | Custom data. |

More fields and nested objects: `kobana schema banking.financial.companies.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Company created |
| `422` | Validation failure |

## financial companies get

Get a Company

```text
kobana banking financial companies get [UID] [OPTIONS]
```

`GET /v2/financial/companies/{uid}`

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company found |
| `404` | Company not found |

## financial companies update

Update a Company

```text
kobana banking financial companies update [UID] [OPTIONS]
```

`PUT /v2/financial/companies/{uid}`

Updates the company. Its financial accounts get the company's data as the account holder.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Legal name. |
| `trade_name` | `string` | no | Trade name. |
| `cnpj` | `string` | no | CNPJ without mask, in uppercase (accepts the alphanumeric CNPJ). |
| `active` | `boolean` | no | Active company. Only one active company per CNPJ in the account. |
| `municipal_registration` | `string` | no | Municipal registration. |
| `tax_regime` | `string` | no | Tax regime. Values: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`, `null`. |
| `addresses` | `array<object>` | no |  |
| `emails` | `array<object>` | no |  |
| `phones` | `array<object>` | no |  |
| `external_id` | `string` | no | ID in your system. Unique per account. |
| `tags` | `array<string>` | no |  |
| `custom_data` | `object` | no | Custom data. |

More fields and nested objects: `kobana schema banking.financial.companies.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company updated |

## financial companies delete

Delete a Company

```text
kobana banking financial companies delete [UID] [OPTIONS]
```

`DELETE /v2/financial/companies/{uid}`

Deletes the company. Its financial accounts remain, without a company.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Company deleted |

## financial providers

List Providers

```text
kobana banking financial providers [OPTIONS]
```

`GET /v2/financial/providers`

This endpoint is used to receive the service providers that Kobana is integrated with.

A service provider is any institution regulated by BACEN, whether a commercial bank, financial institution, payment institution, payment service provider, etc.

The `slug` of the service provider is used for creating a financial account.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Providers found |

## financial statement-categories

List Statement Categories

```text
kobana banking financial statement-categories [OPTIONS]
```

`GET /v2/financial/statement_categories`

Catalog of categories that a statement transaction can carry.

Required to use the `category` filter in the listing and summary: the codes are a closed
domain, and without the catalog there is no way to discover that bank fees are code `105`.

The catalog has two natures: about 50 generic codes (`1xx` debits, `2xx` credits,
and `9999` for `others`) and hundreds of bank-specific codes, in the format
`1 + bank_number + bank_code` (e.g.: `107701`).

`kind` is derived from the slug and comes null when the category does not declare a side — which is the case
for most bank-specific codes. It is a convenience, not an exhaustive classification:
to sum inflows and outflows, use the `kind` filter from the listing or `group_by=kind`
from the summary, which operate on the actual type of the transaction.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Category Catalog |
