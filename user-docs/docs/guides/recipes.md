---
id: recipes
title: Receitas
sidebar_label: Receitas
sidebar_position: 1
description: Comandos prontos para tarefas comuns em cada produto — todos com --dry-run para você conferir antes de enviar.
---

# Receitas

Tarefas comuns, produto por produto. Substitua os valores em maiúsculas (`INVOICE_ID`, `UID`…) pelos seus. As receitas que alteram dados trazem `--dry-run`: rode assim primeiro, confira a requisição e só então tire a flag. Veja [Alterações seguras](/usage/safe-writes).

> **Antes de começar:** cada produto tem a sua credencial. Veja [Autenticação](/getting-started/authentication).

## Gateway Bancário (`banking`)

### Emitir um boleto

```bash
kobana banking +emitir --valor 150.50 --vencimento 2026-12-01 \
  --nome "Maria Silva" --cpf-cnpj "012.345.678-90" --carteira 1 --dry-run
```

### Cobrar por Pix

```bash
kobana banking +cobrar --valor 99.90 --vencimento 2026-12-01T23:59:59-03:00 \
  --conta-pix UID --nome "João" --cpf-cnpj "012.345.678-90" --dry-run
```

Sem o helper, a mesma cobrança é uma chamada direta:

```bash
kobana banking charge pix create \
  --json '{"amount": 99.90, "pix_account_uid": "UID"}' --dry-run
```

### Exportar boletos em aberto para CSV

```bash
kobana banking bank-billets list --params '{"status": "opened"}' \
  --fields "id,amount,due_at" --output-format csv --output boletos.csv
```

### Cancelar boletos

```bash
kobana banking bank-billets cancel 12345 --dry-run
kobana banking +cancelar-lote --ids "123,456,789" --dry-run
```

### Consultar o saldo de uma conta

```bash
kobana banking financial accounts balances list \
  --params '{"financial_account_uid": "UID"}'
```

### Transferir por Pix

```bash
kobana banking transfer pix create \
  --json '{"amount": 500, "pix_key": "email@example.com"}' --dry-run
```

## Faturamento Automático (`billing`)

### Pausar e retomar uma assinatura

```bash
kobana billing subscriptions pause SUBSCRIPTION_ID --dry-run
kobana billing subscriptions resume SUBSCRIPTION_ID --dry-run
```

### Fechar uma fatura e emitir a nota fiscal

```bash
kobana billing invoices finalize INVOICE_ID --dry-run
kobana billing invoices issue-nfe INVOICE_ID --dry-run
```

## Financeiro Inteligente (`finance`)

Toda rota do `finance` precisa do [espaço de trabalho](/getting-started/authentication#espaço-de-trabalho-x-workspace-id): `export KOBANA_WORKSPACE_ID=...`.

### Contas a pagar pendentes

```bash
kobana finance payables list --params '{"status": "pending"}' \
  --fields "id,description,amount,due_date"
```

### Totais em vez da lista inteira

```bash
kobana finance payables aggregates --params '{"group_by": "category"}'
```

Os endpoints `aggregates` devolvem somas, não registros — a forma mais barata de responder "quanto?" sem encher o contexto de um agente.

### Fluxo de caixa do mês

```bash
kobana finance cash-flow \
  --params '{"start_date": "2026-01-01", "end_date": "2026-01-31"}'
```

### Enviar uma conta para pagamento no banco

```bash
kobana finance payables send-to-bank PAYABLE_ID --dry-run
```

## Régua de Cobrança (`dunning`)

```bash
# Cobranças vencidas
kobana dunning charges list --params '{"status": "overdue"}' \
  --fields "id,person_id,current_amount,due_date"

# Avisar o devedor
kobana dunning charges notify CHARGE_ID --dry-run

# Aceitar um acordo de parcelamento
kobana dunning agreements accept AGREEMENT_ID --dry-run
```

## Inbox Autônomo (`inbox`)

```bash
# Injetar um .eml bruto numa caixa (multipart)
kobana inbox emails create --file file=./mensagem.eml \
  --json '{"inbox_id": "INBOX_ID"}' --dry-run

# Reenviar uma entrega de webhook que falhou
kobana inbox webhooks deliveries replay WEBHOOK_ID DELIVERY_ID --dry-run
```

## Dashboard Mágico (`dashboards`)

```bash
kobana dashboards data-sources list --fields "id,name,status"
kobana dashboards data-sources sync DATA_SOURCE_ID --dry-run
```

## Agente Kadu (`kadu`)

```bash
# Pré-checagem e disparo de uma Task
kobana kadu tasks readiness TASK_ID
kobana kadu tasks run TASK_ID \
  --json '{"reference_date": "2026-10-30", "environment_id": "ENVIRONMENT_ID"}' --dry-run

# Acompanhar a execução
kobana kadu executions logs EXECUTION_ID

# Todas as execuções, seguindo o cursor
kobana kadu executions list --page-all --fields "id,status,created_at"
```

## Para qualquer produto

### Contar e filtrar com `jq`

```bash
kobana billing invoices list --page-all --fields "id,status" \
  | jq -s 'group_by(.status) | map({status: .[0].status, total: length})'
```

### Testar em sandbox

```bash
kobana banking bank-billets list --env sandbox
export KOBANA_ENVIRONMENT=sandbox
```

`--env sandbox` num produto sem sandbox (`inbox`, `dashboards`, `kadu`) termina com erro em vez de ir para produção.

### Repetir sem duplicar

```bash
kobana banking charge pix create --json '{"amount": 99.90, "pix_account_uid": "UID"}' \
  --idempotency-key pedido-4812
```

### Trocar de espaço de trabalho numa chamada

```bash
kobana finance payables list --params '{"X-Workspace-Id": "outro-uuid"}'
```
