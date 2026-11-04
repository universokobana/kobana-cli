---
id: reports
title: "kobana banking reports"
sidebar_label: "reports"
---

# kobana banking reports


Comandos de `kobana banking reports`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`reports bank-billets`](#reports-bank-billets) | `GET /v1/reports/bank_billets` | Contagem de Boletos. |

## reports bank-billets

Contagem de Boletos.

```text
kobana banking reports bank-billets [OPÇÕES]
```

`GET /v1/reports/bank_billets`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `expire_from` | `string` | não | A partir da Data de vencimento. |
| `expire_to` | `string` | não | Até a Data de vencimento. |
| `paid_from` | `string` | não | A partir da Data de pagamento. |
| `paid_to` | `string` | não | Até a Data de pagamento. |
| `registered_from` | `string` | não | A partir da Data de registro. |
| `registered_to` | `string` | não | Até a Data de registro. |
| `created_from` | `string` | não | A partir da Data de criação. |
| `created_to` | `string` | não | Até a Data de criação. |
| `status` | `string` | não | Status do boleto. Valores: `generating`, `opened`, `paid`, `overdue`, `canceled`, `draft`, `generation_failed`, `validation_failed`, `blocked`, `chargeback`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de boletos |
