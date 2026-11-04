---
id: people
title: "kobana dunning people"
sidebar_label: "people"
---

# kobana dunning people


Comandos de `kobana dunning people`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`people bulk delete`](#people-bulk-delete) | `DELETE /v1/people/bulk` | Excluir clientes em massa |
| [`people contact-points list`](#people-contact-points-list) | `GET /v1/people/{id}/contact-points` | Listar pontos de contato |
| [`people contact-points create`](#people-contact-points-create) | `POST /v1/people/{id}/contact-points` | Criar ponto de contato |
| [`people contact-points update`](#people-contact-points-update) | `PATCH /v1/people/{id}/contact-points/{cpId}` | Atualizar ponto de contato |
| [`people contact-points delete`](#people-contact-points-delete) | `DELETE /v1/people/{id}/contact-points/{cpId}` | Excluir ponto de contato |
| [`people list`](#people-list) | `GET /v1/people` | Listar clientes |
| [`people create`](#people-create) | `POST /v1/people` | Criar cliente |
| [`people get`](#people-get) | `GET /v1/people/{id}` | Consultar cliente |
| [`people update`](#people-update) | `PUT /v1/people/{id}` | Atualizar cliente |
| [`people delete`](#people-delete) | `DELETE /v1/people/{id}` | Excluir cliente |
| [`people statement-pdf`](#people-statement-pdf) | `GET /v1/people/{id}/statement-pdf` | Extrato do devedor (PDF) |

## people bulk delete

Excluir clientes em massa

```text
kobana dunning people bulk delete [OPÇÕES]
```

`DELETE /v1/people/bulk`

Exclui (soft delete, irreversível) até 100 clientes numa chamada, com os ids no corpo JSON. Cliente com cobrança em aberto (`pending`/`overdue`/`negotiated`) não é excluível e volta em `failed` com `has_active_charges`. Resultado parcial não é erro. Exige o nível de acesso `delete` (ou `all`) — `write` não basta. Substituiu o `POST /people/bulk-archive`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | IDs dos registros-alvo (1 a 100 por chamada). Duplicados são ignorados. |

Outros campos e objetos aninhados: `kobana schema dunning.people.bulk.delete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado por item da exclusão em massa. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people contact-points list

Listar pontos de contato

```text
kobana dunning people contact-points list [ID] [OPÇÕES]
```

`GET /v1/people/{id}/contact-points`

Lista os pontos de contato de um cliente, com paginação e filtros por canal e status.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `channel` | `string` | não | Filtra por canal (`email` \| `sms` \| `whatsapp` \| `voice` \| `postal`). Valores: `email`, `sms`, `whatsapp`, `voice`, `postal`. |
| `status` | `string` | não | Filtra por status (`active` \| `deleted`). Valores: `active`, `deleted`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de pontos de contato. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people contact-points create

Criar ponto de contato

```text
kobana dunning people contact-points create [ID] [OPÇÕES]
```

`POST /v1/people/{id}/contact-points`

Cria um ponto de contato para o cliente. Retorna 409 se já existir um contato ativo com o mesmo canal e valor.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `channel` | `string` | sim | Canal (`email` \| `sms` \| `whatsapp` \| `voice` \| `postal`). Valores: `email`, `sms`, `whatsapp`, `voice`, `postal`. |
| `value` | `string` | sim | Valor NORMALIZADO do contato: e-mail em minúsculas; telefone e CEP só dígitos. |
| `label` | `string` | não | Rótulo livre (ex.: personal/commercial/mobile/whatsapp). |
| `is_primary` | `boolean` | não | Indica o contato primário do canal (usado primeiro na resolução do destinatário). |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |

Outros campos e objetos aninhados: `kobana schema dunning.people.contact-points.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Ponto de contato criado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## people contact-points update

Atualizar ponto de contato

```text
kobana dunning people contact-points update [ID] [CPID] [OPÇÕES]
```

`PATCH /v1/people/{id}/contact-points/{cpId}`

Atualiza rótulo/primário/verificação de um ponto de contato e permite marcar/desmarcar opt-out e terceiro manualmente.

**Argumentos do caminho:** `id`, `cpId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não | Rótulo livre (ex.: personal/commercial/mobile/whatsapp). |
| `is_primary` | `boolean` | não | Indica o contato primário do canal (usado primeiro na resolução do destinatário). |
| `verified` | `boolean` | não | `true` marca o contato como verificado agora; `false` remove a verificação. |
| `opt_out` | `boolean` | não | `true` registra opt-out por contato agora; `false` reverte. |
| `third_party` | `boolean` | não | `true` marca o contato como de terceiro agora; `false` reverte. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.people.contact-points.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Ponto de contato atualizado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people contact-points delete

Excluir ponto de contato

```text
kobana dunning people contact-points delete [ID] [CPID] [OPÇÕES]
```

`DELETE /v1/people/{id}/contact-points/{cpId}`

Exclui (soft delete) um ponto de contato. Supressões e histórico permanecem para a trilha de auditoria.

**Argumentos do caminho:** `id`, `cpId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Ponto de contato excluído. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people list

Listar clientes

```text
kobana dunning people list [OPÇÕES]
```

`GET /v1/people`

Lista os clientes da organização com paginação e filtros.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `sort_by` | `string` | não | Campo de ordenação (nome público em snake_case, ex.: `due_date`). |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca por nome, documento ou e-mail. |
| `classification_id` | `string` | não | Filtra pela classificação (UUID). |
| `status` | `string` | não | Filtra por status do cadastro (`active` \| `deleted`). Valores: `active`, `deleted`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de clientes. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people create

Criar cliente

```text
kobana dunning people create [OPÇÕES]
```

`POST /v1/people`

Cria um cliente na carteira. `external_id` e `document_number` são usados para deduplicação em importações e integrações.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do cliente. |
| `classification_id` | `string (uuid)` | não | Classificação do cliente (segmentação da régua). |
| `document_number` | `string` | não | CPF/CNPJ (somente dígitos). |
| `document_type` | `string` | não | Tipo do documento (`cpf` \| `cnpj`). Valores: `cpf`, `cnpj`, `null`. |
| `kind` | `string` | não | Natureza (`natural` = pessoa física, `juridical` = jurídica). Valores: `natural`, `juridical`, `null`. |
| `legal_name` | `string` | não | Razão social. |
| `nickname` | `string` | não | Apelido/nome fantasia. |
| `birthday` | `string (date-time)` | não | Data de nascimento (ISO 8601). |
| `addresses` | `array<object>` | não | Endereços. |
| `phones` | `array<object>` | não | Telefones. |
| `emails` | `array<object>` | não | E-mails. |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `tags` | `array<string>` | não | Tags livres. |
| `notes` | `string` | não | Observações internas. |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |

Outros campos e objetos aninhados: `kobana schema dunning.people.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Cliente criado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people get

Consultar cliente

```text
kobana dunning people get [ID] [OPÇÕES]
```

`GET /v1/people/{id}`

Retorna um cliente pelo ID.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cliente encontrado. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people update

Atualizar cliente

```text
kobana dunning people update [ID] [OPÇÕES]
```

`PUT /v1/people/{id}`

Atualiza os dados de um cliente (parcial).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `classification_id` | `string (uuid)` | não | Classificação do cliente (segmentação da régua). |
| `document_number` | `string` | não | CPF/CNPJ (somente dígitos). |
| `document_type` | `string` | não | Tipo do documento (`cpf` \| `cnpj`). Valores: `cpf`, `cnpj`, `null`. |
| `kind` | `string` | não | Natureza (`natural` = pessoa física, `juridical` = jurídica). Valores: `natural`, `juridical`, `null`. |
| `name` | `string` | não | Nome do cliente. |
| `legal_name` | `string` | não | Razão social. |
| `nickname` | `string` | não | Apelido/nome fantasia. |
| `birthday` | `string (date-time)` | não | Data de nascimento (ISO 8601). |
| `addresses` | `array<object>` | não | Endereços. |
| `phones` | `array<object>` | não | Telefones. |
| `emails` | `array<object>` | não | E-mails. |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `tags` | `array<string>` | não | Tags livres. |
| `notes` | `string` | não | Observações internas. |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_data_mode` | `string` | não | Como aplicar `custom_data` no update: `replace` (default) sobrescreve o objeto inteiro; `merge` mescla (deep) o payload sobre o valor armazenado — objetos aninhados mesclam, escalares/arrays/null sobrescrevem a chave. Valores: `replace`, `merge`. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.people.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cliente atualizado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people delete

Excluir cliente

```text
kobana dunning people delete [ID] [OPÇÕES]
```

`DELETE /v1/people/{id}`

Exclui (soft delete) um cliente. Cobranças e histórico permanecem para trilha de auditoria.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Cliente excluído. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## people statement-pdf

Extrato do devedor (PDF)

```text
kobana dunning people statement-pdf [ID] [OPÇÕES]
```

`GET /v1/people/{id}/statement-pdf`

Gera o extrato do devedor em PDF: todas as cobranças da pessoa (documento, vencimento, status, valor original e atual) com os totais em aberto, vencido e já pago. Escopado pela organização. Responde `application/pdf` como anexo binário.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | PDF do extrato do devedor (anexo binário `application/pdf`). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
