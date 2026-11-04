---
id: mailbox
title: "kobana banking mailbox"
sidebar_label: "mailbox"
description: "Caixa postal (EDI, arquivos)"
---

# kobana banking mailbox

Caixa postal (EDI, arquivos)

Comandos de `kobana banking mailbox`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`mailbox entries email`](#mailbox-entries-email) | `GET /v2/mailbox/entries/{entry_uid}/email` | Exibir Canal de E-mail |
| [`mailbox entries email activate`](#mailbox-entries-email-activate) | `PUT /v2/mailbox/entries/{entry_uid}/email/activate` | Ativar Canal de E-mail |
| [`mailbox entries email deactivate`](#mailbox-entries-email-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/email/deactivate` | Desativar Canal de E-mail |
| [`mailbox entries email create`](#mailbox-entries-email-create) | `POST /v2/mailbox/entries/{entry_uid}/email` | Criar Canal de E-mail |
| [`mailbox entries email update`](#mailbox-entries-email-update) | `PATCH /v2/mailbox/entries/{entry_uid}/email` | Atualizar Canal de E-mail |
| [`mailbox entries email delete`](#mailbox-entries-email-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/email` | Excluir Canal de E-mail |
| [`mailbox entries s3`](#mailbox-entries-s3) | `GET /v2/mailbox/entries/{entry_uid}/s3` | Exibir Canal S3 |
| [`mailbox entries s3 activate`](#mailbox-entries-s3-activate) | `PUT /v2/mailbox/entries/{entry_uid}/s3/activate` | Ativar Canal S3 |
| [`mailbox entries s3 deactivate`](#mailbox-entries-s3-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/s3/deactivate` | Desativar Canal S3 |
| [`mailbox entries s3 fetch`](#mailbox-entries-s3-fetch) | `PUT /v2/mailbox/entries/{entry_uid}/s3/fetch` | Buscar arquivos do Canal S3 |
| [`mailbox entries s3 update-password`](#mailbox-entries-s3-update-password) | `PUT /v2/mailbox/entries/{entry_uid}/s3/update_password` | Atualizar credenciais AWS do Canal S3 |
| [`mailbox entries s3 create`](#mailbox-entries-s3-create) | `POST /v2/mailbox/entries/{entry_uid}/s3` | Criar Canal S3 |
| [`mailbox entries s3 delete`](#mailbox-entries-s3-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/s3` | Excluir Canal S3 |
| [`mailbox entries sftp`](#mailbox-entries-sftp) | `GET /v2/mailbox/entries/{entry_uid}/sftp` | Exibir Canal SFTP |
| [`mailbox entries sftp activate`](#mailbox-entries-sftp-activate) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/activate` | Ativar Canal SFTP |
| [`mailbox entries sftp deactivate`](#mailbox-entries-sftp-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/deactivate` | Desativar Canal SFTP |
| [`mailbox entries sftp fetch`](#mailbox-entries-sftp-fetch) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/fetch` | Buscar arquivos do Canal SFTP |
| [`mailbox entries sftp update-password`](#mailbox-entries-sftp-update-password) | `PUT /v2/mailbox/entries/{entry_uid}/sftp/update_password` | Atualizar credenciais SSH do Canal SFTP |
| [`mailbox entries sftp create`](#mailbox-entries-sftp-create) | `POST /v2/mailbox/entries/{entry_uid}/sftp` | Criar Canal SFTP |
| [`mailbox entries sftp update`](#mailbox-entries-sftp-update) | `PATCH /v2/mailbox/entries/{entry_uid}/sftp` | Atualizar Canal SFTP |
| [`mailbox entries sftp delete`](#mailbox-entries-sftp-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/sftp` | Excluir Canal SFTP |
| [`mailbox entries syncthing`](#mailbox-entries-syncthing) | `GET /v2/mailbox/entries/{entry_uid}/syncthing` | Exibir Canal Syncthing |
| [`mailbox entries syncthing activate`](#mailbox-entries-syncthing-activate) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/activate` | Ativar Canal Syncthing |
| [`mailbox entries syncthing deactivate`](#mailbox-entries-syncthing-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/deactivate` | Desativar Canal Syncthing |
| [`mailbox entries syncthing resend-invites`](#mailbox-entries-syncthing-resend-invites) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/resend_invites` | Reenviar Convites Syncthing |
| [`mailbox entries syncthing update-status`](#mailbox-entries-syncthing-update-status) | `PUT /v2/mailbox/entries/{entry_uid}/syncthing/update_status` | Atualizar Status do Servidor Syncthing |
| [`mailbox entries syncthing create`](#mailbox-entries-syncthing-create) | `POST /v2/mailbox/entries/{entry_uid}/syncthing` | Criar Canal Syncthing |
| [`mailbox entries syncthing update`](#mailbox-entries-syncthing-update) | `PATCH /v2/mailbox/entries/{entry_uid}/syncthing` | Atualizar Canal Syncthing |
| [`mailbox entries syncthing delete`](#mailbox-entries-syncthing-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/syncthing` | Excluir Canal Syncthing |
| [`mailbox entries whatsapp`](#mailbox-entries-whatsapp) | `GET /v2/mailbox/entries/{entry_uid}/whatsapp` | Exibir Canal WhatsApp |
| [`mailbox entries whatsapp activate`](#mailbox-entries-whatsapp-activate) | `PUT /v2/mailbox/entries/{entry_uid}/whatsapp/activate` | Ativar Canal WhatsApp |
| [`mailbox entries whatsapp deactivate`](#mailbox-entries-whatsapp-deactivate) | `PUT /v2/mailbox/entries/{entry_uid}/whatsapp/deactivate` | Desativar Canal WhatsApp |
| [`mailbox entries whatsapp create`](#mailbox-entries-whatsapp-create) | `POST /v2/mailbox/entries/{entry_uid}/whatsapp` | Criar Canal WhatsApp |
| [`mailbox entries whatsapp update`](#mailbox-entries-whatsapp-update) | `PATCH /v2/mailbox/entries/{entry_uid}/whatsapp` | Atualizar Canal WhatsApp |
| [`mailbox entries whatsapp delete`](#mailbox-entries-whatsapp-delete) | `DELETE /v2/mailbox/entries/{entry_uid}/whatsapp` | Excluir Canal WhatsApp |
| [`mailbox entries list`](#mailbox-entries-list) | `GET /v2/mailbox/entries` | Listar Caixas Postais |
| [`mailbox entries create`](#mailbox-entries-create) | `POST /v2/mailbox/entries` | Criar Caixa Postal |
| [`mailbox entries files`](#mailbox-entries-files) | `POST /v2/mailbox/entries/{entry_uid}/files` | Criar Arquivo de Mailbox |
| [`mailbox entries get`](#mailbox-entries-get) | `GET /v2/mailbox/entries/{uid}` | Visualizar Caixa Postal |
| [`mailbox entries update`](#mailbox-entries-update) | `PATCH /v2/mailbox/entries/{uid}` | Atualizar Caixa Postal |
| [`mailbox entries delete`](#mailbox-entries-delete) | `DELETE /v2/mailbox/entries/{uid}` | Deletar Caixa Postal |
| [`mailbox files list`](#mailbox-files-list) | `GET /v2/mailbox/files` | Listar Arquivos de Mailbox |
| [`mailbox files get`](#mailbox-files-get) | `GET /v2/mailbox/files/{uid}` | Visualizar Arquivo de Mailbox |
| [`mailbox files update`](#mailbox-files-update) | `PATCH /v2/mailbox/files/{uid}` | Atualizar Arquivo de Mailbox |
| [`mailbox files delete`](#mailbox-files-delete) | `DELETE /v2/mailbox/files/{uid}` | Deletar Arquivo de Mailbox |

## mailbox entries email

Exibir Canal de E-mail

```text
kobana banking mailbox entries email [ENTRY_UID] [OPÇÕES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/email`

Retorna o canal de e-mail configurado na Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal de e-mail encontrado. |
| `404` | Canal não encontrado. |

## mailbox entries email activate

Ativar Canal de E-mail

```text
kobana banking mailbox entries email activate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/email/activate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Ativação enfileirada. |

## mailbox entries email deactivate

Desativar Canal de E-mail

```text
kobana banking mailbox entries email deactivate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/email/deactivate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Desativação enfileirada. |

## mailbox entries email create

Criar Canal de E-mail

```text
kobana banking mailbox entries email create [ENTRY_UID] [OPÇÕES]
```

`POST /v2/mailbox/entries/{entry_uid}/email`

Cria um canal de e-mail para a Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `inbox_emails` | `array<object>` | não |  |
| `outbox_emails` | `array<object>` | não |  |

Outros campos e objetos aninhados: `kobana schema banking.mailbox.entries.email.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Canal criado. |

## mailbox entries email update

Atualizar Canal de E-mail

```text
kobana banking mailbox entries email update [ENTRY_UID] [OPÇÕES]
```

`PATCH /v2/mailbox/entries/{entry_uid}/email`

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `inbox_emails` | `array<object>` | não |  |
| `outbox_emails` | `array<object>` | não |  |

Outros campos e objetos aninhados: `kobana schema banking.mailbox.entries.email.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal atualizado. |
| `404` | Canal não encontrado. |

## mailbox entries email delete

Excluir Canal de E-mail

```text
kobana banking mailbox entries email delete [ENTRY_UID] [OPÇÕES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/email`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Canal excluído. |
| `422` | Canal ativo não pode ser excluído. |

## mailbox entries s3

Exibir Canal S3

```text
kobana banking mailbox entries s3 [ENTRY_UID] [OPÇÕES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/s3`

Retorna o canal S3 configurado na Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal S3 encontrado. |
| `404` | Canal não encontrado. |

## mailbox entries s3 activate

Ativar Canal S3

```text
kobana banking mailbox entries s3 activate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/activate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Ativação enfileirada. |

## mailbox entries s3 deactivate

Desativar Canal S3

```text
kobana banking mailbox entries s3 deactivate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/deactivate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Desativação enfileirada. |

## mailbox entries s3 fetch

Buscar arquivos do Canal S3

```text
kobana banking mailbox entries s3 fetch [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/fetch`

Enfileira a busca de arquivos do bucket S3 configurado.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Busca enfileirada. |

## mailbox entries s3 update-password

Atualizar credenciais AWS do Canal S3

```text
kobana banking mailbox entries s3 update-password [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/s3/update_password`

Enfileira a atualização das credenciais AWS da Caixa Postal e define o canal como atualizando senha.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Atualização enfileirada. |

## mailbox entries s3 create

Criar Canal S3

```text
kobana banking mailbox entries s3 create [ENTRY_UID] [OPÇÕES]
```

`POST /v2/mailbox/entries/{entry_uid}/s3`

Cria um canal S3 para a Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Canal criado. |

## mailbox entries s3 delete

Excluir Canal S3

```text
kobana banking mailbox entries s3 delete [ENTRY_UID] [OPÇÕES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/s3`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Canal excluído. |
| `422` | Canal ativo não pode ser excluído. |

## mailbox entries sftp

Exibir Canal SFTP

```text
kobana banking mailbox entries sftp [ENTRY_UID] [OPÇÕES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/sftp`

Retorna o canal SFTP configurado na Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal SFTP encontrado. |
| `404` | Canal não encontrado. |

## mailbox entries sftp activate

Ativar Canal SFTP

```text
kobana banking mailbox entries sftp activate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/activate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Ativação enfileirada. |

## mailbox entries sftp deactivate

Desativar Canal SFTP

```text
kobana banking mailbox entries sftp deactivate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/deactivate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Desativação enfileirada. |

## mailbox entries sftp fetch

Buscar arquivos do Canal SFTP

```text
kobana banking mailbox entries sftp fetch [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/fetch`

Enfileira a busca de arquivos do servidor SFTP configurado.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Busca enfileirada. |

## mailbox entries sftp update-password

Atualizar credenciais SSH do Canal SFTP

```text
kobana banking mailbox entries sftp update-password [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/sftp/update_password`

Enfileira a atualização das credenciais SSH da Caixa Postal e define o canal como atualizando senha.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Atualização enfileirada. |

## mailbox entries sftp create

Criar Canal SFTP

```text
kobana banking mailbox entries sftp create [ENTRY_UID] [OPÇÕES]
```

`POST /v2/mailbox/entries/{entry_uid}/sftp`

Cria um canal SFTP para a Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | não | Chave privada SSH (write-only) |

Outros campos e objetos aninhados: `kobana schema banking.mailbox.entries.sftp.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Canal criado. |

## mailbox entries sftp update

Atualizar Canal SFTP

```text
kobana banking mailbox entries sftp update [ENTRY_UID] [OPÇÕES]
```

`PATCH /v2/mailbox/entries/{entry_uid}/sftp`

Atualiza o canal SFTP. Campo password e campos gerenciados pelo sistema (host, port, inbox_path, outbox_path) não são aceitos via API.

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | não | Chave privada SSH (write-only) |

Outros campos e objetos aninhados: `kobana schema banking.mailbox.entries.sftp.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal atualizado. |
| `404` | Canal não encontrado. |

## mailbox entries sftp delete

Excluir Canal SFTP

```text
kobana banking mailbox entries sftp delete [ENTRY_UID] [OPÇÕES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/sftp`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Canal excluído. |
| `422` | Canal ativo não pode ser excluído. |

## mailbox entries syncthing

Exibir Canal Syncthing

```text
kobana banking mailbox entries syncthing [ENTRY_UID] [OPÇÕES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/syncthing`

Retorna o canal Syncthing configurado na Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal Syncthing encontrado. |
| `404` | Canal não encontrado. |

## mailbox entries syncthing activate

Ativar Canal Syncthing

```text
kobana banking mailbox entries syncthing activate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/activate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Ativação enfileirada. |

## mailbox entries syncthing deactivate

Desativar Canal Syncthing

```text
kobana banking mailbox entries syncthing deactivate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/deactivate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Desativação enfileirada. |

## mailbox entries syncthing resend-invites

Reenviar Convites Syncthing

```text
kobana banking mailbox entries syncthing resend-invites [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/resend_invites`

Reenvia os convites de ativação do canal Syncthing, forçando o processo de ativação.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Reenvio de convites enfileirado. |

## mailbox entries syncthing update-status

Atualizar Status do Servidor Syncthing

```text
kobana banking mailbox entries syncthing update-status [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/syncthing/update_status`

Solicita atualização do status de conexão com o servidor Syncthing.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Atualização de status enfileirada. |

## mailbox entries syncthing create

Criar Canal Syncthing

```text
kobana banking mailbox entries syncthing create [ENTRY_UID] [OPÇÕES]
```

`POST /v2/mailbox/entries/{entry_uid}/syncthing`

Cria um canal Syncthing para a Caixa Postal. O device_id deve seguir o formato padrão do Syncthing.

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim |  |
| `device_id` | `string` | sim |  |

Outros campos e objetos aninhados: `kobana schema banking.mailbox.entries.syncthing.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Canal criado. |

## mailbox entries syncthing update

Atualizar Canal Syncthing

```text
kobana banking mailbox entries syncthing update [ENTRY_UID] [OPÇÕES]
```

`PATCH /v2/mailbox/entries/{entry_uid}/syncthing`

Atualiza o nome ou device_id do canal Syncthing.

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `device_id` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema banking.mailbox.entries.syncthing.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal atualizado. |
| `404` | Canal não encontrado. |

## mailbox entries syncthing delete

Excluir Canal Syncthing

```text
kobana banking mailbox entries syncthing delete [ENTRY_UID] [OPÇÕES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/syncthing`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Canal excluído. |
| `422` | Canal ativo não pode ser excluído. |

## mailbox entries whatsapp

Exibir Canal WhatsApp

```text
kobana banking mailbox entries whatsapp [ENTRY_UID] [OPÇÕES] [COMANDO]
```

`GET /v2/mailbox/entries/{entry_uid}/whatsapp`

Retorna o canal WhatsApp configurado na Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal WhatsApp encontrado. |
| `404` | Canal não encontrado. |

## mailbox entries whatsapp activate

Ativar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp activate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/whatsapp/activate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Ativação enfileirada. |

## mailbox entries whatsapp deactivate

Desativar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp deactivate [ENTRY_UID] [OPÇÕES]
```

`PUT /v2/mailbox/entries/{entry_uid}/whatsapp/deactivate`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Desativação enfileirada. |

## mailbox entries whatsapp create

Criar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp create [ENTRY_UID] [OPÇÕES]
```

`POST /v2/mailbox/entries/{entry_uid}/whatsapp`

Cria um canal WhatsApp para a Caixa Postal. O activation_token é gerado automaticamente.

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `inbox_phones` | `array<object>` | não |  |
| `outbox_phones` | `array<object>` | não |  |

Outros campos e objetos aninhados: `kobana schema banking.mailbox.entries.whatsapp.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Canal criado. |

## mailbox entries whatsapp update

Atualizar Canal WhatsApp

```text
kobana banking mailbox entries whatsapp update [ENTRY_UID] [OPÇÕES]
```

`PATCH /v2/mailbox/entries/{entry_uid}/whatsapp`

Atualiza os telefones do canal WhatsApp. O activation_token não é aceito como parâmetro.

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `inbox_phones` | `array<object>` | não |  |
| `outbox_phones` | `array<object>` | não |  |

Outros campos e objetos aninhados: `kobana schema banking.mailbox.entries.whatsapp.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal atualizado. |
| `404` | Canal não encontrado. |

## mailbox entries whatsapp delete

Excluir Canal WhatsApp

```text
kobana banking mailbox entries whatsapp delete [ENTRY_UID] [OPÇÕES]
```

`DELETE /v2/mailbox/entries/{entry_uid}/whatsapp`

**Argumentos do caminho:** `entry_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Canal excluído. |
| `422` | Canal ativo não pode ser excluído. |

## mailbox entries list

Listar Caixas Postais

```text
kobana banking mailbox entries list [OPÇÕES]
```

`GET /v2/mailbox/entries`

Lista as Caixas Postais da conta.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `per_page` | `integer` | não | Quantidade de registros por página. Máximo de 50. |
| `page` | `integer` | não | Número da página. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Caixas Postais encontradas. |

## mailbox entries create

Criar Caixa Postal

```text
kobana banking mailbox entries create [OPÇÕES]
```

`POST /v2/mailbox/entries`

Cria uma nova Caixa Postal.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.mailbox.entries.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Caixa Postal criada |
| `422` | Falha na validação |

## mailbox entries files

Criar Arquivo de Mailbox

```text
kobana banking mailbox entries files [ENTRY_UID] [OPÇÕES]
```

`POST /v2/mailbox/entries/{entry_uid}/files`

Cria um novo Arquivo de Mailbox associado a uma Caixa Postal.

**Argumentos do caminho:** `entry_uid`

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.mailbox.entries.files`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Arquivo criado |
| `422` | Base64 inválido. |

## mailbox entries get

Visualizar Caixa Postal

```text
kobana banking mailbox entries get [UID] [OPÇÕES]
```

`GET /v2/mailbox/entries/{uid}`

Consulta informações de uma Caixa Postal específica.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Caixa Postal encontrada |
| `404` | Caixa Postal não encontrada |

## mailbox entries update

Atualizar Caixa Postal

```text
kobana banking mailbox entries update [UID] [OPÇÕES]
```

`PATCH /v2/mailbox/entries/{uid}`

Atualiza uma Caixa Postal específica.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.mailbox.entries.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Caixa Postal atualizada |
| `404` | Caixa Postal não encontrada |

## mailbox entries delete

Deletar Caixa Postal

```text
kobana banking mailbox entries delete [UID] [OPÇÕES]
```

`DELETE /v2/mailbox/entries/{uid}`

Deleta uma Caixa Postal específica. Não é possível deletar uma Caixa Postal que possui arquivos.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Caixa Postal deletada |
| `404` | Caixa Postal não encontrada |
| `422` | Caixa Postal possui arquivos ou não pode ser excluída |

## mailbox files list

Listar Arquivos de Mailbox

```text
kobana banking mailbox files list [OPÇÕES]
```

`GET /v2/mailbox/files`

Lista os Arquivos de Mailbox da conta.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `per_page` | `integer` | não | Quantidade de registros por página. Máximo de 50. |
| `page` | `integer` | não | Número da página. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivos encontrados. |

## mailbox files get

Visualizar Arquivo de Mailbox

```text
kobana banking mailbox files get [UID] [OPÇÕES]
```

`GET /v2/mailbox/files/{uid}`

Consulta informações de um Arquivo de Mailbox específico.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivo encontrado |
| `404` | Arquivo não encontrado |

## mailbox files update

Atualizar Arquivo de Mailbox

```text
kobana banking mailbox files update [UID] [OPÇÕES]
```

`PATCH /v2/mailbox/files/{uid}`

Atualiza um Arquivo de Mailbox específico.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.mailbox.files.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivo atualizado |
| `404` | Arquivo não encontrado |

## mailbox files delete

Deletar Arquivo de Mailbox

```text
kobana banking mailbox files delete [UID] [OPÇÕES]
```

`DELETE /v2/mailbox/files/{uid}`

Deleta um Arquivo de Mailbox específico.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Arquivo deletado |
| `404` | Arquivo não encontrado |
