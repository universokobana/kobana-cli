---
id: installments
title: "kobana banking installments"
sidebar_label: "installments"
---

# kobana banking installments


Commands of `kobana banking installments`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`installments list`](#installments-list) | `GET /v1/installments` | List Carnês |
| [`installments create`](#installments-create) | `POST /v1/installments` | Create a Carnê |
| [`installments get`](#installments-get) | `GET /v1/installments/{id}` | Carnê Information |
| [`installments delete`](#installments-delete) | `DELETE /v1/installments/{id}` | Delete Carnê |

## installments list

List Carnês

```text
kobana banking installments list [OPTIONS]
```

`GET /v1/installments`

List Carnês.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing carnês |

## installments create

Create a Carnê

```text
kobana banking installments create [OPTIONS]
```

`POST /v1/installments`

Creates a carnê for generation of multiple billets with periodic due dates.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `bank_billet_account_id` | `integer` | yes | Collection wallet ID. If not provided, it will use the default wallet. |
| `amount` | `number` | yes | Amount |
| `customer_person_name` | `string` | yes | Customer name |
| `customer_cnpj_cpf` | `string` | yes | Customer CPF/CNPJ |
| `customer_state` | `string` | yes | State |
| `customer_city_name` | `string` | yes | City (Name must be correct and complete) |
| `customer_zipcode` | `string` | yes | ZIP Code (format 99999999) |
| `customer_address` | `string` | yes | Address |
| `customer_neighborhood` | `string` | yes | Neighborhood |
| `start_at` | `string (date)` | yes | Date of first charge |
| `total` | `integer` | yes | Number of installments |
| `document_type` | `string` | yes | Document type: * `Code` \| `Abbreviation` \| Description * `01` \| `CH` \| Check * `02` \| `DM` \| Commercial Draft (Default) * `03` \| `DMI` \| Commercial Draft for Indication * `04` \| `DS` \| Service Draft * `05` \| `DSI` \| Service Draft for Indication * `06` \| `DR` \| Rural Draft * `07` \| `LC` \| Bill of Exchange * `08` \| `NCC` \| Commercial Credit Note * `09` \| `NCE` \| Export Credit Note * `10` \| `NCI` \| Industrial Credit Note * `11` \| `NCR` \| Rural Credit Note * `12` \| `NP` \| Promissory note * `13` \| `NPR` \| Rural Promissory note * `14` \| `TM` \| Commercial Triplicata * `15` \| `TS` \| Service Triplicata * `16` \| `NS` \| Insurance Note * `17` \| `RC` \| Receipt * `18` \| `FAT` \| Invoice * `19` \| `ND` \| Debit Note * `20` \| `AP` \| Insurance Policy * `21` \| `ME` \| School Monthly Fee * `22` \| `PC` \| Consortium Installment * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Debt Document * `25` \| `CPR` \| Commodity Futures Note * `26` \| `CTR` \| Contract * `27` \| `CSG` \| Co-insurance * `28` \| `EC` \| Condominium Charges * `29` \| `CPS` \| Service Provision Account * `30` \| `WR` \| Warrant * `31` \| `DP` \| Installment Draft * `32` \| `CSR` \| Serial Collection * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Elementary Risks Policy * `35` \| `CC` \| Credit Card * `36` \| `BDP` \| Proposal Boleto * `37` \| `NPD` \| Direct Promissory note * `38` \| `DAE` \| State Active Debt * `39` \| `DAM` \| Municipal Active Debt * `40` \| `DAU` \| Federal Active Debt * `41` \| `CCB` \| Banking Credit Cell * `42` \| `FI` \| Financing * `43` \| `RD` \| Expense Sharing * `44` \| `DRI` \| Rural Draft for Indication * `45` \| `ECI` \| Condominium Charges for Indication * `99` \| `Others` \| Others Values: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `id` | `integer` | no | Voucher ID |
| `bank_billet_layout_id` | `integer` | no | Boleto template ID |
| `customer_id` | `integer` | no | Customer ID. When this ID is passed, the fields `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` and `customer_neighborhood` are not required. |
| `customer_address_complement` | `string` | no | Complement |
| `customer_address_number` | `string` | no | Number |
| `customer_email` | `string (email)` | no | Payer's email |
| `customer_email_cc` | `string (email)` | no | Alternative payer email |
| `customer_phone_number` | `string` | no | Phone (with area code) |
| `customer_ignore_email` | `boolean` | no | Never send email to this customer |
| `customer_ignore_sms` | `boolean` | no | Never send SMS to this customer |
| `customer_mobile_local_code` | `string` | no | Mobile area code |
| `customer_mobile_number` | `string` | no | Mobile phone |
| `customer_nickname` | `string` | no | Nickname or trade name of the payer |
| `customer_notes` | `string` | no | Payer notes |
| `customer_contact_person` | `string` | no | Contact |
| `customer_update` | `string` | no | Contact |
| `end_at` | `string (date)` | no | Date of last charge |
| `cycle` | `string` | no | Voucher cycle. Default: monthly * `biweekly` Biweekly * `bimonthly` Bimonthly * `monthly` Monthly * `quarterly` Quarterly * `semiannual` Semiannual * `annual` Annual Values: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `custom_bank_billets` | `array<object>` | no | Custom voucher. When sent, the values of amount, start_at and end_at will be ignored. |
| `bank_billet_ids` | `array<integer>` | no | IDs of boletos linked to the voucher |
| `interest_type` | `integer` | no | Interest/penalty type: * `0` Non-existent (Default) * `1` For daily percentage * `2` For daily amount Values: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | no | Number of days after the due date when the penalty will start to accrue. The default value is 1 day (the day after the due date). |
| `interest_percentage` | `number` | no | Daily interest percentage. From 0.0 to 100.0 (Ex 1.5% = 1.5) Required if interest_type equals 1. |
| `interest_value` | `number` | no | Daily interest amount (R$). Required if interest_type equals 2. |
| `interest_days_type` | `integer` | no | Type of days for interest: * `0` Calendar * `1` Business Values: `0`, `1`. |
| `fine_type` | `integer` | no | Fine type: * `0` Non-existent (Default) * `1` For percentage of boleto amount * `2` For fixed amount Values: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Number of days after the due date when the fine will start to accrue. The default value is 1 day (the day after the due date). |
| `fine_percentage` | `number` | no | Late fee percentage Ex: 2% x R$ 250.00 = R$ 5.00. Required if fine_type equals 1 |
| `fine_value` | `number` | no | Fine amount (R$). Required if fine_type equals 2. |
| `discount_type` | `integer` | no | Discount type: The discount type will be the same for all three discounts, if they exist. : * `0` Non-existent (Default) * `1` For fixed amount * `2` For percentage of boleto amount Values: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | no | Days for discount. Required if discount_type is different from 0 (zero). Cannot result in a date earlier than creation. |
| `discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the discount. Required if discount_type equals 2 |
| `discount_value` | `number` | no | Discount amount (R$). Required if discount_type equals 1. |
| `days_for_second_discount` | `integer` | no | Days for second discount. |
| `second_discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the second discount. |
| `second_discount_value` | `number` | no | Second discount amount (R$). |
| `days_for_third_discount` | `integer` | no | Days for third discount. |
| `third_discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the third discount. |
| `third_discount_value` | `number` | no | Third discount amount (R$). |
| `tags` | `array<string>` | no | Tags associated with the boleto |
| `guarantor_name` | `string` | no | Final beneficiary name (Drawer/Guarantor) |
| `guarantor_cnpj_cpf` | `string` | no | Final beneficiary CNPJ/CPF (Drawer/Guarantor) |
| `guarantor_address_number` | `string` | no | Final beneficiary number (Drawer/Guarantor) |
| `guarantor_neighborhood` | `string` | no | Final beneficiary neighborhood (Drawer/Guarantor) |
| `guarantor_phone_number` | `string` | no | Final beneficiary phone (with area code) (Drawer/Guarantor) |
| `guarantor_city_name` | `string` | no | City (Name must be correct and complete) of final beneficiary (Drawer/Guarantor) |
| `guarantor_state` | `string` | no | State of final beneficiary (Drawer/Guarantor) |
| `guarantor_zipcode` | `string` | no | Postal code (format 99999999) of final beneficiary (Drawer/Guarantor) |
| `guarantor_address` | `string` | no | Address of final beneficiary (Drawer/Guarantor) |
| `guarantor_address_complement` | `string` | no | Complement of final beneficiary (Drawer/Guarantor) |
| `description` | `string` | no | Product or service description |
| `instructions` | `string` | no | Instructions for cashier |
| `document_date` | `string (date)` | no | Document date |
| `document_number` | `string` | no | Document number, Size depends on the bank and remittance format: * Bank \|	Format\| Size \| * Abc \|`Cnab240`\|	15	\| * Ailos \|`Cnab240`\|	15  \| * Arbi \|`Cnab240`\|	15	\| * Banese \|`Cnab240`\|	15	\| * Banestes \|`Cnab400`\|	10	\| * Banrisul \|`Cnab240`\|	15	\| * Banrisul \|`Cnab400`\|	10	\| * Bb \|`Cnab240`\|	15	\| * Bb \|`Cnab400`\|	10	\| * Bib \|`Cnab240`\|	15	\| * Bnb \|`Cnab400`\|	10	\| * Bnpparibas \|`Cnab400`\|	10	\| * Bradesco \|`Cnab240`\|	15	\| * Bradesco \|`Cnab400`\|	10	\| * Brb \|`Cnab400`\|	10	\| * Caixa \|`Cnab240`\|	11	\| * Caixa \|`Cnab400`\|	10	\| * Caruana \|`Cnab400`\|	10	\| * Citibank \|`Cnab400`\|	10	\| * Credisis \|`Cnab240`\|	15	\| * Cresol \|`Cnab240`\|	10	\| * Cresol \|`Cnab400`\|	10	\| * Cresol Bradesco \|`Cnab240`\|	15	\| * Cresol Bradesco \|`Cnab400`\|	10	\| * Daycoval \|`Cnab400`\|	10	\| * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| * Mercantil \|`Cnab240`\|	10	\| * Moneyplus \|`Cnab240`\|	10	\| * Rendimento \|`Cnab400`\|	10	\| * Safra \|`Cnab400`\|	102	\| * Santander \|`Cnab240`\|	15	\| * Santander \|`Cnab400`\|	10	\| * Semear \|`Cnab400`\|	10	\| * Sicoob \|`Cnab240`\|	15	\| * Sicoob \|`Cnab400`\|	10	\| * Sicredi \|`Cnab240`\|	15	\| * Sicredi \|`Cnab400`\|	10	\| * Sofisa \|`Cnab240`\|	15	\| * Unicred \|`Cnab240`\|	15	\| * Uniprime \|`Cnab400`\|	10	\| * Uniprime99 \|`Cnab400`\|	10	\| * Santander \|`Cnab400`\|	10 |
| `days_for_revoke` | `integer` | no | Calendar days for withdrawal/return: Null/Blank: Follows the bank's default. 0: Withdrawal/Return on the same day as maturity. 1 or more: Withdrawal/Return after maturity (Maturity + X calendar days). |
| `days_for_negativation` | `integer` | no | Calendar days for negative registry: Available only for the following banks and formats. * Bank \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Yes \|	Yes	\| No * Itaú \| No \| Yes \| No |
| `days_for_sue` | `integer` | no | Calendar days for protest |
| `divergent_payment_type` | `integer` | no | Type of divergent payment: Valid only for Itaú and Caixa. * `1` Accepts any divergent amount * `2` Accepts payments within a range of amounts or percentages * `3` Does not accept divergent amount payments * `4` Accepts payments above a minimum amount or percentage Values: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Type of value to consider for payment limits: Valid only for Itaú and Caixa. * `1` Reports divergent payments by amounts * `2` Reports divergent payments by percentages Values: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Maximum value for divergent payment range. |
| `divergent_payment_minimum_value` | `number` | no | Minimum value for divergent payment range. |
| `divergent_payment_maximum_percentage` | `number` | no | Maximum percentage for divergent payment range. |
| `divergent_payment_minimum_percentage` | `number` | no | Minimum percentage for divergent payment range. |
| `divergent_payment_limit` | `integer` | no | Number of payments allowed. Required if divergent payment data is provided. Used only by Caixa. |
| `prevent_registration` | `boolean` | no | Prevent sending registration to bank: If `true`, prevents the boleto from being registered. To be used in cases where the boleto was already registered outside of Kobana but you want to include it in the system. |
| `notes` | `string` | no | Notes |
| `custom_attachment_name` | `string` | no | Name to be used in boleto files sent to the customer in notifications. Accepts use of variables. If left empty, the default is the word "boleto" followed by the ID. |
| `created_via_api` | `boolean` | no | Indicates if the boleto was created by API |
| `created_at` | `string (date-time)` | no | Boleto creation date and time |
| `updated_at` | `string (date-time)` | no | Boleto last update date and time |
| `payment_count` | `integer` | no | Number of partial payments accepted for this boleto. |
| `url` | `string` | no | URL for viewing the voucher |
| `status` | `string` | no | Voucher status: * `created` Generating * `processed` Installments created * `generated` Open Values: `created`, `processed`, `generated`. |
| `import_id` | `integer` | no | Import ID |
| `ignore_whatsapp` | `boolean` | no | Never send this voucher by WhatsApp |

More fields and nested objects: `kobana schema banking.installments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Carnê Created |
| `422` | Invalid carnê |

## installments get

Carnê Information

```text
kobana banking installments get [ID] [OPTIONS]
```

`GET /v1/installments/{id}`

Query specific information from a Carnê.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Carnê found successfully |
| `404` | Carnê not found |

## installments delete

Delete Carnê

```text
kobana banking installments delete [ID] [OPTIONS]
```

`DELETE /v1/installments/{id}`

Deletes a specific Carnê. No billet generated by the carnê will be deleted.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Carnê cancelled |
| `404` | Carnê not found |
