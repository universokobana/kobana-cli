---
id: negativations
title: "kobana dunning negativations"
sidebar_label: "negativations"
---

# kobana dunning negativations


Comandos de `kobana dunning negativations`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`negativations list`](#negativations-list) | `GET /v1/negativations` | Listar negativações |
| [`negativations create`](#negativations-create) | `POST /v1/negativations` | Criar negativação |
| [`negativations get`](#negativations-get) | `GET /v1/negativations/{id}` | Consultar negativação |
| [`negativations update`](#negativations-update) | `PUT /v1/negativations/{id}` | Atualizar negativação |
| [`negativations remove`](#negativations-remove) | `POST /v1/negativations/{id}/remove` | Solicitar baixa da negativação |

## negativations list

Listar negativações

```text
kobana dunning negativations list [OPÇÕES]
```

`GET /v1/negativations`

Lista as negativações da organização com paginação e filtros. Use `include=person,charge` para embutir os resumos de cliente e cobrança.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `sort_by` | `string` | não | Campo de ordenação (`created_at` \| `updated_at` \| `amount` \| `registered_at`). Valores: `createdAt`, `updatedAt`, `amount`, `registeredAt`. |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra por status (`pending` \| `active` \| `removed` \| `failed`). Valores: `pending`, `active`, `removed`, `failed`. |
| `bureau` | `string` | não | Filtra pelo birô de crédito (`serasa` \| `spc` \| `boa_vista`). Valores: `serasa`, `spc`, `boa_vista`. |
| `person_id` | `string` | não | Filtra pelo cliente (UUID). |
| `charge_id` | `string` | não | Filtra pela cobrança (UUID). |
| `include` | `string` | não | Relações a embutir, separadas por vírgula (`person`, `charge`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de negativações. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## negativations create

Criar negativação

```text
kobana dunning negativations create [OPÇÕES]
```

`POST /v1/negativations`

Registra uma negativação para uma cobrança. O registro nasce `pending` e SÓ é enviado ao birô após revisão e aprovação humana (workflow jurídico; evento `negativation.review_required`). Retorna 409 se já existir negativação `pending`/`active` para a mesma cobrança no mesmo birô.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_id` | `string (uuid)` | sim | Cliente negativado (UUID). |
| `charge_id` | `string (uuid)` | sim | Cobrança negativada (UUID). |
| `bureau` | `string` | sim | Birô de crédito (`serasa` \| `spc` \| `boa_vista`). Valores: `serasa`, `spc`, `boa_vista`. |
| `amount` | `number` | sim | Valor a negativar (número decimal positivo). |
| `external_id` | `string` | não | Identificador no SEU sistema ou protocolo no birô. |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.negativations.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Negativação criada (aguardando revisão humana). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## negativations get

Consultar negativação

```text
kobana dunning negativations get [ID] [OPÇÕES]
```

`GET /v1/negativations/{id}`

Retorna uma negativação pelo ID, com os resumos de cliente e cobrança.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Negativação encontrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## negativations update

Atualizar negativação

```text
kobana dunning negativations update [ID] [OPÇÕES]
```

`PUT /v1/negativations/{id}`

Atualiza uma negativação (parcial). É a etapa de aprovação do workflow jurídico: exige permissão de revisão (`approve`) e é onde o status sai de `pending` para `active` após a revisão humana.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Status (`pending` = aguardando revisão humana obrigatória, `active` = efetivada no birô, `removed` = baixada, `failed` = falhou). Valores: `pending`, `active`, `removed`, `failed`. |
| `external_id` | `string` | não | Identificador no SEU sistema ou protocolo no birô. |
| `registered_at` | `string (date-time)` | não | Data em que a negativação foi efetivada no birô (ISO 8601). |
| `removed_at` | `string (date-time)` | não | Data da baixa no birô (ISO 8601). |
| `removal_reason` | `string` | não | Motivo da baixa da negativação. |
| `response_data` | `object` | não | Resposta bruta do birô (payload da integração). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.negativations.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Negativação atualizada. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## negativations remove

Solicitar baixa da negativação

```text
kobana dunning negativations remove [ID] [OPÇÕES]
```

`POST /v1/negativations/{id}/remove`

Solicita a baixa de uma negativação no birô. Só é permitido para registros `active`; o status vira `removed` e o motivo (se enviado) fica em `removal_reason`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `removal_reason` | `string` | não | Motivo da baixa da negativação. |

Outros campos e objetos aninhados: `kobana schema dunning.negativations.remove`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Baixa registrada; negativação atualizada. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
