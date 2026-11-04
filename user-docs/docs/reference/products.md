---
id: products
title: Produtos
sidebar_label: Produtos
sidebar_position: 1
description: Os sete produtos do CLI, seus hosts, variáveis de token e ambientes.
---

# Produtos

Cada produto Kobana é uma API independente, com hosts e credenciais próprios. O produto é sempre o primeiro segmento do comando.

| Produto | Comando | Credencial | Documentação da API |
|---|---|---|---|
| [Gateway Bancário](https://www.kobana.com.br/produtos/gateway-bancario) | `banking` | `KOBANA_BANKING_TOKEN` | [docs.banking.kobana.com.br](https://docs.banking.kobana.com.br) |
| [Financeiro Inteligente](https://www.kobana.com.br/produtos/financeiro-inteligente) | `finance` | `KOBANA_FINANCE_TOKEN` | [docs.finance.kobana.com.br](https://docs.finance.kobana.com.br) |
| [Faturamento Automático](https://www.kobana.com.br/produtos/faturamento-automatico) | `billing` | `KOBANA_BILLING_TOKEN` | [docs.billing.kobana.com.br](https://docs.billing.kobana.com.br) |
| Inbox Autônomo | `inbox` | `KOBANA_INBOX_TOKEN` | [docs.inbox.kobana.com.br](https://docs.inbox.kobana.com.br) |
| [Régua de Cobrança](https://www.kobana.com.br/produtos/regua-de-cobranca) | `dunning` | `KOBANA_DUNNING_TOKEN` | [docs.dunning.kobana.com.br](https://docs.dunning.kobana.com.br) |
| [Dashboard Mágico](https://www.kobana.com.br/produtos/dashboard-magico) | `dashboards` | `KOBANA_DASHBOARDS_TOKEN` | [docs.dashboards.kobana.com.br](https://docs.dashboards.kobana.com.br) |
| [Agente Kadu](https://www.kobana.com.br/ia/kadu) | `kadu` | `KOBANA_KADU_TOKEN` ou `KADU_TOKEN` | [docs.kadu.ia.br](https://docs.kadu.ia.br) |

> **Um token vale para um produto só.** Veja [Autenticação](/getting-started/authentication).

## Hosts por ambiente

| Produto | `production` (padrão) | `sandbox` |
|---|---|---|
| `banking` | `api.kobana.com.br` | `api.sandbox.kobana.com.br` |
| `finance` | `api.finance.kobana.com.br` | `api.finance.sandbox.kobana.com.br` |
| `billing` | `api.billing.kobana.com.br` | `api.billing.sandbox.kobana.com.br` |
| `inbox` | `api.inbox.kobana.com.br` | — |
| `dunning` | `api.dunning.kobana.com.br` | `api.dunning.sandbox.kobana.com.br` |
| `dashboards` | `api.dashboards.kobana.com.br` | — |
| `kadu` | `api.kadu.ia.br` | — |

`--env sandbox` num produto sem sandbox (`inbox`, `dashboards`, `kadu`) termina com erro.

## O que cada produto faz

- **`banking`** — operações bancárias unificadas: boletos, cobranças e pagamentos Pix, TED, extratos e conciliação, em mais de 40 bancos. Convivem a API v1 (`bank-billets`, `customers`, `webhooks`) e a v2 (`charge`, `payment`, `transfer`, `financial`…); o comando não distingue.
- **`finance`** — contas a pagar e a receber, conciliação, fluxo de caixa e multiempresa. Toda rota exige o [espaço de trabalho](/getting-started/authentication#espaço-de-trabalho-x-workspace-id).
- **`billing`** — cobrança recorrente: assinaturas, planos, faturas, propostas e notas fiscais.
- **`inbox`** — caixas de entrada com agentes: recebe e-mails, processa com agentes e entrega por webhooks.
- **`dunning`** — recuperação de inadimplência: cobranças, réguas, acordos, negativações e protestos.
- **`dashboards`** — dashboards e portais do cliente sobre fontes de dados conectadas.
- **`kadu`** — agente de IA que executa rotinas financeiras (Tasks) a partir das skills da organização.

Os recursos e comandos de cada um estão no [Manual](/manual).
