---
id: products
title: Products
sidebar_label: Products
sidebar_position: 1
description: The seven CLI products, their hosts, token variables and environments.
---

# Products

Each Kobana product is an independent API, with its own hosts and credentials. The product is always the first segment of the command.

| Product | Command | Credential | API documentation |
|---|---|---|---|
| [Banking Gateway](https://www.kobana.com.br/en/products/banking-gateway) | `banking` | `KOBANA_BANKING_TOKEN` | [docs.banking.kobana.com.br](https://docs.banking.kobana.com.br) |
| [Smart Finance](https://www.kobana.com.br/en/products/smart-finance) | `finance` | `KOBANA_FINANCE_TOKEN` | [docs.finance.kobana.com.br](https://docs.finance.kobana.com.br) |
| [Automatic Billing](https://www.kobana.com.br/en/products/automatic-billing) | `billing` | `KOBANA_BILLING_TOKEN` | [docs.billing.kobana.com.br](https://docs.billing.kobana.com.br) |
| Autonomous Inbox | `inbox` | `KOBANA_INBOX_TOKEN` | [docs.inbox.kobana.com.br](https://docs.inbox.kobana.com.br) |
| [Dunning](https://www.kobana.com.br/en/products/dunning) | `dunning` | `KOBANA_DUNNING_TOKEN` | [docs.dunning.kobana.com.br](https://docs.dunning.kobana.com.br) |
| [Magic Dashboard](https://www.kobana.com.br/en/products/magic-dashboard) | `dashboards` | `KOBANA_DASHBOARDS_TOKEN` | [docs.dashboards.kobana.com.br](https://docs.dashboards.kobana.com.br) |
| [Kadu Agent](https://www.kobana.com.br/en/ia/kadu) | `kadu` | `KOBANA_KADU_TOKEN` or `KADU_TOKEN` | [docs.kadu.ia.br](https://docs.kadu.ia.br) |

> **A token works for one product only.** See [Authentication](/getting-started/authentication).

## Hosts by environment

| Product | `production` (default) | `sandbox` |
|---|---|---|
| `banking` | `api.kobana.com.br` | `api.sandbox.kobana.com.br` |
| `finance` | `api.finance.kobana.com.br` | `api.finance.sandbox.kobana.com.br` |
| `billing` | `api.billing.kobana.com.br` | `api.billing.sandbox.kobana.com.br` |
| `inbox` | `api.inbox.kobana.com.br` | — |
| `dunning` | `api.dunning.kobana.com.br` | `api.dunning.sandbox.kobana.com.br` |
| `dashboards` | `api.dashboards.kobana.com.br` | — |
| `kadu` | `api.kadu.ia.br` | — |

`--env sandbox` on a product without a sandbox (`inbox`, `dashboards`, `kadu`) ends with an error.

## What each product does

- **`banking`** — unified banking operations: bank slips (boletos), Pix charges and payments, TED, statements and reconciliation, across more than 40 banks. API v1 (`bank-billets`, `customers`, `webhooks`) and v2 (`charge`, `payment`, `transfer`, `financial`…) coexist; the command does not distinguish them.
- **`finance`** — accounts payable and receivable, reconciliation, cash flow and multi-company. Every route requires the [workspace](/getting-started/authentication#workspace-x-workspace-id).
- **`billing`** — recurring billing: subscriptions, plans, invoices, proposals and tax invoices.
- **`inbox`** — inboxes with agents: receives emails, processes them with agents and delivers through webhooks.
- **`dunning`** — delinquency recovery: charges, collection rules, agreements, credit bureau listings and protests.
- **`dashboards`** — dashboards and customer portals over connected data sources.
- **`kadu`** — AI agent that runs financial routines (Tasks) from the organization's skills.

The resources and commands of each are in the [Manual](/manual).
