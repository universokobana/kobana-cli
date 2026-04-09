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
    cliHelp: "kobana mailbox --help"
---

# mailbox — Caixa Postal

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana mailbox <resource> <method> [flags]
```

## API Resources

### entries
- `list` — Listar Caixas Postais
- `create` — Criar Caixa Postal
- `get` — Visualizar Caixa Postal. Params: `uid`
- `update` — Atualizar Caixa Postal. Params: `uid`
- `delete` — Deletar Caixa Postal. Params: `uid`

### entries email
- `get` — Exibir Canal de E-mail. Params: `entry_id`
- `create` — Criar Canal de E-mail. Params: `entry_id`
- `update` — Atualizar Canal de E-mail. Params: `entry_id`
- `delete` — Excluir Canal de E-mail. Params: `entry_id`
- `activate` — Ativar Canal de E-mail. Params: `entry_id`
- `deactivate` — Desativar Canal de E-mail. Params: `entry_id`

### entries sftp
- `get` — Exibir Canal SFTP. Params: `entry_id`
- `create` — Criar Canal SFTP. Params: `entry_id`
- `update` — Atualizar Canal SFTP. Params: `entry_id`
- `delete` — Excluir Canal SFTP. Params: `entry_id`
- `activate` — Ativar Canal SFTP. Params: `entry_id`
- `deactivate` — Desativar Canal SFTP. Params: `entry_id`
- `fetch` — Buscar arquivos do Canal SFTP. Params: `entry_id`
- `update-password` — Atualizar credenciais SSH. Params: `entry_id`

### entries s3
- `get` — Exibir Canal S3. Params: `entry_id`
- `create` — Criar Canal S3. Params: `entry_id`
- `delete` — Excluir Canal S3. Params: `entry_id`
- `activate` — Ativar Canal S3. Params: `entry_id`
- `deactivate` — Desativar Canal S3. Params: `entry_id`
- `fetch` — Buscar arquivos do Canal S3. Params: `entry_id`
- `update-password` — Atualizar credenciais AWS. Params: `entry_id`

### entries syncthing
- `get` — Exibir Canal Syncthing. Params: `entry_id`
- `create` — Criar Canal Syncthing. Params: `entry_id`
- `update` — Atualizar Canal Syncthing. Params: `entry_id`
- `delete` — Excluir Canal Syncthing. Params: `entry_id`
- `activate` — Ativar Canal Syncthing. Params: `entry_id`
- `deactivate` — Desativar Canal Syncthing. Params: `entry_id`
- `resend-invites` — Reenviar Convites. Params: `entry_id`
- `update-status` — Atualizar Status do Servidor. Params: `entry_id`

### entries whatsapp
- `get` — Exibir Canal WhatsApp. Params: `entry_id`
- `create` — Criar Canal WhatsApp. Params: `entry_id`
- `update` — Atualizar Canal WhatsApp. Params: `entry_id`
- `delete` — Excluir Canal WhatsApp. Params: `entry_id`
- `activate` — Ativar Canal WhatsApp. Params: `entry_id`
- `deactivate` — Desativar Canal WhatsApp. Params: `entry_id`

### files
- `list` — Listar Arquivos
- `create` — Criar Arquivo. Params: `entry_uid`
- `get` — Visualizar Arquivo. Params: `uid`
- `update` — Atualizar Arquivo. Params: `uid`
- `delete` — Deletar Arquivo. Params: `uid`

## Discovering Commands

```bash
kobana mailbox --help
kobana mailbox entries --help
kobana schema mailbox.entries.create
```
