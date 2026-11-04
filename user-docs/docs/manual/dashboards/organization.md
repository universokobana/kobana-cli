---
id: organization
title: "kobana dashboards organization"
sidebar_label: "organization"
description: "Obter organização atual"
---

# kobana dashboards organization

Obter organização atual

Comandos de `kobana dashboards organization`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Obter organização atual |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Atualizar organização atual |

## organization

Obter organização atual

```text
kobana dashboards organization [OPÇÕES] [COMANDO]
```

`GET /v1/organization`

Retorna os dados da organização à qual o token autenticado pertence. Não há parâmetro `:id` — a organização é sempre resolvida a partir da chave da API.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da organização atual. |
| `404` | Recurso não encontrado na organização atual. |

## organization update

Atualizar organização atual

```text
kobana dashboards organization update [OPÇÕES]
```

`PATCH /v1/organization`

Atualiza o perfil da organização autenticada. Só `addresses` e `custom_metadata` são editáveis (`custom_metadata: null` mantém o valor atual). `name`, `email` e `phone` são espelho da Conta Kobana: só a sincronização com a Conta Kobana os grava, e o corpo que os traz responde 200 sem alterá-los. Identidade (`id`, `public_id`, `external_id`, `status`, `slug`) também é ignorada. Campos comerciais (como `bypass_subscription`, `courtesy` e `monthly_cost_limit`) são definidos pela Kobana e respondem 403.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `addresses` | `array<object>` | não | Lista de endereços da organização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema dashboards.organization.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Organização atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
