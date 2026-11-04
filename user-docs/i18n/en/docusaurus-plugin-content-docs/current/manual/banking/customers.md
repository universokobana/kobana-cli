---
id: customers
title: "kobana banking customers"
sidebar_label: "customers"
---

# kobana banking customers


Commands of `kobana banking customers`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`customers cnpj-cpf`](#customers-cnpj-cpf) | `GET /v1/customers/cnpj_cpf` | Search Customer by CNPJ/CPF |
| [`customers email`](#customers-email) | `GET /v1/customers/email` | Search Customer by Email |
| [`customers people-sync`](#customers-people-sync) | `PUT /v1/customers/people_sync` | Enable or disable the Customer &lt;-> Person mirror |
| [`customers list`](#customers-list) | `GET /v1/customers` | List Customers |
| [`customers create`](#customers-create) | `POST /v1/customers` | Create a Customer |
| [`customers get`](#customers-get) | `GET /v1/customers/{id}` | View Customer |
| [`customers update`](#customers-update) | `PUT /v1/customers/{id}` | Update Customer |

## customers cnpj-cpf

Search Customer by CNPJ/CPF

```text
kobana banking customers cnpj-cpf [OPTIONS]
```

`GET /v1/customers/cnpj_cpf`

View a customer by searching for cnpj or cpf.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `q` | `string` | yes | Customer CNPJ/CPF |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Customer found |
| `404` | Customer not found |

## customers email

Search Customer by Email

```text
kobana banking customers email [OPTIONS]
```

`GET /v1/customers/email`

View a customer by searching for email.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `q` | `string` | yes | Customer email |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Customer found |
| `404` | Customer not found |

## customers people-sync

Enable or disable the Customer &lt;-> Person mirror

```text
kobana banking customers people-sync [OPTIONS]
```

`PUT /v1/customers/people_sync`

Turns on or off, for the account, the automatic mirror between Customers and People (each Customer gets an equivalent Person and changes propagate both ways). It only changes the setting: existing Customers do not get a Person until the backfill runs. In a sub-account, `enabled` in the response also reflects inheritance from the main account.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `enabled` | `boolean` | yes | true turns the mirror on; false turns it off. |

More fields and nested objects: `kobana schema banking.customers.people-sync`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Configuration updated |
| `422` | Missing or invalid enabled parameter |

## customers list

List Customers

```text
kobana banking customers list [OPTIONS]
```

`GET /v1/customers`

List customers.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing customers |

## customers create

Create a Customer

```text
kobana banking customers create [OPTIONS]
```

`POST /v1/customers`

Creates a new customer for association with collections, billets and subscriptions.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_name` | `string` | yes | Name or Legal Name |
| `address` | `string` | yes | Address |
| `neighborhood` | `string` | yes | Neighborhood |
| `zipcode` | `string` | yes | ZIP Code (format 99999999) |
| `city_name` | `string` | yes | City (Name must be correct and complete) |
| `id` | `integer` | no | ID |
| `external_code` | `string` | no | External code |
| `cnpj_cpf` | `string` | no | CPF/CNPJ |
| `contact_person` | `string` | no | Contact person (when company) |
| `nickname` | `string` | no | Nickname or trade name of the payer |
| `birthday` | `string (date)` | no | Date of Birth |
| `email` | `string (email)` | no | Main email |
| `email_cc` | `string (email)` | no | Alternative email |
| `address_complement` | `string` | no | Address complement |
| `address_number` | `string` | no | Number |
| `state` | `string` | no | State |
| `phone_number` | `string` | no | Phone (with area code, international code is optional) |
| `mobile_local_code` | `string` | no | Mobile area code |
| `mobile_number` | `string` | no | Mobile phone |
| `ignore_email` | `boolean` | no | Never send email to this customer |
| `ignore_sms` | `boolean` | no | Never send SMS to this customer |
| `ignore_whatsapp` | `boolean` | no | Never send WhatsApp to this customer |
| `notes` | `string` | no | Notes |
| `tags` | `array<string>` | no | Tags associated with the record |
| `tag_list` | `string` | no | Tags associated with the record |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `created_via_api` | `boolean` | no | Indicates if the record was created by API |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.customers.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Customer created |
| `422` | Invalid customer |

## customers get

View Customer

```text
kobana banking customers get [ID] [OPTIONS]
```

`GET /v1/customers/{id}`

Query specific information from a Customer.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Customer found |
| `404` | Customer not found |

## customers update

Update Customer

```text
kobana banking customers update [ID] [OPTIONS]
```

`PUT /v1/customers/{id}`

Updates specific information of a Customer.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_name` | `string` | yes | Name or Legal Name |
| `address` | `string` | yes | Address |
| `neighborhood` | `string` | yes | Neighborhood |
| `zipcode` | `string` | yes | ZIP Code (format 99999999) |
| `city_name` | `string` | yes | City (Name must be correct and complete) |
| `id` | `integer` | no | ID |
| `external_code` | `string` | no | External code |
| `contact_person` | `string` | no | Contact person (when company) |
| `nickname` | `string` | no | Nickname or trade name of the payer |
| `birthday` | `string (date)` | no | Date of Birth |
| `email` | `string (email)` | no | Main email |
| `email_cc` | `string (email)` | no | Alternative email |
| `address_complement` | `string` | no | Address complement |
| `address_number` | `string` | no | Number |
| `state` | `string` | no | State |
| `phone_number` | `string` | no | Phone (with area code, international code is optional) |
| `mobile_local_code` | `string` | no | Mobile area code |
| `mobile_number` | `string` | no | Mobile phone |
| `ignore_email` | `boolean` | no | Never send email to this customer |
| `ignore_sms` | `boolean` | no | Never send SMS to this customer |
| `ignore_whatsapp` | `boolean` | no | Never send WhatsApp to this customer |
| `notes` | `string` | no | Notes |
| `tags` | `array<string>` | no | Tags associated with the record |
| `tag_list` | `string` | no | Tags associated with the record |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `created_via_api` | `boolean` | no | Indicates if the record was created by API |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.customers.update`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Customer updated |
| `404` | Customer not found |
