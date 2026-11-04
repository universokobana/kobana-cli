---
id: usage
title: "kobana kadu usage"
sidebar_label: "usage"
description: "Uso e custo das execuções, das conversas e da consolidação de memórias"
---

# kobana kadu usage

Uso e custo das execuções, das conversas e da consolidação de memórias

Comandos de `kobana kadu usage`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`usage`](#usage) | `GET /v1/usage` | Uso e custo das execuções, das conversas e da consolidação de memórias |
| [`usage performance`](#usage-performance) | `GET /v1/usage/performance` | Desempenho: duração, sucesso e erro, memória, arquivos e volumes |

## usage

Uso e custo das execuções, das conversas e da consolidação de memórias

```text
kobana kadu usage [OPÇÕES] [COMANDO]
```

`GET /v1/usage`

Agregado por dia (no fuso da organização) × grupo, somando execuções, turnos de conversa do bot do Slack (origem `slack_chat`, sem workspace/atividade/Task) e consolidação de memórias (origem `memory_consolidation`, no workspace/atividade/Task da execução de origem). `by_kind` separa os três: o que é conversa e o que não é. Padrão: últimos 30 dias; período máximo de 366 dias. `group` é o id (tier, workspace, activity, task) ou a origem; null = sem valor.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `from` | `string` | não |  |
| `to` | `string` | não |  |
| `group_by` | `string` | não | Valores: `tier`, `workspace`, `activity`, `task`, `source`. |
| `workspace_id` | `string` | não |  |
| `activity_id` | `string` | não |  |
| `task_id` | `string` | não |  |
| `source` | `string` | não |  |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## usage performance

Desempenho: duração, sucesso e erro, memória, arquivos e volumes

```text
kobana kadu usage performance [OPÇÕES]
```

`GET /v1/usage/performance`

Série diária (no fuso da organização) com a duração média das execuções (total, bootstrap e execução efetiva), as execuções terminadas por resultado e as memórias existentes no fim do dia (quantidade e bytes de conteúdo + resumo) e o tamanho acumulado dos arquivos de entrada e saída, filtrável pela extensão (`file_type`), e o dos volumes (`volume_id`). Padrão: últimos 30 dias; período máximo de 366 dias. Memórias não têm tarefa nem origem: só workspace e atividade filtram a memória. Exige Analytics › Desempenho liberado.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `from` | `string` | não |  |
| `to` | `string` | não |  |
| `workspace_id` | `string` | não |  |
| `activity_id` | `string` | não |  |
| `task_id` | `string` | não |  |
| `source` | `string` | não |  |
| `file_type` | `string` | não | Extensão dos arquivos no gráfico de arquivos (ex.: `pdf`); `__none__` = sem extensão. Omitido = todas. |
| `volume_id` | `string` | não | Só este volume na série de volumes. Id fora dos volumes da organização = todos. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
