---
name: kobana-banking
description: "Kobana Gateway Bancário (banking): mapa das skills do produto — boletos, Pix, pagamentos, transferências, extratos, EDI, caixa postal, administração e tokens. Leia primeiro para escolher a skill certa."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana banking --help"
---

# banking — Gateway Bancário

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

O Gateway Bancário é o produto mais largo do CLI (mais de 40 bancos, API v1 e v2),
por isso suas skills são divididas por domínio. **Esta skill só aponta para a
certa**: escolha pela tarefa, abra a skill indicada e use os comandos dela.

```bash
kobana banking <recurso> <método> [flags]
```

O token do produto é `KOBANA_BANKING_TOKEN` (ou `kobana banking auth login`).
Ele não vale para outros produtos.

## Qual skill usar

| Quero… | Skill | Comando |
|--------|-------|---------|
| Emitir, listar, cancelar e baixar **boletos** (v1), remessas, carnês, clientes, webhooks | `kobana-banking-v1` | `kobana banking bank-billets`, `+emitir`, `+cancelar-lote` |
| **Cobrar por Pix** (QR Code, cobrança com vencimento) e Pix Automático | `kobana-banking-charge` | `kobana banking charge pix`, `charge automatic-pix`, `+cobrar` |
| **Pagar** boletos, tributos, contas de consumo; DDA; pagamentos em lote | `kobana-banking-payment` | `kobana banking payment bank-billets`, `payment taxes`, `payment utilities`, `payment dda-bank-billets` |
| **Transferir** por Pix, TED ou entre contas; lotes | `kobana-banking-transfer` | `kobana banking transfer pix`, `transfer ted`, `transfer internal` |
| Ver **saldo**, extrato e provedores; comandos de uma conta | `kobana-banking-financial` | `kobana banking financial accounts balances`, `financial accounts statement-transactions` |
| **Consultar** boletos e QR Codes Pix | `kobana-banking-data` | `kobana banking data bank-billet-queries`, `data pix-qrcode-queries` |
| Trocar arquivos com os bancos por **EDI** | `kobana-banking-edi` | `kobana banking edi boxes` |
| Configurar a **caixa postal** e seus canais (e-mail, SFTP, S3, WhatsApp) | `kobana-banking-mailbox` | `kobana banking mailbox entries` |
| Gerir **subcontas**, usuários, conexões e certificados | `kobana-banking-admin` | `kobana banking admin subaccounts`, `admin certificates`, `admin connections` |
| Criar e revogar **tokens de acesso** à API | `kobana-banking-security` | `kobana banking security access-tokens` |

## Atalhos que valem para o produto inteiro

- `kobana banking me` — a conta do token (está na skill `kobana-banking-admin`).
- `kobana banking payments <método>` — pagamentos de qualquer tipo, numa
  listagem só (skill `kobana-banking-payment`).
- `kobana banking transfers <método>` — transferências de qualquer tipo (skill
  `kobana-banking-transfer`).

## Duas gerações de API, um comando

Os recursos legados da v1 (`bank-billets`, `customers`, `webhooks`…) ficam lado a
lado com os domínios da v2 (`charge`, `payment`, `transfer`, `financial`…). A
versão nunca entra no comando: o CLI resolve `/v1` e `/v2` sozinho. Para boletos
e clientes, o caminho é a skill `kobana-banking-v1`; para Pix, pagamentos e
transferências, as skills de domínio.

## Antes de gravar

Pagamentos, transferências e cobranças movem dinheiro. Rode **sempre** com
`--dry-run` primeiro, confira a requisição e só então tire a flag. Em teste, use
`--env sandbox`.

## Discovering Commands

```bash
kobana banking --help
kobana banking <recurso> --help
kobana schema banking.<recurso>.<método>
```
