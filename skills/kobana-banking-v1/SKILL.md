---
name: kobana-banking-v1
description: "Kobana Gateway Bancário (recursos legados v1): boletos, clientes, assinaturas, webhooks, CNAB, carnês."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana banking --help"
---

# banking — recursos legados (API v1)

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana banking <resource> <method> [flags]
```

The resources below are served by the API v1, but the version is never part of
the command — they sit directly under `banking`, next to the v2 domains
(`charge`, `payment`, `transfer`, …) documented in the sibling skills.

## Helper Commands

| Command | Description |
|---------|-------------|
| `kobana banking +emitir` | Emitir boleto com interface simplificada |
| `kobana banking +cancelar-lote` | Cancelar múltiplos boletos por ID |

## API Resources

### bank-billet-accounts
- `list` — Listar Carteiras
- `create` — Criar Carteira de Cobrança
- `get` — Informações da Carteira. Params: `id`
- `update` — Atualizar a Carteira. Params: `id`
- `delete` — Excluir a Carteira de Cobrança. Params: `id`
- `ask` — Solicitar Homologação da Carteira de Cobrança. Params: `id`
- `set-default` — Alterar Carteira de Cobrança padrão. Params: `id`
- `validate` — Validar Carteira de Cobrança. Params: `id`

### bank-billet-batch-exports
- `get` — Arquivo de Exportação do Lote. Params: `id`

### bank-billet-batches
- `list` — Listar Lotes
- `create` — Criar Lote
- `get` — Informações do Lote. Params: `id`
- `delete` — Excluir o lote. Params: `id`
- `add-bank-billets` — Incluir Boletos no Lote. Params: `id`
- `pdf` — Exportar Lote no Formato .pdf. Params: `id`
- `remove-bank-billet` — Excluir Boleto do Lote. Params: `id`
- `zip` — Exportar Lote no Formato .zip. Params: `id`

### bank-billet-discharges
- `list` — Listar Registros de Retorno
- `get` — Informações do Registro de Retorno. Params: `id`

### bank-billet-payments
- `list` — Listar Pagamentos de Boleto
- `create` — Efetuar Pagamento de Boleto
- `get` — Informações do Pagamento de Boleto. Params: `id`
- `delete` — Excluir Pagamento de Boleto. Params: `id`

### bank-billet-registrations
- `list` — Listar Registros de Boleto
- `get` — Informações do Registro de Boleto. Params: `id`

### bank-billet-remittances
- `list` — Listar Registros de Remessa
- `get` — Informações do Registro de Remessa. Params: `id`
- `delete` — Excluir Pendências. Params: `id`

### bank-billet-remittances occurrence
- `create` — Criar Pendências. Params: `id`, `our_code`

### bank-billet-remittances pending
- `list` — Listar Registros de Remessa Pendentes

### bank-billets
- `list` — Listar Boletos
- `create` — Criar um Boleto
- `cancel-all` — Cancelar Boletos em Lote
- `get` — Visualizar o Boleto. Params: `id`
- `update` — Atualizar o Boleto. Params: `id`
- `cancel` — Cancelar o Boleto. Params: `id`
- `duplicate` — Duplicar Boleto. Params: `id`
- `pay` — Marcar Boleto Como Pago. Params: `id`
- `send-email` — Enviar Boleto por E-mail. Params: `id`
- `send-sms` — Enviar Boleto por SMS. Params: `id`

### bank-billets protest
- `update` — Protestar Boleto. Params: `id`, `type`

### customer-subscriptions
- `list` — Listar Assinaturas
- `create` — Criar uma Assinatura
- `get` — Informações da Assinatura. Params: `id`
- `update` — Atualizar a Assinatura. Params: `id`
- `delete` — Excluir a Assinatura. Params: `id`
- `next-charge` — Gerar Próxima Cobrança. Params: `id`

### customers
- `list` — Listar Clientes
- `create` — Criar um Cliente
- `get` — Visualizar o Cliente. Params: `id`
- `update` — Atualizar Cliente. Params: `id`

### customers cnpj-cpf
- `list` — Busca Cliente por CNPJ/CPF

### customers email
- `list` — Busca Cliente por E-mail

### customers people-sync
- `update` — Habilitar ou desabilitar o espelho Cliente <-> Pessoa

### discharges
- `list` — Listar CNABs
- `create` — Enviar CNAB
- `get` — Informações do CNAB. Params: `id`
- `download` — Download do CNAB. Params: `id`
- `pay-off` — Quitar Boletos. Params: `id`
- `reprocess` — Reprocessar CNAB. Params: `id`

### email-deliveries
- `list` — Listar E-mails Enviados
- `get` — Informações do E-mail Enviado. Params: `id`
- `resend` — Reenviar E-mail Enviado. Params: `id`

### events
- `list` — Listar Eventos
- `get` — Informações do Evento. Params: `id`

### imports
- `list` — Listar Importações. Params: `collection_name`
- `create` — Importar. Params: `collection_name`
- `get` — Visualizar Importação. Params: `collection_name`, `id`

### installments
- `list` — Listar Carnês
- `create` — Criar um Carnê
- `get` — Informações do Carnê. Params: `id`
- `delete` — Excluir o Carnê. Params: `id`

### remittances
- `list` — Listar CNABs
- `create` — Criar CNAB
- `bulk` — Criar CNABs em Lote
- `get` — Informações do CNAB. Params: `id`
- `delete` — Apagar CNAB. Params: `id`
- `raw` — Raw(text/plain) do CNAB. Params: `id`

### reports bank-billets
- `list` — Contagem de Boletos

### sms-deliveries
- `list` — Listar SMS Enviados
- `get` — Informações do SMS Enviado. Params: `id`
- `resend` — Reenviar SMS Enviado. Params: `id`

### userinfo
- `list` — Informações do Usuário (descontinuado)

### webhook-deliveries
- `list` — Listar Webhooks Enviados
- `get` — Informações do Webhook Enviado. Params: `id`

### webhook-deliveries resend
- `update` — Reenviar Webhooks Enviados

### webhooks
- `list` — Listar Webhooks
- `create` — Criar Webhook
- `get` — Informações do Webhook. Params: `id`
- `update` — Atualizar Webhook. Params: `id`
- `delete` — Excluir Webhook. Params: `id`

## Discovering Commands

```bash
kobana banking --help
kobana banking bank-billets --help
kobana schema banking.bank-billets.create
```
