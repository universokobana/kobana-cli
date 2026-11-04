---
id: builder
title: "kobana dashboards builder"
sidebar_label: "builder"
description: "Consultar o construtor do portal"
---

# kobana dashboards builder

Consultar o construtor do portal

Comandos de `kobana dashboards builder`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`builder`](#builder) | `GET /v1/builder` | Consultar o construtor do portal |
| [`builder draft update`](#builder-draft-update) | `PUT /v1/builder/draft` | Salvar o rascunho do portal |
| [`builder theme update`](#builder-theme-update) | `PATCH /v1/builder/theme` | Alterar as cores do tema do portal |
| [`builder tools create-page`](#builder-tools-create-page) | `POST /v1/builder/tools/create_page` | Criar uma página |
| [`builder tools delete-page`](#builder-tools-delete-page) | `POST /v1/builder/tools/delete_page` | Excluir uma página |
| [`builder tools describe-block-type`](#builder-tools-describe-block-type) | `GET /v1/builder/tools/describe_block_type` | Descrever um tipo de bloco |
| [`builder tools duplicate-block`](#builder-tools-duplicate-block) | `POST /v1/builder/tools/duplicate_block` | Duplicar um bloco |
| [`builder tools get-page`](#builder-tools-get-page) | `GET /v1/builder/tools/get_page` | Consultar uma página do rascunho |
| [`builder tools get-theme`](#builder-tools-get-theme) | `GET /v1/builder/tools/get_theme` | Consultar as cores do tema |
| [`builder tools insert-block`](#builder-tools-insert-block) | `POST /v1/builder/tools/insert_block` | Inserir um bloco |
| [`builder tools list-actions`](#builder-tools-list-actions) | `GET /v1/builder/tools/list_actions` | Listar as ações do botão |
| [`builder tools list-block-types`](#builder-tools-list-block-types) | `GET /v1/builder/tools/list_block_types` | Listar os tipos de bloco |
| [`builder tools list-pages`](#builder-tools-list-pages) | `GET /v1/builder/tools/list_pages` | Listar as páginas do rascunho |
| [`builder tools move-block`](#builder-tools-move-block) | `POST /v1/builder/tools/move_block` | Mover um bloco |
| [`builder tools move-page`](#builder-tools-move-page) | `POST /v1/builder/tools/move_page` | Mover uma página no menu |
| [`builder tools preview-portal-user-mapping`](#builder-tools-preview-portal-user-mapping) | `GET /v1/builder/tools/preview_portal_user_mapping` | Pré-visualizar o mapeamento de usuários do portal |
| [`builder tools read-asset`](#builder-tools-read-asset) | `GET /v1/builder/tools/read_asset` | Ler o conteúdo de um arquivo |
| [`builder tools register-entity`](#builder-tools-register-entity) | `POST /v1/builder/tools/register_entity` | Registrar uma entidade a partir de uma fonte de dados |
| [`builder tools remove-block`](#builder-tools-remove-block) | `POST /v1/builder/tools/remove_block` | Remover um bloco |
| [`builder tools rename-entity-fields`](#builder-tools-rename-entity-fields) | `POST /v1/builder/tools/rename_entity_fields` | Renomear os campos de uma entidade |
| [`builder tools save-computed-field`](#builder-tools-save-computed-field) | `POST /v1/builder/tools/save_computed_field` | Salvar um campo calculado |
| [`builder tools update-block`](#builder-tools-update-block) | `POST /v1/builder/tools/update_block` | Alterar um bloco |
| [`builder tools update-page`](#builder-tools-update-page) | `POST /v1/builder/tools/update_page` | Alterar uma página |
| [`builder versions get`](#builder-versions-get) | `GET /v1/builder/versions/{number}` | Consultar uma versão publicada |
| [`builder versions restore`](#builder-versions-restore) | `POST /v1/builder/versions/{number}/restore` | Restaurar uma versão no rascunho |
| [`builder versions rollback`](#builder-versions-rollback) | `POST /v1/builder/versions/{number}/rollback` | Colocar uma versão de volta no ar |
| [`builder encrypt-secret`](#builder-encrypt-secret) | `POST /v1/builder/encrypt_secret` | Criptografar um segredo de bloco |
| [`builder publish`](#builder-publish) | `POST /v1/builder/publish` | Publicar o portal |
| [`builder test-action`](#builder-test-action) | `POST /v1/builder/test_action` | Testar um botão de ação do rascunho |
| [`builder test-form`](#builder-test-form) | `POST /v1/builder/test_form` | Testar um formulário do rascunho |
| [`builder upload-og-image`](#builder-upload-og-image) | `POST /v1/builder/upload_og_image` | Enviar a og:image do portal |

## builder

Consultar o construtor do portal

```text
kobana dashboards builder [OPÇÕES] [COMANDO]
```

`GET /v1/builder`

Retorna o portal do workspace como o construtor o vê: identidade do portal, rascunho (páginas na ordem do menu, cada uma com sua marca em relação à versão no ar, `revision` e `dirty`), número da versão no ar, versões publicadas, tema e SEO padrão. Leia a `revision` daqui antes de salvar o rascunho.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O construtor do portal. |
| `404` | Recurso não encontrado na organização atual. |

## builder draft update

Salvar o rascunho do portal

```text
kobana dashboards builder draft update [OPÇÕES]
```

`PUT /v1/builder/draft`

Substitui o rascunho inteiro — páginas na ordem do menu, estado do editor e, opcionalmente, tema e SEO padrão — protegido pela `revision` lida por último. Uma `revision` desatualizada responde 409 e não altera nada. Nada vai ao ar: para publicar, use `POST /builder/publish`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `revision` | `integer` | sim | A `revision` do rascunho lida por último (`0` quando ainda não há rascunho). |
| `pages` | `array<object>` | sim | Todas as páginas do rascunho, na ordem do menu (até 200). Páginas omitidas saem do rascunho. |
| `editor_state` | `object` | não | Estado do editor; devolva o que leu. Omitido = `{}`. |
| `theme` | `object` | não |  |
| `seo_defaults` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.draft.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Rascunho salvo: nova `revision`, se difere da versão no ar e a marca de cada página. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |

## builder theme update

Alterar as cores do tema do portal

```text
kobana dashboards builder theme update [OPÇÕES]
```

`PATCH /v1/builder/theme`

Altera cores do tema das páginas do portal (a aba Visual do construtor) no rascunho, por modo, em hex. Só as cores enviadas mudam; o portal no ar muda quando o rascunho for publicado. Para ler as cores atuais e o que cada uma faz, use `GET /builder/tools/get_theme`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `light` | `object` | não |  |
| `dark` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.theme.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Tema do rascunho: nova `revision`, se o tema mudou e quantas cores foram enviadas. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## builder tools create-page

Criar uma página

```text
kobana dashboards builder tools create-page [OPÇÕES]
```

`POST /v1/builder/tools/create_page`

Cria uma página no rascunho e a adiciona ao menu do portal. template: blank (só o título), list (título + KPIs + tabela de `entity`), detail (título + detalhe do registro + documentos). O título é um bloco PageHeader: a resposta lista os blocos criados — altere-os com update_block em vez de inseri-los de novo.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Menu label. |
| `path` | `string` | não | Route, e.g. /contratos. Omitted = derived from the name. |
| `title` | `string` | não | Browser title; omitted = the name. |
| `template` | `string` | não | Valores: `blank`, `list`, `detail`. |
| `entity` | `string` | não | Entity the list/detail template binds to. |
| `show_in_menu` | `boolean` | não | Listed in the portal menu (default true); false for a detail page reached by links. |
| `position` | `integer` | não | Place in the menu; omitted = last. |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.create-page`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools delete-page

Excluir uma página

```text
kobana dashboards builder tools delete-page [OPÇÕES]
```

`POST /v1/builder/tools/delete_page`

Remove uma página (e os blocos dela) do rascunho e do menu. Só quando o operador pedir.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page_id` | `string` | sim | Page id (list_pages). |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.delete-page`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools describe-block-type

Descrever um tipo de bloco

```text
kobana dashboards builder tools describe-block-type [OPÇÕES]
```

`GET /v1/builder/tools/describe_block_type`

As props de um tipo de bloco: JSON Schema, valores padrão e slots. Leia antes de inserir ou alterar um bloco que você ainda não usou.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `type` | `string` | sim | Block type, e.g. DataTable. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta leu, em `data`. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools duplicate-block

Duplicar um bloco

```text
kobana dashboards builder tools duplicate-block [OPÇÕES]
```

`POST /v1/builder/tools/duplicate_block`

Copia um bloco (com os blocos dentro dele) logo depois dele mesmo, com ids novos.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `block_id` | `string` | sim | Block id (get_page). |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.duplicate-block`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools get-page

Consultar uma página do rascunho

```text
kobana dashboards builder tools get-page [OPÇÕES]
```

`GET /v1/builder/tools/get_page`

Uma página do rascunho: as configurações (nome, rota, título, SEO) e a árvore de blocos, cada bloco com id, tipo, props e slots.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page_id` | `string` | sim | Page id (list_pages). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta leu, em `data`. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools get-theme

Consultar as cores do tema

```text
kobana dashboards builder tools get-theme [OPÇÕES]
```

`GET /v1/builder/tools/get_theme`

As cores das páginas do portal (a aba Visual do rascunho), em hex, nos modos claro e escuro: para que serve cada cor e o valor atual.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta leu, em `data`. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools insert-block

Inserir um bloco

```text
kobana dashboards builder tools insert-block [OPÇÕES]
```

`POST /v1/builder/tools/insert_block`

Insere um bloco do catálogo numa página: na raiz da página ou dentro de um slot de um bloco Columns/Tabs (parent_block_id + slot). Props não enviadas usam o valor padrão.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page_id` | `string` | sim | Page id (list_pages). |
| `type` | `string` | sim | Block type (list_block_types). |
| `props` | `object` | não | Block props (describe_block_type). Never include id or slot props. |
| `parent_block_id` | `string` | não | Columns or Tabs block that receives the new block. |
| `slot` | `string` | não | Slot of the parent: col1..col3 (Columns) or content (Tabs). Omitted = the first. |
| `position` | `integer` | não | 0-based index; omitted = at the end. |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.insert-block`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools list-actions

Listar as ações do botão

```text
kobana dashboards builder tools list-actions [OPÇÕES]
```

`GET /v1/builder/tools/list_actions`

O que um ActionButton pode fazer (o seu actionType): abrir um link (uma página do portal ou uma URL) ou disparar um webhook, e as \{\{variáveis\}\} que a URL, os cabeçalhos e o corpo do webhook podem usar.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta leu, em `data`. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools list-block-types

Listar os tipos de bloco

```text
kobana dashboards builder tools list-block-types [OPÇÕES]
```

`GET /v1/builder/tools/list_block_types`

Lista o catálogo de blocos: todo tipo de bloco que o portal sabe renderizar, com para que serve. Só esses tipos existem.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta leu, em `data`. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools list-pages

Listar as páginas do rascunho

```text
kobana dashboards builder tools list-pages [OPÇÕES]
```

`GET /v1/builder/tools/list_pages`

Lista as páginas do rascunho na ordem do menu: id, nome, rota, título e quantidade de blocos.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta leu, em `data`. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools move-block

Mover um bloco

```text
kobana dashboards builder tools move-block [OPÇÕES]
```

`POST /v1/builder/tools/move_block`

Move um bloco dentro da página: para outra posição na raiz (parent_block_id null) ou para um slot de um bloco Columns/Tabs.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `block_id` | `string` | sim | Block id (get_page). |
| `position` | `integer` | sim | 0-based index in the target. |
| `parent_block_id` | `string` | não | Columns/Tabs block to move into; null or omitted = the page root. |
| `slot` | `string` | não |  |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.move-block`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools move-page

Mover uma página no menu

```text
kobana dashboards builder tools move-page [OPÇÕES]
```

`POST /v1/builder/tools/move_page`

Move uma página para outro lugar do menu do portal.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page_id` | `string` | sim | Page id (list_pages). |
| `position` | `integer` | sim | 0-based index in the menu. |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.move-page`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools preview-portal-user-mapping

Pré-visualizar o mapeamento de usuários do portal

```text
kobana dashboards builder tools preview-portal-user-mapping [OPÇÕES]
```

`GET /v1/builder/tools/preview_portal_user_mapping`

Confere um mapeamento de usuários do portal antes de salvá-lo com `dashboards_portal_user_mappings_update`: a entidade cujos registros virariam os usuários do portal, os campos de ID do tenant, e-mail e nome, quantos usuários seriam e como os IDs de tenant casam com os titulares dos registros das outras entidades. Não salva nada.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `entity` | `string` | sim | Entity key. |
| `tenant_id_field` | `string` | sim | The value the other entities' owners hold (the owner field of the entity, or its identifier). |
| `email_field` | `string` | sim | The field with the customer's e-mail. |
| `name_field` | `string` | não | The field with the customer's name, if any. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta leu, em `data`. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools read-asset

Ler o conteúdo de um arquivo

```text
kobana dashboards builder tools read-asset [OPÇÕES]
```

`GET /v1/builder/tools/read_asset`

Abre um arquivo da biblioteca de Arquivos para ver o conteúdo: imagem ou PDF vem como imagem, arquivo de texto (txt, csv, json, md, xml) ou planilha .xlsx vem como texto. Word/PowerPoint/zip não podem ser lidos (só os detalhes deles).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `asset_id` | `string` | sim | Asset id (list_assets, or the asset_id of a file sent in the chat). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta leu, em `data`. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools register-entity

Registrar uma entidade a partir de uma fonte de dados

```text
kobana dashboards builder tools register-entity [OPÇÕES]
```

`POST /v1/builder/tools/register_entity`

Registra uma entidade (os blocos de dados se ligam a ela) a partir de uma operação de uma fonte de dados conectada (describe_data_source), com os campos inferidos de uma amostra dos registros, e inicia a primeira sincronização. Use quando o operador quer dados que nenhuma entidade tem ainda. Vale no workspace na hora (não faz parte do rascunho); a nova entidade já pode ser usada por blocos na mesma resposta. O workspace limita quantos campos uma entidade nova guarda: quando a fonte tem mais, a recusa lista todos os campos com um valor de exemplo — chame de novo com `fields` contendo os que o portal precisa (o que ele mostra, filtra ou soma, e o que identifica o cliente dono do registro).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `data_source_id` | `string (uuid)` | sim |  |
| `operation_key` | `string` | sim | Operation key (describe_data_source); prefer a "list" operation. |
| `params` | `object` | não | Fixed parameters the operation needs, if any. |
| `key` | `string` | não | Entity key; omitted = inferred from the operation. |
| `name` | `string` | não | Singular name in Portuguese, e.g. "Contrato". |
| `name_plural` | `string` | não | Plural name, e.g. "Contratos". |
| `fields` | `array<string>` | não | Field keys to keep, from the list a refusal gives; omitted = every visible field the sample shows. |
| `field_names` | `object` | não | Labels in Brazilian Portuguese for the kept fields, by key (\{ "amount": "Valor", "expire_at": "Vencimento" \}): they are the column headers the customers read. |
| `owner_field` | `string` | não | The field saying which customer the record belongs to (the titular), when the inferred one is wrong or missing. On a CUSTOMERS entity it is its own id (each customer owns their own record); on a data entity, the customer id it carries (e.g. customer_id). When the portal is keyed by CPF/CNPJ, the document field instead. |
| `owner_match` | `string` | não | How the owner is compared to the users' tenant ID: external_id = as text (an id); document_number = as a CPF/CNPJ, digits only on both sides, so "32.013.964/0001-60" matches "32013964000160". Use document_number whenever the owner field holds a CPF/CNPJ. Valores: `external_id`, `document_number`. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.register-entity`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools remove-block

Remover um bloco

```text
kobana dashboards builder tools remove-block [OPÇÕES]
```

`POST /v1/builder/tools/remove_block`

Remove um bloco (e os blocos dentro dele) da página.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `block_id` | `string` | sim | Block id (get_page). |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.remove-block`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools rename-entity-fields

Renomear os campos de uma entidade

```text
kobana dashboards builder tools rename-entity-fields [OPÇÕES]
```

`POST /v1/builder/tools/rename_entity_fields`

Altera uma entidade existente: renomeia os campos — os rótulos que os blocos mostram (cabeçalhos de tabela, rótulos de cards) — e, opcionalmente, a própria entidade, e pode definir o campo titular (de quem é o registro). Use quando os rótulos vieram da fonte em inglês ou como chaves cruas ("Expire at", "customer_cnpj_cpf"): os clientes leem, então vão em português do Brasil; e quando o titular está errado ou faltando (uma entidade de clientes é titular dos próprios registros pelo id). As chaves ficam; um novo titular mapeia de novo os registros guardados. Vale no workspace na hora, não no rascunho.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `entity` | `string` | sim | Entity key (list_entities). |
| `names` | `object` | não | New label by field key, e.g. \{ "amount": "Valor", "expire_at": "Vencimento" \}. |
| `name` | `string` | não | New singular name of the entity. |
| `name_plural` | `string` | não | New plural name of the entity. |
| `owner_field` | `string` | não | New owner field (a field key of the entity, a computed one included). |
| `owner_match` | `string` | não | How the owner is compared to the users' tenant ID: external_id = as text (an id); document_number = as a CPF/CNPJ, digits only on both sides, so "32.013.964/0001-60" matches "32013964000160". Use document_number whenever the owner field holds a CPF/CNPJ. Valores: `external_id`, `document_number`. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.rename-entity-fields`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools save-computed-field

Salvar um campo calculado

```text
kobana dashboards builder tools save-computed-field [OPÇÕES]
```

`POST /v1/builder/tools/save_computed_field`

Adiciona um campo calculado a uma entidade — um valor derivado dos outros campos por uma expressão (no estilo do n8n) — ou altera a expressão de um campo calculado que ela já tem (mesma chave). Use quando o operador quer um valor que a fonte não entrega pronto: parte de um texto, a junção de campos, uma data ou valor formatado, um rótulo a partir de uma condição. A expressão é testada antes nos registros guardados (uma expressão que falha em todos é recusada e nada é salvo), depois salva na entidade no workspace na hora (não faz parte do rascunho) e calculada em cada registro guardado com os dados que ele já tem (sem buscar de novo na fonte). Blocos e sample_records já a veem na mesma resposta.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `entity` | `string` | sim | Entity key (list_entities). |
| `key` | `string` | sim | Field key, snake_case. Must not be a field the source maps. |
| `name` | `string` | sim | Field label in Portuguese, e.g. "Descrição curta". |
| `expression` | `string` | sim | Template with \{\{ \}\}, e.g. "\{\{ $resource.description.split('\\r\\n')[0] \}\}". |
| `data_type` | `string` | não | What the expression gives: string, int, decimal, money, bool, date, datetime… Valores: `string`, `int`, `decimal`, `money`, `bool`, `date`, `datetime`, `email`, `enum`, `document`, `phone`, `url`, `file`, `array`, `json`. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.save-computed-field`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools update-block

Alterar um bloco

```text
kobana dashboards builder tools update-block [OPÇÕES]
```

`POST /v1/builder/tools/update_block`

Altera props de um bloco. Só as props enviadas mudam; o resultado é validado contra o tipo do bloco.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `block_id` | `string` | sim | Block id (get_page). |
| `props` | `object` | sim | Block props (describe_block_type). Never include id or slot props. |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.update-block`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder tools update-page

Alterar uma página

```text
kobana dashboards builder tools update-page [OPÇÕES]
```

`POST /v1/builder/tools/update_page`

Altera as configurações de uma página: nome no menu, rota, título no navegador, se ela aparece no menu do portal e as tags de SEO / Open Graph. Só os campos enviados mudam.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page_id` | `string` | sim | Page id (list_pages). |
| `name` | `string` | não |  |
| `path` | `string` | não |  |
| `title` | `string` | não |  |
| `show_in_menu` | `boolean` | não | false = the page stays reachable by links (e.g. a record detail page) but is not in the header menu. |
| `seo` | `object` | não |  |
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.tools.update-page`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O que a ferramenta fez, em `data`, e a nova `revision` do rascunho (a mesma de antes se nada mudou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | A ferramenta recusou o passo (`BUILDER_OPERATION_REJECTED`): tipo de bloco inexistente, props que não passam no esquema do bloco, caminho em uso, entidade desconhecida… A mensagem diz o motivo e nada é salvo. |

## builder versions get

Consultar uma versão publicada

```text
kobana dashboards builder versions get [NUMBER] [OPÇÕES]
```

`GET /v1/builder/versions/{number}`

Retorna uma versão publicada do portal, pelo número, com as páginas que ela congelou.

**Argumentos do caminho:** `number`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A versão e suas páginas. |
| `404` | Recurso não encontrado na organização atual. |

## builder versions restore

Restaurar uma versão no rascunho

```text
kobana dashboards builder versions restore [NUMBER] [OPÇÕES]
```

`POST /v1/builder/versions/{number}/restore`

Copia uma versão publicada para o rascunho, substituindo as páginas dele — para continuar editando a partir de uma versão antiga. Nada vai ao ar: o portal só muda quando o rascunho for publicado. (Colocar uma versão antiga no ar é `.../rollback`.) `revision` é opcional: enviada e desatualizada, responde 409.

**Argumentos do caminho:** `number`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `revision` | `integer` | não | A `revision` do rascunho lida por último. Opcional: se enviada e desatualizada, responde 409 e não salva nada. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.versions.restore`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Rascunho com as páginas da versão: nova `revision`, número restaurado e quantas páginas. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |

## builder versions rollback

Colocar uma versão de volta no ar

```text
kobana dashboards builder versions rollback [NUMBER] [OPÇÕES]
```

`POST /v1/builder/versions/{number}/rollback`

Coloca essa versão de volta no ar como o próximo número — os visitantes do portal a veem na hora. Retorna a nova versão e o construtor como ficou.

**Argumentos do caminho:** `number`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Nova versão no ar e o construtor atualizado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Essa versão já está no ar (`ALREADY_LIVE`). |

## builder encrypt-secret

Criptografar um segredo de bloco

```text
kobana dashboards builder encrypt-secret [OPÇÕES]
```

`POST /v1/builder/encrypt_secret`

Criptografa um valor que um bloco guarda no JSON da página (um cabeçalho de webhook como `Authorization`). Retorna `secret:<cifrado>`, vinculado a este workspace e à origem de `url` (obrigatória), que só o servidor abre para enviar a requisição a essa origem. Nada é armazenado: salve o valor retornado no bloco junto com o rascunho. Substitui `POST /builder/secrets`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `value` | `string` | sim | O valor em texto puro a criptografar. Nunca é devolvido. |
| `url` | `string` | sim | A URL do webhook a que o valor se destina (obrigatória): o segredo só abre para uma requisição à mesma origem (esquema, domínio e porta). O domínio precisa ser fixo — sem `{{variáveis}}`. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.encrypt-secret`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O valor criptografado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | A `url` não tem uma origem http(s) fixa — por exemplo, uma `{{variável}}` no domínio (`ACTION_SECRET_URL_REQUIRED`). |

## builder publish

Publicar o portal

```text
kobana dashboards builder publish [OPÇÕES]
```

`POST /v1/builder/publish`

Salva o rascunho como enviado (mesmo corpo do salvamento, mais uma `note` opcional) e o congela como a próxima versão, numa única transação: os visitantes do portal passam a vê-la na hora. Um campo de formulário cuja expressão regular pode travar a validação responde 422 `UNSAFE_FIELD_PATTERN`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `revision` | `integer` | sim | A `revision` do rascunho lida por último (`0` quando ainda não há rascunho). |
| `pages` | `array<object>` | sim | Todas as páginas do rascunho, na ordem do menu (até 200). Páginas omitidas saem do rascunho. |
| `editor_state` | `object` | não | Estado do editor; devolva o que leu. Omitido = `{}`. |
| `theme` | `object` | não |  |
| `seo_defaults` | `object` | não |  |
| `note` | `string` | não | O que mudou, exibido no histórico de versões. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.publish`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Versão publicada, com a nova `revision` do rascunho. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | A `revision` enviada está desatualizada: o rascunho foi salvo por outra pessoa ou aba. Leia de novo e reenvie. |
| `422` | Uma expressão regular de campo de formulário pode travar a validação (`UNSAFE_FIELD_PATTERN`); `details` aponta página, bloco e campo. |

## builder test-action

Testar um botão de ação do rascunho

```text
kobana dashboards builder test-action [OPÇÕES]
```

`POST /v1/builder/test_action`

Executa o webhook de um Botão de ação como a prévia do construtor: a requisição sai de verdade, marcada como `preview: true`, e a resposta traz o status HTTP devolvido pelo destino. Cabeçalhos podem levar valores `secret:` criados por `POST /builder/encrypt_secret`, que só abrem para a origem para a qual foram criados. Limitado a 20 execuções por minuto por operador, 60 por minuto e 500 por dia por organização. Substitui `POST /builder/action_runs`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `block_id` | `string` | sim | Id do bloco Botão de ação. |
| `request` | `object` | sim |  |
| `label` | `string` | não |  |
| `page_path` | `string` | não | Caminho da página onde o bloco está. Padrão `/`. |
| `submitter` | `object` | não | Cliente que a prévia representa, para `{{customer_name}}` / `{{customer_email}}`. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.test-action`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da execução. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Requisição mal configurada (URL sem https, cabeçalho inválido, segredo de outro workspace…). |

## builder test-form

Testar um formulário do rascunho

```text
kobana dashboards builder test-form [OPÇÕES]
```

`POST /v1/builder/test_form`

Executa a ação de um bloco Formulário como a prévia do construtor: o e-mail configurado sai de verdade (ou o webhook escolhido dispara), marcado como `preview: true`. A configuração vem no corpo (o rascunho de quem chama). O e-mail de teste vai só para o operador que o token nomeia. Limitado a 20 execuções por minuto por operador, 60 por minuto e 500 por dia por organização. Substitui `POST /builder/form_submissions`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `form_id` | `string` | sim | Id do bloco Formulário. |
| `fields` | `array<object>` | sim |  |
| `action` | `object` | sim |  |
| `form_title` | `string` | não |  |
| `page_path` | `string` | não | Caminho da página onde o bloco está. Padrão `/`. |
| `values` | `object` | não | Valores preenchidos, por nome do campo. |
| `submitter` | `object` | não | Cliente que a prévia representa, para `{{customer_name}}` / `{{customer_email}}`. |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.test-form`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Envio registrado: destinatários, notificações ou entrega do webhook. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Valores que não passam nas regras do formulário, expressão regular perigosa ou ação mal configurada; `field_errors` lista os campos. |

## builder upload-og-image

Enviar a og:image do portal

```text
kobana dashboards builder upload-og-image [OPÇÕES]
```

`POST /v1/builder/upload_og_image`

Envia a og:image de uma página (ou a padrão do portal) — a imagem que as prévias de link mostram — em base64 (PNG, JPEG ou WebP, até ~1,5 MB) e devolve a URL pública, para guardar em `seo.og_image_url` (ou `seo_defaults.og_image_url`) no próximo salvamento do rascunho. Substitui `POST /builder/og_image`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `filename` | `string` | sim | Nome original do arquivo. |
| `content_type` | `string` | sim | Valores: `image/png`, `image/jpeg`, `image/webp`. |
| `content_base64` | `string` | sim | Os bytes da imagem em base64 (um prefixo `data:…;base64,` é aceito). |

Outros campos e objetos aninhados: `kobana schema dashboards.builder.upload-og-image`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Imagem enviada, com a URL pública. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `413` | O corpo da requisição passou de 2 MB. |
| `422` | Tipo não aceito, base64 inválido, imagem acima do limite ou conteúdo que não corresponde ao `content_type`. |
