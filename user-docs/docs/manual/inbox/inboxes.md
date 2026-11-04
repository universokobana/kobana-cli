---
id: inboxes
title: "kobana inbox inboxes"
sidebar_label: "inboxes"
---

# kobana inbox inboxes


Comandos de `kobana inbox inboxes`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`inboxes recipients remove`](#inboxes-recipients-remove) | `POST /v1/inboxes/{id}/recipients/remove` | Remover destinatário permitido |
| [`inboxes recipients create`](#inboxes-recipients-create) | `POST /v1/inboxes/{id}/recipients` | Adicionar destinatário permitido |
| [`inboxes list`](#inboxes-list) | `GET /v1/inboxes` | Listar inboxes |
| [`inboxes create`](#inboxes-create) | `POST /v1/inboxes` | Criar inbox |
| [`inboxes update`](#inboxes-update) | `PUT /v1/inboxes` | Criar ou atualizar caixa |
| [`inboxes get`](#inboxes-get) | `GET /v1/inboxes/{id}` | Obter inbox |
| [`inboxes delete`](#inboxes-delete) | `DELETE /v1/inboxes/{id}` | Excluir inbox |

## inboxes recipients remove

Remover destinatário permitido

```text
kobana inbox inboxes recipients remove [ID] [OPÇÕES]
```

`POST /v1/inboxes/{id}/recipients/remove`

Aceita tanto o UUID interno quanto o `external_id` do recurso. Remove um email de `allowed_recipients`. 404 quando o endereço não está na lista. Substitui `DELETE /v1/inboxes/{id}/recipients`: remover um endereço é uma alteração da inbox (`write`), não uma exclusão.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string (email)` | sim |  |

Outros campos e objetos aninhados: `kobana schema inbox.inboxes.recipients.remove`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Destinatário removido. |
| `400` | `email` ausente ou não é um endereço válido. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## inboxes recipients create

Adicionar destinatário permitido

```text
kobana inbox inboxes recipients create [ID] [OPÇÕES]
```

`POST /v1/inboxes/{id}/recipients`

Aceita tanto o UUID interno quanto o `external_id` do recurso. Adiciona um email a `allowed_recipients`. Idempotente — retorna 200 com a lista atual quando o item já está presente.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string (email)` | sim |  |

Outros campos e objetos aninhados: `kobana schema inbox.inboxes.recipients.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Destinatário já presente (nenhuma ação realizada). |
| `201` | Destinatário adicionado. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## inboxes list

Listar inboxes

```text
kobana inbox inboxes list [OPÇÕES]
```

`GET /v1/inboxes`

Paginação keyset. Filtros são query params tipados (`campo=valor`, `campo[op]=valor`); filtros desconhecidos, campos não ordenáveis e valores malformados são rejeitados com 400 `VALIDATION_ERROR`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `updated_at`, `label`, `slug`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `updated_at`, `label`, `slug`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Igualdade. Também `status[in]=a,b`. Valores: `active`, `inactive`. |
| `agent_id` | `string` | não | Igualdade. Também `agent_id[in]=a,b`. |
| `slug` | `string` | não | Igualdade. Também `slug[in]=a,b`. |
| `email` | `string` | não | Igualdade. Também `email[in]=a,b`. |
| `external_id` | `string` | não | Igualdade. Também `external_id[in]=a,b`. |
| `created_at` | `string` | não | Igualdade. Também `created_at[in]=a,b` e `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | não |  |
| `created_at[lte]` | `string` | não |  |
| `updated_at` | `string` | não | Igualdade. Também `updated_at[in]=a,b` e `updated_at[gt\|gte\|lt\|lte]`. |
| `updated_at[gte]` | `string` | não |  |
| `updated_at[lte]` | `string` | não |  |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de resultados. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## inboxes create

Criar inbox

```text
kobana inbox inboxes create [OPÇÕES]
```

`POST /v1/inboxes`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `external_id` | `string` | não | Opcional. Usa por padrão o `id` interno gerado. |
| `email` | `string (email)` | não | Opcional. Deve usar o domínio de inbox configurado (`SES_DOMAIN`) e uma parte local que comece com "\{organization-slug\}.\{workspace-slug\}." de quem chama; caso contrário retorna 422. Endereços duplicados retornam 409. Quando omitido, o servidor o compõe como "\{organization-slug\}.\{workspace-slug\}.\{inbox-slug\}@&lt;domain>". |
| `slug` | `string` | não | Opcional. Slug minúsculo e dasherized, único dentro do workspace, usado como último segmento do endereço. Retorna 409 quando colide com outra caixa do mesmo workspace e 422 quando a parte local composta excede 64 caracteres. Quando omitido, o servidor deriva do `label` dasherized. |
| `agent_id` | `string (uuid)` | não |  |
| `allowed_recipients` | `array<string (email)>` | não |  |
| `filters` | `array<object>` | não |  |
| `attachments_allow` | `boolean` | não |  |
| `attachments_max_file_number` | `integer` | não |  |
| `attachments_max_file_size` | `integer` | não |  |
| `attachments_max_total_size` | `integer` | não |  |
| `attachments_allow_rules` | `array<object>` | não | Lista de anexos permitidos. Um anexo é processado quando atende a pelo menos uma regra; dentro de uma regra, todas as condições informadas precisam ser atendidas. `null` ou `[]` processa todos os anexos. Um anexo fora da lista é ignorado, nunca um motivo para rejeitar o e-mail: o agente não o vê, os limites de anexos não o contam e um passo de workflow não pode referenciá-lo. No update, omita para manter e envie `null` para limpar. |
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. Envie `null` para limpar. Máx. 64 KB serializados, 16 níveis de profundidade. |

Outros campos e objetos aninhados: `kobana schema inbox.inboxes.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## inboxes update

Criar ou atualizar caixa

```text
kobana inbox inboxes update [OPÇÕES]
```

`PUT /v1/inboxes`

Upsert. Recebe o mesmo corpo do POST mais as chaves de busca `id` e `external_id`, casadas nessa ordem, depois por `email`. Um match é atualizado in place e responde 200 — campos ausentes do corpo mantêm o valor armazenado. Sem chave, ou sem match, cria a caixa exatamente como o POST faria e responde 201.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `external_id` | `string` | não | Casa pelo id do sistema de origem. Também gravado no registro quando ele é criado. |
| `email` | `string (email)` | não | Opcional. Deve usar o domínio de inbox configurado (`SES_DOMAIN`) e uma parte local que comece com "\{organization-slug\}.\{workspace-slug\}." de quem chama; caso contrário retorna 422. Endereços duplicados retornam 409. Quando omitido, o servidor o compõe como "\{organization-slug\}.\{workspace-slug\}.\{inbox-slug\}@&lt;domain>". |
| `slug` | `string` | não | Opcional. Slug minúsculo e dasherized, único dentro do workspace, usado como último segmento do endereço. Retorna 409 quando colide com outra caixa do mesmo workspace e 422 quando a parte local composta excede 64 caracteres. Quando omitido, o servidor deriva do `label` dasherized. |
| `agent_id` | `string (uuid)` | não |  |
| `allowed_recipients` | `array<string (email)>` | não |  |
| `filters` | `array<object>` | não |  |
| `attachments_allow` | `boolean` | não |  |
| `attachments_max_file_number` | `integer` | não |  |
| `attachments_max_file_size` | `integer` | não |  |
| `attachments_max_total_size` | `integer` | não |  |
| `attachments_allow_rules` | `array<object>` | não | Lista de anexos permitidos. Um anexo é processado quando atende a pelo menos uma regra; dentro de uma regra, todas as condições informadas precisam ser atendidas. `null` ou `[]` processa todos os anexos. Um anexo fora da lista é ignorado, nunca um motivo para rejeitar o e-mail: o agente não o vê, os limites de anexos não o contam e um passo de workflow não pode referenciá-lo. No update, omita para manter e envie `null` para limpar. |
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. Envie `null` para limpar. Máx. 64 KB serializados, 16 níveis de profundidade. |
| `id` | `string (uuid)` | não | Casa pelo id interno. Verificado antes de `external_id`. |

Outros campos e objetos aninhados: `kobana schema inbox.inboxes.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## inboxes get

Obter inbox

```text
kobana inbox inboxes get [ID] [OPÇÕES]
```

`GET /v1/inboxes/{id}`

Aceita tanto o UUID interno quanto o `external_id` do recurso.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## inboxes delete

Excluir inbox

```text
kobana inbox inboxes delete [ID] [OPÇÕES]
```

`DELETE /v1/inboxes/{id}`

Aceita tanto o UUID interno quanto o `external_id` do recurso.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Excluído (soft delete). |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
