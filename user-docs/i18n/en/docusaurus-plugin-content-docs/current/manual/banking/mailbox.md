---
id: mailbox
title: "kobana banking mailbox"
sidebar_label: "mailbox"
description: "Mailbox (EDI, files)"
---

# kobana banking mailbox

Mailbox (EDI, files)

Commands of `kobana banking mailbox`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`mailbox entries email`](#mailbox-entries-email) | `GET /v2/mailbox/entries/{entry_uid}/email` | Display Email Channel |
| [`mailbox entries email activate`](#mailbox-entries-email-activate) | `PUT /v2/mailbox/entries/{entry_uid}/email/activate` | Enable Email Channel |
| [`mailbox entries email deactivate`](#mailbox-entries-email-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/email/deactivate` | Disable Email Channel |
| [`mailbox entries email create`](#mailbox-entries-email-create) | `POST /v2/mailbox/entries/{entry_uid}/email` | Create Email Channel |
| [`mailbox entries email update`](#mailbox-entries-email-update) | `PATCH /v2/mailbox/entries/{entry_uid}/email` | Update Email Channel |
| [`mailbox entries email delete`](#mailbox-entries-email-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/email` | Delete Email Channel |
| [`mailbox entries s3`](#mailbox-entries-s3) | `GET /v2/mailbox/entries/{entry_uid}/s3` | Display S3 Channel |
| [`mailbox entries s3 activate`](#mailbox-entries-s3-activate) | `PUT /v2/mailbox/entries/{entry_uid}/s3/activate` | Enable S3 Channel |
| [`mailbox entries s3 deactivate`](#mailbox-entries-s3-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/s3/deactivate` | Disable S3 Channel |
| [`mailbox entries s3 fetch`](#mailbox-entries-s3-fetch) | `PUT /v2/mailbox/entries/{entry_uid}/s3/fetch` | Search S3 Channel files |
| [`mailbox entries s3 update-password`](#mailbox-entries-s3-update-password) | `PUT /v2/mailbox/entries/{entry_uid}/s3/update_password` | Update S3 Channel AWS credentials |
| [`mailbox entries s3 create`](#mailbox-entries-s3-create) | `POST /v2/mailbox/entries/{entry_uid}/s3` | Create S3 Channel |
| [`mailbox entries s3 delete`](#mailbox-entries-s3-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/s3` | Delete S3 Channel |
| [`mailbox entries sftp`](#mailbox-entries-sftp) | `GET /v2/mailbox/entries/{entry_uid}/sftp` | Display SFTP Channel |
| [`mailbox entries sftp activate`](#mailbox-entries-sftp-activate) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/activate` | Enable SFTP Channel |
| [`mailbox entries sftp deactivate`](#mailbox-entries-sftp-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/deactivate` | Disable SFTP Channel |
| [`mailbox entries sftp fetch`](#mailbox-entries-sftp-fetch) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/fetch` | Search SFTP Channel files |
| [`mailbox entries sftp update-password`](#mailbox-entries-sftp-update-password) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/update_password` | Update SSH credentials of SFTP Channel |
| [`mailbox entries sftp create`](#mailbox-entries-sftp-create) | `POST /v2/mailbox/entries/{entry_uid}/sftp` | Create SFTP Channel |
| [`mailbox entries sftp update`](#mailbox-entries-sftp-update) | `PATCH /v2/mailbox/entries/{entry_uid}/sftp` | Update SFTP Channel |
| [`mailbox entries sftp delete`](#mailbox-entries-sftp-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/sftp` | Delete SFTP Channel |
| [`mailbox entries syncthing`](#mailbox-entries-syncthing) | `GET /v2/mailbox/entries/{entry_uid}/syncthing` | Display Syncthing Channel |
| [`mailbox entries syncthing activate`](#mailbox-entries-syncthing-activate) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/activate` | Enable Syncthing Channel |
| [`mailbox entries syncthing deactivate`](#mailbox-entries-syncthing-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/deactivate` | Disable Syncthing Channel |
| [`mailbox entries syncthing resend-invites`](#mailbox-entries-syncthing-resend-invites) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/resend_invites` | Resend Syncthing Invites |
| [`mailbox entries syncthing update-status`](#mailbox-entries-syncthing-update-status) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/update_status` | Update Syncthing Server Status |
| [`mailbox entries syncthing create`](#mailbox-entries-syncthing-create) | `POST /v2/mailbox/entries/{entry_uid}/syncthing` | Create Syncthing Channel |
| [`mailbox entries syncthing update`](#mailbox-entries-syncthing-update) | `PATCH /v2/mailbox/entries/{entry_uid}/syncthing` | Update Syncthing Channel |
| [`mailbox entries syncthing delete`](#mailbox-entries-syncthing-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/syncthing` | Delete Syncthing Channel |
| [`mailbox entries whatsapp`](#mailbox-entries-whatsapp) | `GET /v2/mailbox/entries/{entry_uid}/whatsapp` | Display WhatsApp Channel |
| [`mailbox entries whatsapp activate`](#mailbox-entries-whatsapp-activate) | `PUT /v2/mailbox/entries/{entry_uid}/whatsapp/activate` | Enable WhatsApp Channel |
| [`mailbox entries whatsapp deactivate`](#mailbox-entries-whatsapp-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/whatsapp/deactivate` | Disable WhatsApp Channel |
| [`mailbox entries whatsapp create`](#mailbox-entries-whatsapp-create) | `POST /v2/mailbox/entries/{entry_uid}/whatsapp` | Create WhatsApp Channel |
| [`mailbox entries whatsapp update`](#mailbox-entries-whatsapp-update) | `PATCH /v2/mailbox/entries/{entry_uid}/whatsapp` | Update WhatsApp Channel |
| [`mailbox entries whatsapp delete`](#mailbox-entries-whatsapp-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/whatsapp` | Delete WhatsApp Channel |
| [`mailbox entries list`](#mailbox-entries-list) | `GET /v2/mailbox/entries` | List Mailboxes |
| [`mailbox entries create`](#mailbox-entries-create) | `POST /v2/mailbox/entries` | Create Mailbox |
| [`mailbox entries files`](#mailbox-entries-files) | `POST /v2/mailbox/entries/{entry_uid}/files` | Create Mailbox File |
| [`mailbox entries get`](#mailbox-entries-get) | `GET /v2/mailbox/entries/{uid}` | View Mailbox |
| [`mailbox entries update`](#mailbox-entries-update) | `PATCH /v2/mailbox/entries/{uid}` | Update Mailbox |
| [`mailbox entries delete`](#mailbox-entries-delete) | `DELETE /v2/mailbox/entries/{uid}` | Delete Mailbox |
| [`mailbox files list`](#mailbox-files-list) | `GET /v2/mailbox/files` | List Mailbox Files |
| [`mailbox files get`](#mailbox-files-get) | `GET /v2/mailbox/files/{uid}` | View Mailbox File |
| [`mailbox files update`](#mailbox-files-update) | `PATCH /v2/mailbox/files/{uid}` | Update Mailbox File |
| [`mailbox files delete`](#mailbox-files-delete) | `DELETE /v2/mailbox/files/{uid}` | Delete Mailbox File |

## mailbox entries email

Display Email Channel

```text
kobana banking mailbox entries email [ENTRY_UID] [OPTIONS] [COMMAND]
```

`GET /v2/mailbox/entries/{entry_uid}/email`

Returns the email channel configured in the Mailbox.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Email channel found. |
| `404` | Channel not found. |

## mailbox entries email activate

Enable Email Channel

```text
kobana banking mailbox entries email activate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/email/activate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Activation queued. |

## mailbox entries email deactivate

Disable Email Channel

```text
kobana banking mailbox entries email deactivate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/email/deactivate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Deactivation queued. |

## mailbox entries email create

Create Email Channel

```text
kobana banking mailbox entries email create [ENTRY_UID] [OPTIONS]
```

`POST /v2/mailbox/entries/{entry_uid}/email`

Creates an email channel for the Mailbox.

**Path arguments:** `entry_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `inbox_emails` | `array<object>` | no |  |
| `outbox_emails` | `array<object>` | no |  |

More fields and nested objects: `kobana schema banking.mailbox.entries.email.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Channel created. |

## mailbox entries email update

Update Email Channel

```text
kobana banking mailbox entries email update [ENTRY_UID] [OPTIONS]
```

`PATCH /v2/mailbox/entries/{entry_uid}/email`

**Path arguments:** `entry_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `inbox_emails` | `array<object>` | no |  |
| `outbox_emails` | `array<object>` | no |  |

More fields and nested objects: `kobana schema banking.mailbox.entries.email.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Channel updated. |
| `404` | Channel not found. |

## mailbox entries email delete

Delete Email Channel

```text
kobana banking mailbox entries email delete [ENTRY_UID] [OPTIONS]
```

`DELETE /v2/mailbox/entries/{entry_uid}/email`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Channel deleted. |
| `422` | Active channel cannot be deleted. |

## mailbox entries s3

Display S3 Channel

```text
kobana banking mailbox entries s3 [ENTRY_UID] [OPTIONS] [COMMAND]
```

`GET /v2/mailbox/entries/{entry_uid}/s3`

Returns the S3 channel configured in the Mailbox.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | S3 channel found. |
| `404` | Channel not found. |

## mailbox entries s3 activate

Enable S3 Channel

```text
kobana banking mailbox entries s3 activate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/activate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Activation queued. |

## mailbox entries s3 deactivate

Disable S3 Channel

```text
kobana banking mailbox entries s3 deactivate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/deactivate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Deactivation queued. |

## mailbox entries s3 fetch

Search S3 Channel files

```text
kobana banking mailbox entries s3 fetch [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/fetch`

Queues the search for files in the configured S3 bucket.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Search queued. |

## mailbox entries s3 update-password

Update S3 Channel AWS credentials

```text
kobana banking mailbox entries s3 update-password [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/update_password`

Queues the update of the AWS credentials of the Mailbox and sets the channel as updating password.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Update queued. |

## mailbox entries s3 create

Create S3 Channel

```text
kobana banking mailbox entries s3 create [ENTRY_UID] [OPTIONS]
```

`POST /v2/mailbox/entries/{entry_uid}/s3`

Creates an S3 channel for the Mailbox.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `201` | Channel created. |

## mailbox entries s3 delete

Delete S3 Channel

```text
kobana banking mailbox entries s3 delete [ENTRY_UID] [OPTIONS]
```

`DELETE /v2/mailbox/entries/{entry_uid}/s3`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Channel deleted. |
| `422` | Active channel cannot be deleted. |

## mailbox entries sftp

Display SFTP Channel

```text
kobana banking mailbox entries sftp [ENTRY_UID] [OPTIONS] [COMMAND]
```

`GET /v2/mailbox/entries/{entry_uid}/sftp`

Returns the SFTP channel configured in the Mailbox.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | SFTP channel found. |
| `404` | Channel not found. |

## mailbox entries sftp activate

Enable SFTP Channel

```text
kobana banking mailbox entries sftp activate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/activate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Activation queued. |

## mailbox entries sftp deactivate

Disable SFTP Channel

```text
kobana banking mailbox entries sftp deactivate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/deactivate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Deactivation queued. |

## mailbox entries sftp fetch

Search SFTP Channel files

```text
kobana banking mailbox entries sftp fetch [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/fetch`

Queues the search for files in the configured SFTP server.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Search queued. |

## mailbox entries sftp update-password

Update SSH credentials of SFTP Channel

```text
kobana banking mailbox entries sftp update-password [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/update_password`

Queues the update of the SSH credentials of the Mailbox and sets the channel as updating password.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Update queued. |

## mailbox entries sftp create

Create SFTP Channel

```text
kobana banking mailbox entries sftp create [ENTRY_UID] [OPTIONS]
```

`POST /v2/mailbox/entries/{entry_uid}/sftp`

Creates an SFTP channel for the Mailbox.

**Path arguments:** `entry_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | no | SSH private key (write-only) |

More fields and nested objects: `kobana schema banking.mailbox.entries.sftp.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Channel created. |

## mailbox entries sftp update

Update SFTP Channel

```text
kobana banking mailbox entries sftp update [ENTRY_UID] [OPTIONS]
```

`PATCH /v2/mailbox/entries/{entry_uid}/sftp`

Updates the SFTP channel. Password field and fields managed by the system (host, port, inbox_path, outbox_path) are not accepted via API.

**Path arguments:** `entry_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | no | SSH private key (write-only) |

More fields and nested objects: `kobana schema banking.mailbox.entries.sftp.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Channel updated. |
| `404` | Channel not found. |

## mailbox entries sftp delete

Delete SFTP Channel

```text
kobana banking mailbox entries sftp delete [ENTRY_UID] [OPTIONS]
```

`DELETE /v2/mailbox/entries/{entry_uid}/sftp`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Channel deleted. |
| `422` | Active channel cannot be deleted. |

## mailbox entries syncthing

Display Syncthing Channel

```text
kobana banking mailbox entries syncthing [ENTRY_UID] [OPTIONS] [COMMAND]
```

`GET /v2/mailbox/entries/{entry_uid}/syncthing`

Returns the Syncthing channel configured in the Mailbox.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Syncthing channel found. |
| `404` | Channel not found. |

## mailbox entries syncthing activate

Enable Syncthing Channel

```text
kobana banking mailbox entries syncthing activate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/activate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Activation queued. |

## mailbox entries syncthing deactivate

Disable Syncthing Channel

```text
kobana banking mailbox entries syncthing deactivate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/deactivate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Deactivation queued. |

## mailbox entries syncthing resend-invites

Resend Syncthing Invites

```text
kobana banking mailbox entries syncthing resend-invites [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/resend_invites`

Resends the activation invites of the Syncthing channel, forcing the activation process.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Invite resend queued. |

## mailbox entries syncthing update-status

Update Syncthing Server Status

```text
kobana banking mailbox entries syncthing update-status [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/update_status`

Requests update of the connection status with the Syncthing server.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Status update queued. |

## mailbox entries syncthing create

Create Syncthing Channel

```text
kobana banking mailbox entries syncthing create [ENTRY_UID] [OPTIONS]
```

`POST /v2/mailbox/entries/{entry_uid}/syncthing`

Creates a Syncthing channel for the Mailbox. The device_id must follow the standard Syncthing format.

**Path arguments:** `entry_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes |  |
| `device_id` | `string` | yes |  |

More fields and nested objects: `kobana schema banking.mailbox.entries.syncthing.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Channel created. |

## mailbox entries syncthing update

Update Syncthing Channel

```text
kobana banking mailbox entries syncthing update [ENTRY_UID] [OPTIONS]
```

`PATCH /v2/mailbox/entries/{entry_uid}/syncthing`

Updates the name or device_id of the Syncthing channel.

**Path arguments:** `entry_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no |  |
| `device_id` | `string` | no |  |

More fields and nested objects: `kobana schema banking.mailbox.entries.syncthing.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Channel updated. |
| `404` | Channel not found. |

## mailbox entries syncthing delete

Delete Syncthing Channel

```text
kobana banking mailbox entries syncthing delete [ENTRY_UID] [OPTIONS]
```

`DELETE /v2/mailbox/entries/{entry_uid}/syncthing`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Channel deleted. |
| `422` | Active channel cannot be deleted. |

## mailbox entries whatsapp

Display WhatsApp Channel

```text
kobana banking mailbox entries whatsapp [ENTRY_UID] [OPTIONS] [COMMAND]
```

`GET /v2/mailbox/entries/{entry_uid}/whatsapp`

Returns the WhatsApp channel configured in the Mailbox.

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | WhatsApp channel found. |
| `404` | Channel not found. |

## mailbox entries whatsapp activate

Enable WhatsApp Channel

```text
kobana banking mailbox entries whatsapp activate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/whatsapp/activate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Activation queued. |

## mailbox entries whatsapp deactivate

Disable WhatsApp Channel

```text
kobana banking mailbox entries whatsapp deactivate [ENTRY_UID] [OPTIONS]
```

`PUT /v2/mailbox/entries/{entry_uid}/whatsapp/deactivate`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Deactivation queued. |

## mailbox entries whatsapp create

Create WhatsApp Channel

```text
kobana banking mailbox entries whatsapp create [ENTRY_UID] [OPTIONS]
```

`POST /v2/mailbox/entries/{entry_uid}/whatsapp`

Creates a WhatsApp channel for the Mailbox. The activation_token is generated automatically.

**Path arguments:** `entry_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `inbox_phones` | `array<object>` | no |  |
| `outbox_phones` | `array<object>` | no |  |

More fields and nested objects: `kobana schema banking.mailbox.entries.whatsapp.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Channel created. |

## mailbox entries whatsapp update

Update WhatsApp Channel

```text
kobana banking mailbox entries whatsapp update [ENTRY_UID] [OPTIONS]
```

`PATCH /v2/mailbox/entries/{entry_uid}/whatsapp`

Updates the phone numbers of the WhatsApp channel. The activation_token is not accepted as a parameter.

**Path arguments:** `entry_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `inbox_phones` | `array<object>` | no |  |
| `outbox_phones` | `array<object>` | no |  |

More fields and nested objects: `kobana schema banking.mailbox.entries.whatsapp.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Channel updated. |
| `404` | Channel not found. |

## mailbox entries whatsapp delete

Delete WhatsApp Channel

```text
kobana banking mailbox entries whatsapp delete [ENTRY_UID] [OPTIONS]
```

`DELETE /v2/mailbox/entries/{entry_uid}/whatsapp`

**Path arguments:** `entry_uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Channel deleted. |
| `422` | Active channel cannot be deleted. |

## mailbox entries list

List Mailboxes

```text
kobana banking mailbox entries list [OPTIONS]
```

`GET /v2/mailbox/entries`

Lists the Mailboxes of the account.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `per_page` | `integer` | no | Number of records per page. Maximum of 50. |
| `page` | `integer` | no | Page number. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Mailboxes found. |

## mailbox entries create

Create Mailbox

```text
kobana banking mailbox entries create [OPTIONS]
```

`POST /v2/mailbox/entries`

Creates a new Mailbox.

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.mailbox.entries.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Mailbox created |
| `422` | Validation failure |

## mailbox entries files

Create Mailbox File

```text
kobana banking mailbox entries files [ENTRY_UID] [OPTIONS]
```

`POST /v2/mailbox/entries/{entry_uid}/files`

Creates a new Mailbox File associated with a Mailbox.

**Path arguments:** `entry_uid`

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.mailbox.entries.files`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | File created |
| `422` | Invalid Base64. |

## mailbox entries get

View Mailbox

```text
kobana banking mailbox entries get [UID] [OPTIONS]
```

`GET /v2/mailbox/entries/{uid}`

Consults information of a specific Mailbox.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Mailbox found |
| `404` | Mailbox not found |

## mailbox entries update

Update Mailbox

```text
kobana banking mailbox entries update [UID] [OPTIONS]
```

`PATCH /v2/mailbox/entries/{uid}`

Updates a specific Mailbox.

**Path arguments:** `uid`

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.mailbox.entries.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Mailbox updated |
| `404` | Mailbox not found |

## mailbox entries delete

Delete Mailbox

```text
kobana banking mailbox entries delete [UID] [OPTIONS]
```

`DELETE /v2/mailbox/entries/{uid}`

Deletes a specific Mailbox. It is not possible to delete a Mailbox that contains files.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Mailbox deleted |
| `404` | Mailbox not found |
| `422` | Mailbox has files or cannot be deleted |

## mailbox files list

List Mailbox Files

```text
kobana banking mailbox files list [OPTIONS]
```

`GET /v2/mailbox/files`

Lists the Mailbox Files of the account.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `per_page` | `integer` | no | Number of records per page. Maximum of 50. |
| `page` | `integer` | no | Page number. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Files found. |

## mailbox files get

View Mailbox File

```text
kobana banking mailbox files get [UID] [OPTIONS]
```

`GET /v2/mailbox/files/{uid}`

Consults information of a specific Mailbox File.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | File found |
| `404` | File not found |

## mailbox files update

Update Mailbox File

```text
kobana banking mailbox files update [UID] [OPTIONS]
```

`PATCH /v2/mailbox/files/{uid}`

Updates a specific Mailbox File.

**Path arguments:** `uid`

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.mailbox.files.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | File updated |
| `404` | File not found |

## mailbox files delete

Delete Mailbox File

```text
kobana banking mailbox files delete [UID] [OPTIONS]
```

`DELETE /v2/mailbox/files/{uid}`

Deletes a specific Mailbox File.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | File deleted |
| `404` | File not found |
