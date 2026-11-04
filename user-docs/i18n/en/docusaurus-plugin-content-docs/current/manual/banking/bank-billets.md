---
id: bank-billets
title: "kobana banking bank-billets"
sidebar_label: "bank-billets"
---

# kobana banking bank-billets


Commands of `kobana banking bank-billets`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`bank-billets protest update`](#bank-billets-protest-update) | `PUT /v1/bank_billets/{id}/protest/{type}` | Protest Billet |
| [`bank-billets list`](#bank-billets-list) | `GET /v1/bank_billets` | List Billets |
| [`bank-billets create`](#bank-billets-create) | `POST /v1/bank_billets` | Create a Billet |
| [`bank-billets cancel-all`](#bank-billets-cancel-all) | `POST /v1/bank_billets/cancel_all` | Cancel Billets in Batch |
| [`bank-billets get`](#bank-billets-get) | `GET /v1/bank_billets/{id}` | View the Billet |
| [`bank-billets update`](#bank-billets-update) | `PUT /v1/bank_billets/{id}` | Update the Billet |
| [`bank-billets cancel`](#bank-billets-cancel) | `PUT /v1/bank_billets/{id}/cancel` | Cancel the Billet |
| [`bank-billets duplicate`](#bank-billets-duplicate) | `POST /v1/bank_billets/{id}/duplicate` | Duplicate Billet |
| [`bank-billets pay`](#bank-billets-pay) | `PUT /v1/bank_billets/{id}/pay` | Mark Billet as Paid |
| [`bank-billets send-email`](#bank-billets-send-email) | `POST /v1/bank_billets/{id}/send_email` | Send Billet by Email |
| [`bank-billets send-sms`](#bank-billets-send-sms) | `POST /v1/bank_billets/{id}/send_sms` | Send Billet by SMS |

## bank-billets protest update

Protest Billet

```text
kobana banking bank-billets protest update [ID] [TYPE] [OPTIONS]
```

`PUT /v1/bank_billets/{id}/protest/{type}`

Protests or cancels the protest of an overdue billet. The parameter :type in the route is mandatory: use "send" to protest the billet or "cancel" to cancel an existing protest.

**Path arguments:** `id`, `type`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Billet protested successfully |
| `404` | Billet not found |
| `422` | Invalid protest type |

## bank-billets list

List Billets

```text
kobana banking bank-billets list [OPTIONS]
```

`GET /v1/bank_billets`

List billets.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `bank_billet_account_id` | `string` | no | Filter by Wallet ID. Find all records that have the ID. |
| `status` | `string` | no | Filter by Status. Find all records with the status. Values: `generating`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `our_number` | `string` | no | Filter by Our Number |
| `processed_our_number_raw` | `string` | no | Filter by Our Number with DV (clean). |
| `cnpj_cpf` | `string` | no | Filter by CPF/CNPJ of the client. Must be formatted with mask. Find all records whose field matches the string. |
| `created_from` | `string` | no | Start date for creation date filter. |
| `created_to` | `string` | no | End date for creation date filter. |
| `expire_from` | `string` | no | Start date for due date filter |
| `expire_to` | `string` | no | End date for due date filter. |
| `paid_from` | `string` | no | Start date for payment date filter. |
| `paid_to` | `string` | no | End date for payment date filter. |
| `registered_to` | `string` | no | Start date for registration date filter. |
| `registered_from` | `string` | no | End date for registration date filter. |
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `updated_from` | `string` | no | Start date for update date filter. |
| `updated_to` | `string` | no | End date for update date filter. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing billets |

## bank-billets create

Create a Billet

```text
kobana banking bank-billets create [OPTIONS]
```

`POST /v1/bank_billets`

Creates a new bank billet for collection. The billet will be processed and registered at the financial institution according to the collection wallet settings used.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount |
| `expire_at` | `string (date)` | yes | Due date |
| `customer_person_name` | `string` | yes | Customer name |
| `customer_cnpj_cpf` | `string` | yes | Customer CPF/CNPJ |
| `customer_state` | `string` | yes | State |
| `customer_city_name` | `string` | yes | City (Name must be correct and complete) |
| `customer_zipcode` | `string` | yes | ZIP Code (format 99999999) |
| `customer_address` | `string` | yes | Address |
| `customer_neighborhood` | `string` | yes | Neighborhood |
| `id` | `integer` | no | Boleto ID |
| `bank_billet_account_id` | `integer` | no | Collection wallet ID. If not provided, it will use the default wallet. |
| `bank_billet_layout_id` | `integer` | no | Boleto template ID |
| `customer_id` | `integer` | no | Customer ID. When this ID is passed, the fields `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` and `customer_neighborhood` are not required. |
| `customer_address_complement` | `string` | no | Complement |
| `customer_address_number` | `string` | no | Number |
| `customer_email` | `string (email)` | no | Payer's email |
| `customer_email_cc` | `string (email)` | no | Alternative payer email |
| `customer_phone_number` | `string` | no | Phone (with area code, international code is optional) |
| `customer_ignore_email` | `boolean` | no | Never send email to this customer |
| `customer_ignore_sms` | `boolean` | no | Never send SMS to this customer |
| `customer_mobile_local_code` | `string` | no | Mobile area code |
| `customer_mobile_number` | `string` | no | Mobile phone |
| `customer_nickname` | `string` | no | Nickname or trade name of the payer |
| `customer_notes` | `string` | no | Payer notes |
| `customer_contact_person` | `string` | no | Contact |
| `interest_type` | `integer` | no | Interest/penalty type: * `0` Non-existent (Default) * `1` For daily percentage * `2` For daily amount * `7` For monthly percentage - Supported banks: Bradesco, BB, BTG, Caixa, Inter, Itaú, Safra, Santander, Sicoob and Sicredi Values: `0`, `1`, `2`, `7`. |
| `days_for_interest` | `integer` | no | Number of days after the due date when the penalty will start to accrue. The default value is 1 day (the day after the due date). |
| `interest_percentage` | `number` | no | Interest percentage. From 0.0 to 100.0 (Ex 1.5% = 1.5) Required if interest_type equals 1 or 7. Up to 2 decimal places. |
| `interest_value` | `number` | no | Daily interest amount (R$). Required if interest_type equals 2. Up to 2 decimal places. |
| `interest_days_type` | `integer` | no | Type of days for interest: * `0` Calendar * `1` Business Values: `0`, `1`. |
| `fine_type` | `integer` | no | Fine type: * `0` Non-existent (Default) * `1` For percentage of boleto amount * `2` For fixed amount Values: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Number of days after the due date when the fine will start to accrue. The default value is 1 day (the day after the due date). |
| `fine_percentage` | `number` | no | Late fee percentage Ex: 2% x R$ 250.00 = R$ 5.00. Required if fine_type equals 1. Up to 2 decimal places. |
| `fine_value` | `number` | no | Fine amount (R$). Required if fine_type equals 2. Up to 2 decimal places. |
| `discount_type` | `integer` | no | Discount type: The discount type will be the same for all three discounts, if they exist. : * `0` Non-existent (Default) * `1` For fixed amount * `2` For percentage of boleto amount Values: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | no | Days for discount. Required if discount_type is different from 0 (zero) |
| `discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the discount. Required if discount_type equals 2 |
| `discount_value` | `number` | no | Discount amount (R$). Required if discount_type equals 1. |
| `days_for_second_discount` | `integer` | no | Days for second discount. |
| `second_discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the second discount. |
| `second_discount_value` | `number` | no | Second discount amount (R$). |
| `days_for_third_discount` | `integer` | no | Days for third discount. |
| `third_discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the third discount. |
| `third_discount_value` | `number` | no | Third discount amount (R$). |
| `tags` | `array<string>` | no | Tags associated with the boleto |
| `tag_list` | `string` | no | Tags associated with the boleto |
| `charge_type` | `integer` | no | Charge type: * `1` Simple * `2` Linked * `3` Discounted * `4` Vendor Values: `1`, `2`, `3`, `4`. |
| `dispatch_type` | `integer` | no | Charge type: When the boleto needs to be sent by mail. It is necessary to contract the service with the bank and you will pay a fee. * `1` Customer * `2` Bank Values: `1`, `2`. |
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
| `document_type` | `string` | no | Document type: * `Code` \| `Abbreviation` \| Description * `01` \| `CH` \| Check * `02` \| `DM` \| Commercial Draft (Default) * `03` \| `DMI` \| Commercial Draft for Indication * `04` \| `DS` \| Service Draft * `05` \| `DSI` \| Service Draft for Indication * `06` \| `DR` \| Rural Draft * `07` \| `LC` \| Bill of Exchange * `08` \| `NCC` \| Commercial Credit Note * `09` \| `NCE` \| Export Credit Note * `10` \| `NCI` \| Industrial Credit Note * `11` \| `NCR` \| Rural Credit Note * `12` \| `NP` \| Promissory note * `13` \| `NPR` \| Rural Promissory note * `14` \| `TM` \| Commercial Triplicata * `15` \| `TS` \| Service Triplicata * `16` \| `NS` \| Insurance Note * `17` \| `RC` \| Receipt * `18` \| `FAT` \| Invoice * `19` \| `ND` \| Debit Note * `20` \| `AP` \| Insurance Policy * `21` \| `ME` \| School Monthly Fee * `22` \| `PC` \| Consortium Installment * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Debt Document * `25` \| `CPR` \| Commodity Futures Note * `26` \| `CTR` \| Contract * `27` \| `CSG` \| Co-insurance * `28` \| `EC` \| Condominium Charges * `29` \| `CPS` \| Service Provision Account * `30` \| `WR` \| Warrant * `31` \| `DP` \| Installment Draft * `32` \| `CSR` \| Serial Collection * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Elementary Risks Policy * `35` \| `CC` \| Credit Card * `36` \| `BDP` \| Proposal Boleto * `37` \| `NPD` \| Direct Promissory note * `38` \| `DAE` \| State Active Debt * `39` \| `DAM` \| Municipal Active Debt * `40` \| `DAU` \| Federal Active Debt * `41` \| `CCB` \| Banking Credit Cell * `42` \| `FI` \| Financing * `43` \| `RD` \| Expense Sharing * `44` \| `DRI` \| Rural Draft for Indication * `45` \| `ECI` \| Condominium Charges for Indication * `99` \| `Others` \| Others Values: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `document_type_label` | `string` | no | Document type (Abbreviation) |
| `document_number` | `string` | no | Document number, Size depends on the bank and remittance format: * Bank \|	Format\| Size \| * Abc \|`Cnab240`\|	15	\| * Ailos \|`Cnab240`\|	15  \| * Arbi \|`Cnab240`\|	15	\| * Banese \|`Cnab240`\|	15	\| * Banestes \|`Cnab400`\|	10	\| * Banrisul \|`Cnab240`\|	15	\| * Banrisul \|`Cnab400`\|	10	\| * Bb \|`Cnab240`\|	15	\| * Bb \|`Cnab400`\|	10	\| * Bib \|`Cnab240`\|	15	\| * Bnb \|`Cnab400`\|	10	\| * Bnpparibas \|`Cnab400`\|	10	\| * Bradesco \|`Cnab240`\|	15	\| * Bradesco \|`Cnab400`\|	10	\| * Brb \|`Cnab400`\|	10	\| * Caixa \|`Cnab240`\|	11	\| * Caixa \|`Cnab400`\|	10	\| * Caruana \|`Cnab400`\|	10	\| * Citibank \|`Cnab400`\|	10	\| * Credisis \|`Cnab240`\|	15	\| * Cresol \|`Cnab240`\|	10	\| * Cresol \|`Cnab400`\|	10	\| * Cresol Bradesco \|`Cnab240`\|	15	\| * Cresol Bradesco \|`Cnab400`\|	10	\| * Daycoval \|`Cnab400`\|	10	\| * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| * Mercantil \|`Cnab240`\|	10	\| * Moneyplus \|`Cnab240`\|	10	\| * Rendimento \|`Cnab400`\|	10	\| * Safra \|`Cnab400`\|	102	\| * Santander \|`Cnab240`\|	15	\| * Santander \|`Cnab400`\|	10	\| * Semear \|`Cnab400`\|	10	\| * Sicoob \|`Cnab240`\|	15	\| * Sicoob \|`Cnab400`\|	10	\| * Sicredi \|`Cnab240`\|	15	\| * Sicredi \|`Cnab400`\|	10	\| * Sofisa \|`Cnab240`\|	15	\| * Unicred \|`Cnab240`\|	15	\| * Uniprime \|`Cnab400`\|	10	\| * Uniprime99 \|`Cnab400`\|	10	\| * Santander \|`Cnab400`\|	10 |
| `acceptance` | `string` | no | Acceptance: * `N` No (Default) * `S` Yes Values: `N`, `S`. |
| `our_number` | `string` | no | Our Number. If not provided, it will use the Next Our Number from the Collection Wallet. |
| `processed_our_number` | `string` | no | Our Number calculated with check digit (formatted) |
| `processed_our_number_raw` | `string` | no | Our Number calculated with check digit (clean) |
| `paid_amount` | `number` | no | Amount paid |
| `paid_at` | `string (date)` | no | Payment date |
| `days_for_revoke` | `integer` | no | Calendar days for withdrawal/return: Null/Blank: Follows the bank's default. 0: Withdrawal/Return on the same day as maturity. 1 or more: Withdrawal/Return after maturity (Maturity + X calendar days). |
| `credit_bureau` | `integer` | no | Credit Bureau/Negative Registry Agency. `0`: Serasa. `1`: Quod. `2`: SPC Options available for each supported bank: * Banco do Brasil: Serasa and Quod. Values: `null`, `0`, `1`, `2`. |
| `days_for_negativation` | `integer` | no | Calendar days for negative registry: Available only for the following banks and formats. * Bank \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Yes \|	Yes	\| No * Itaú \| No \| Yes \| No |
| `days_for_sue_type` | `integer` | no | Type of days for protest: * `0` Calendar * `1` Business Values: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | no | Calendar days for protest |
| `sue_history` | `array<object>` | no | History of boleto protest events |
| `sue_code` | `string` | no | Protest code (CNAB 240). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `revoke_code` | `string` | no | Withdrawal code (CNAB 240). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `first_instruction` | `string` | no | First instruction (CNAB 400). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `second_instruction` | `string` | no | Second instruction (CNAB 400). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `watermark` | `boolean` | no | Address |
| `payment_count` | `integer` | no | Number of partial payments accepted for this boleto. |
| `divergent_payment_type` | `integer` | no | Type of divergent payment: Valid only for Itaú and Caixa. * `1` Accepts any divergent amount * `2` Accepts payments within a range of amounts or percentages * `3` Does not accept divergent amount payments * `4` Accepts payments above a minimum amount or percentage Values: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Type of value to consider for payment limits: Valid only for Itaú and Caixa. * `1` Reports divergent payments by amounts * `2` Reports divergent payments by percentages Values: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Maximum value for divergent payment range. |
| `divergent_payment_minimum_value` | `number` | no | Minimum value for divergent payment range. |
| `divergent_payment_maximum_percentage` | `number` | no | Maximum percentage for divergent payment range. |
| `divergent_payment_minimum_percentage` | `number` | no | Minimum percentage for divergent payment range. |
| `divergent_payment_limit` | `integer` | no | Number of payments allowed. Required if divergent payment data is provided. Used only by Caixa. |
| `prevent_registration` | `boolean` | no | Prevent sending registration to bank: If `true`, prevents the boleto from being registered. To be used in cases where the boleto was already registered outside of Kobana but you want to include it in the system. |
| `issued_at` | `string (date-time)` | no | Boleto issuance date. Accepted only when `prevent_registration: true`. |
| `control_number` | `string` | no | Control number: May contain any information of interest to the Company. The information contained in this field will always be returned with the respective title in the return file. |
| `ignore_email` | `boolean` | no | Do not send this boleto by email |
| `ignore_sms` | `boolean` | no | Never send this boleto by SMS |
| `ignore_whatsapp` | `boolean` | no | Never send this boleto by WhatsApp |
| `addons` | `string (jsonb)` | no | Address |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `meta` | `object` | no | Hash with key and value in JSON format. |
| `notes` | `string` | no | Notes |
| `custom_attachment_name` | `string` | no | Name to be used in boleto files sent to the customer in notifications. Accepts use of variables. If left empty, the default is the word "boleto" followed by the ID. |
| `split_payment` | `boolean` | no | Payment split. Valid only for ABC Brasil, Bradesco and Itaú. If true, the boleto split will be registered. Provide the split accounts in `split_accounts`. For Itaú, you must provide the type of payment split in `split_payment_type` |
| `split_payment_type` | `integer` | no | Type of payment split: Valid only for Itau. Used only with Payment split true. * `1` Credit split by percentage (%) – Nominal value of the title * `2` Credit split in amount (R$) – Nominal value of the title * `3` Credit split by percentage (%) – Net amount received * `4` Credit split in amount (R$) – Net amount received, split proportionally Values: `1`, `2`, `3`, `4`. |
| `split_accounts` | `array<object>` | no | Accounts for payment split. |
| `payment_place` | `string` | no | Payment location |
| `installment_id` | `integer` | no | Carnê ID |
| `installment_number` | `integer` | no | Installment number of the voucher |
| `installment_total` | `integer` | no | Total installments of the voucher |
| `customer_subscription_id` | `integer` | no | Subscription ID |
| `beneficiary_name` | `string` | no | Beneficiary Name |
| `beneficiary_cnpj_cpf` | `string` | no | CNPJ/CPF of Beneficiary |
| `beneficiary_address` | `string` | no | Beneficiary address |
| `beneficiary_assignor_code` | `string` | no | Agency/Beneficiary code |
| `bank_contract_slug` | `string` | no | Wallet slug |
| `agency_number` | `string` | no | Agency |
| `agency_digit` | `string` | no | Branch digit |
| `account_number` | `string` | no | Account |
| `account_digit` | `string` | no | Account digit |
| `extra1` | `string` | no | Extra field 1 |
| `extra1_digit` | `string` | no | Digit of extra field 1 |
| `extra2` | `string` | no | Extra field 2 |
| `extra2_digit` | `string` | no | Digit of extra field 2 |
| `created_via_api` | `boolean` | no | Indicates if the boleto was created by API |
| `created_at` | `string (date-time)` | no | Boleto creation date and time |
| `updated_at` | `string (date-time)` | no | Boleto last update date and time |
| `registration_status` | `string` | no | Registration status at the bank: * `pending` Pending * `skipped` Skipped * `requested` Requested * `confirmed` Confirmed * `rejected` Rejected (will be retried) * `failed` Failed (will not be retried) Values: `pending`, `skipped`, `requested`, `confirmed`, `rejected`, `failed`. |
| `registered_at` | `string (date-time)` | no | Registration date and time (when confirmed) |
| `register_type` | `integer` | no | Registration type: * `1` API * `2` Bank Values: `1`, `2`. |
| `cancel_type` | `integer` | no | Cancellation type: * `1` Customer * `2` Bank Values: `null`, `1`, `2`. |
| `cancellation_reason` | `integer` | no | Cancellation reason: * `1` Other * `2` Fraud * `3` Death * `4` Operational error * `5` Settlement paid Values: `null`, `1`, `2`, `3`, `4`, `5`. |
| `line` | `string` | no | Digitizable Line |
| `barcode` | `string` | no | Bar code |
| `shorten_url` | `string` | no | URL for viewing the boleto |
| `url` | `string` | no | URL for viewing the boleto |
| `carne_url` | `string` | no | URL for viewing the voucher (When it is an installment) |
| `formats` | `object` | no | URLs with available formats. Ex.: PDF, Image, Pix etc |
| `pix_enabled` | `boolean` | no | Indicates if the boleto is hybrid and has Pix QR code |
| `pix_qrcode` | `string` | no | Hybrid boleto Pix QR code |
| `pix_txid` | `string` | no | Pix txid field. Automatically generated by default if not provided. |
| `prevent_pix` | `boolean` | no | If true, prevents Pix creation for wallets with Pix enabled. It is not considered if the wallet does not have Pix enabled. |
| `status` | `string` | no | Boleto status: * `generating` Generating * `draft` Draft * `generation_failed` Generation failed * `opened` Open * `canceled` Canceled * `paid` Paid * `overdue` Overdue * `validation_failed` Invalid * `chargeback` Charged back Values: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `recipient_account` | `string` | no | Recipient account + check digit |
| `reduction_type` | `integer` | no | Type of reduction: `1`: Amount. `2`: Percentage. `3`: No reduction Values: `1`, `2`, `3`. |
| `reduction_amount` | `number` | no | Reduction amount. Required if reduction_type equals 1. |
| `reduction_percentage` | `number` | no | Reduction percentage. Ex: 2% x R$ 250.00 = R$ 5.00. Required if reduction_type equals 2. Up to 2 decimal places. |
| `instructions_mode` | `integer` | no | Date calculation in the cashier instruction: * `0` Do not fill in cashier instructions * `1` Calculate date through registration method (API/CNAB) * `2` Use the date configured in the boleto Values: `0`, `1`, `2`. |
| `import_id` | `integer` | no | Import ID |
| `virtual_bank_billet_id` | `integer` | no | Boleto ID generated by BackOffice contract member. (BackOffice must be enabled). |
| `external_id` | `string` | no | Boleto ID in the customer's system. Optional for internal control and search. |
| `financial_provider_external_id` | `string` | no | ID in the financial institution |
| `after_create` | `array<string>` | no | Automatic execution of commands after the boleto is created. Allowed values: * `sync`: Synchronization with the financial provider. |

More fields and nested objects: `kobana schema banking.bank-billets.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Billet created |
| `422` | Itau partial payment requires specific authorization type |

## bank-billets cancel-all

Cancel Billets in Batch

```text
kobana banking bank-billets cancel-all [OPTIONS]
```

`POST /v1/bank_billets/cancel_all`

Cancel billets in batch

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Boleto status: * `generating` Generating * `draft` Draft * `generation_failed` Generation failed * `opened` Open * `canceled` Canceled * `paid` Paid * `overdue` Overdue * `validation_failed` Invalid * `chargeback` Charged back Values: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `expire_from` | `string (date)` | no | From due date |
| `expire_to` | `string (date)` | no | Until the due date |
| `cnpj_cpf` | `string` | no | Payer CNPJ or CPF |
| `bank_billet_ids` | `array<integer>` | no | IDs of boletos to be canceled |

More fields and nested objects: `kobana schema banking.bank-billets.cancel-all`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Request completed successfully |
| `422` | Unprocessable Entity |

## bank-billets get

View the Billet

```text
kobana banking bank-billets get [ID] [OPTIONS]
```

`GET /v1/bank_billets/{id}`

Query specific information from a Billet.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Billet found |
| `404` | Billet not found |

## bank-billets update

Update the Billet

```text
kobana banking bank-billets update [ID] [OPTIONS]
```

`PUT /v1/bank_billets/{id}`

Updates specific information of a Billet.

You can modify billets with status Opened (`opened`) or Overdue (`overdue`)

You will receive `generating` and `opened` webhooks for the modified billet.

In registered wallets, the modification will enter the remittance and a bank fee may be charged.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | no | Boleto amount |
| `expire_at` | `string (date)` | no | Due date |
| `tags` | `array<string>` | no | Tags associated with the boleto |
| `notes` | `string` | no | Notes |
| `custom_data` | `object` | no | Hash with key and value in JSON format. Can be changed in any boleto status. |
| `days_for_sue_type` | `integer` | no | Type of days for protest: * `0` Calendar * `1` Business Values: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | no | Calendar days for protest |
| `days_for_revoke` | `integer` | no | Calendar days for withdrawal Supported banks: Itaú |
| `sue_code` | `string` | no | Protest code (CNAB 240 only). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `instructions` | `string` | no | Instructions for cashier |
| `description` | `string` | no | Product or service description |
| `interest_type` | `integer` | no | Interest/penalty type: * `0` Non-existent (Default) * `1` For daily percentage * `2` For daily amount * `7` For monthly percentage - Supported banks: Bradesco, BB, BTG, Caixa, Inter, Itaú, Safra, Santander, Sicoob and Sicredi Values: `0`, `1`, `2`, `7`. |
| `days_for_interest` | `integer` | no | Number of days after the due date when the penalty will start to accrue. The default value is 1 day (the day after the due date). |
| `interest_percentage` | `number` | no | Interest percentage. From 0.0 to 100.0 (Ex 1.5% = 1.5) Required if interest_type equals 1 or 7. Up to 2 decimal places. |
| `interest_value` | `number` | no | Daily interest amount (R$). Required if interest_type equals 2. Up to 2 decimal places. |
| `discount_type` | `integer` | no | Discount type: The discount type will be the same for all three discounts, if they exist. : * `0` Non-existent (Default) * `1` For fixed amount * `2` For percentage of boleto amount Values: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | no | Days for discount. Required if discount_type is different from 0 (zero) |
| `discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the discount. Required if discount_type equals 2 |
| `discount_value` | `number` | no | Discount amount (R$). Required if discount_type equals 1. |
| `days_for_second_discount` | `integer` | no | Days for second discount. |
| `second_discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the second discount. |
| `second_discount_value` | `number` | no | Second discount amount (R$). |
| `days_for_third_discount` | `integer` | no | Days for third discount. |
| `third_discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the third discount. |
| `third_discount_value` | `number` | no | Third discount amount (R$). |
| `reduction_type` | `integer` | no | Type of reduction: `1`: Amount. `2`: Percentage. `3`: No reduction Values: `1`, `2`, `3`. |
| `reduction_amount` | `number` | no | Reduction amount. Required if reduction_type equals 1. |
| `reduction_percentage` | `number` | no | Reduction percentage. Ex: 2% x R$ 250.00 = R$ 5.00. Required if reduction_type equals 2. Up to 2 decimal places. |
| `divergent_payment_type` | `integer` | no | Type of divergent payment: * `1` Accepts any divergent amount * `2` Accepts payments within a range of amounts or percentages * `3` Does not accept divergent amount payments * `4` Accepts payments above a minimum amount or percentage Supported banks: Itaú Values: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Type of value to consider for payment limits: Valid only for Itaú and Caixa. * `1` Reports divergent payments by amounts * `2` Reports divergent payments by percentages Supported banks: Itaú Values: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Maximum value for divergent payment range. Supported banks: Itaú |
| `divergent_payment_minimum_value` | `number` | no | Minimum value for divergent payment range. Supported banks: Itaú |
| `divergent_payment_maximum_percentage` | `number` | no | Maximum percentage for divergent payment range. Supported banks: Itaú |
| `divergent_payment_minimum_percentage` | `number` | no | Minimum percentage for divergent payment range. Supported banks: Itaú |
| `fine_type` | `integer` | no | Fine type: * `0` Non-existent (Default) * `1` For percentage of boleto amount * `2` For fixed amount Values: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Number of days after the due date when the fine will start to accrue. The default value is 1 day (the day after the due date). |
| `credit_bureau` | `integer` | no | Credit Bureau/Negative Registry Agency. `0`: Serasa. `1`: Quod. `2`: SPC Options available for each supported bank: * Banco do Brasil: Serasa and Quod. Values: `null`, `0`, `1`, `2`. |
| `days_for_negativation` | `integer` | no | Number of days after maturity to register negative on the title. |
| `fine_percentage` | `number` | no | Late fee percentage Ex: 2% x R$ 250.00 = R$ 5.00. Required if fine_type equals 1. Up to 2 decimal places. |
| `fine_value` | `number` | no | Fine amount (R$). Required if fine_type equals 2. Up to 2 decimal places. |
| `document_number` | `string` | no | Document number, Size depends on the bank, format and sending method (API / EDI): * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| |
| `customer_person_name` | `string` | no | Customer name. Supported banks: Santander |
| `customer_cnpj_cpf` | `string` | no | Customer CPF/CNPJ. Supported banks: Santander |
| `customer_state` | `string` | no | State. Supported banks: Santander |
| `customer_city_name` | `string` | no | City (Name must be correct and complete). Supported banks: Santander |
| `customer_zipcode` | `string` | no | Postal code (format 99999999). Supported banks: Santander |
| `customer_address` | `string` | no | Address. Supported banks: Santander |
| `customer_neighborhood` | `string` | no | Neighborhood. Supported banks: Santander |
| `guarantor_name` | `string` | no | Final beneficiary name (Drawer/Guarantor). Supported banks: Santander |
| `guarantor_cnpj_cpf` | `string` | no | Final beneficiary CNPJ/CPF (Drawer/Guarantor). Supported banks: Santander |
| `document_type` | `string` | no | Document type: * `Code` \| `Abbreviation` \| Description * `01` \| `CH` \| Check * `02` \| `DM` \| Commercial Draft (Default) * `03` \| `DMI` \| Commercial Draft for Indication * `04` \| `DS` \| Service Draft * `05` \| `DSI` \| Service Draft for Indication * `06` \| `DR` \| Rural Draft * `07` \| `LC` \| Bill of Exchange * `08` \| `NCC` \| Commercial Credit Note * `09` \| `NCE` \| Export Credit Note * `10` \| `NCI` \| Industrial Credit Note * `11` \| `NCR` \| Rural Credit Note * `12` \| `NP` \| Promissory note * `13` \| `NPR` \| Rural Promissory note * `14` \| `TM` \| Commercial Triplicata * `15` \| `TS` \| Service Triplicata * `16` \| `NS` \| Insurance Note * `17` \| `RC` \| Receipt * `18` \| `FAT` \| Invoice * `19` \| `ND` \| Debit Note * `20` \| `AP` \| Insurance Policy * `21` \| `ME` \| School Monthly Fee * `22` \| `PC` \| Consortium Installment * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Debt Document * `25` \| `CPR` \| Commodity Futures Note * `26` \| `CTR` \| Contract * `27` \| `CSG` \| Co-insurance * `28` \| `EC` \| Condominium Charges * `29` \| `CPS` \| Service Provision Account * `30` \| `WR` \| Warrant * `31` \| `DP` \| Installment Draft * `32` \| `CSR` \| Serial Collection * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Elementary Risks Policy * `35` \| `CC` \| Credit Card * `36` \| `BDP` \| Proposal Boleto * `37` \| `NPD` \| Direct Promissory note * `38` \| `DAE` \| State Active Debt * `39` \| `DAM` \| Municipal Active Debt * `40` \| `DAU` \| Federal Active Debt * `41` \| `CCB` \| Banking Credit Cell * `42` \| `FI` \| Financing * `43` \| `RD` \| Expense Sharing * `44` \| `DRI` \| Rural Draft for Indication * `45` \| `ECI` \| Condominium Charges for Indication * `99` \| `Others` \| Others Supported banks: Santander Values: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `control_number` | `string` | no | Control number: May contain any information of interest to the Company. The information contained in this field will always be returned with the respective title in the return file. Supported banks: Santander |
| `payment_count` | `integer` | no | Number of partial payments accepted for this boleto. Supported banks: Santander |
| `issued_at` | `string (date)` | no | Issuance date. Supported banks: Santander |
| `bolepix_key` | `string` | no | Bolepix key. Supported banks: Santander |
| `pix_txid` | `string` | no | Pix TxId. Supported banks: Santander |

More fields and nested objects: `kobana schema banking.bank-billets.update`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Billet updated |
| `404` | Billet not found |
| `422` | Fields not supported by the bank |

## bank-billets cancel

Cancel the Billet

```text
kobana banking bank-billets cancel [ID] [OPTIONS]
```

`PUT /v1/bank_billets/{id}/cancel`

Cancels billets with status Opened (opened) or Overdue (overdue)

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `cancellation_reason` | `integer` | no | Cancellation reason: * `1` Other * `2` Fraud * `3` Death * `4` Operational error * `5` Settlement paid Values: `1`, `2`, `3`, `4`, `5`. |

More fields and nested objects: `kobana schema banking.bank-billets.cancel`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Billet cancelled |
| `404` | Billet not found |
| `422` | Billet cannot be cancelled |

## bank-billets duplicate

Duplicate Billet

```text
kobana banking bank-billets duplicate [ID] [OPTIONS]
```

`POST /v1/bank_billets/{id}/duplicate`

Duplicates billet allowing modification of new billet information and/or cancellation of the duplicated billet

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `expire_at_in_days` | `integer` | no | Number of days until maturity from today. Default: 7 |
| `amount` | `number` | no | New boleto amount |
| `cancel` | `boolean` | no | Cancel the boleto that is being duplicated. Default: true |
| `with_fines` | `boolean` | no | Update the new boleto amount with interest and fine. Default: false |
| `bank_billet_account_id` | `integer` | no | Collection wallet ID. If not provided, it will use the default wallet. |
| `bank_billet_layout_id` | `integer` | no | Boleto template ID |
| `customer_id` | `integer` | no | Customer ID. When this ID is passed, the fields `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` and `customer_neighborhood` are not required. |
| `customer_person_name` | `string` | no | Customer name |
| `customer_cnpj_cpf` | `string` | no | Customer CPF/CNPJ |
| `customer_state` | `string` | no | State |
| `customer_city_name` | `string` | no | City (Name must be correct and complete) |
| `customer_zipcode` | `string` | no | ZIP Code (format 99999999) |
| `customer_address` | `string` | no | Address |
| `customer_address_complement` | `string` | no | Complement |
| `customer_address_number` | `string` | no | Number |
| `customer_email` | `string (email)` | no | Payer's email |
| `customer_email_cc` | `string (email)` | no | Alternative payer email |
| `customer_neighborhood` | `string` | no | Neighborhood |
| `customer_phone_number` | `string` | no | Phone (with area code, international code is optional) |
| `customer_ignore_email` | `boolean` | no | Never send email to this customer |
| `customer_ignore_sms` | `boolean` | no | Never send SMS to this customer |
| `customer_mobile_local_code` | `string` | no | Mobile area code |
| `customer_mobile_number` | `string` | no | Mobile phone |
| `customer_nickname` | `string` | no | Nickname or trade name of the payer |
| `customer_notes` | `string` | no | Payer notes |
| `customer_contact_person` | `string` | no | Contact |
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
| `days_for_discount` | `integer` | no | Days for discount. Required if discount_type is different from 0 (zero) |
| `discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the discount. Required if discount_type equals 2 |
| `discount_value` | `number` | no | Discount amount (R$). Required if discount_type equals 1. |
| `days_for_second_discount` | `integer` | no | Days for second discount. |
| `second_discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the second discount. |
| `second_discount_value` | `number` | no | Second discount amount (R$). |
| `days_for_third_discount` | `integer` | no | Days for third discount. |
| `third_discount_percentage` | `number` | no | Percentage of the boleto amount equivalent to the third discount. |
| `third_discount_value` | `number` | no | Third discount amount (R$). |
| `tags` | `array<string>` | no | Tags associated with the boleto |
| `tag_list` | `string` | no | Tags associated with the boleto |
| `charge_type` | `integer` | no | Charge type: * `1` Simple * `2` Linked * `3` Discounted * `4` Vendor Values: `1`, `2`, `3`, `4`. |
| `dispatch_type` | `integer` | no | Charge type: When the boleto needs to be sent by mail. It is necessary to contract the service with the bank and you will pay a fee. * `1` Customer * `2` Bank Values: `1`, `2`. |
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
| `document_type` | `string` | no | Document type: * `Code` \| `Abbreviation` \| Description * `01` \| `CH` \| Check * `02` \| `DM` \| Commercial Draft (Default) * `03` \| `DMI` \| Commercial Draft for Indication * `04` \| `DS` \| Service Draft * `05` \| `DSI` \| Service Draft for Indication * `06` \| `DR` \| Rural Draft * `07` \| `LC` \| Bill of Exchange * `08` \| `NCC` \| Commercial Credit Note * `09` \| `NCE` \| Export Credit Note * `10` \| `NCI` \| Industrial Credit Note * `11` \| `NCR` \| Rural Credit Note * `12` \| `NP` \| Promissory note * `13` \| `NPR` \| Rural Promissory note * `14` \| `TM` \| Commercial Triplicata * `15` \| `TS` \| Service Triplicata * `16` \| `NS` \| Insurance Note * `17` \| `RC` \| Receipt * `18` \| `FAT` \| Invoice * `19` \| `ND` \| Debit Note * `20` \| `AP` \| Insurance Policy * `21` \| `ME` \| School Monthly Fee * `22` \| `PC` \| Consortium Installment * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Debt Document * `25` \| `CPR` \| Commodity Futures Note * `26` \| `CTR` \| Contract * `27` \| `CSG` \| Co-insurance * `28` \| `EC` \| Condominium Charges * `29` \| `CPS` \| Service Provision Account * `30` \| `WR` \| Warrant * `31` \| `DP` \| Installment Draft * `32` \| `CSR` \| Serial Collection * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Elementary Risks Policy * `35` \| `CC` \| Credit Card * `36` \| `BDP` \| Proposal Boleto * `37` \| `NPD` \| Direct Promissory note * `38` \| `DAE` \| State Active Debt * `39` \| `DAM` \| Municipal Active Debt * `40` \| `DAU` \| Federal Active Debt * `41` \| `CCB` \| Banking Credit Cell * `42` \| `FI` \| Financing * `43` \| `RD` \| Expense Sharing * `44` \| `DRI` \| Rural Draft for Indication * `45` \| `ECI` \| Condominium Charges for Indication * `99` \| `Others` \| Others Values: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `document_type_label` | `string` | no | Document type (Abbreviation) |
| `document_number` | `string` | no | Document number |
| `acceptance` | `string` | no | Acceptance: * `N` No (Default) * `S` Yes Values: `N`, `S`. |
| `our_number` | `string` | no | Our Number. If not provided, it will use the Next Our Number from the Collection Wallet. |
| `processed_our_number` | `string` | no | Our Number calculated with check digit (formatted) |
| `processed_our_number_raw` | `string` | no | Our Number calculated with check digit (clean) |
| `paid_amount` | `number` | no | Amount paid |
| `paid_at` | `string (date)` | no | Payment date |
| `days_for_revoke` | `integer` | no | Calendar days for withdrawal/return: Null/Blank: Follows the bank's default. 0: Withdrawal/Return on the same day as maturity. 1 or more: Withdrawal/Return after maturity (Maturity + X calendar days). |
| `days_for_negativation` | `integer` | no | Calendar days for negative registry: Available only for the following banks and formats. * Bank \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Yes \|	Yes	\| No * Itaú \| No \| Yes \| No |
| `days_for_sue_type` | `integer` | no | Type of days for protest: * `0` Calendar * `1` Business Values: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | no | Calendar days for protest |
| `sue_code` | `string` | no | Protest code (CNAB 240). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `revoke_code` | `string` | no | Withdrawal code (CNAB 240). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `first_instruction` | `string` | no | First instruction (CNAB 400). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `second_instruction` | `string` | no | Second instruction (CNAB 400). Check the possible values &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">for each bank&lt;/a>. |
| `watermark` | `boolean` | no | Address |
| `divergent_payment_type` | `integer` | no | Type of divergent payment: Valid only for Itaú and Caixa. * `1` Accepts any divergent amount * `2` Accepts payments within a range of amounts or percentages * `3` Does not accept divergent amount payments * `4` Accepts payments above a minimum amount or percentage Values: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Type of value to consider for payment limits: Valid only for Itaú and Caixa. * `1` Reports divergent payments by amounts * `2` Reports divergent payments by percentages Values: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Minimum value for divergent payment range. |
| `divergent_payment_minimum_value` | `number` | no | Maximum value for divergent payment range. |
| `divergent_payment_maximum_percentage` | `number` | no | Minimum percentage for divergent payment range. |
| `divergent_payment_minimum_percentage` | `number` | no | Maximum percentage for divergent payment range. |
| `divergent_payment_limit` | `integer` | no | Number of payments allowed. Required if divergent payment data is provided. Used only by Caixa. |
| `prevent_registration` | `boolean` | no | Prevent sending registration to bank: If `true`, prevents the boleto from being registered. To be used in cases where the boleto was already registered outside of Kobana but you want to include it in the system. |
| `control_number` | `string` | no | Control number: May contain any information of interest to the Company. The information contained in this field will always be returned with the respective title in the return file. |
| `ignore_email` | `boolean` | no | Do not send this boleto by email |
| `ignore_sms` | `boolean` | no | Never send this boleto by SMS |
| `ignore_whatsapp` | `boolean` | no | Never send this boleto by WhatsApp |
| `addons` | `string (jsonb)` | no | Address |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `meta` | `object` | no | Hash with key and value in JSON format. |
| `notes` | `string` | no | Notes |
| `custom_attachment_name` | `string` | no | Name to be used in boleto files sent to the customer in notifications. Accepts use of variables. If left empty, the default is the word "boleto" followed by the ID. |
| `split_payment` | `boolean` | no | Payment Split. Valid only for ABC Brasil (CNAB 240 remittance), Itaú and Bradesco (CNAB 400 remittance). If true, the boleto split will be registered. Provide the split accounts in `split_accounts` |
| `split_payment_type` | `integer` | no | Type of payment split: Valid only for Itau. Used only with Payment split true. * `1` Credit split by percentage (%) – Nominal value of the title * `2` Credit split in amount (R$) – Nominal value of the title * `3` Credit split by percentage (%) – Net amount received * `4` Credit split in amount (R$) – Net amount received, split proportionally Values: `1`, `2`, `3`, `4`. |
| `split_accounts` | `array<object>` | no | Accounts for payment split. |
| `payment_place` | `string` | no | Payment location |
| `installment_id` | `integer` | no | Carnê ID |
| `installment_number` | `integer` | no | Installment number of the voucher |
| `installment_total` | `integer` | no | Total installments of the voucher |
| `customer_subscription_id` | `integer` | no | Subscription ID |
| `beneficiary_name` | `string` | no | Beneficiary Name |
| `beneficiary_cnpj_cpf` | `string` | no | CNPJ/CPF of Beneficiary |
| `beneficiary_address` | `string` | no | Beneficiary address |
| `beneficiary_assignor_code` | `string` | no | Agency/Beneficiary code |
| `bank_contract_slug` | `string` | no | Wallet slug |
| `agency_number` | `string` | no | Agency |
| `agency_digit` | `string` | no | Branch digit |
| `account_number` | `string` | no | Account |
| `account_digit` | `string` | no | Account digit |
| `extra1` | `string` | no | Extra field 1 |
| `extra1_digit` | `string` | no | Digit of extra field 1 |
| `extra2` | `string` | no | Extra field 2 |
| `extra2_digit` | `string` | no | Digit of extra field 2 |
| `created_via_api` | `boolean` | no | Indicates if the boleto was created by API |
| `created_at` | `string (date-time)` | no | Boleto creation date and time |
| `updated_at` | `string (date-time)` | no | Boleto last update date and time |
| `registered_at` | `string (date-time)` | no | Registration date and time |
| `register_type` | `integer` | no | Registration type: * `1` API * `2` Bank Values: `1`, `2`. |
| `cancel_type` | `integer` | no | Cancellation type: * `1` Customer * `2` Bank Values: `1`, `2`. |
| `payment_count` | `integer` | no | Number of partial payments accepted for this boleto. |
| `line` | `string` | no | Digitizable Line |
| `barcode` | `string` | no | Address |
| `shorten_url` | `string` | no | URL for viewing the boleto |
| `url` | `string` | no | URL for viewing the boleto |
| `carne_url` | `string` | no | URL for viewing the voucher (When it is an installment) |
| `formats` | `object` | no | URLs with available formats. Ex.: PDF, Image, Pix etc |
| `pix_enabled` | `boolean` | no | Indicates if the boleto is hybrid and has Pix QR code |
| `pix_qrcode` | `string` | no | Hybrid boleto Pix QR code |
| `status` | `string` | no | Boleto status: * `generating` Generating * `draft` Draft * `generation_failed` Generation failed * `opened` Open * `canceled` Canceled * `paid` Paid * `overdue` Overdue * `validation_failed` Invalid * `chargeback` Charged back Values: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `recipient_account` | `string` | no | Recipient account + check digit |

More fields and nested objects: `kobana schema banking.bank-billets.duplicate`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Billet duplicated |
| `404` | Billet not found |

## bank-billets pay

Mark Billet as Paid

```text
kobana banking bank-billets pay [ID] [OPTIONS]
```

`PUT /v1/bank_billets/{id}/pay`

Mark billet as paid

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `paid_amount` | `number` | yes | Amount paid of the boleto. |
| `paid_at` | `string (date)` | yes | Boleto payment date |
| `bank_rate` | `string` | no | Bank fee amount. |
| `direct_payment` | `boolean` | no | Informs if payment was made directly to the beneficiary. |

More fields and nested objects: `kobana schema banking.bank-billets.pay`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Billet paid |
| `404` | Billet not found |

## bank-billets send-email

Send Billet by Email

```text
kobana banking bank-billets send-email [ID] [OPTIONS]
```

`POST /v1/bank_billets/{id}/send_email`

Send billet by email

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `form` | `object` | yes | Email form. |

More fields and nested objects: `kobana schema banking.bank-billets.send-email`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Email sent successfully |
| `400` | Failed to send the email |

## bank-billets send-sms

Send Billet by SMS

```text
kobana banking bank-billets send-sms [ID] [OPTIONS]
```

`POST /v1/bank_billets/{id}/send_sms`

Send billet by sms

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `form` | `object` | yes | SMS form. |

More fields and nested objects: `kobana schema banking.bank-billets.send-sms`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | SMS sent successfully |
| `400` | Failed to send the sms |
