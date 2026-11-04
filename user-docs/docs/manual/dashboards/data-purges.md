---
id: data-purges
title: "kobana dashboards data-purges"
sidebar_label: "data-purges"
---

# kobana dashboards data-purges


Comandos de `kobana dashboards data-purges`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`data-purges list`](#data-purges-list) | `GET /v1/data_purges` | Contar dados apagáveis |
| [`data-purges delete`](#data-purges-delete) | `DELETE /v1/data_purges` | Apagar dados da organização |
| [`data-purges get`](#data-purges-get) | `GET /v1/data_purges/{id}` | Acompanhar limpeza |

## data-purges list

Contar dados apagáveis

```text
kobana dashboards data-purges list [OPÇÕES]
```

`GET /v1/data_purges`

Quantos registros cada recurso apagável (`portal_users`, `people`, `usage_history`) tem somando todos os workspaces. Exige `data.purge` (por padrão só donos da organização) e acesso a todos os workspaces.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | As contagens. |
| `404` | Recurso não encontrado na organização atual. |

## data-purges delete

Apagar dados da organização

```text
kobana dashboards data-purges delete [OPÇÕES]
```

`DELETE /v1/data_purges`

IRREVERSÍVEL: apaga em definitivo os recursos escolhidos em TODOS os workspaces da organização. No painel, a operação é confirmada com um código enviado por e-mail. Por isso, um token que identifica uma pessoa (token pessoal, OAuth ou chamada pelo MCP) recebe 403: a pessoa faz a limpeza pelo painel. Só tokens de máquina podem limpar pela API, e só com o verbo `delete` (ou `all`) — `write` não basta; com escopos de recurso, o token também precisa alcançar `dashboards/admin` (ou `dashboards/all`); sem isso, 403 `insufficient_scope`. A confirmação deles é `confirm`, que precisa repetir o ID da organização (de `GET /organization`) — uma chamada repetida ou endereçada à organização errada não passa por engano. Exige `data.purge` (por padrão só donos da organização) e acesso a todos os workspaces. Assíncrono: 202 com `job_id`. É um `DELETE` com corpo JSON (`resources`, `confirm`) — até 2026-10-29 era `POST`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `resources` | `array<string>` | sim | O que apagar: `portal_users`, `people`, `usage_history`. |
| `confirm` | `string` | sim | Confirmação: o ID da organização, digitado de novo. |

Outros campos e objetos aninhados: `kobana schema dashboards.data-purges.delete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Limpeza agendada; acompanhe em `GET /data_purges/{id}`. |
| `400` | Corpo ausente, JSON inválido ou campos incompatíveis. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | `confirm` não corresponde ao ID da organização — nada foi apagado. |

## data-purges get

Acompanhar limpeza

```text
kobana dashboards data-purges get [ID] [OPÇÕES]
```

`GET /v1/data_purges/{id}`

Progresso e resultado de um job de limpeza. Job de outra organização: 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O job. |
| `404` | Recurso não encontrado na organização atual. |
