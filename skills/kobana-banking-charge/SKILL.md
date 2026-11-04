---
name: kobana-banking-charge
description: "Kobana Cobranças: Pix, Pix Automático, contas Pix, recebimentos."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana banking charge --help"
---

# charge — Cobranças

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana banking charge <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| `kobana banking +cobrar` | Criar cobrança Pix com interface simplificada |

## API Resources

### automatic-pix accounts
- `list` — Lista todas as contas PIX automáticas
- `create` — Criar nova conta PIX automática
- `get` — Visualizar conta PIX automática. Params: `uid`
- `update` — Atualizar conta PIX automática. Params: `uid`
- `delete` — Excluir conta PIX automática. Params: `uid`

### automatic-pix locations
- `list` — Lista todas as Locations do Pix Automático
- `create` — Criar nova Location do PIX Automático
- `get` — Consulta uma Location do PIX Automático específica. Params: `uid`

### automatic-pix pix
- `list` — Lista todas as cobranças PIX automáticas
- `get` — Informações da cobrança PIX automática. Params: `uid`
- `update` — Atualizar cobrança PIX automática. Params: `uid`
- `cancel` — Cancelar cobrança PIX automática. Params: `uid`
- `retry` — Retentar cobrança PIX automática. Params: `uid`

### automatic-pix recurrences
- `list` — Lista todas as recorrências PIX automáticas
- `create` — Criar nova recorrência PIX automática
- `pix` — Criar nova cobrança PIX automática. Params: `recurrence_uid`
- `requests` — Criar nova solicitação PIX automática. Params: `recurrence_uid`
- `get` — Informações da recorrência PIX automática. Params: `uid`
- `update` — Atualizar recorrência PIX automática. Params: `uid`
- `cancel` — Cancelar recorrência PIX automática. Params: `uid`

### automatic-pix requests
- `list` — Lista todas as solicitações PIX automáticas
- `get` — Informações da solicitação PIX automática. Params: `uid`
- `update` — Atualizar solicitação PIX automática. Params: `uid`
- `cancel` — Cancelar solicitação PIX automática. Params: `uid`

### payments
- `list` — Listar Recebimentos
- `create` — Informar Recebimento
- `get` — Informações do Recebimento. Params: `uid`
- `delete` — Excluir Recebimento. Params: `uid`

### pix
- `list` — Listar Pix
- `create` — Criar um Pix
- `cancel` — Cancelar um Pix. Params: `pix_uid`
- `update` — Atualizar um Pix. Params: `pix_uid`
- `get` — Visualizar um Pix. Params: `uid`
- `delete` — Excluir um Pix. Params: `uid`

### pix commands
- `get` — Visualizar um Comando. Params: `pix_uid`, `uid`
- `list` — Listar Comandos de um Pix. Params: `pix_uid`

### pix-accounts
- `list` — Listar Contas Pix
- `create` — Criar Pix Account
- `get` — Visualizar uma conta Pix. Params: `uid`
- `update` — Atualizar Pix Account. Params: `uid`
- `delete` — Deletar Pix Account. Params: `uid`

## Discovering Commands

```bash
kobana banking charge --help
kobana banking charge pix --help
kobana schema banking.charge.pix.create
```
