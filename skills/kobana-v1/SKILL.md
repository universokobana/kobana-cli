---
name: kobana-v1
description: "Kobana API v1: Boletos, clientes, assinaturas, webhooks, CNAB, carnês."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana v1 --help"
---

# v1 — API Legada

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana v1 <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| `+emitir` | Emitir boleto com interface simplificada |
| `+cancelar-lote` | Cancelar múltiplos boletos por ID |

## API Resources

### bank-billets
- `list` — Listar Boletos
- `create` — Criar um Boleto
- `get` — Visualizar o Boleto. Params: `id`
- `update` — Atualizar o Boleto. Params: `id`
- `cancel` — Cancelar o Boleto. Params: `id`
- `cancel-all` — Cancelar Boletos em Lote
- `duplicate` — Duplicar Boleto. Params: `id`
- `pay` — Marcar Boleto Como Pago. Params: `id`
- `protest` — Protestar Boleto. Params: `id`, `type`
- `send-email` — Enviar Boleto por E-mail. Params: `id`
- `send-sms` — Enviar Boleto por SMS. Params: `id`

### bank-billet-accounts
- `list` — Listar Carteiras de Cobrança
- `create` — Criar Carteira de Cobrança
- `get` — Informações da Carteira. Params: `id`
- `update` — Atualizar a Carteira. Params: `id`
- `ask` — Solicitar Homologação. Params: `id`
- `set-default` — Alterar Carteira Padrão. Params: `id`
- `validate` — Validar Carteira. Params: `id`

### bank-billet-batches
- `list` — Listar Lotes
- `create` — Criar Lote
- `get` — Informações do Lote. Params: `id`
- `delete` — Excluir Lote. Params: `id`
- `add-bank-billets` — Incluir Boletos no Lote. Params: `id`
- `remove-bank-billet` — Excluir Boleto do Lote. Params: `id`
- `pdf` — Exportar Lote em PDF. Params: `id`
- `zip` — Exportar Lote em ZIP. Params: `id`

### bank-billet-discharges
- `list` — Listar Registros de Retorno
- `get` — Informações do Registro de Retorno. Params: `id`

### bank-billet-payments
- `list` — Listar Pagamentos de Boleto
- `create` — Efetuar Pagamento de Boleto
- `get` — Informações do Pagamento. Params: `id`
- `delete` — Excluir Pagamento. Params: `id`

### bank-billet-registrations
- `list` — Listar Registros de Boleto
- `get` — Informações do Registro. Params: `id`

### bank-billet-remittances
- `list` — Listar Registros de Remessa
- `get` — Informações do Registro. Params: `id`
- `delete` — Excluir Pendências. Params: `id`

### customers
- `list` — Listar Clientes
- `create` — Criar um Cliente
- `get` — Visualizar o Cliente. Params: `id`
- `update` — Atualizar Cliente. Params: `id`

### customer-subscriptions
- `list` — Listar Assinaturas
- `create` — Criar uma Assinatura
- `get` — Informações da Assinatura. Params: `id`
- `update` — Atualizar a Assinatura. Params: `id`
- `delete` — Excluir a Assinatura. Params: `id`
- `next-charge` — Gerar Próxima Cobrança. Params: `id`

### installments
- `list` — Listar Carnês
- `create` — Criar um Carnê
- `get` — Informações do Carnê. Params: `id`
- `delete` — Excluir o Carnê. Params: `id`

### discharges
- `list` — Listar CNABs
- `create` — Enviar CNAB
- `get` — Informações do CNAB. Params: `id`
- `download` — Download do CNAB. Params: `id`
- `pay-off` — Quitar Boletos. Params: `id`
- `reprocess` — Reprocessar CNAB. Params: `id`

### remittances
- `list` — Listar CNABs de Remessa
- `create` — Criar CNAB
- `get` — Informações do CNAB. Params: `id`
- `delete` — Apagar CNAB. Params: `id`
- `raw` — Raw (text/plain) do CNAB. Params: `id`

### events
- `list` — Listar Eventos
- `get` — Informações do Evento. Params: `id`

### webhooks
- `list` — Listar Webhooks
- `create` — Criar Webhook
- `get` — Informações do Webhook. Params: `id`
- `update` — Atualizar Webhook. Params: `id`
- `delete` — Excluir Webhook. Params: `id`

### webhook-deliveries
- `list` — Listar Webhooks Enviados
- `get` — Informações do Webhook Enviado. Params: `id`
- `resend` — Reenviar Webhooks Enviados

### email-deliveries
- `list` — Listar E-mails Enviados
- `get` — Informações do E-mail. Params: `id`
- `resend` — Reenviar E-mail. Params: `id`

### sms-deliveries
- `list` — Listar SMS Enviados
- `get` — Informações do SMS. Params: `id`
- `resend` — Reenviar SMS. Params: `id`

### imports
- `list` — Listar Importações. Params: `collection_name`
- `create` — Importar. Params: `collection_name`
- `get` — Visualizar Importação. Params: `collection_name`, `id`

### reports
- `bank-billets` — Contagem de Boletos

### userinfo
- `get` — Informações do Usuário (descontinuado)

## Discovering Commands

```bash
kobana v1 --help
kobana v1 bank-billets --help
kobana schema v1.bank-billets.create
```
