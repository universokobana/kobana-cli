---
name: kobana-mailbox
description: "Kobana Caixa Postal: Entries, canais (email, SFTP, S3, Syncthing, WhatsApp), arquivos."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana banking mailbox --help"
---

# mailbox — Caixa Postal

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana banking mailbox <resource> <method> [flags]
```

## API Resources

### entries
- `list` — Listar Caixas Postais
- `create` — Criar Caixa Postal
- `files` — Criar Arquivo de Mailbox. Params: `entry_uid`
- `get` — Visualizar Caixa Postal. Params: `uid`
- `update` — Atualizar Caixa Postal. Params: `uid`
- `delete` — Deletar Caixa Postal. Params: `uid`

### entries email
- `get` — Exibir Canal de E-mail. Params: `entry_uid`
- `create` — Criar Canal de E-mail. Params: `entry_uid`
- `update` — Atualizar Canal de E-mail. Params: `entry_uid`
- `delete` — Excluir Canal de E-mail. Params: `entry_uid`

### entries email activate
- `update` — Ativar Canal de E-mail. Params: `entry_uid`

### entries email deactivate
- `update` — Desativar Canal de E-mail. Params: `entry_uid`

### entries s3
- `get` — Exibir Canal S3. Params: `entry_uid`
- `create` — Criar Canal S3. Params: `entry_uid`
- `delete` — Excluir Canal S3. Params: `entry_uid`

### entries s3 activate
- `update` — Ativar Canal S3. Params: `entry_uid`

### entries s3 deactivate
- `update` — Desativar Canal S3. Params: `entry_uid`

### entries s3 fetch
- `update` — Buscar arquivos do Canal S3. Params: `entry_uid`

### entries s3 update-password
- `update` — Atualizar credenciais AWS do Canal S3. Params: `entry_uid`

### entries sftp
- `get` — Exibir Canal SFTP. Params: `entry_uid`
- `create` — Criar Canal SFTP. Params: `entry_uid`
- `update` — Atualizar Canal SFTP. Params: `entry_uid`
- `delete` — Excluir Canal SFTP. Params: `entry_uid`

### entries sftp activate
- `update` — Ativar Canal SFTP. Params: `entry_uid`

### entries sftp deactivate
- `update` — Desativar Canal SFTP. Params: `entry_uid`

### entries sftp fetch
- `update` — Buscar arquivos do Canal SFTP. Params: `entry_uid`

### entries sftp update-password
- `update` — Atualizar credenciais SSH do Canal SFTP. Params: `entry_uid`

### entries syncthing
- `get` — Exibir Canal Syncthing. Params: `entry_uid`
- `create` — Criar Canal Syncthing. Params: `entry_uid`
- `update` — Atualizar Canal Syncthing. Params: `entry_uid`
- `delete` — Excluir Canal Syncthing. Params: `entry_uid`

### entries syncthing activate
- `update` — Ativar Canal Syncthing. Params: `entry_uid`

### entries syncthing deactivate
- `update` — Desativar Canal Syncthing. Params: `entry_uid`

### entries syncthing resend-invites
- `update` — Reenviar Convites Syncthing. Params: `entry_uid`

### entries syncthing update-status
- `update` — Atualizar Status do Servidor Syncthing. Params: `entry_uid`

### entries whatsapp
- `get` — Exibir Canal WhatsApp. Params: `entry_uid`
- `create` — Criar Canal WhatsApp. Params: `entry_uid`
- `update` — Atualizar Canal WhatsApp. Params: `entry_uid`
- `delete` — Excluir Canal WhatsApp. Params: `entry_uid`

### entries whatsapp activate
- `update` — Ativar Canal WhatsApp. Params: `entry_uid`

### entries whatsapp deactivate
- `update` — Desativar Canal WhatsApp. Params: `entry_uid`

### files
- `list` — Listar Arquivos de Mailbox
- `get` — Visualizar Arquivo de Mailbox. Params: `uid`
- `update` — Atualizar Arquivo de Mailbox. Params: `uid`
- `delete` — Deletar Arquivo de Mailbox. Params: `uid`

## Discovering Commands

```bash
kobana banking mailbox --help
kobana banking mailbox entries --help
kobana schema banking.mailbox.entries.create
```
