---
id: customer-subscriptions
title: "kobana banking customer-subscriptions"
sidebar_label: "customer-subscriptions"
---

# kobana banking customer-subscriptions


Commands of `kobana banking customer-subscriptions`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`customer-subscriptions list`](#customer-subscriptions-list) | `GET /v1/customer_subscriptions` | List Subscriptions |
| [`customer-subscriptions create`](#customer-subscriptions-create) | `POST /v1/customer_subscriptions` | Create a Subscription |
| [`customer-subscriptions get`](#customer-subscriptions-get) | `GET /v1/customer_subscriptions/{id}` | Subscription Information |
| [`customer-subscriptions update`](#customer-subscriptions-update) | `PUT /v1/customer_subscriptions/{id}` | Update Subscription |
| [`customer-subscriptions delete`](#customer-subscriptions-delete) | `DELETE /v1/customer_subscriptions/{id}` | Delete Subscription |
| [`customer-subscriptions next-charge`](#customer-subscriptions-next-charge) | `POST /v1/customer_subscriptions/{id}/next_charge` | Generate Next Charge |

## customer-subscriptions list

List Subscriptions

```text
kobana banking customer-subscriptions list [OPTIONS]
```

`GET /v1/customer_subscriptions`

List subscriptions.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `customer_person_name` | `string` | no | Filter by Name/Business Name. |
| `customer_cnpj_cpf` | `string` | no | Filter by CPF/CNPJ of the subscription. Must be formatted with mask. Find all records whose field matches the string. |
| `customer_email` | `string` | no | Filter by customer email. Find all records whose field matches the string. |
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing subscriptions |

## customer-subscriptions create

Create a Subscription

```text
kobana banking customer-subscriptions create [OPTIONS]
```

`POST /v1/customer_subscriptions`

Creates a new recurring subscription for automatic collection of customers with bank billets.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `customer_person_name` | `string` | yes | Customer name |
| `customer_cnpj_cpf` | `string` | yes | Customer CPF/CNPJ |
| `customer_zipcode` | `string` | yes | ZIP Code (format 99999999) |
| `customer_address` | `string` | yes | Address |
| `customer_city_name` | `string` | yes | City (Name must be correct and complete) |
| `customer_state` | `string` | yes | State |
| `customer_neighborhood` | `string` | yes | Neighborhood |
| `bank_billet_account_id` | `integer` | yes | Collection wallet ID. If not provided, it will use the default wallet. |
| `amount` | `number` | yes | Subscription amount (R$) |
| `id` | `integer` | no | Subscription ID |
| `customer_id` | `integer` | no | Customer ID. When this ID is passed, the fields `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` and `customer_neighborhood` are not required. |
| `customer_nickname` | `string` | no | Nickname or trade name of the payer |
| `customer_email` | `string (email)` | no | Payer's email |
| `customer_email_cc` | `string (email)` | no | Alternative payer email |
| `customer_address_number` | `string` | no | Number |
| `customer_address_complement` | `string` | no | Complement |
| `customer_phone_number` | `string` | no | Phone (with area code, international code is optional) |
| `customer_person_type` | `string` | no | Type of payer. * `individual` Individual * `juridical` Legal entity Values: `individual`, `juridical`. |
| `customer_mobile_local_code` | `string` | no | Mobile area code |
| `customer_mobile_number` | `string` | no | Mobile phone |
| `customer_notes` | `string` | no | Payer notes |
| `customer_ignore_email` | `boolean` | no | Never send email to this customer |
| `customer_ignore_sms` | `boolean` | no | Never send SMS to this customer |
| `customer_contact_person` | `string` | no | Contact |
| `customer_update` | `string` | no | Contact |
| `cycle` | `string` | no | Subscription cycle. Default: monthly * `biweekly` Biweekly * `bimonthly` Bimonthly * `monthly` Monthly * `quarterly` Quarterly * `semiannual` Semiannual * `annual` Annual Values: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `next_billing` | `string (date)` | no | Date of first or next charge. If no date is sent, this field will be calculated to have the value of the day the subscription was created plus the chosen cycle. Ex.: Monthly (Today + 30 days) |
| `end_at` | `string (date)` | no | Date when you want to stop charging. If blank, charges will be generated automatically until you provide a date or delete the subscription. |
| `description` | `string` | no | Product or service description |
| `instructions` | `string` | no | Instructions for cashier |
| `days_in_advance` | `integer` | no | How many days before the due date the charge will be generated. Default: 7. |
| `fine_type` | `integer` | no | Fine type: * `0` Non-existent (Default) * `1` For percentage of boleto amount * `2` For fixed amount Values: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Number of days after the due date when the fine will start to accrue. The default value is 1 day (the day after the due date). |
| `fine_percentage` | `number` | no | Late fee percentage Ex: 2% x R$ 250.00 = R$ 5.00. Required if fine_type equals 1 |
| `fine_value` | `number` | no | Fine amount (R$). Required if fine_type equals 2. |
| `interest_type` | `integer` | no | Interest/penalty type: * `0` Non-existent (Default) * `1` For daily percentage * `2` For daily amount Values: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | no | Number of days after the due date when the penalty will start to accrue. The default value is 1 day (the day after the due date). |
| `interest_percentage` | `number` | no | Daily interest percentage. From 0.0 to 100.0 (Ex 1.5% = 1.5) Required if interest_type equals 1. |
| `interest_value` | `number` | no | Daily interest amount (R$). Required if interest_type equals 2. |
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
| `bank_billet_layout_id` | `integer` | no | Boleto template ID |
| `notes` | `string` | no | Notes |
| `tags` | `array<string>` | no | Tags associated with the boleto |
| `bank_billet_ids` | `array<integer>` | no | IDs of boletos linked to the voucher |
| `prevent_registration` | `boolean` | no | Prevent sending registration to bank: If `true`, prevents the boleto from being registered. To be used in cases where the boleto was already registered outside of Kobana but you want to include it in the system. |
| `divergent_payment_type` | `integer` | no | Type of divergent payment: Valid only for Itaú and Caixa. * `1` Accepts any divergent amount * `2` Accepts payments within a range of amounts or percentages * `3` Does not accept divergent amount payments * `4` Accepts payments above a minimum amount or percentage Values: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Type of value to consider for payment limits: Valid only for Itaú and Caixa. * `1` Reports divergent payments by amounts * `2` Reports divergent payments by percentages Values: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Maximum value for divergent payment range. |
| `divergent_payment_minimum_value` | `number` | no | Minimum value for divergent payment range. |
| `divergent_payment_maximum_percentage` | `number` | no | Maximum percentage for divergent payment range. |
| `divergent_payment_minimum_percentage` | `number` | no | Minimum percentage for divergent payment range. |
| `divergent_payment_limit` | `integer` | no | Number of payments allowed. Required if divergent payment data is provided. Used only by Caixa. |
| `custom_attachment_name` | `string` | no | Name to be used in boleto files sent to the customer in notifications. Accepts use of variables. If left empty, the default is the word "boleto" followed by the ID. |
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
| `days_for_revoke` | `integer` | no | Calendar days for withdrawal/return: Null/Blank: Follows the bank's default. 0: Withdrawal/Return on the same day as maturity. 1 or more: Withdrawal/Return after maturity (Maturity + X calendar days). |
| `days_for_negativation` | `integer` | no | Calendar days for negative registry: Available only for the following banks and formats. * Bank \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Yes \|	Yes	\| No * Itaú \| No \| Yes \| No |
| `days_for_sue` | `integer` | no | Calendar days for protest |
| `payment_count` | `integer` | no | Number of partial payments accepted for this boleto. |
| `import_id` | `integer` | no | Import ID |
| `ignore_whatsapp` | `boolean` | no | Never send this subscription by WhatsApp |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.customer-subscriptions.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Subscription created |
| `422` | Invalid subscription |

## customer-subscriptions get

Subscription Information

```text
kobana banking customer-subscriptions get [ID] [OPTIONS]
```

`GET /v1/customer_subscriptions/{id}`

Query specific information from a Subscription.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription found |
| `404` | Subscription not found |

## customer-subscriptions update

Update Subscription

```text
kobana banking customer-subscriptions update [ID] [OPTIONS]
```

`PUT /v1/customer_subscriptions/{id}`

Updates specific information of a Subscription

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `customer_person_name` | `string` | yes | Customer name |
| `customer_cnpj_cpf` | `string` | yes | Customer CPF/CNPJ |
| `customer_zipcode` | `string` | yes | ZIP Code (format 99999999) |
| `customer_address` | `string` | yes | Address |
| `customer_city_name` | `string` | yes | City (Name must be correct and complete) |
| `customer_state` | `string` | yes | State |
| `customer_neighborhood` | `string` | yes | Neighborhood |
| `bank_billet_account_id` | `integer` | yes | Collection wallet ID. If not provided, it will use the default wallet. |
| `amount` | `number` | yes | Subscription amount (R$) |
| `id` | `integer` | no | Subscription ID |
| `customer_id` | `integer` | no | Customer ID. When this ID is passed, the fields `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` and `customer_neighborhood` are not required. |
| `customer_nickname` | `string` | no | Nickname or trade name of the payer |
| `customer_email` | `string (email)` | no | Payer's email |
| `customer_email_cc` | `string (email)` | no | Alternative payer email |
| `customer_address_number` | `string` | no | Number |
| `customer_address_complement` | `string` | no | Complement |
| `customer_phone_number` | `string` | no | Phone (with area code, international code is optional) |
| `customer_person_type` | `string` | no | Type of payer. * `individual` Individual * `juridical` Legal entity Values: `individual`, `juridical`. |
| `customer_mobile_local_code` | `string` | no | Mobile area code |
| `customer_mobile_number` | `string` | no | Mobile phone |
| `customer_notes` | `string` | no | Payer notes |
| `customer_ignore_email` | `boolean` | no | Never send email to this customer |
| `customer_ignore_sms` | `boolean` | no | Never send SMS to this customer |
| `customer_contact_person` | `string` | no | Contact |
| `customer_update` | `string` | no | Contact |
| `cycle` | `string` | no | Subscription cycle. Default: monthly * `biweekly` Biweekly * `bimonthly` Bimonthly * `monthly` Monthly * `quarterly` Quarterly * `semiannual` Semiannual * `annual` Annual Values: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `next_billing` | `string (date)` | no | Date of first or next charge. If no date is sent, this field will be calculated to have the value of the day the subscription was created plus the chosen cycle. Ex.: Monthly (Today + 30 days) |
| `end_at` | `string (date)` | no | Date when you want to stop charging. If blank, charges will be generated automatically until you provide a date or delete the subscription. |
| `description` | `string` | no | Product or service description |
| `instructions` | `string` | no | Instructions for cashier |
| `days_in_advance` | `integer` | no | How many days before the due date the charge will be generated. Default: 7. |
| `fine_type` | `integer` | no | Fine type: * `0` Non-existent (Default) * `1` For percentage of boleto amount * `2` For fixed amount Values: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Number of days after the due date when the fine will start to accrue. The default value is 1 day (the day after the due date). |
| `fine_percentage` | `number` | no | Late fee percentage Ex: 2% x R$ 250.00 = R$ 5.00. Required if fine_type equals 1 |
| `fine_value` | `number` | no | Fine amount (R$). Required if fine_type equals 2. |
| `interest_type` | `integer` | no | Interest/penalty type: * `0` Non-existent (Default) * `1` For daily percentage * `2` For daily amount Values: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | no | Number of days after the due date when the penalty will start to accrue. The default value is 1 day (the day after the due date). |
| `interest_percentage` | `number` | no | Daily interest percentage. From 0.0 to 100.0 (Ex 1.5% = 1.5) Required if interest_type equals 1. |
| `interest_value` | `number` | no | Daily interest amount (R$). Required if interest_type equals 2. |
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
| `bank_billet_layout_id` | `integer` | no | Boleto template ID |
| `notes` | `string` | no | Notes |
| `tags` | `array<string>` | no | Tags associated with the boleto |
| `bank_billet_ids` | `array<integer>` | no | IDs of boletos linked to the voucher |
| `prevent_registration` | `boolean` | no | Prevent sending registration to bank: If `true`, prevents the boleto from being registered. To be used in cases where the boleto was already registered outside of Kobana but you want to include it in the system. |
| `divergent_payment_type` | `integer` | no | Type of divergent payment: Valid only for Itaú and Caixa. * `1` Accepts any divergent amount * `2` Accepts payments within a range of amounts or percentages * `3` Does not accept divergent amount payments * `4` Accepts payments above a minimum amount or percentage Values: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Type of value to consider for payment limits: Valid only for Itaú and Caixa. * `1` Reports divergent payments by amounts * `2` Reports divergent payments by percentages Values: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Maximum value for divergent payment range. |
| `divergent_payment_minimum_value` | `number` | no | Minimum value for divergent payment range. |
| `divergent_payment_maximum_percentage` | `number` | no | Maximum percentage for divergent payment range. |
| `divergent_payment_minimum_percentage` | `number` | no | Minimum percentage for divergent payment range. |
| `divergent_payment_limit` | `integer` | no | Number of payments allowed. Required if divergent payment data is provided. Used only by Caixa. |
| `custom_attachment_name` | `string` | no | Name to be used in boleto files sent to the customer in notifications. Accepts use of variables. If left empty, the default is the word "boleto" followed by the ID. |
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
| `days_for_revoke` | `integer` | no | Calendar days for withdrawal/return: Null/Blank: Follows the bank's default. 0: Withdrawal/Return on the same day as maturity. 1 or more: Withdrawal/Return after maturity (Maturity + X calendar days). |
| `days_for_negativation` | `integer` | no | Calendar days for negative registry: Available only for the following banks and formats. * Bank \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Yes \|	Yes	\| No * Itaú \| No \| Yes \| No |
| `days_for_sue` | `integer` | no | Calendar days for protest |
| `payment_count` | `integer` | no | Number of partial payments accepted for this boleto. |
| `import_id` | `integer` | no | Import ID |
| `ignore_whatsapp` | `boolean` | no | Never send this subscription by WhatsApp |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.customer-subscriptions.update`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Subscription updated |
| `404` | Subscription not found |

## customer-subscriptions delete

Delete Subscription

```text
kobana banking customer-subscriptions delete [ID] [OPTIONS]
```

`DELETE /v1/customer_subscriptions/{id}`

Deletes a Subscription.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Subscription deleted |
| `404` | Subscription not found |

## customer-subscriptions next-charge

Generate Next Charge

```text
kobana banking customer-subscriptions next-charge [ID] [OPTIONS]
```

`POST /v1/customer_subscriptions/{id}/next_charge`

Generates the next charge.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `201` | Next charge generated |
| `404` | Subscription not found |
