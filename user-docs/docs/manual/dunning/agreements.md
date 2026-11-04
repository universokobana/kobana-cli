---
id: agreements
title: "kobana dunning agreements"
sidebar_label: "agreements"
---

# kobana dunning agreements


Comandos de `kobana dunning agreements`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`agreements installments`](#agreements-installments) | `GET /v1/agreements/{id}/installments` | Listar parcelas do acordo |
| [`agreements installments update`](#agreements-installments-update) | `PUT /v1/agreements/{id}/installments` | Atualizar parcela do acordo |
| [`agreements list`](#agreements-list) | `GET /v1/agreements` | Listar acordos |
| [`agreements create`](#agreements-create) | `POST /v1/agreements` | Criar acordo |
| [`agreements get`](#agreements-get) | `GET /v1/agreements/{id}` | Consultar acordo |
| [`agreements update`](#agreements-update) | `PUT /v1/agreements/{id}` | Atualizar acordo |
| [`agreements delete`](#agreements-delete) | `DELETE /v1/agreements/{id}` | Excluir acordo |
| [`agreements accept`](#agreements-accept) | `POST /v1/agreements/{id}/accept` | Aceitar acordo |
| [`agreements cancel`](#agreements-cancel) | `POST /v1/agreements/{id}/cancel` | Cancelar acordo |
| [`agreements pdf`](#agreements-pdf) | `GET /v1/agreements/{id}/pdf` | Termo de acordo (PDF) |

## agreements installments

Listar parcelas do acordo

```text
kobana dunning agreements installments [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/agreements/{id}/installments`

Lista as parcelas de um acordo (ordenadas por número) com um resumo agregado.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Parcelas do acordo com resumo. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements installments update

Atualizar parcela do acordo

```text
kobana dunning agreements installments update [ID] [OPÇÕES]
```

`PUT /v1/agreements/{id}/installments`

Atualiza uma parcela (pagamento/status). Permitido apenas em acordos `accepted` ou `active`; parcelas canceladas não podem ser alteradas (409). Primeiro pagamento ativa o acordo; com todas as parcelas pagas, o acordo é concluído (`completed`).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `installment_number` | `integer` | sim | Número da parcela (a partir de 1). |
| `paid_amount` | `number` | não | Valor pago da parcela. |
| `paid_at` | `string` | não | Data do pagamento da parcela. |
| `status` | `string` | não | Status (`pending` \| `paid` \| `overdue` \| `cancelled`). Valores: `pending`, `paid`, `overdue`, `cancelled`. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.agreements.installments.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Parcela atualizada (retorna a parcela e o acordo atualizado). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements list

Listar acordos

```text
kobana dunning agreements list [OPÇÕES]
```

`GET /v1/agreements`

Lista os acordos da organização com paginação e filtros.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `sort_by` | `string` | não | Campo de ordenação (nome público em snake_case, ex.: `due_date`). |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra por status do acordo. Valores: `proposed`, `accepted`, `active`, `completed`, `cancelled`, `defaulted`. |
| `person_id` | `string` | não | Filtra pelos acordos de um cliente (UUID). |
| `include` | `string` | não | Relações a incluir na resposta (`person`, `installments`, `negotiation_offer`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de acordos. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements create

Criar acordo

```text
kobana dunning agreements create [OPÇÕES]
```

`POST /v1/agreements`

Cria um acordo com status `proposed`. `final_total` = `original_total` − `discount_amount` (deve ser maior que zero) e `installment_value` = `final_total` / `number_of_installments`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_id` | `string (uuid)` | sim | Cliente (devedor) do acordo. |
| `original_total` | `number` | sim | Total original da dívida renegociada. |
| `number_of_installments` | `integer` | sim | Número de parcelas (1 a 120). |
| `first_due_date` | `string` | sim | Vencimento da primeira parcela (as demais vencem mensalmente). |
| `negotiation_offer_id` | `string (uuid)` | não | Oferta de negociação que originou o acordo. |
| `discount_amount` | `number` | não | Desconto concedido (padrão 0). |
| `terms_accepted` | `string` | não | Termos aceitos pelo devedor (texto/versão). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.agreements.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Acordo criado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements get

Consultar acordo

```text
kobana dunning agreements get [ID] [OPÇÕES]
```

`GET /v1/agreements/{id}`

Retorna um acordo pelo ID (sempre inclui dados resumidos do cliente).

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `include` | `string` | não | Relações a incluir na resposta (`installments`, `negotiation_offer`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Acordo encontrado. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements update

Atualizar acordo

```text
kobana dunning agreements update [ID] [OPÇÕES]
```

`PUT /v1/agreements/{id}`

Atualiza um acordo (parcial). Permitido apenas com status `proposed` ou `accepted` (409 nos demais).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `original_total` | `number` | não | Total original da dívida renegociada. |
| `discount_amount` | `number` | não | Desconto concedido (padrão 0). |
| `number_of_installments` | `integer` | não | Número de parcelas (1 a 120). |
| `first_due_date` | `string` | não | Vencimento da primeira parcela (as demais vencem mensalmente). |
| `terms_accepted` | `string` | não | Termos aceitos pelo devedor (texto/versão). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_data_mode` | `string` | não | Como aplicar `custom_data` no update: `replace` (default) sobrescreve o objeto inteiro; `merge` mescla (deep) o payload sobre o valor armazenado — objetos aninhados mesclam, escalares/arrays/null sobrescrevem a chave. Valores: `replace`, `merge`. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.agreements.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Acordo atualizado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements delete

Excluir acordo

```text
kobana dunning agreements delete [ID] [OPÇÕES]
```

`DELETE /v1/agreements/{id}`

Exclui (soft delete) um acordo, marcando-o como `cancelled`. Acordos `completed` não podem ser excluídos (409).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Acordo excluído. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements accept

Aceitar acordo

```text
kobana dunning agreements accept [ID] [OPÇÕES]
```

`POST /v1/agreements/{id}/accept`

Aceita um acordo `proposed` (409 nos demais status): registra `accepted_at`/`accepted_ip` e cria as parcelas com vencimentos mensais a partir de `first_due_date`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `accepted_ip` | `string` | não | IP registrado no aceite (trilha de auditoria). |
| `terms_accepted` | `string` | não | Termos aceitos pelo devedor (texto/versão). |

Outros campos e objetos aninhados: `kobana schema dunning.agreements.accept`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Acordo aceito (retorna o acordo com as parcelas criadas). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements cancel

Cancelar acordo

```text
kobana dunning agreements cancel [ID] [OPÇÕES]
```

`POST /v1/agreements/{id}/cancel`

Cancela um acordo, registrando `cancelled_at` e o motivo, e cancela as parcelas pendentes/vencidas. Acordos `completed` ou já cancelados retornam 409.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `reason` | `string` | sim | Motivo do cancelamento (obrigatório; gravado em `cancellation_reason`). |

Outros campos e objetos aninhados: `kobana schema dunning.agreements.cancel`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Acordo cancelado (retorna o acordo com as parcelas). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## agreements pdf

Termo de acordo (PDF)

```text
kobana dunning agreements pdf [ID] [OPÇÕES]
```

`GET /v1/agreements/{id}/pdf`

Gera o termo de acordo em PDF (documento jurídico) com credor, devedor, totais e parcelas, escopado pela organização. Responde `application/pdf` como anexo binário.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | PDF do termo de acordo (anexo binário `application/pdf`). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
