---
id: usage
title: "kobana inbox usage"
sidebar_label: "usage"
---

# kobana inbox usage


Comandos de `kobana inbox usage`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`usage cost`](#usage-cost) | `GET /v1/usage/cost` | Custo por modelo |
| [`usage summary`](#usage-summary) | `GET /v1/usage/summary` | Resumo de uso |

## usage cost

Custo por modelo

```text
kobana inbox usage cost [OPÇÕES]
```

`GET /v1/usage/cost`

Custo e tokens por modelo LLM no período: totais por modelo e série diária.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `from` | `string` | não | Primeiro dia UTC, `YYYY-MM-DD`. Padrão: 29 dias antes de `to`. |
| `to` | `string` | não | Último dia UTC (inclusive), `YYYY-MM-DD`. Padrão: hoje. `from`–`to` pode abranger no máximo 366 dias; um intervalo maior é 400 `VALIDATION_ERROR`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |

## usage summary

Resumo de uso

```text
kobana inbox usage summary [OPÇÕES]
```

`GET /v1/usage/summary`

E-mails processados, tokens e custo por dia UTC no período (somente execuções de agente com sucesso), mais os contadores mensais correntes versus os limites do workspace. Custo em USD, como faturado pelo AI Gateway.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `from` | `string` | não | Primeiro dia UTC, `YYYY-MM-DD`. Padrão: 29 dias antes de `to`. |
| `to` | `string` | não | Último dia UTC (inclusive), `YYYY-MM-DD`. Padrão: hoje. `from`–`to` pode abranger no máximo 366 dias; um intervalo maior é 400 `VALIDATION_ERROR`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
