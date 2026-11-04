---
id: protests
title: "kobana dunning protests"
sidebar_label: "protests"
---

# kobana dunning protests


Comandos de `kobana dunning protests`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`protests list`](#protests-list) | `GET /v1/protests` | Listar protestos |
| [`protests create`](#protests-create) | `POST /v1/protests` | Criar protesto |
| [`protests get`](#protests-get) | `GET /v1/protests/{id}` | Consultar protesto |
| [`protests update`](#protests-update) | `PUT /v1/protests/{id}` | Atualizar protesto |
| [`protests cancel`](#protests-cancel) | `POST /v1/protests/{id}/cancel` | Solicitar cancelamento do protesto |

## protests list

Listar protestos

```text
kobana dunning protests list [OPÇÕES]
```

`GET /v1/protests`

Lista os protestos da organização com paginação e filtros. Use `include=person,charge` para embutir os resumos de cliente e cobrança.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `sort_by` | `string` | não | Campo de ordenação (`created_at` \| `updated_at` \| `amount` \| `protest_date` \| `sent_at`). Valores: `createdAt`, `updatedAt`, `amount`, `protestDate`, `sentAt`. |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra por status (`pending` \| `sent` \| `intimated` \| `protested` \| `paid` \| `cancelled`). Valores: `pending`, `sent`, `intimated`, `protested`, `paid`, `cancelled`. |
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
| `200` | Lista paginada de protestos. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## protests create

Criar protesto

```text
kobana dunning protests create [OPÇÕES]
```

`POST /v1/protests`

Registra um protesto para uma cobrança. O registro nasce `pending` e SÓ é enviado ao cartório após revisão e aprovação humana (workflow jurídico; evento `protest.review_required`). Retorna 409 se já existir protesto ativo (`pending`/`sent`/`intimated`/`protested`) para a mesma cobrança.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_id` | `string (uuid)` | sim | Cliente protestado (UUID). |
| `charge_id` | `string (uuid)` | sim | Cobrança protestada (UUID). |
| `amount` | `number` | sim | Valor a protestar (número decimal positivo). |
| `notary` | `string` | não | Nome do cartório de protesto. |
| `notary_code` | `string` | não | Código/identificador do cartório. |
| `fees` | `number` | não | Emolumentos/custas do cartório (número decimal). |
| `external_id` | `string` | não | Identificador no SEU sistema ou protocolo no cartório. |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.protests.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Protesto criado (aguardando revisão humana). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## protests get

Consultar protesto

```text
kobana dunning protests get [ID] [OPÇÕES]
```

`GET /v1/protests/{id}`

Retorna um protesto pelo ID, com os resumos de cliente e cobrança.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Protesto encontrado. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## protests update

Atualizar protesto

```text
kobana dunning protests update [ID] [OPÇÕES]
```

`PUT /v1/protests/{id}`

Atualiza um protesto (parcial). É a etapa de aprovação do workflow jurídico: exige permissão de revisão (`approve`) e é onde o status avança de `pending` para `sent`/`intimated`/`protested` após a revisão humana.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Status (`pending` = aguardando revisão humana obrigatória, `sent` = enviado ao cartório, `intimated` = devedor intimado, `protested` = protestado, `paid` = pago, `cancelled` = cancelado). Valores: `pending`, `sent`, `intimated`, `protested`, `paid`, `cancelled`. |
| `notary` | `string` | não | Nome do cartório de protesto. |
| `notary_code` | `string` | não | Código/identificador do cartório. |
| `fees` | `number` | não | Emolumentos/custas do cartório (número decimal). |
| `external_id` | `string` | não | Identificador no SEU sistema ou protocolo no cartório. |
| `sent_at` | `string (date-time)` | não | Data de envio ao cartório (ISO 8601). |
| `intimation_date` | `string (date-time)` | não | Data de intimação do devedor (ISO 8601). |
| `protest_date` | `string (date-time)` | não | Data de lavratura do protesto (ISO 8601). |
| `payment_date` | `string (date-time)` | não | Data de pagamento em cartório (ISO 8601). |
| `cancellation_date` | `string (date-time)` | não | Data de cancelamento do protesto (ISO 8601). |
| `response_data` | `object` | não | Resposta bruta do cartório (payload da integração). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.protests.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Protesto atualizado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## protests cancel

Solicitar cancelamento do protesto

```text
kobana dunning protests cancel [ID] [OPÇÕES]
```

`POST /v1/protests/{id}/cancel`

Solicita o cancelamento de um protesto. Não é permitido para protestos já `cancelled` nem `paid`; o status vira `cancelled` e o motivo (se enviado) fica em `responseData.cancellationReason`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cancellation_reason` | `string` | não | Motivo do cancelamento. |

Outros campos e objetos aninhados: `kobana schema dunning.protests.cancel`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cancelamento registrado; protesto atualizado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
