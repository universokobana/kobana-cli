---
id: products
title: Productos
sidebar_label: Productos
sidebar_position: 1
description: Los siete productos del CLI, sus hosts, variables de token y entornos.
---

# Productos

Cada producto Kobana es una API independiente, con hosts y credenciales propios. El producto es siempre el primer segmento del comando.

| Producto | Comando | Credencial | Documentación de la API |
|---|---|---|---|
| [Gateway Bancario](https://www.kobana.com.br/es/productos/gateway-bancario) | `banking` | `KOBANA_BANKING_TOKEN` | [docs.banking.kobana.com.br](https://docs.banking.kobana.com.br) |
| [Financiero Inteligente](https://www.kobana.com.br/es/productos/financiero-inteligente) | `finance` | `KOBANA_FINANCE_TOKEN` | [docs.finance.kobana.com.br](https://docs.finance.kobana.com.br) |
| [Facturación Automática](https://www.kobana.com.br/es/productos/facturacion-automatica) | `billing` | `KOBANA_BILLING_TOKEN` | [docs.billing.kobana.com.br](https://docs.billing.kobana.com.br) |
| Inbox Autónomo | `inbox` | `KOBANA_INBOX_TOKEN` | [docs.inbox.kobana.com.br](https://docs.inbox.kobana.com.br) |
| [Cobranza](https://www.kobana.com.br/es/productos/cobranza) | `dunning` | `KOBANA_DUNNING_TOKEN` | [docs.dunning.kobana.com.br](https://docs.dunning.kobana.com.br) |
| [Dashboard Mágico](https://www.kobana.com.br/es/productos/dashboard-magico) | `dashboards` | `KOBANA_DASHBOARDS_TOKEN` | [docs.dashboards.kobana.com.br](https://docs.dashboards.kobana.com.br) |
| [Agente Kadu](https://www.kobana.com.br/es/ia/kadu) | `kadu` | `KOBANA_KADU_TOKEN` o `KADU_TOKEN` | [docs.kadu.ia.br](https://docs.kadu.ia.br) |

> **Un token vale para un solo producto.** Consulta [Autenticación](/getting-started/authentication).

## Hosts por entorno

| Producto | `production` (por defecto) | `sandbox` |
|---|---|---|
| `banking` | `api.kobana.com.br` | `api.sandbox.kobana.com.br` |
| `finance` | `api.finance.kobana.com.br` | `api.finance.sandbox.kobana.com.br` |
| `billing` | `api.billing.kobana.com.br` | `api.billing.sandbox.kobana.com.br` |
| `inbox` | `api.inbox.kobana.com.br` | — |
| `dunning` | `api.dunning.kobana.com.br` | `api.dunning.sandbox.kobana.com.br` |
| `dashboards` | `api.dashboards.kobana.com.br` | — |
| `kadu` | `api.kadu.ia.br` | — |

`--env sandbox` en un producto sin sandbox (`inbox`, `dashboards`, `kadu`) termina con error.

## Qué hace cada producto

- **`banking`** — operaciones bancarias unificadas: boletos, cobros y pagos Pix, TED, extractos y conciliación, en más de 40 bancos. Conviven la API v1 (`bank-billets`, `customers`, `webhooks`) y la v2 (`charge`, `payment`, `transfer`, `financial`…); el comando no las distingue.
- **`finance`** — cuentas por pagar y por cobrar, conciliación, flujo de caja y multiempresa. Toda ruta exige el [espacio de trabajo](/getting-started/authentication#espacio-de-trabajo-x-workspace-id).
- **`billing`** — cobro recurrente: suscripciones, planes, facturas, propuestas y facturas electrónicas.
- **`inbox`** — bandejas de entrada con agentes: recibe correos electrónicos, los procesa con agentes y los entrega por webhooks.
- **`dunning`** — recuperación de morosidad: cobros, flujos de cobranza, acuerdos, reportes a burós de crédito y protestos.
- **`dashboards`** — dashboards y portales del cliente sobre fuentes de datos conectadas.
- **`kadu`** — agente de IA que ejecuta rutinas financieras (Tasks) a partir de las skills de la organización.

Los recursos y comandos de cada uno están en el [Manual](/manual).
