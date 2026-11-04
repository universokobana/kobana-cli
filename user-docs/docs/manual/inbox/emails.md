---
id: emails
title: "kobana inbox emails"
sidebar_label: "emails"
---

# kobana inbox emails


Comandos de `kobana inbox emails`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`emails list`](#emails-list) | `GET /v1/emails` | Listar emails |
| [`emails create`](#emails-create) | `POST /v1/emails` | Enviar um .eml bruto |
| [`emails get`](#emails-get) | `GET /v1/emails/{id}` | Obter email |
| [`emails delete`](#emails-delete) | `DELETE /v1/emails/{id}` | Excluir e-mail |
| [`emails eml`](#emails-eml) | `GET /v1/emails/{id}/eml` | Baixar o .eml original |
| [`emails reprocess`](#emails-reprocess) | `POST /v1/emails/{id}/reprocess` | Reprocessar email |

## emails list

Listar emails

```text
kobana inbox emails list [OPÇÕES]
```

`GET /v1/emails`

Paginação keyset. Filtros são query params tipados (`campo=valor`, `campo[op]=valor`); filtros desconhecidos, campos não ordenáveis e valores malformados são rejeitados com 400 `VALIDATION_ERROR`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `updated_at`, `from`, `status`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `updated_at`, `from`, `status`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Igualdade. Também `status[in]=a,b`. Valores: `received`, `approved`, `rejected`, `processing`, `processed`, `failed`. |
| `inbox_id` | `string` | não | Igualdade. Também `inbox_id[in]=a,b`. |
| `agent_id` | `string` | não | Igualdade. Também `agent_id[in]=a,b`. |
| `message_id` | `string` | não | Igualdade. Também `message_id[in]=a,b`. |
| `from` | `string` | não | Igualdade. Também `from[in]=a,b`. |
| `to` | `string` | não | Linhas cujo `to` contém o valor. |
| `created_at` | `string` | não | Igualdade. Também `created_at[in]=a,b` e `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | não |  |
| `created_at[lte]` | `string` | não |  |
| `updated_at` | `string` | não | Igualdade. Também `updated_at[in]=a,b` e `updated_at[gt\|gte\|lt\|lte]`. |
| `updated_at[gte]` | `string` | não |  |
| `updated_at[lte]` | `string` | não |  |
| `received_at` | `string` | não | Igualdade. Também `received_at[in]=a,b` e `received_at[gt\|gte\|lt\|lte]`. |
| `received_at[gte]` | `string` | não |  |
| `received_at[lte]` | `string` | não |  |
| `processed_at` | `string` | não | Igualdade. Também `processed_at[in]=a,b` e `processed_at[gt\|gte\|lt\|lte]`. |
| `processed_at[gte]` | `string` | não |  |
| `processed_at[lte]` | `string` | não |  |

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

## emails create

Enviar um .eml bruto

```text
kobana inbox emails create [OPÇÕES]
```

`POST /v1/emails`

Armazena uma mensagem RFC 5322 exatamente como a ingestão SES faria: objeto bruto no bucket raw, `Email` em `received` e um job `process-email` — filtros, agente e webhooks rodam como para um e-mail real. Envie a mensagem como `multipart/form-data` (campo `file`, `inbox_id` opcional) ou como corpo da requisição com `Content-Type: message/rfc822` (`?inbox_id=` opcional). Sem `inbox_id`, a caixa é resolvida pelo cabeçalho `To:` entre as caixas do workspace do token. Um `Message-ID` já armazenado na mesma caixa é rejeitado com 409 (`error.details.email_id` aponta para o e-mail existente); o mesmo `Message-ID` em outra caixa é outro e-mail.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `inbox_id` | `string` | não | Caixa de destino. Sobrescrito pelo campo multipart `inbox_id` quando ambos são enviados. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema inbox.emails.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Armazenado e enfileirado para processamento. |
| `400` | Corpo vazio ou campo multipart `file` ausente. |
| `404` | `inbox_id` não existe no workspace do token (`INBOX_NOT_FOUND`). |
| `409` | `DUPLICATE_MESSAGE_ID`: o `Message-ID` já está armazenado na caixa de destino. |
| `413` | `PAYLOAD_TOO_LARGE`: mensagem acima de 20 MB. |
| `415` | `UNSUPPORTED_MEDIA_TYPE`: o corpo não é multipart nem `message/rfc822`. |
| `422` | `NO_INBOX_MATCHES_TO_HEADER` (nenhuma caixa do workspace corresponde aos destinatários do `To:`), `MISSING_RECIPIENT` (sem cabeçalho `To:` utilizável; informe `inbox_id`) ou `INVALID_MESSAGE_ID` (caracteres de controle ou mais de 998 caracteres). |

## emails get

Obter email

```text
kobana inbox emails get [ID] [OPÇÕES]
```

`GET /v1/emails/{id}`

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

## emails delete

Excluir e-mail

```text
kobana inbox emails delete [ID] [OPÇÕES]
```

`DELETE /v1/emails/{id}`

Soft-delete do e-mail (define `deleted_at`). Ele deixa de aparecer em listagens e detalhe; o objeto bruto é purgado pelo job de retenção.

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

## emails eml

Baixar o .eml original

```text
kobana inbox emails eml [ID] [OPÇÕES]
```

`GET /v1/emails/{id}/eml`

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Fluxo bruto da mensagem no formato RFC 5322. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## emails reprocess

Reprocessar email

```text
kobana inbox emails reprocess [ID] [OPÇÕES]
```

`POST /v1/emails/{id}/reprocess`

Devolve o e-mail ao processamento (filtros, agente, webhooks). Só é elegível um e-mail em status terminal — `processed`, `rejected` ou `failed`; um e-mail ainda `received`, `approved` ou `processing` é recusado com 409 `CONFLICT`, para que um agente (e seu workflow) nunca rode duas vezes para o mesmo e-mail. Rodar o agente exige um plano ativo: sem ele a chamada é recusada com 403 `ENTITLEMENT_REQUIRED` (`error.details.reason` = `no_plan`) e nada é enfileirado.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Aceito e enfileirado. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | `CONFLICT`: o e-mail ainda está `received`, `approved` ou `processing`. |
