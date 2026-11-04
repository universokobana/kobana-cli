---
id: proposals
title: "kobana billing proposals"
sidebar_label: "proposals"
---

# kobana billing proposals


Comandos de `kobana billing proposals`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`proposals bulk delete`](#proposals-bulk-delete) | `DELETE /v1/proposals/bulk` | Excluir propostas em lote |
| [`proposals items list`](#proposals-items-list) | `GET /v1/proposals/{id}/items` | Listar itens da proposta |
| [`proposals items create`](#proposals-items-create) | `POST /v1/proposals/{id}/items` | Adicionar item à proposta |
| [`proposals items get`](#proposals-items-get) | `GET /v1/proposals/{id}/items/{item_id}` | Obter item da proposta |
| [`proposals items update`](#proposals-items-update) | `PATCH /v1/proposals/{id}/items/{item_id}` | Atualizar item da proposta |
| [`proposals items delete`](#proposals-items-delete) | `DELETE /v1/proposals/{id}/items/{item_id}` | Excluir item da proposta |
| [`proposals stats`](#proposals-stats) | `GET /v1/proposals/stats` | Obter estatísticas de propostas |
| [`proposals list`](#proposals-list) | `GET /v1/proposals` | Listar propostas |
| [`proposals create`](#proposals-create) | `POST /v1/proposals` | Criar proposta |
| [`proposals update`](#proposals-update) | `PUT /v1/proposals` | Criar ou atualizar proposta |
| [`proposals bulk-cancel`](#proposals-bulk-cancel) | `POST /v1/proposals/bulk-cancel` | Cancelar propostas em lote |
| [`proposals bulk-send`](#proposals-bulk-send) | `POST /v1/proposals/bulk-send` | Enviar propostas em lote |
| [`proposals get`](#proposals-get) | `GET /v1/proposals/{id}` | Obter proposta |
| [`proposals delete`](#proposals-delete) | `DELETE /v1/proposals/{id}` | Excluir proposta |
| [`proposals accept`](#proposals-accept) | `POST /v1/proposals/{id}/accept` | Aceitar proposta |
| [`proposals cancel`](#proposals-cancel) | `POST /v1/proposals/{id}/cancel` | Cancelar proposta |
| [`proposals duplicate`](#proposals-duplicate) | `POST /v1/proposals/{id}/duplicate` | Duplicar proposta |
| [`proposals generate-pdf`](#proposals-generate-pdf) | `POST /v1/proposals/{id}/generate-pdf` | Gerar PDF da proposta |
| [`proposals json`](#proposals-json) | `GET /v1/proposals/{id}/json` | Obter snapshot JSON da proposta |
| [`proposals pricing`](#proposals-pricing) | `GET /v1/proposals/{id}/pricing` | Obter cálculo de preço da proposta |
| [`proposals publish`](#proposals-publish) | `POST /v1/proposals/{id}/publish` | Publicar proposta |
| [`proposals send`](#proposals-send) | `POST /v1/proposals/{id}/send` | Enviar proposta por e-mail |
| [`proposals undo-accept`](#proposals-undo-accept) | `POST /v1/proposals/{id}/undo-accept` | Desfazer aceite da proposta |
| [`proposals unpublish`](#proposals-unpublish) | `POST /v1/proposals/{id}/unpublish` | Despublicar proposta |

## proposals bulk delete

Excluir propostas em lote

```text
kobana billing proposals bulk delete [OPÇÕES]
```

`DELETE /v1/proposals/bulk`

Exclui várias propostas de uma vez (até 100). Retorna contagens de sucesso/falha e erros por id. Exige o nível de acesso `delete`. Substitui `POST /v1/proposals/bulk-delete`, removido.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | Lista de UUIDs das propostas alvo. Mínimo 1, máximo 100. |

Outros campos e objetos aninhados: `kobana schema billing.proposals.bulk.delete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da exclusão em lote. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals items list

Listar itens da proposta

```text
kobana billing proposals items list [ID] [OPÇÕES]
```

`GET /v1/proposals/{id}/items`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de itens. |
| `404` | Recurso não encontrado na organização atual. |

## proposals items create

Adicionar item à proposta

```text
kobana billing proposals items create [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/items`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `product_id` | `string (uuid)` | sim | UUID do produto. **Obrigatório**. |
| `price_id` | `string (uuid)` | sim | UUID do preço. **Obrigatório**. |
| `unit_amount` | `number` | sim | Valor unitário em reais (float, até 4 casas decimais). |
| `quantity` | `number` | não | Quantidade. Padrão: 1. |
| `metadata` | `object` | não | Metadados arbitrários do item. |
| `custom_metadata` | `object` | não | Campos personalizados do item. |

Outros campos e objetos aninhados: `kobana schema billing.proposals.items.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Item criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals items get

Obter item da proposta

```text
kobana billing proposals items get [ID] [ITEM_ID] [OPÇÕES]
```

`GET /v1/proposals/{id}/items/{item_id}`

**Argumentos do caminho:** `id`, `item_id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do item. |
| `404` | Recurso não encontrado na organização atual. |

## proposals items update

Atualizar item da proposta

```text
kobana billing proposals items update [ID] [ITEM_ID] [OPÇÕES]
```

`PATCH /v1/proposals/{id}/items/{item_id}`

Atualiza um item. Só é permitido em propostas em rascunho (`status = draft`).

**Argumentos do caminho:** `id`, `item_id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `quantity` | `integer` | não | Atualiza a quantidade. |
| `unit_amount` | `number` | não | Atualiza o valor unitário em reais (float, até 4 casas decimais). |
| `setup_amount` | `number` | não | Atualiza o setup unitário em reais (float, até 4 casas decimais). |
| `custom_metadata` | `object` | não | proposals.items.update.body.custom_metadata |

Outros campos e objetos aninhados: `kobana schema billing.proposals.items.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Item atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals items delete

Excluir item da proposta

```text
kobana billing proposals items delete [ID] [ITEM_ID] [OPÇÕES]
```

`DELETE /v1/proposals/{id}/items/{item_id}`

Remove um item. Só é permitido em propostas em rascunho (`status = draft`).

**Argumentos do caminho:** `id`, `item_id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Item excluído. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals stats

Obter estatísticas de propostas

```text
kobana billing proposals stats [OPÇÕES]
```

`GET /v1/proposals/stats`

Retorna contagens de propostas por status (`draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired`, `canceled`) para a organização atual.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Estatísticas de propostas. |
| `404` | Recurso não encontrado na organização atual. |

## proposals list

Listar propostas

```text
kobana billing proposals list [OPÇÕES]
```

`GET /v1/proposals`

Retorna uma lista paginada de propostas comerciais da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `person_id` | `string` | não | Filtra por UUID da pessoa associada. |
| `person_document_number` | `string` | não | Filtra por CPF/CNPJ da pessoa (exato). |
| `person_name_contains` | `string` | não | Filtra por nome da pessoa (busca parcial, sem distinção de maiúsculas). |
| `status` | `string` | não | Filtra por status: `draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired` ou `canceled`. Valores: `draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired`, `canceled`. |
| `billing_account_id` | `string` | não | Filtra por `billing_account_id` vinculado. |
| `prospect_id` | `string` | não | Filtra por `prospect_id`. |
| `contact_id` | `string` | não | Filtra por `contact_id` vinculado. |
| `coupon_id` | `string` | não | Filtra por `coupon_id` aplicado. |
| `company_id` | `string` | não | Filtra por `company_id` emissor. |
| `template_id` | `string` | não | Filtra por `template_id` usado para gerar o PDF. |
| `code` | `string` | não | Filtra por código curto da proposta (ex.: `PROP-0001`). |
| `external_id` | `string` | não | Filtra por identificador externo. |
| `search` | `string` | não | Busca textual livre. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `expires_after` | `string` | não | Filtra por `expires_at` inicial (ISO 8601, inclusivo). |
| `expires_before` | `string` | não | Filtra por `expires_at` final (ISO 8601, inclusivo). |
| `accepted_after` | `string` | não | Filtra por `accepted_at` inicial (ISO 8601, inclusivo). |
| `accepted_before` | `string` | não | Filtra por `accepted_at` final (ISO 8601, inclusivo). |
| `sent_after` | `string` | não | Filtra por `sent_at` inicial (ISO 8601, inclusivo). |
| `sent_before` | `string` | não | Filtra por `sent_at` final (ISO 8601, inclusivo). |
| `viewed_after` | `string` | não | Filtra por `viewed_at` inicial (ISO 8601, inclusivo). |
| `viewed_before` | `string` | não | Filtra por `viewed_at` final (ISO 8601, inclusivo). |
| `rejected_after` | `string` | não | Filtra por `rejected_at` inicial (ISO 8601, inclusivo). |
| `rejected_before` | `string` | não | Filtra por `rejected_at` final (ISO 8601, inclusivo). |
| `setup_amount_min` | `number,null` | não | Filtra por `setup_amount` mínimo (em reais). |
| `setup_amount_max` | `number,null` | não | Filtra por `setup_amount` máximo (em reais). |
| `monthly_amount_min` | `number,null` | não | Filtra por `monthly_amount` mínimo (em reais). |
| `monthly_amount_max` | `number,null` | não | Filtra por `monthly_amount` máximo (em reais). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de propostas. |
| `404` | Recurso não encontrado na organização atual. |

## proposals create

Criar proposta

```text
kobana billing proposals create [OPÇÕES]
```

`POST /v1/proposals`

Cria uma proposta comercial. É obrigatório informar `billing_account_id`, `prospect_id` ou `prospect_data` (criação inline do prospect).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | não | UUID da empresa emissora. Use `null` para limpar. |
| `title` | `string` | não | Título da proposta. Padrão: "Proposta Comercial". |
| `description` | `string` | não | Descrição livre da proposta. |
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança alvo (cliente existente). Obrigatório se `prospect_id` e `prospect_data` não forem informados. |
| `prospect_id` | `string (uuid)` | não | UUID de um prospect existente. Alternativa a `billing_account_id`/`prospect_data`. |
| `contact_id` | `string (uuid)` | não | UUID de um contato existente vinculado ao prospect. |
| `prospect_data` | `object` | não | Dados para criar um prospect inline (pessoa jurídica). Alternativa a `prospect_id`/`billing_account_id`. |
| `contact_data` | `object` | não | Dados para criar um contato inline (pessoa física responsável). |
| `coupon_id` | `string (uuid)` | não | UUID do cupom a aplicar. |
| `template_id` | `string (uuid)` | não | UUID do template para gerar o PDF. |
| `discount_percent` | `number` | não | Percentual de desconto negociado (0–100), aplicado sobre o valor mensal; o desconto por ciclo se aplica sobre o valor com desconto. Não envie junto com `discount_amount`. |
| `discount_amount` | `number` | não | Desconto fixo negociado por mês em reais (float, até 4 casas decimais), aplicado sobre o valor mensal; o desconto por ciclo se aplica sobre o valor com desconto. Não envie junto com `discount_percent`. |
| `commitment_months` | `integer` | não | Fidelidade em meses (1–120). Use `null` para remover. Copiado para a assinatura quando a proposta é aceita. |
| `expires_at` | `string` | não | Data de expiração. |
| `notes` | `string` | não | Anotações internas. |
| `items` | `array<object>` | não | Itens da proposta. Mínimo 1. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Campos personalizados (validados contra o schema da organização). |

Outros campos e objetos aninhados: `kobana schema billing.proposals.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Proposta criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals update

Criar ou atualizar proposta

```text
kobana billing proposals update [OPÇÕES]
```

`PUT /v1/proposals`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | não | UUID da empresa emissora. Use `null` para limpar. |
| `title` | `string` | não | Título da proposta. Padrão: "Proposta Comercial". |
| `description` | `string` | não | Descrição livre da proposta. |
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança alvo (cliente existente). Obrigatório se `prospect_id` e `prospect_data` não forem informados. |
| `prospect_id` | `string (uuid)` | não | UUID de um prospect existente. Alternativa a `billing_account_id`/`prospect_data`. |
| `contact_id` | `string (uuid)` | não | UUID de um contato existente vinculado ao prospect. |
| `prospect_data` | `object` | não | Dados para criar um prospect inline (pessoa jurídica). Alternativa a `prospect_id`/`billing_account_id`. |
| `contact_data` | `object` | não | Dados para criar um contato inline (pessoa física responsável). |
| `coupon_id` | `string (uuid)` | não | UUID do cupom a aplicar. |
| `template_id` | `string (uuid)` | não | UUID do template para gerar o PDF. |
| `discount_percent` | `number` | não | Percentual de desconto negociado (0–100), aplicado sobre o valor mensal; o desconto por ciclo se aplica sobre o valor com desconto. Não envie junto com `discount_amount`. |
| `discount_amount` | `number` | não | Desconto fixo negociado por mês em reais (float, até 4 casas decimais), aplicado sobre o valor mensal; o desconto por ciclo se aplica sobre o valor com desconto. Não envie junto com `discount_percent`. |
| `commitment_months` | `integer` | não | Fidelidade em meses (1–120). Use `null` para remover. Copiado para a assinatura quando a proposta é aceita. |
| `expires_at` | `string` | não | Data de expiração. |
| `notes` | `string` | não | Anotações internas. |
| `items` | `array<object>` | não | Itens da proposta. Mínimo 1. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Campos personalizados (validados contra o schema da organização). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.proposals.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals bulk-cancel

Cancelar propostas em lote

```text
kobana billing proposals bulk-cancel [OPÇÕES]
```

`POST /v1/proposals/bulk-cancel`

Cancela várias propostas de uma vez (até 100). Retorna contagens de sucesso/falha e erros por id.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | Lista de UUIDs das propostas alvo. Mínimo 1, máximo 100. |

Outros campos e objetos aninhados: `kobana schema billing.proposals.bulk-cancel`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do cancelamento em lote. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals bulk-send

Enviar propostas em lote

```text
kobana billing proposals bulk-send [OPÇÕES]
```

`POST /v1/proposals/bulk-send`

Envia várias propostas de uma vez (até 100). Retorna contagens de sucesso/falha e erros por id.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | Lista de UUIDs das propostas alvo. Mínimo 1, máximo 100. |

Outros campos e objetos aninhados: `kobana schema billing.proposals.bulk-send`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do envio em lote. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals get

Obter proposta

```text
kobana billing proposals get [ID] [OPÇÕES]
```

`GET /v1/proposals/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da proposta. |
| `404` | Recurso não encontrado na organização atual. |

## proposals delete

Excluir proposta

```text
kobana billing proposals delete [ID] [OPÇÕES]
```

`DELETE /v1/proposals/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Proposta excluída. |
| `404` | Recurso não encontrado na organização atual. |

## proposals accept

Aceitar proposta

```text
kobana billing proposals accept [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/accept`

Aceita a proposta e cria (ou confirma) a assinatura correspondente no ciclo informado.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cycle` | `string` | sim | Ciclo de cobrança da assinatura criada no aceite: `monthly`, `quarterly`, `semiannual` ou `annual`. Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |

Outros campos e objetos aninhados: `kobana schema billing.proposals.accept`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Proposta aceita — retorna a proposta, `subscription_id` e `invoice_id` quando aplicáveis. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals cancel

Cancelar proposta

```text
kobana billing proposals cancel [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/cancel`

Cancela a proposta (`status → canceled`).

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Proposta cancelada. |
| `404` | Recurso não encontrado na organização atual. |

## proposals duplicate

Duplicar proposta

```text
kobana billing proposals duplicate [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/duplicate`

Cria uma nova proposta a partir da proposta original (rascunho), preservando itens e configurações.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Proposta duplicada. |
| `404` | Recurso não encontrado na organização atual. |

## proposals generate-pdf

Gerar PDF da proposta

```text
kobana billing proposals generate-pdf [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/generate-pdf`

Gera (ou regenera) o PDF da proposta via Google App Scripts. Requer um `template` com `app_script_url` configurado.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | PDF gerado — retorna a proposta atualizada e as URLs do documento/PDF. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals json

Obter snapshot JSON da proposta

```text
kobana billing proposals json [ID] [OPÇÕES]
```

`GET /v1/proposals/{id}/json`

Retorna o snapshot completo dos dados usados para renderizar o template do PDF (espelha o que o AppScript recebe).

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Snapshot JSON da proposta. |
| `404` | Recurso não encontrado na organização atual. |

## proposals pricing

Obter cálculo de preço da proposta

```text
kobana billing proposals pricing [ID] [OPÇÕES]
```

`GET /v1/proposals/{id}/pricing`

Retorna o detalhamento de preços por ciclo de cobrança (mensal, trimestral, semestral, anual), com setup e descontos aplicados.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhamento de preços. |
| `404` | Recurso não encontrado na organização atual. |

## proposals publish

Publicar proposta

```text
kobana billing proposals publish [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/publish`

Publica a proposta (`draft → sent`) sem disparar o e-mail, tornando-a acessível no portal.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Proposta publicada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals send

Enviar proposta por e-mail

```text
kobana billing proposals send [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/send`

Envia (ou reenvia) a proposta por e-mail ao destinatário. Estados aceitos: `draft`, `sent` ou `viewed`. Opcionalmente, é possível customizar `subject` e `message` (HTML).

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `subject` | `string` | não | Assunto customizado do e-mail. Padrão: `Proposta {code}`. |
| `message` | `string` | não | Mensagem HTML customizada do corpo do e-mail. |

Outros campos e objetos aninhados: `kobana schema billing.proposals.send`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Proposta enviada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## proposals undo-accept

Desfazer aceite da proposta

```text
kobana billing proposals undo-accept [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/undo-accept`

Desfaz a aceitação: remove a assinatura vinculada e volta o status para `sent`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Aceitação desfeita. |
| `404` | Recurso não encontrado na organização atual. |

## proposals unpublish

Despublicar proposta

```text
kobana billing proposals unpublish [ID] [OPÇÕES]
```

`POST /v1/proposals/{id}/unpublish`

Volta a proposta para rascunho (`sent`/`viewed` → `draft`).

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Proposta despublicada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
