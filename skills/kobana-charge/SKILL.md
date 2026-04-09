---
name: kobana-charge
description: "Kobana Cobranças: Pix, Pix Automático, contas Pix, recebimentos."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana charge --help"
---

# charge — Cobranças

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana charge <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| `+cobrar` | Criar cobrança Pix com interface simplificada |

## API Resources

### pix
- `list` — Listar Pix
- `create` — Criar um Pix
- `get` — Visualizar um Pix. Params: `uid`
- `update` — Atualizar um Pix. Params: `pix_uid`
- `cancel` — Cancelar um Pix. Params: `pix_uid`
- `delete` — Excluir um Pix. Params: `uid`

### pix-accounts
- `list` — Listar Contas Pix
- `create` — Criar Conta Pix
- `get` — Visualizar uma Conta Pix. Params: `uid`
- `update` — Atualizar Conta Pix. Params: `uid`
- `delete` — Deletar Conta Pix. Params: `uid`

### pix commands
- `list` — Listar Comandos de um Pix. Params: `pix_uid`
- `get` — Visualizar um Comando. Params: `pix_uid`, `id`

### automatic-pix accounts
- `list` — Listar contas Pix Automático
- `create` — Criar conta Pix Automático
- `get` — Visualizar conta. Params: `uid`
- `update` — Atualizar conta. Params: `uid`
- `delete` — Excluir conta. Params: `uid`

### automatic-pix locations
- `list` — Listar Locations do Pix Automático
- `create` — Criar Location
- `get` — Consultar Location. Params: `uid`

### automatic-pix pix
- `list` — Listar cobranças Pix Automático
- `get` — Visualizar cobrança. Params: `uid`
- `update` — Atualizar cobrança. Params: `uid`
- `cancel` — Cancelar cobrança. Params: `uid`
- `retry` — Retentar cobrança. Params: `uid`

### automatic-pix recurrences
- `list` — Listar recorrências Pix Automático
- `create` — Criar recorrência
- `get` — Visualizar recorrência. Params: `uid`
- `update` — Atualizar recorrência. Params: `uid`
- `cancel` — Cancelar recorrência. Params: `uid`

### automatic-pix requests
- `list` — Listar solicitações Pix Automático
- `get` — Visualizar solicitação. Params: `uid`
- `update` — Atualizar solicitação. Params: `uid`
- `cancel` — Cancelar solicitação. Params: `uid`

### payments
- `list` — Listar Recebimentos
- `create` — Informar Recebimento
- `get` — Informações do Recebimento. Params: `uid`
- `delete` — Excluir Recebimento. Params: `uid`

## Discovering Commands

```bash
kobana charge --help
kobana charge pix --help
kobana schema charge.pix.create
```
