---
id: charges
title: "kobana dunning charges"
sidebar_label: "charges"
---

# kobana dunning charges


Comandos de `kobana dunning charges`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`charges bulk delete`](#charges-bulk-delete) | `DELETE /v1/charges/bulk` | Excluir cobranças em massa |
| [`charges list`](#charges-list) | `GET /v1/charges` | Listar cobranças |
| [`charges create`](#charges-create) | `POST /v1/charges` | Criar cobrança |
| [`charges bulk-assign-rule`](#charges-bulk-assign-rule) | `POST /v1/charges/bulk-assign-rule` | Aplicar régua em massa |
| [`charges bulk-cancel`](#charges-bulk-cancel) | `POST /v1/charges/bulk-cancel` | Cancelar cobranças em massa |
| [`charges get`](#charges-get) | `GET /v1/charges/{id}` | Consultar cobrança |
| [`charges update`](#charges-update) | `PUT /v1/charges/{id}` | Atualizar cobrança |
| [`charges delete`](#charges-delete) | `DELETE /v1/charges/{id}` | Excluir cobrança |
| [`charges notify`](#charges-notify) | `POST /v1/charges/{id}/notify` | Notificar cobrança |
| [`charges settlement-pdf`](#charges-settlement-pdf) | `GET /v1/charges/{id}/settlement-pdf` | Carta de quitação (PDF) |

## charges bulk delete

Excluir cobranças em massa

```text
kobana dunning charges bulk delete [OPÇÕES]
```

`DELETE /v1/charges/bulk`

Exclui (soft delete, irreversível) até 100 cobranças numa chamada, com os ids no corpo JSON, cascateando para tarefas e ofertas relacionadas. Cobrança com registros associados (notificações, interações, negativações, protestos) não é excluível e volta em `failed` com `has_related_records`. Exige o nível de acesso `delete` (ou `all`) — `write` não basta. Substituiu o `POST /charges/bulk-archive`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | IDs dos registros-alvo (1 a 100 por chamada). Duplicados são ignorados. |

Outros campos e objetos aninhados: `kobana schema dunning.charges.bulk.delete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado por item da exclusão em massa. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## charges list

Listar cobranças

```text
kobana dunning charges list [OPÇÕES]
```

`GET /v1/charges`

Lista as cobranças da organização com paginação e filtros. Cobranças pendentes vencidas são promovidas a `overdue` na leitura.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `sort_by` | `string` | não | Campo de ordenação (nome público em snake_case, ex.: `due_date`). |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca por número do documento, descrição, `external_id`, nome ou documento do cliente. |
| `person_id` | `string` | não | Filtra pelas cobranças de um cliente (UUID). |
| `status` | `string` | não | Filtra por status. Aceita um valor ou lista separada por vírgula (`pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`). |
| `due_date_from` | `string` | não | Vencimento a partir de (inclusive, `YYYY-MM-DD` ou datetime ISO). |
| `due_date_to` | `string` | não | Vencimento até (inclusive, `YYYY-MM-DD` ou datetime ISO). |
| `include` | `string` | não | Relações a incluir na resposta (`person`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de cobranças. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## charges create

Criar cobrança

```text
kobana dunning charges create [OPÇÕES]
```

`POST /v1/charges`

Cria uma cobrança para um cliente. Sem `collection_rule_id`, a régua padrão da organização é aplicada. `current_amount` = original + juros + multa − desconto; cobrança já vencida entra como `overdue`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_id` | `string (uuid)` | sim | Cliente (devedor) da cobrança. |
| `original_amount` | `number` | sim | Valor original do título. |
| `issue_date` | `string` | sim | Data de emissão. |
| `due_date` | `string` | sim | Data de vencimento. |
| `collection_rule_id` | `string (uuid)` | não | Régua de cobrança que acompanha o título. Se omitido na criação, usa a régua padrão da organização. |
| `document_number` | `string` | não | Número do documento de origem (nota fiscal, contrato etc.). |
| `description` | `string` | não | Descrição livre da cobrança. |
| `interest_amount` | `number` | não | Juros acumulados (padrão 0). |
| `fine_amount` | `number` | não | Multa (padrão 0). |
| `discount_amount` | `number` | não | Desconto aplicado (padrão 0). |
| `status` | `string` | não | Status (`pending` \| `paid` \| `overdue` \| `cancelled` \| `negotiated` \| `protested` \| `negatived` \| `written_off`). Valores: `pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`. |
| `payment_method` | `string` | não | Meio de pagamento (`boleto` \| `pix` \| `credit_card` \| `transfer`). Valores: `boleto`, `pix`, `credit_card`, `transfer`, `null`. |
| `barcode` | `string` | não | Linha digitável/código de barras do boleto. |
| `pix_emv` | `string` | não | Código Pix copia-e-cola (EMV). |
| `payment_url` | `string (uri)` | não | URL da página de pagamento. |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |
| `charge_policy` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema dunning.charges.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Cobrança criada. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## charges bulk-assign-rule

Aplicar régua em massa

```text
kobana dunning charges bulk-assign-rule [OPÇÕES]
```

`POST /v1/charges/bulk-assign-rule`

Aplica uma régua de cobrança a até 100 cobranças. A entrada na régua respeita a idempotência do motor: cobrança com enrollment ativo/pausado mantém o enrollment vigente. Só cobranças `pending`/`overdue` são elegíveis; as demais voltam em `failed` com `invalid_status`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | IDs dos registros-alvo (1 a 100 por chamada). Duplicados são ignorados. |
| `collection_rule_id` | `string (uuid)` | sim | ID da régua de cobrança a aplicar (deve pertencer à organização). |

Outros campos e objetos aninhados: `kobana schema dunning.charges.bulk-assign-rule`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado por item da aplicação de régua em massa. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## charges bulk-cancel

Cancelar cobranças em massa

```text
kobana dunning charges bulk-cancel [OPÇÕES]
```

`POST /v1/charges/bulk-cancel`

Cancela até 100 cobranças numa chamada. Cada cancelamento passa pelo motor da régua (valida a transição de status, encerra o enrollment e suprime envios pendentes). Resultado parcial não é erro: itens não canceláveis voltam em `failed` com o motivo (`not_found`, `invalid_status`).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | IDs dos registros-alvo (1 a 100 por chamada). Duplicados são ignorados. |

Outros campos e objetos aninhados: `kobana schema dunning.charges.bulk-cancel`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado por item do cancelamento em massa. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## charges get

Consultar cobrança

```text
kobana dunning charges get [ID] [OPÇÕES]
```

`GET /v1/charges/{id}`

Retorna uma cobrança pelo ID. Use `include=engine` para receber a telemetria do motor (enrollment na régua e execuções de etapa).

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `include` | `string` | não | Relações a incluir na resposta (`person`, `engine` — telemetria da régua). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cobrança encontrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## charges update

Atualizar cobrança

```text
kobana dunning charges update [ID] [OPÇÕES]
```

`PUT /v1/charges/{id}`

Atualiza os dados de uma cobrança (parcial). `status`, `paid_amount` e `paid_at` são do motor: reenviar o valor gravado passa, mas mudá-los responde 422 `ENGINE_MANAGED_FIELD` — pagamento entra por `POST /charges/{id}/pay`. A única transição aceita aqui é `status: cancelled` (409 se o status atual não permite).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_id` | `string (uuid)` | não | Cliente (devedor) da cobrança. |
| `collection_rule_id` | `string (uuid)` | não | Régua de cobrança que acompanha o título. Se omitido na criação, usa a régua padrão da organização. |
| `document_number` | `string` | não | Número do documento de origem (nota fiscal, contrato etc.). |
| `description` | `string` | não | Descrição livre da cobrança. |
| `original_amount` | `number` | não | Valor original do título. |
| `interest_amount` | `number` | não | Juros acumulados (padrão 0). |
| `fine_amount` | `number` | não | Multa (padrão 0). |
| `discount_amount` | `number` | não | Desconto aplicado (padrão 0). |
| `issue_date` | `string` | não | Data de emissão. |
| `due_date` | `string` | não | Data de vencimento. |
| `status` | `string` | não | Status (`pending` \| `paid` \| `overdue` \| `cancelled` \| `negotiated` \| `protested` \| `negatived` \| `written_off`). Valores: `pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`. |
| `payment_method` | `string` | não | Meio de pagamento (`boleto` \| `pix` \| `credit_card` \| `transfer`). Valores: `boleto`, `pix`, `credit_card`, `transfer`, `null`. |
| `barcode` | `string` | não | Linha digitável/código de barras do boleto. |
| `pix_emv` | `string` | não | Código Pix copia-e-cola (EMV). |
| `payment_url` | `string (uri)` | não | URL da página de pagamento. |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |
| `charge_policy` | `object` | não |  |
| `paid_amount` | `number` | não | Valor efetivamente pago. |
| `paid_at` | `string` | não | Data do pagamento. |
| `custom_data_mode` | `string` | não | Como aplicar `custom_data` no update: `replace` (default) sobrescreve o objeto inteiro; `merge` mescla (deep) o payload sobre o valor armazenado — objetos aninhados mesclam, escalares/arrays/null sobrescrevem a chave. Valores: `replace`, `merge`. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.charges.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cobrança atualizada. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## charges delete

Excluir cobrança

```text
kobana dunning charges delete [ID] [OPÇÕES]
```

`DELETE /v1/charges/{id}`

Exclui (soft delete) uma cobrança e as tarefas e ofertas relacionadas. Cobrança com notificações, interações, negativações ou protestos associados não pode ser excluída (409).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Cobrança excluída (retorna mensagem de confirmação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## charges notify

Notificar cobrança

```text
kobana dunning charges notify [ID] [OPÇÕES]
```

`POST /v1/charges/{id}/notify`

Cria uma notificação manual para a cobrança e a envia para a fila de processamento. O destinatário tem de ser um contato cadastrado da pessoa (422 `UNKNOWN_RECIPIENT` caso contrário). Atualiza `last_notification_at`; o progresso da régua (`current_step`) não muda — só o motor o avança.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `channel` | `string` | sim | Canal (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `recipient` | `string` | sim | Destinatário: e-mail para `email`, telefone para `sms`/`whatsapp`/`voice` — sempre um contato cadastrado da pessoa da cobrança; livre para `manual`. |
| `body` | `string` | sim | Corpo da mensagem. |
| `subject` | `string` | não | Assunto (usado em e-mail). |
| `collection_rule_step_id` | `string (uuid)` | não | Etapa da régua da cobrança a associar à notificação (rótulo; não altera o `current_step`). |

Outros campos e objetos aninhados: `kobana schema dunning.charges.notify`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Notificação criada e enfileirada (status `queued`). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## charges settlement-pdf

Carta de quitação (PDF)

```text
kobana dunning charges settlement-pdf [ID] [OPÇÕES]
```

`GET /v1/charges/{id}/settlement-pdf`

Gera a carta de quitação da cobrança em PDF (documento jurídico), escopada pela organização. Só é permitida quando a cobrança está paga (status `paid`); caso contrário retorna 409 (`CHARGE_NOT_SETTLED`). Responde `application/pdf` como anexo binário.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | PDF da carta de quitação (anexo binário `application/pdf`). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Cobrança não está quitada — a carta só pode ser gerada para status `paid` (código `CHARGE_NOT_SETTLED`). |
| `413` | Corpo da requisição acima de 2 MB. |
