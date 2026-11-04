---
id: admin
title: "kobana banking admin"
sidebar_label: "admin"
description: "Administration (subaccounts, users, connections)"
---

# kobana banking admin

Administration (subaccounts, users, connections)

Commands of `kobana banking admin`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`admin certificates list`](#admin-certificates-list) | `GET /v2/admin/certificates` | List Certificates |
| [`admin certificates create`](#admin-certificates-create) | `POST /v2/admin/certificates` | Add a Certificate |
| [`admin certificates delete`](#admin-certificates-delete) | `DELETE /v2/admin/certificates/{uid}` | Delete a Certificate |
| [`admin commands list`](#admin-commands-list) | `GET /v2/admin/commands` | List Commands |
| [`admin commands get`](#admin-commands-get) | `GET /v2/admin/commands/{uid}` | View Command |
| [`admin connections associations create`](#admin-connections-associations-create) | `POST /v2/admin/connections/{uid}/associations` | Connect Service Account |
| [`admin connections associations delete`](#admin-connections-associations-delete) | `DELETE /v2/admin/connections/{uid}/associations` | Disconnect Service Account |
| [`admin connections list`](#admin-connections-list) | `GET /v2/admin/connections` | List Connections |
| [`admin connections create`](#admin-connections-create) | `POST /v2/admin/connections` | Add a Connection |
| [`admin connections get`](#admin-connections-get) | `GET /v2/admin/connections/{uid}` | Connection information |
| [`admin connections update`](#admin-connections-update) | `PUT /v2/admin/connections/{uid}` | Update Connection |
| [`admin connections delete`](#admin-connections-delete) | `DELETE /v2/admin/connections/{uid}` | Delete a Connection |
| [`admin subaccounts list`](#admin-subaccounts-list) | `GET /v2/admin/subaccounts` | List Sub-accounts |
| [`admin subaccounts create`](#admin-subaccounts-create) | `POST /v2/admin/subaccounts` | Create a Sub-account |
| [`admin subaccounts get`](#admin-subaccounts-get) | `GET /v2/admin/subaccounts/{uid}` | View a Sub-account |
| [`admin subaccounts update`](#admin-subaccounts-update) | `PUT /v2/admin/subaccounts/{uid}` | Alter Sub-account |
| [`admin users list`](#admin-users-list) | `GET /v2/admin/users` | List Users |
| [`admin users create`](#admin-users-create) | `POST /v2/admin/users` | Add a User |
| [`admin users update`](#admin-users-update) | `PUT /v2/admin/users/{uid}` | Update User Data |
| [`admin users delete`](#admin-users-delete) | `DELETE /v2/admin/users/{uid}` | Delete a User |

## admin certificates list

List Certificates

```text
kobana banking admin certificates list [OPTIONS]
```

`GET /v2/admin/certificates`

Lists all certificates registered in the account with optional pagination filters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Certificates found |

## admin certificates create

Add a Certificate

```text
kobana banking admin certificates create [OPTIONS]
```

`POST /v2/admin/certificates`

Add a certificate.

> **Certificate Type:** Depending on the type of certificate provided, it is necessary to send the files object with different contents. If the certificate type is `CRT` it is mandatory to send the `crt_file` and the `crt_private_key_file`, if the type is `PFX` it is mandatory to send `pfx_file` and `pfx_password`.

**Request body**

Sent as a `multipart/form-data` form: files go in `--file FIELD=PATH`.

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes | Name given to the certificate |
| `cnpj_cpf` | `string` | yes | CPF/CNPJ |
| `type` | `string` | yes | Certificate type: If the certificate type is `CRT` it is required to send `crt_file` and `crt_private_key_file`, if the type is `PFX` it is required `pfx_file` and `pfx_password`. * `crt` CRT * `pfx` PFX Values: `crt`, `pfx`. |
| `files` | `object` | yes | Certificate file and password. If the certificate type is `CRT` it is required to send `crt_file` and `crt_private_key_file`, if the type is `PFX` it is required `pfx_file` and `pfx_password`. |
| `subaccounts` | `boolean` | no | Indicates if the certificate is enabled for use in sub-accounts |

More fields and nested objects: `kobana schema banking.admin.certificates.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Certificate added |
| `422` | Invalid certificate data |

## admin certificates delete

Delete a Certificate

```text
kobana banking admin certificates delete [UID] [OPTIONS]
```

`DELETE /v2/admin/certificates/{uid}`

Permanently removes a certificate. It is only possible to delete certificates that are not linked to any active connection and that were not created by the system.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Certificate deleted |
| `404` | Certificate not found |
| `422` | System-created certificate cannot be deleted |

## admin commands list

List Commands

```text
kobana banking admin commands list [OPTIONS]
```

`GET /v2/admin/commands`

Lists all Commands in the account, with support for filters by status, operation, and confirmation origin.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Filter by Command status. |
| `operation` | `string` | no | Filter by operation type. |
| `confirmation_via` | `string` | no | Filter by confirmation origin. Values: `api`, `edi`. |
| `created_from` | `string` | no | Filter by creation date (start). Format: YYYY-MM-DD. |
| `created_to` | `string` | no | Filter by creation date (end). Format: YYYY-MM-DD. |
| `resource_type` | `string` | no | Filter by resource type (e.g.: `BankBillet`, `Pix`, `Financial::Transfer`). |
| `resource_uid` | `string` | no | Filter Commands of a specific resource (boleto, Pix or financial account) by its uid. |
| `financial_provider_slug` | `string` | no | Filter by financial provider slug. |
| `bank_billet_account_uid` | `string` | no | Filter Commands of boletos from a specific wallet (BankBilletAccount) by its uid. |
| `pix_account_uid` | `string` | no | Filter Commands of Pix from a specific Pix Account (PixAccount) by its uid. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of Commands |

## admin commands get

View Command

```text
kobana banking admin commands get [UID] [OPTIONS]
```

`GET /v2/admin/commands/{uid}`

Returns the details of a specific Command by its UID.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command found |
| `404` | Command not found |

## admin connections associations create

Connect Service Account

```text
kobana banking admin connections associations create [UID] [OPTIONS]
```

`POST /v2/admin/connections/{uid}/associations`

This endpoint is used to associate a Service Account with a Connection.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `resource` | `object` | yes |  |

More fields and nested objects: `kobana schema banking.admin.connections.associations.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Association created |
| `422` | Invalid association data |

## admin connections associations delete

Disconnect Service Account

```text
kobana banking admin connections associations delete [UID] [OPTIONS]
```

`DELETE /v2/admin/connections/{uid}/associations`

This endpoint is used to remove the association of a Connection with a Service Account.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `resource` | `object` | yes |  |

More fields and nested objects: `kobana schema banking.admin.connections.associations.delete`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Connection |
| `404` | Connection not found |

## admin connections list

List Connections

```text
kobana banking admin connections list [OPTIONS]
```

`GET /v2/admin/connections`

Lists all registered connections with optional filters by provider, certificate, enablement and validation status.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `provider_slug` | `string` | no | Financial provider |
| `certificate_uid` | `string` | no | Certificate UUID |
| `enabled` | `boolean` | no | Enabled connections |
| `validated` | `boolean` | no | Validated connections |
| `created_from` | `string` | no | Start date of the connection creation date filter. |
| `created_to` | `string` | no | End date of the connection creation date filter. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Connections found |

## admin connections create

Add a Connection

```text
kobana banking admin connections create [OPTIONS]
```

`POST /v2/admin/connections`

Registers a new connection with a financial provider.

> **Asynchronous Processing:** the `201` response is returned before the credentials are validated with the provider. The connection is created with `validated_at` and `validate_failed_at` null, and validation finishes in the background.

**Validation cycle**

| State | `validated_at` | `validate_failed_at` |
| --- | --- | --- |
| Validating | `null` | `null` |
| Validated | filled | `null` |
| Failed | `null` | filled |

When validation succeeds, `enabled_at` is also filled, even if the connection was created with `enabled: false`.

**Webhook Events**

- `integration.connection.db.created`: fired by this operation, with the connection not yet validated.
- `integration.connection.db.updated`: fired when validation finishes, whether it succeeds or fails. Read the outcome in `object.validated_at` and `object.validate_failed_at`: the failure does not appear in `changes`.

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.admin.connections.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Connection created |
| `422` | Invalid connection data |

## admin connections get

Connection information

```text
kobana banking admin connections get [UID] [OPTIONS]
```

`GET /v2/admin/connections/{uid}`

Retrieves a connection. Right after creation, null `validated_at` and `validate_failed_at` indicate that credential validation is still in progress (see Create a Connection).

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Connection found |
| `404` | Connection not found |

## admin connections update

Update Connection

```text
kobana banking admin connections update [UID] [OPTIONS]
```

`PUT /v2/admin/connections/{uid}`

Updates a connection. With `revalidate: true`, credential revalidation may finish in the background: in that case the response contains `validation_status: pending` and the outcome appears in `validated_at` and `validate_failed_at`, as in creation.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no | Connection Name |
| `environment` | `string` | no | Connection environment with the financial provider. `production`: Production. `homologation`: Homologation * This field will be ignored in our Sandbox environment Values: `production`, `homologation`. |
| `apis` | `array<string>` | no | APIs enabled for the financial provider. `charge/bank_billet`: boleto. `charge/pix`: Pix. `payment`: Payment. `transfer`: Transfer |
| `credentials` | `object` | no | Credentials. |
| `certificate_uid` | `string (uuid)` | no | Certificate UUID. |
| `keypair_uid` | `string (uuid)` | no | Keypair UUID. |
| `enabled` | `boolean` | no | Enable connection |
| `revalidate` | `boolean` | no | Revalidates credentials with the financial provider. If passed true, the connection will only be changed if the credentials and permissions are valid. |

More fields and nested objects: `kobana schema banking.admin.connections.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Connection updated |
| `404` | Connection not found |
| `422` | Credentials already used by another connection in the account |

## admin connections delete

Delete a Connection

```text
kobana banking admin connections delete [UID] [OPTIONS]
```

`DELETE /v2/admin/connections/{uid}`

Deletes a connection.
        The deletion of a connection will result in the removal of all its associations, making it impossible to use the previously connected API.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Connection |
| `404` | Connection not found |

## admin subaccounts list

List Sub-accounts

```text
kobana banking admin subaccounts list [OPTIONS]
```

`GET /v2/admin/subaccounts`

Lists all sub-accounts of the main account with optional filters by email, CNPJ and creation date.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `email` | `string` | no | Filter by email. Search sub-account by email |
| `business_cnpj` | `string` | no | Filter by CNPJ. Search sub-account by CNPJ |
| `created_from` | `string` | no | Start date of the sub-account creation date filter |
| `created_to` | `string` | no | End date of the sub-account creation date filter |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Sub-accounts found |

## admin subaccounts create

Create a Sub-account

```text
kobana banking admin subaccounts create [OPTIONS]
```

`POST /v2/admin/subaccounts`

Create a sub-account

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `nickname` | `string` | yes | Nickname |
| `id` | `integer` | no | ID of the sub-account (deprecated, use uid) |
| `uid` | `string` | no | UID of the sub-account |
| `parent_id` | `integer` | no | ID of the main account (deprecated, use parent_uid) |
| `parent_uid` | `string (uuid)` | no | UID of the main account |
| `email` | `string (email)` | no | Email |
| `business_name` | `string` | no | Business name |
| `business_cnpj` | `string` | no | CNPJ of the Company |
| `business_legal_name` | `string` | no | Legal name |
| `configuration` | `string (json)` | no | Account default data configuration |
| `api_access_token` | `string` | no | Access token of the sub-account |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Tags associated with the Sub-account |
| `created_via_api` | `boolean` | no | Indicates if the record was created by API |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.admin.subaccounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Sub-account created |
| `422` | Invalid sub-account data |

## admin subaccounts get

View a Sub-account

```text
kobana banking admin subaccounts get [UID] [OPTIONS]
```

`GET /v2/admin/subaccounts/{uid}`

View a sub-account

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Sub-account found |
| `404` | Sub-account not found |

## admin subaccounts update

Alter Sub-account

```text
kobana banking admin subaccounts update [UID] [OPTIONS]
```

`PUT /v2/admin/subaccounts/{uid}`

Alter Sub-account. It is not possible to change the CNPJ of the sub-account.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `enabled` | `integer` | no | Indicates if the sub-account is active or inactive. Use `1` to activate and `0` to deactivate. Values: `0`, `1`. |
| `id` | `integer` | no | ID of the sub-account (deprecated, use uid) |
| `uid` | `string` | no | UID of the sub-account |
| `parent_id` | `integer` | no | ID of the main account (deprecated, use parent_uid) |
| `parent_uid` | `string (uuid)` | no | UID of the main account |
| `email` | `string (email)` | no | Email |
| `business_cnpj` | `string` | no | CNPJ of the Company |
| `nickname` | `string` | no | Nickname |
| `business_legal_name` | `string` | no | Legal name |
| `account_type` | `string` | no | Type of Account: * `individual` Individual * `juridical` Legal Entity Values: `individual`, `juridical`. |
| `first_name` | `string` | no | First Name of the Responsible |
| `middle_name` | `string` | no | Middle Name of the Responsible |
| `last_name` | `string` | no | Last Name of the Responsible |
| `full_name` | `string` | no | Full name of the Responsible |
| `gender` | `string` | no | Gender |
| `cpf` | `string` | no | CPF |
| `address_street_name` | `string` | no | Address |
| `address_state` | `string` | no | State |
| `address_neighborhood` | `string` | no | Neighborhood |
| `address_postal_code` | `string` | no | ZIP code |
| `address_number` | `string` | no | Number |
| `address_complement` | `string` | no | Complement |
| `address_city_name` | `string` | no | City |
| `phone_number` | `string` | no | Phone |
| `date_of_birth` | `string (date)` | no | Date of Birth |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Tags associated with the Sub-account |
| `business_category` | `integer` | no | Category: \| Category Code \| Description \| \|---\|---\| \| 1000 \| Art and collectible objects \| \| 1001 \| Babies \| \| 1002 \| Beauty and fragrances \| \| 1022 \| Toys and hobbies \| \| 1015 \| Home and garden \| \| 1006 \| Computers \| \| 1004 \| Business to business (B2B) \| \| 1007 \| Education \| \| 1008 \| Electronics and telecommunications \| \| 1009 \| Entertainment and media \| \| 1021 \| Sports and outdoor activities \| \| 1013 \| Government \| \| 1003 \| Books and magazines \| \| 1012 \| Gifts and flowers \| \| 1018 \| Religion and spirituality (for profit) \| \| 1014 \| Health and personal care \| \| 1016 \| Non-profit \| \| 1025 \| Service \| \| 1020 \| Services - Other \| \| 1010 \| Financial services and products \| \| 1019 \| Retail (not classified elsewhere) \| \| 1011 \| Retail and food services \| \| 1024 \| Vehicle sales \| \| 1005 \| Clothing \| \| 1023 \| Travel \| Values: `null`, `1000`, `1001`, `1002`, `1022`, `1015`, `1006`, `1004`, `1007`, `1008`, `1009`, `1021`, `1013`, `1003`, `1012`, `1018`, `1014`, `1016`, `1025`, `1020`, `1010`, `1019`, `1011`, `1024`, `1005`, `1023`. |
| `business_subcategory` | `integer` | no | Subcategory: \| Category Code \| Subcategory Code \| Description \| \|---\|---\|---\| \| 1017 \| 2173 \| Rare or special pet animals \| \| 1017 \| 2171 \| Medications and supplements \| \| 1017 \| 2172 \| Pet shops, pet food and supplies \| \| 1017 \| 2174 \| Veterinary services \| \| 1000 \| 2000 \| Antiques \| \| 1000 \| 2004 \| Digital art \| \| 1000 \| 2007 \| Sewing, embroidery and fabrics \| \| 1000 \| 2003 \| Cameras and photographic supplies \| \| 1000 \| 2005 \| Souvenirs \| \| 1000 \| 2006 \| Music store - instruments and sheet music \| \| 1000 \| 2002 \| Merchants and galleries \| \| 1000 \| 2009 \| Stationery, printing and writing paper \| \| 1000 \| 2008 \| Stamps and coins \| \| 1000 \| 2001 \| Craft supplies \| \| 1000 \| 2010 \| Vintage and collectible items \| \| 1001 \| 2012 \| Furniture \| \| 1001 \| 2013 \| Baby products - Other \| \| 1001 \| 2014 \| Health and safety \| \| 1001 \| 2011 \| Clothing \| \| 1002 \| 2015 \| Bath and body \| \| 1002 \| 2016 \| Fragrances and perfumes \| \| 1002 \| 2017 \| Makeup and cosmetics \| \| 1022 \| 2252 \| Handicrafts \| \| 1022 \| 2253 \| Cameras and photographic supplies \| \| 1022 \| 2255 \| Souvenirs \| \| 1022 \| 2256 \| Music store - instruments and sheet music \| \| 1022 \| 2254 \| Hobby shops, toys and games \| \| 1022 \| 2258 \| Stationery, printing and writing paper \| \| 1022 \| 2257 \| Stamps and coins \| \| 1022 \| 2260 \| Video games and systems \| \| 1022 \| 2259 \| Vintage and collectible items \| \| 1015 \| 2145 \| Antiques \| \| 1015 \| 2146 \| Appliances \| \| 1015 \| 2148 \| Bed and bath \| \| 1015 \| 2150 \| Curtains and tapestry \| \| 1015 \| 2157 \| Home decoration \| \| 1015 \| 2162 \| Security and surveillance equipment \| \| 1015 \| 2152 \| Fireplaces and fireplace screens \| \| 1015 \| 2147 \| Merchants and galleries \| \| 1015 \| 2149 \| Building materials \| \| 1015 \| 2156 \| Machinery and tools \| \| 1015 \| 2153 \| Furniture \| \| 1015 \| 2160 \| Landscaping \| \| 1015 \| 2163 \| Pools and spas \| \| 1015 \| 2151 \| Pest extermination and disinfection services \| \| 1015 \| 2154 \| Garden supplies \| \| 1015 \| 2161 \| Carpets and rugs \| \| 1015 \| 2159 \| Kitchen utensils \| \| 1015 \| 2158 \| Household utensils \| \| 1015 \| 2155 \| Glass, paint and wallpaper \| \| 1006 \| 2057 \| Computers and data processing services \| \| 1006 \| 2059 \| Digital content \| \| 1006 \| 2058 \| Desktops, laptops and notebooks \| \| 1006 \| 2069 \| Web hosting and design \| \| 1006 \| 2064 \| Online games \| \| 1006 \| 2062 \| Monitors and projectors \| \| 1006 \| 2066 \| Peripherals \| \| 1006 \| 2065 \| Parts and accessories \| \| 1006 \| 2063 \| Communication network \| \| 1006 \| 2060 \| E-commerce services \| \| 1006 \| 2061 \| Repair and maintenance services \| \| 1006 \| 2068 \| Training services \| \| 1006 \| 2067 \| Software \| \| 1004 \| 2027 \| Agricultural \| \| 1004 \| 2047 \| Wholesale \| \| 1004 \| 2031 \| Construction \| \| 1004 \| 2025 \| Accounting \| \| 1004 \| 2043 \| Publishing and printing \| \| 1004 \| 2045 \| Packaging and shipping \| \| 1004 \| 2030 \| Commercial photography, art and graphics \| \| 1004 \| 2038 \| Direct mail \| \| 1004 \| 2039 \| Marketing \| \| 1004 \| 2040 \| Multi-level marketing \| \| 1004 \| 2041 \| Office and commercial furniture \| \| 1004 \| 2029 \| Chemicals and related products \| \| 1004 \| 2026 \| Advertising \| \| 1004 \| 2028 \| Architecture, engineering and prospecting services \| \| 1004 \| 2032 \| Consulting services \| \| 1004 \| 2044 \| Quick copy and reproduction services \| \| 1004 \| 2046 \| Stenography and secretarial services \| \| 1004 \| 2034 \| Equipment leasing and rental services \| \| 1004 \| 2036 \| Recruitment and selection services \| \| 1004 \| 2035 \| Equipment repair services \| \| 1004 \| 2033 \| Educational services \| \| 1004 \| 2042 \| Office supplies and equipment \| \| 1004 \| 2037 \| Industrial and production supplies \| \| 1007 \| 2073 \| Gyms, studios and dance schools \| \| 1007 \| 2070 \| Commercial and secretarial schools \| \| 1007 \| 2074 \| Elementary and high schools \| \| 1007 \| 2075 \| Vocational and commercial schools \| \| 1007 \| 2072 \| Colleges and universities \| \| 1007 \| 2071 \| Daily childcare services \| \| 1008 \| 2079 \| General electronic accessories \| \| 1008 \| 2085 \| Telephone cards \| \| 1008 \| 2076 \| Cameras, camcorders and equipment \| \| 1008 \| 2081 \| Residential electronics \| \| 1008 \| 2083 \| Telecommunications equipment and sales \| \| 1008 \| 2082 \| Security and surveillance \| \| 1008 \| 2084 \| Telecommunication services \| \| 1008 \| 2077 \| Car audio and electronics \| \| 1008 \| 2078 \| Cell phones, PDAs and pagers \| \| 1008 \| 2080 \| Residential audio \| \| 1009 \| 2097 \| Program animators \| \| 1009 \| 2094 \| Toys and games \| \| 1009 \| 2095 \| Slot machines \| \| 1009 \| 2096 \| Digital content \| \| 1009 \| 2091 \| Adult digital content \| \| 1009 \| 2088 \| Movies - DVDs, video tapes \| \| 1009 \| 2087 \| Cinema tickets \| \| 1009 \| 2093 \| Theater tickets \| \| 1009 \| 2092 \| Concert tickets \| \| 1009 \| 2098 \| Game \| \| 1009 \| 2099 \| Online games \| \| 1009 \| 2086 \| Souvenirs \| \| 1009 \| 2089 \| Music - CDs, cassettes and albums \| \| 1009 \| 2090 \| Cable, satellite and other paid TV and radio \| \| 1009 \| 2100 \| Video games and systems \| \| 1021 \| 2241 \| Gyms, studios and dance schools \| \| 1021 \| 2244 \| Firearm accessories \| \| 1021 \| 2248 \| Martial arts weapons \| \| 1021 \| 2245 \| Firearms \| \| 1021 \| 2237 \| Sports shoes \| \| 1021 \| 2240 \| Camping and outdoor activities \| \| 1021 \| 2246 \| Hunting \| \| 1021 \| 2243 \| Watch gear \| \| 1021 \| 2250 \| Sports equipment \| \| 1021 \| 2242 \| Exercise and physical conditioning \| \| 1021 \| 2247 \| Knives \| \| 1021 \| 2249 \| Sports games and toys \| \| 1021 \| 2238 \| Bike shop, service and repair \| \| 1021 \| 2251 \| Pools and spas \| \| 1021 \| 2239 \| Rowing, sailing and accessories \| \| 1013 \| 2137 \| Government services (not classified elsewhere) \| \| 1003 \| 2019 \| Digital content \| \| 1003 \| 2023 \| Publishing and printing \| \| 1003 \| 2021 \| Fiction and non-fiction \| \| 1003 \| 2020 \| Textbooks and educational books \| \| 1003 \| 2018 \| Audiobooks \| \| 1003 \| 2024 \| Rare and used books \| \| 1003 \| 2022 \| Magazines \| \| 1012 \| 2134 \| Gourmet foods \| \| 1012 \| 2132 \| Florists \| \| 1012 \| 2133 \| Gift, card, novelty and souvenir stores \| \| 1012 \| 2136 \| Party supplies \| \| 1012 \| 2135 \| Nurseries and flower shops \| \| 1018 \| 2176 \| Merchandise \| \| 1018 \| 2177 \| Services (not classified elsewhere) \| \| 1018 \| 2175 \| Association services \| \| 1014 \| 2141 \| Medical assistance \| \| 1014 \| 2140 \| Dental care \| \| 1014 \| 2142 \| Medical equipment and supplies \| \| 1014 \| 2138 \| Pharmacy (excluding prescription drugs) \| \| 1014 \| 2139 \| Pharmacy (including prescription drugs) \| \| 1014 \| 2143 \| Ophthalmology \| \| 1014 \| 2144 \| Vitamins and supplements \| \| 1016 \| 2169 \| Educational \| \| 1016 \| 2164 \| Charitable entity \| \| 1016 \| 2167 \| Other \| \| 1016 \| 2168 \| Personal \| \| 1016 \| 2165 \| Political \| \| 1016 \| 2166 \| Religious \| \| 1025 \| 2297 \| Accessories \| \| 1025 \| 2296 \| Truck and utility trailer rental \| \| 1025 \| 2293 \| Trailer and recreational vehicle rental \| \| 1025 \| 2291 \| Boat rental and leasing \| \| 1025 \| 2294 \| Tools and equipment \| \| 1025 \| 2292 \| Car washing \| \| 1025 \| 2288 \| Car rental \| \| 1025 \| 2284 \| New motor vehicle parts and supplies \| \| 1025 \| 2285 \| Used motor vehicle parts \| \| 1025 \| 2287 \| Auto repair and painting \| \| 1025 \| 2295 \| Towing service \| \| 1025 \| 2289 \| Auto services \| \| 1025 \| 2290 \| Automotive tire supply and services \| \| 1025 \| 2286 \| Audio and video \| \| 1020 \| 2232 \| Alterations and adjustments \| \| 1020 \| 2216 \| Medical assistance \| \| 1020 \| 2188 \| Carpentry \| \| 1020 \| 2217 \| Clubs and member organizations \| \| 1020 \| 2192 \| Computers and data processing services \| \| 1020 \| 2225 \| Radio, television and stereo repair \| \| 1020 \| 2226 \| Real estate broker \| \| 1020 \| 2197 \| Dental care \| \| 1020 \| 2205 \| Graphic and commercial design \| \| 1020 \| 2230 \| Packaging and shipping \| \| 1020 \| 2204 \| General contractors \| \| 1020 \| 2220 \| Online dating \| \| 1020 \| 2200 \| Entertainment \| \| 1020 \| 2222 \| Photography studios - portraits \| \| 1020 \| 2221 \| Photo finishing \| \| 1020 \| 2191 \| Commercial photography \| \| 1020 \| 2227 \| Property management \| \| 1020 \| 2207 \| IDs, certificates and passports \| \| 1020 \| 2208 \| Import and export \| \| 1020 \| 2203 \| Game \| \| 1020 \| 2190 \| Cleaning and maintenance \| \| 1020 \| 2215 \| Lotteries and contests \| \| 1020 \| 2218 \| Miscellaneous publishing and printing \| \| 1020 \| 2219 \| Moving and storage \| \| 1020 \| 2235 \| Ophthalmology \| \| 1020 \| 2212 \| Landscaping and horticulture \| \| 1020 \| 2202 \| Event and wedding planning \| \| 1020 \| 2185 \| Advertising \| \| 1020 \| 2199 \| Small appliance and electrical device repair \| \| 1020 \| 2228 \| Furniture and upholstery repairs \| \| 1020 \| 2236 \| Watch and jewelry repairs \| \| 1020 \| 2210 \| Insurance - auto and homeowner \| \| 1020 \| 2211 \| Insurance - life and annuity \| \| 1020 \| 2214 \| Local delivery service \| \| 1020 \| 2233 \| Telecommunication service \| \| 1020 \| 2229 \| Services (not classified elsewhere) \| \| 1020 \| 2198 \| E-commerce services \| \| 1020 \| 2195 \| Counseling services \| \| 1020 \| 2187 \| Career services \| \| 1020 \| 2194 \| Consulting services \| \| 1020 \| 2189 \| Childcare services \| \| 1020 \| 2224 \| Quick copy and reproduction services \| \| 1020 \| 2196 \| Delivery services \| \| 1020 \| 2201 \| Equipment leasing and rental services \| \| 1020 \| 2231 \| Pool services \| \| 1020 \| 2223 \| Protection and security services \| \| 1020 \| 2209 \| Information recovery services \| \| 1020 \| 2193 \| Computer network services \| \| 1020 \| 2186 \| Services and shopping clubs \| \| 1020 \| 2213 \| Legal services and lawyers \| \| 1020 \| 2206 \| Health and beauty spas \| \| 1020 \| 2234 \| Utilities \| \| 1010 \| 2122 \| Securities agents and brokers \| \| 1010 \| 2115 \| Mortgage agents or brokers \| \| 1010 \| 2103 \| Commodity and futures exchange \| \| 1010 \| 2118 \| Pre-paid and stored value cards \| \| 1010 \| 2109 \| Financial company \| \| 1010 \| 2110 \| Financial and investment advisory \| \| 1010 \| 2101 \| Accounting \| \| 1010 \| 2106 \| Credit union \| \| 1010 \| 2119 \| Real estate broker \| \| 1010 \| 2107 \| Brokers and foreign exchange \| \| 1010 \| 2108 \| Securities custody \| \| 1010 \| 2114 \| Financial services company \| \| 1010 \| 2104 \| Consumer credit information companies \| \| 1010 \| 2120 \| Delivery \| \| 1010 \| 2102 \| Collection agency \| \| 1010 \| 2121 \| Property management \| \| 1010 \| 2113 \| Investments - general \| \| 1010 \| 2116 \| Online gaming currency \| \| 1010 \| 2117 \| Advance payment or salary loan \| \| 1010 \| 2111 \| Insurance - auto and homeowner \| \| 1010 \| 2112 \| Insurance - life and annuity \| \| 1010 \| 2105 \| Debt guidance service \| \| 1010 \| 2123 \| Electronic transfer and payment order \| \| 1019 \| 2181 \| Durable goods \| \| 1019 \| 2182 \| Non-durable goods \| \| 1019 \| 2179 \| Department store \| \| 1019 \| 2180 \| Discount store \| \| 1019 \| 2183 \| Used goods and second-hand store \| \| 1019 \| 2184 \| Variety store \| \| 1019 \| 2178 \| Chemicals and related products \| \| 1011 \| 2124 \| Alcoholic beverages \| \| 1011 \| 2126 \| Coffee and tea \| \| 1011 \| 2127 \| Gourmet foods \| \| 1011 \| 2130 \| Tobacco \| \| 1011 \| 2128 \| Grocery and specialty stores \| \| 1011 \| 2129 \| Restaurant \| \| 1011 \| 2125 \| Buffet services \| \| 1011 \| 2131 \| Vitamins and supplements \| \| 1024 \| 2277 \| Aviation \| \| 1024 \| 2283 \| Vintage and collectible items \| \| 1024 \| 2275 \| Car dealer - new and used \| \| 1024 \| 2276 \| Car dealer - used only \| \| 1024 \| 2278 \| Boat dealer \| \| 1024 \| 2280 \| Motorcycle dealer \| \| 1024 \| 2279 \| Trailer dealer \| \| 1024 \| 2281 \| Utility and recreational trailer dealer \| \| 1024 \| 2282 \| Recreational vehicle dealer \| \| 1005 \| 2053 \| Accessories \| \| 1005 \| 2055 \| Wholesale - precious stones and metals \| \| 1005 \| 2056 \| Fashion jewelry \| \| 1005 \| 2050 \| Women's clothing \| \| 1005 \| 2048 \| Children's clothing \| \| 1005 \| 2049 \| Men's clothing \| \| 1005 \| 2051 \| Shoes \| \| 1005 \| 2052 \| Military and civil service uniforms \| \| 1005 \| 2054 \| Retail - fine jewelry and watches \| \| 1023 \| 2268 \| Sports and recreation camping \| \| 1023 \| 2274 \| Travel agency \| \| 1023 \| 2264 \| Cruises \| \| 1023 \| 2265 \| Lodging and accommodations \| \| 1023 \| 2261 \| Airline \| \| 1023 \| 2263 \| Bus line \| \| 1023 \| 2262 \| Car rental \| \| 1023 \| 2266 \| Luggage and leather goods \| \| 1023 \| 2272 \| Trailer parks or campgrounds \| \| 1023 \| 2270 \| Shared leisure properties \| \| 1023 \| 2267 \| Recreation services \| \| 1023 \| 2273 \| Transportation services - other \| \| 1023 \| 2269 \| Taxis and limousines \| \| 1023 \| 2271 \| Travel \| Values: `null`, `2173`, `2171`, `2172`, `2174`, `2000`, `2004`, `2007`, `2003`, `2005`, `2006`, `2002`, `2009`, `2008`, `2001`, `2010`, `2012`, `2013`, `2014`, `2011`, `2015`, `2016`, `2017`, `2252`, `2253`, `2255`, `2256`, `2254`, `2258`, `2257`, `2260`, `2259`, `2145`, `2146`, `2148`, `2150`, `2157`, `2162`, `2152`, `2147`, `2149`, `2156`, `2153`, `2160`, `2163`, `2151`, `2154`, `2161`, `2159`, `2158`, `2155`, `2057`, `2059`, `2058`, `2069`, `2064`, `2062`, `2066`, `2065`, `2063`, `2060`, `2061`, `2068`, `2067`, `2027`, `2047`, `2031`, `2025`, `2043`, `2045`, `2030`, `2038`, `2039`, `2040`, `2041`, `2029`, `2026`, `2028`, `2032`, `2044`, `2046`, `2034`, `2036`, `2035`, `2033`, `2042`, `2037`, `2073`, `2070`, `2074`, `2075`, `2072`, `2071`, `2079`, `2085`, `2076`, `2081`, `2083`, `2082`, `2084`, `2077`, `2078`, `2080`, `2097`, `2094`, `2095`, `2096`, `2091`, `2088`, `2087`, `2093`, `2092`, `2098`, `2099`, `2086`, `2089`, `2090`, `2100`, `2241`, `2244`, `2248`, `2245`, `2237`, `2240`, `2246`, `2243`, `2250`, `2242`, `2247`, `2249`, `2238`, `2251`, `2239`, `2137`, `2019`, `2023`, `2021`, `2020`, `2018`, `2024`, `2022`, `2134`, `2132`, `2133`, `2136`, `2135`, `2176`, `2177`, `2175`, `2141`, `2140`, `2142`, `2138`, `2139`, `2143`, `2144`, `2169`, `2164`, `2167`, `2168`, `2165`, `2166`, `2297`, `2296`, `2293`, `2291`, `2294`, `2292`, `2288`, `2284`, `2285`, `2287`, `2295`, `2289`, `2290`, `2286`, `2232`, `2216`, `2188`, `2217`, `2192`, `2225`, `2226`, `2197`, `2205`, `2230`, `2204`, `2220`, `2200`, `2222`, `2221`, `2191`, `2227`, `2207`, `2208`, `2203`, `2190`, `2215`, `2218`, `2219`, `2235`, `2212`, `2202`, `2185`, `2199`, `2228`, `2236`, `2210`, `2211`, `2214`, `2233`, `2229`, `2198`, `2195`, `2187`, `2194`, `2189`, `2224`, `2196`, `2201`, `2231`, `2223`, `2209`, `2193`, `2186`, `2213`, `2206`, `2234`, `2122`, `2115`, `2103`, `2118`, `2109`, `2110`, `2101`, `2106`, `2119`, `2107`, `2108`, `2114`, `2104`, `2120`, `2102`, `2121`, `2113`, `2116`, `2117`, `2111`, `2112`, `2105`, `2123`, `2181`, `2182`, `2179`, `2180`, `2183`, `2184`, `2178`, `2124`, `2126`, `2127`, `2130`, `2128`, `2129`, `2125`, `2131`, `2277`, `2283`, `2275`, `2276`, `2278`, `2280`, `2279`, `2281`, `2282`, `2053`, `2055`, `2056`, `2050`, `2048`, `2049`, `2051`, `2052`, `2054`, `2268`, `2274`, `2264`, `2265`, `2261`, `2263`, `2262`, `2266`, `2272`, `2270`, `2267`, `2273`, `2269`, `2271`. |
| `business_website` | `string` | no | Website |
| `business_name` | `string` | no | Business name |
| `business_type` | `string` | no | Company Type: * `mei` Individual Microentrepreneur * `individual` Micro Enterprise/Small Business * `proprietorship` Limited Liability Company * `partnership` Closed Capital Joint-Stock Company * `corporation` Open Capital Joint-Stock Company * `nonprofit` Non-profit * `government` Governmental Values: `null`, `mei`, `individual`, `proprietorship`, `partnership`, `corporation`, `nonprofit`, `government`. |
| `mother_name` | `string` | no | Mother's Name |
| `father_name` | `string` | no | Father's Name |
| `configuration` | `string (json)` | no | Default data configuration for boleto |
| `api_access_token` | `string` | no | Access token of the sub-account |
| `created_via_api` | `boolean` | no | Indicates if the record was created by API |
| `billing_email` | `string` | no | Billing Email |
| `simple_opting` | `boolean` | no | Opted for Simples |
| `rate_limits` | `object` | no | Request limits - Hash with key and value in JSON format. |

More fields and nested objects: `kobana schema banking.admin.subaccounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Sub-account altered |
| `404` | Sub-account not found |
| `422` | Invalid parameters |

## admin users list

List Users

```text
kobana banking admin users list [OPTIONS]
```

`GET /v2/admin/users`

Lists all users registered in the account, with option to filter by email. Supports pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `email` | `string` | no | Filter by email. Search user by email |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Users found |

## admin users create

Add a User

```text
kobana banking admin users create [OPTIONS]
```

`POST /v2/admin/users`

Add a user

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | `integer` | no | ID of the user (deprecated, use uid) |
| `uid` | `string` | no | Public UID of the user |
| `intercom_id` | `string` | no | ID of the user used for identification in Intercom |
| `email` | `string (email)` | no | User's email |
| `email_verified` | `boolean` | no | Indicates whether the user's email has already been confirmed (email scope) |
| `phone` | `string` | no | Registered phone (phone scope) |
| `first_name` | `string` | no | First name |
| `middle_name` | `string` | no | Middle name |
| `last_name` | `string` | no | Last name |
| `full_name` | `string` | no | Full name |
| `created_via_api` | `boolean` | no | Indicates if it was created via API |
| `permissions` | `array<string>` | no | User permissions |
| `created_at` | `string (date-time)` | no | Date of creation |
| `updated_at` | `string (date-time)` | no | Date of update |
| `enabled_at` | `string (date-time)` | no | Date of enablement |
| `legal_data` | `object` | no | Legal data (civil name and CPF) |
| `profile_photo_url` | `string (uri)` | no | URL of profile photo (ActiveStorage) or Gravatar as fallback |
| `gender` | `string` | no | Gender identity (undisclosed, woman, man, non_binary, other_identity or free value — profile.identity scope) |
| `birthday` | `string (date)` | no | Date of birth |
| `languages` | `array<string>` | no | User's preferred communication languages (profile.communication scope) |
| `email_unsubscribed_at` | `string (date-time)` | no | Date the user opted out of all email communications (general opt-out), null if never opted out (profile.communication scope) |
| `timezone` | `string` | no | User's timezone (profile.preferences scope) |
| `accessibility_resources` | `array<string>` | no | Accessibility resources used on the platform (profile.accessibility scope) |
| `disability_disclosure` | `array<string>` | no | Disabilities reported by the user, optional (profile.accessibility scope) |

More fields and nested objects: `kobana schema banking.admin.users.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | User added |
| `422` | Invalid user data |

## admin users update

Update User Data

```text
kobana banking admin users update [UID] [OPTIONS]
```

`PUT /v2/admin/users/{uid}`

Updates the information of an existing user, including name, email and permissions.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | `integer` | no | ID of the user (deprecated, use uid) |
| `uid` | `string` | no | Public UID of the user |
| `intercom_id` | `string` | no | ID of the user used for identification in Intercom |
| `email` | `string (email)` | no | User's email |
| `email_verified` | `boolean` | no | Indicates whether the user's email has already been confirmed (email scope) |
| `phone` | `string` | no | Registered phone (phone scope) |
| `first_name` | `string` | no | First name |
| `middle_name` | `string` | no | Middle name |
| `last_name` | `string` | no | Last name |
| `full_name` | `string` | no | Full name |
| `created_via_api` | `boolean` | no | Indicates if it was created via API |
| `permissions` | `array<string>` | no | User permissions |
| `created_at` | `string (date-time)` | no | Date of creation |
| `updated_at` | `string (date-time)` | no | Date of update |
| `enabled_at` | `string (date-time)` | no | Date of enablement |
| `legal_data` | `object` | no | Legal data (civil name and CPF) |
| `profile_photo_url` | `string (uri)` | no | URL of profile photo (ActiveStorage) or Gravatar as fallback |
| `gender` | `string` | no | Gender identity (undisclosed, woman, man, non_binary, other_identity or free value — profile.identity scope) |
| `birthday` | `string (date)` | no | Date of birth |
| `languages` | `array<string>` | no | User's preferred communication languages (profile.communication scope) |
| `email_unsubscribed_at` | `string (date-time)` | no | Date the user opted out of all email communications (general opt-out), null if never opted out (profile.communication scope) |
| `timezone` | `string` | no | User's timezone (profile.preferences scope) |
| `accessibility_resources` | `array<string>` | no | Accessibility resources used on the platform (profile.accessibility scope) |
| `disability_disclosure` | `array<string>` | no | Disabilities reported by the user, optional (profile.accessibility scope) |

More fields and nested objects: `kobana schema banking.admin.users.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | User Altered |
| `404` | User not found |
| `422` | Invalid parameters |

## admin users delete

Delete a User

```text
kobana banking admin users delete [UID] [OPTIONS]
```

`DELETE /v2/admin/users/{uid}`

Permanently removes a user from the system.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | User deleted |
| `404` | User not found |
