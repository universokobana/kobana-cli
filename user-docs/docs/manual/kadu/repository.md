---
id: repository
title: "kobana kadu repository"
sidebar_label: "repository"
description: "Kadu Playbook da organização"
---

# kobana kadu repository

Kadu Playbook da organização

Comandos de `kobana kadu repository`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`repository`](#repository) | `GET /v1/repository` | Kadu Playbook da organização |
| [`repository sync`](#repository-sync) | `POST /v1/repository/sync` | Sincroniza o Kadu Playbook |

## repository

Kadu Playbook da organização

```text
kobana kadu repository [OPÇÕES] [COMANDO]
```

`GET /v1/repository`

Conectar o repositório é a instalação do GitHub App, feita no console (fluxo OAuth no navegador).

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## repository sync

Sincroniza o Kadu Playbook

```text
kobana kadu repository sync [OPÇÕES]
```

`POST /v1/repository/sync`

Enfileira a sincronização (o worker clona o repositório e atualiza o catálogo de Tasks). Acompanhe `synced_at` e `sync_error` em `GET /repository`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | accepted |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | NO_PLUGIN_REPO / NO_DEPLOY_KEY |
| `503` | QUEUE_UNAVAILABLE / AS_UNAVAILABLE |
