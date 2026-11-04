---
id: organization
title: "kobana inbox organization"
sidebar_label: "organization"
description: "Obter organização"
---

# kobana inbox organization

Obter organização

Comandos de `kobana inbox organization`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Obter organização |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Atualizar organização |

## organization

Obter organização

```text
kobana inbox organization [OPÇÕES] [COMANDO]
```

`GET /v1/organization`

A organização vem só do token (o claim `org`, como em `GET /v1/me`): não há lista nem `{id}`, e nenhum header, query ou corpo escolhe outra. Nível de organização: `X-Workspace-Id` não é necessário. Exige a permissão `settings.show` (todo papel de sistema a tem).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |

## organization update

Atualizar organização

```text
kobana inbox organization update [OPÇÕES]
```

`PATCH /v1/organization`

A organização vem só do token (o claim `org`, como em `GET /v1/me`): não há lista nem `{id}`, e nenhum header, query ou corpo escolhe outra. Nível de organização: `X-Workspace-Id` não é necessário. Altera o perfil comum a todos os workspaces: `custom_metadata`. Chaves desconhecidas, os campos de identidade (`id`, `public_id`, `external_id`, `slug`, `document_type`, `document_number`) e os espelhos da conta Kobana (`name`, `email`, `phone` — sincronizados a partir dela, por isso não são editáveis aqui) são ignorados. Citar uma configuração comercial da Kobana (cortesia, liberação sem assinatura, limites, parceiro, entitlements) é recusado com 403 `FORBIDDEN` (`error.details.fields`). Exige a permissão `settings.update` (dono e admin). Toda alteração fica no log de auditoria da organização.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. `null` mantém o valor guardado. Máximo de 64 KB serializado, 16 níveis de profundidade. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado. `replace` (padrão) sobrescreve o objeto inteiro; `merge` faz deep-merge sobre o valor atual — objetos aninhados mesclam recursivamente, escalares e arrays sobrescrevem a chave, e chaves `null` são mantidas (não apagam a chave). Somente em update. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema inbox.organization.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
