---
id: customers
title: "kobana billing customers"
sidebar_label: "customers"
---

# kobana billing customers


Comandos de `kobana billing customers`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`customers statement sync`](#customers-statement-sync) | `POST /v1/customers/{id}/statement/sync` | Reconstruir o extrato a partir do histórico |
| [`customers statement list`](#customers-statement-list) | `GET /v1/customers/{id}/statement` | Listar extrato financeiro do cliente |
| [`customers users list`](#customers-users-list) | `GET /v1/customers/{id}/users` | Listar usuários do portal do cliente |
| [`customers users create`](#customers-users-create) | `POST /v1/customers/{id}/users` | Convidar usuário do portal |
| [`customers users invite`](#customers-users-invite) | `POST /v1/customers/{id}/users/invite` | Convidar usuário do portal (verbo) |
| [`customers users get`](#customers-users-get) | `GET /v1/customers/{id}/users/{user_id}` | Obter usuário do portal |
| [`customers users update`](#customers-users-update) | `PATCH /v1/customers/{id}/users/{user_id}` | Atualizar usuário do portal |
| [`customers users delete`](#customers-users-delete) | `DELETE /v1/customers/{id}/users/{user_id}` | Excluir usuário do portal |
| [`customers users email-preview`](#customers-users-email-preview) | `GET /v1/customers/{id}/users/{user_id}/email-preview` | Pré-visualizar e-mail do usuário do portal |
| [`customers users resend`](#customers-users-resend) | `POST /v1/customers/{id}/users/{user_id}/resend` | Reenviar e-mail de convite ou verificação |
| [`customers list`](#customers-list) | `GET /v1/customers` | Listar clientes |
| [`customers create`](#customers-create) | `POST /v1/customers` | Criar cliente |
| [`customers update`](#customers-update) | `PUT /v1/customers` | Criar ou atualizar cliente |
| [`customers get`](#customers-get) | `GET /v1/customers/{id}` | Obter cliente |
| [`customers delete`](#customers-delete) | `DELETE /v1/customers/{id}` | Remover cliente (soft delete) |
| [`customers recalculate-mrr`](#customers-recalculate-mrr) | `POST /v1/customers/{id}/recalculate-mrr` | Recalcular o MRR do cliente |

## customers statement sync

Reconstruir o extrato a partir do histórico

```text
kobana billing customers statement sync [ID] [OPÇÕES]
```

`POST /v1/customers/{id}/statement/sync`

Recria os itens do extrato a partir das faturas, pagamentos e aplicações de crédito existentes. Retorna o resumo do que foi criado.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | não | Necessário quando o cliente possui múltiplas contas de cobrança. |

Outros campos e objetos aninhados: `kobana schema billing.customers.statement.sync`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Extrato reconstruído. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## customers statement list

Listar extrato financeiro do cliente

```text
kobana billing customers statement list [ID] [OPÇÕES]
```

`GET /v1/customers/{id}/statement`

Retorna os lançamentos do extrato (faturas, pagamentos, aplicações de crédito) da conta de cobrança do cliente. Quando o cliente possui múltiplas contas, informe `billing_account_id`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | não | Necessário quando o cliente possui múltiplas contas de cobrança. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de itens do extrato. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## customers users list

Listar usuários do portal do cliente

```text
kobana billing customers users list [ID] [OPÇÕES]
```

`GET /v1/customers/{id}/users`

Retorna usuários com acesso ao portal vinculados à conta de cobrança do cliente.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra usuários do portal por status. Valores: `active`, `pending_verification`, `pending_invitation`, `suspended`, `locked`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de usuários do portal. |
| `404` | Recurso não encontrado na organização atual. |

## customers users create

Convidar usuário do portal

```text
kobana billing customers users create [ID] [OPÇÕES]
```

`POST /v1/customers/{id}/users`

Cria um convite de acesso ao portal para o e-mail informado. O convite expira em 7 dias.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string` | sim | E-mail do convidado (obrigatório). |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.customers.users.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Convite criado e e-mail enfileirado para envio. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Já existe um usuário ou convite para este e-mail. |

## customers users invite

Convidar usuário do portal (verbo)

```text
kobana billing customers users invite [ID] [OPÇÕES]
```

`POST /v1/customers/{id}/users/invite`

Equivalente ao `POST /customers/{id}/users`. Disponível como rota de verbo para SDKs que preferem URLs descritivas.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string` | sim | E-mail do convidado (obrigatório). |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.customers.users.invite`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Convite criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## customers users get

Obter usuário do portal

```text
kobana billing customers users get [ID] [USER_ID] [OPÇÕES]
```

`GET /v1/customers/{id}/users/{user_id}`

O parâmetro `{user_id}` aceita o UUID ou o `external_id` do usuário.

**Argumentos do caminho:** `id`, `user_id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do usuário do portal. |
| `404` | Recurso não encontrado na organização atual. |

## customers users update

Atualizar usuário do portal

```text
kobana billing customers users update [ID] [USER_ID] [OPÇÕES]
```

`PATCH /v1/customers/{id}/users/{user_id}`

Atualiza `email`, `external_id`, `sso_only` e `status`. Transições de status permitidas: `active` ↔ `suspended`.

**Argumentos do caminho:** `id`, `user_id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string` | não |  |
| `external_id` | `string` | não |  |
| `sso_only` | `boolean` | não |  |
| `status` | `string` | não | Transições permitidas: `active` ↔ `suspended`. Valores: `active`, `suspended`. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.customers.users.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Usuário atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## customers users delete

Excluir usuário do portal

```text
kobana billing customers users delete [ID] [USER_ID] [OPÇÕES]
```

`DELETE /v1/customers/{id}/users/{user_id}`

Exclui o usuário do portal (soft delete: `deleted_at`) — um convite pendente e um usuário ativo da mesma forma: ele perde o acesso na hora, as sessões e os links pendentes deixam de valer, e o e-mail pode ser convidado de novo. Exige o nível de acesso `delete`. Até outubro de 2026 esta rota apagava o convite pendente e só suspendia o usuário ativo.

**Argumentos do caminho:** `id`, `user_id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Usuário do portal excluído. |
| `404` | Recurso não encontrado na organização atual. |

## customers users email-preview

Pré-visualizar e-mail do usuário do portal

```text
kobana billing customers users email-preview [ID] [USER_ID] [OPÇÕES]
```

`GET /v1/customers/{id}/users/{user_id}/email-preview`

Retorna o assunto e mensagem do e-mail de convite ou verificação que seria enviado ao usuário, sem disparar o envio. Disponível apenas para usuários `pending_invitation` ou `pending_verification`.

**Argumentos do caminho:** `id`, `user_id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pré-visualização do e-mail. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## customers users resend

Reenviar e-mail de convite ou verificação

```text
kobana billing customers users resend [ID] [USER_ID] [OPÇÕES]
```

`POST /v1/customers/{id}/users/{user_id}/resend`

Reenvia o e-mail de convite (para `pending_invitation`, com rotação de token) ou de verificação (para `pending_verification`). Demais status retornam 400.

**Argumentos do caminho:** `id`, `user_id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | E-mail reenviado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## customers list

Listar clientes

```text
kobana billing customers list [OPÇÕES]
```

`GET /v1/customers`

Retorna uma lista paginada dos clientes da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca textual livre por nome, documento ou e-mail. |
| `kind` | `string` | não | Filtra por tipo de pessoa: `natural` (PF) ou `juridical` (PJ). Valores: `natural`, `juridical`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de clientes. |
| `404` | Recurso não encontrado na organização atual. |

## customers create

Criar cliente

```text
kobana billing customers create [OPÇÕES]
```

`POST /v1/customers`

Cria um cliente e, por padrão, uma conta de cobrança associada. Use `create_billing_account=false` para criar apenas o cliente.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do cliente (obrigatório). |
| `public_id` | `string (uuid)` | não | UUID público do cliente. Gerado automaticamente quando omitido; informe apenas para preservar um identificador do seu sistema. Deve ser único. |
| `external_id` | `string` | não | Identificador externo do cliente no sistema de origem. |
| `nickname` | `string` | não |  |
| `legal_name` | `string` | não | Razão social (para pessoa jurídica). |
| `document_number` | `string` | não | CPF ou CNPJ. O `document_type` é inferido pelo comprimento (≤11 → cpf, senão cnpj). |
| `kind` | `string` | não | Valores: `natural`, `juridical`. |
| `birthday` | `string` | não | Data de nascimento (ISO 8601). |
| `email` | `string` | não | E-mail principal de contato. |
| `phone` | `string` | não |  |
| `notes` | `string` | não |  |
| `addresses` | `array<object>` | não |  |
| `phones` | `array<object>` | não |  |
| `emails` | `array<object>` | não |  |
| `websites` | `array<object>` | não |  |
| `tags` | `array<string>` | não |  |
| `custom_metadata` | `object` | não |  |
| `create_billing_account` | `boolean` | não | Quando `false`, não cria conta de cobrança automaticamente. Padrão: `true`. |
| `billing_email` | `string` | não | E-mail para envio de faturas. Padrão: usa `email` quando omitido. |
| `payment_terms_days` | `integer` | não | Prazo para vencimento de fatura em dias. Padrão: prazo padrão configurado na organização (10 quando não configurado). |
| `auto_collection` | `boolean` | não | Cobrança automática na conta criada. Padrão: `true`. |

Outros campos e objetos aninhados: `kobana schema billing.customers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Cliente criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## customers update

Criar ou atualizar cliente

```text
kobana billing customers update [OPÇÕES]
```

`PUT /v1/customers`

Recebe o mesmo corpo do POST e deduplica por `id`, `public_id`, `external_id` e `document_number`, nesta ordem, parando na primeira chave que encontrar um cliente. Se encontrar, atualiza e responde 200; se não, cria e responde 201. A atualização é parcial: campos ausentes no corpo mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do cliente. Obrigatório apenas quando nenhum cliente é encontrado pelas chaves de deduplicação. |
| `public_id` | `string (uuid)` | não | UUID público do cliente. Localiza o registro e, quando nenhum é encontrado, é gravado no cliente criado. |
| `external_id` | `string` | não | Identificador externo do cliente no sistema de origem. |
| `nickname` | `string` | não |  |
| `legal_name` | `string` | não | Razão social (para pessoa jurídica). |
| `document_number` | `string` | não | CPF ou CNPJ. O `document_type` é inferido pelo comprimento (≤11 → cpf, senão cnpj). |
| `kind` | `string` | não | Valores: `natural`, `juridical`. |
| `birthday` | `string` | não | Data de nascimento (ISO 8601). |
| `email` | `string` | não | E-mail principal de contato. |
| `phone` | `string` | não |  |
| `notes` | `string` | não |  |
| `addresses` | `array<object>` | não |  |
| `phones` | `array<object>` | não |  |
| `emails` | `array<object>` | não |  |
| `websites` | `array<object>` | não |  |
| `tags` | `array<string>` | não |  |
| `custom_metadata` | `object` | não |  |
| `create_billing_account` | `boolean` | não | Quando `false`, não cria conta de cobrança automaticamente. Padrão: `true`. |
| `billing_email` | `string` | não | E-mail para envio de faturas. Padrão: usa `email` quando omitido. |
| `payment_terms_days` | `integer` | não | Prazo para vencimento de fatura em dias. Padrão: prazo padrão configurado na organização (10 quando não configurado). |
| `auto_collection` | `boolean` | não | Cobrança automática na conta criada. Padrão: `true`. |
| `id` | `string (uuid)` | não | UUID do cliente. Usado apenas para localizar o registro — nunca é gravado. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado. `replace` (padrão) substitui todo o objeto; `merge` faz um deep merge sobre o valor atual — objetos aninhados são mesclados recursivamente, escalares e arrays substituem a chave, e chaves `null` são mantidas (não removem a chave). Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema billing.customers.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cliente atualizado. |
| `201` | Cliente criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## customers get

Obter cliente

```text
kobana billing customers get [ID] [OPÇÕES]
```

`GET /v1/customers/{id}`

O parâmetro `{id}` aceita o UUID do cliente ou o `external_id`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do cliente. |
| `404` | Recurso não encontrado na organização atual. |

## customers delete

Remover cliente (soft delete)

```text
kobana billing customers delete [ID] [OPÇÕES]
```

`DELETE /v1/customers/{id}`

Marca o cliente como removido sem apagar registros históricos.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Confirmação de soft delete. |
| `404` | Recurso não encontrado na organização atual. |

## customers recalculate-mrr

Recalcular o MRR do cliente

```text
kobana billing customers recalculate-mrr [ID] [OPÇÕES]
```

`POST /v1/customers/{id}/recalculate-mrr`

Recalcula o MRR (Monthly Recurring Revenue) das contas de cobrança do cliente a partir das assinaturas ativas.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | MRR recalculado. |
| `404` | Recurso não encontrado na organização atual. |
