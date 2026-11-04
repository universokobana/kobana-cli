---
id: metrics
title: "kobana dunning metrics"
sidebar_label: "metrics"
---

# kobana dunning metrics


Comandos de `kobana dunning metrics`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`metrics recovery`](#metrics-recovery) | `GET /v1/metrics/recovery` | Métricas de recuperação |

## metrics recovery

Métricas de recuperação

```text
kobana dunning metrics recovery [OPÇÕES]
```

`GET /v1/metrics/recovery`

Retorna as métricas de recuperação do período (padrão: últimos 6 meses, contados a partir do primeiro dia do mês inicial).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `months` | `integer` | não | Janela em meses (1–24; padrão 6). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Métricas calculadas. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
