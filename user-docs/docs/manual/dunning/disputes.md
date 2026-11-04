---
id: disputes
title: "kobana dunning disputes"
sidebar_label: "disputes"
---

# kobana dunning disputes


Comandos de `kobana dunning disputes`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`disputes list`](#disputes-list) | `GET /v1/disputes` | Listar disputas |
| [`disputes create`](#disputes-create) | `POST /v1/disputes` | Abrir disputa |
| [`disputes get`](#disputes-get) | `GET /v1/disputes/{id}` | Consultar disputa |
| [`disputes update`](#disputes-update) | `PATCH /v1/disputes/{id}` | Executar ação na disputa |

## disputes list

Listar disputas

```text
kobana dunning disputes list [OPÇÕES]
```

`GET /v1/disputes`

Lista as disputas da organização, ordenadas por abertura (mais recente primeiro). Não aceita ordenação customizada.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `status` | `string` | não | Filtra por status (`open` \| `under_review` \| `resolved_valid` \| `resolved_invalid` \| `canceled`). Valores: `open`, `under_review`, `resolved_valid`, `resolved_invalid`, `canceled`. |
| `charge_id` | `string` | não | Filtra pela cobrança (UUID). |
| `person_id` | `string` | não | Filtra pelo cliente (UUID). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de disputas. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## disputes create

Abrir disputa

```text
kobana dunning disputes create [OPÇÕES]
```

`POST /v1/disputes`

Abre uma disputa sobre uma cobrança e pausa a régua imediatamente. Se já houver disputa aberta ou em análise para a cobrança, retorna a existente (não duplica). Suporta `Idempotency-Key`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `charge_id` | `string (uuid)` | sim | Cobrança disputada. |
| `type` | `string` | sim | Tipo da contestação (`already_paid` \| `not_recognized` \| `wrong_amount` \| `service_not_provided` \| `incorrect_invoice` \| `wrong_recipient` \| `other`). Valores: `already_paid`, `not_recognized`, `wrong_amount`, `service_not_provided`, `incorrect_invoice`, `wrong_recipient`, `other`. |
| `reason` | `string` | não | Relato do devedor/operador. |
| `disputed_amount` | `number` | não | Valor contestado (número). Omita para contestar o valor integral. |
| `documents` | `array<object>` | não | Anexos/referências de evidência. |
| `assigned_to_id` | `string (uuid)` | não | Usuário responsável pela análise (precisa pertencer à organização). |
| `sla_days` | `integer` | não | Dias de SLA para resolução (máx. 60; padrão do sistema quando omitido). |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |

Outros campos e objetos aninhados: `kobana schema dunning.disputes.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Disputa aberta (ou disputa aberta existente da cobrança). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## disputes get

Consultar disputa

```text
kobana dunning disputes get [ID] [OPÇÕES]
```

`GET /v1/disputes/{id}`

Retorna uma disputa pelo ID, com cobrança completa, cliente e responsável.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Disputa encontrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## disputes update

Executar ação na disputa

```text
kobana dunning disputes update [ID] [OPÇÕES]
```

`PATCH /v1/disputes/{id}`

Executa uma ação de fluxo na disputa: `start_review`, `resolve` ou `cancel`. Transições inválidas (ex.: `start_review` quando o status não é `open`) retornam 409.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema dunning.disputes.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Disputa atualizada pela ação. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |
