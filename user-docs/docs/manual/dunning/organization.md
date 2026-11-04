---
id: organization
title: "kobana dunning organization"
sidebar_label: "organization"
description: "Consultar a organização"
---

# kobana dunning organization

Consultar a organização

Comandos de `kobana dunning organization`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Consultar a organização |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Atualizar a organização |

## organization

Consultar a organização

```text
kobana dunning organization [OPÇÕES] [COMANDO]
```

`GET /v1/organization`

Retorna a organização do token. A organização vem só da credencial: não há lista nem `{id}`, e nenhum header, query ou corpo escolhe outra. É de nível de organização: não exige o header `X-Workspace-Id`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A organização do token. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## organization update

Atualizar a organização

```text
kobana dunning organization update [OPÇÕES]
```

`PATCH /v1/organization`

Atualiza o perfil da organização do token: nome fantasia, website, fuso horário e `custom_metadata`. Nome, e-mail, telefone, documento e ids são espelho da conta Kobana e não mudam por aqui (o campo enviado é ignorado); campos comerciais (`bypass_subscription`, `courtesy`…) respondem `403`. O perfil vale para todos os workspaces, então exige ser dono da organização ou ter acesso a todos os workspaces. Não exige o header `X-Workspace-Id`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `trade_name` | `string` | não | Nome fantasia; `null` limpa. |
| `website` | `string (uri)` | não | Website (URL); `null` limpa. |
| `timezone` | `string` | não | Nome de fuso IANA (ex.: `America/Sao_Paulo`). Offsets e abreviações são recusados. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre). `null` mantém o valor gravado. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.organization.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A organização atualizada. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
