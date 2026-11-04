---
id: workspaces
title: "kobana dashboards workspaces"
sidebar_label: "workspaces"
---

# kobana dashboards workspaces


Comandos de `kobana dashboards workspaces`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | Listar workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Criar workspace |
| [`workspaces import`](#workspaces-import) | `POST /v1/workspaces/import` | Importar workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Consultar workspace |
| [`workspaces update`](#workspaces-update) | `PATCH /v1/workspaces/{id}` | Renomear workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Excluir workspace |
| [`workspaces export`](#workspaces-export) | `GET /v1/workspaces/{id}/export` | Exportar workspace |
| [`workspaces usage`](#workspaces-usage) | `GET /v1/workspaces/{id}/usage` | Contar o que a exclusão do workspace esconderia |

## workspaces list

Listar workspaces

```text
kobana dashboards workspaces list [OPÇÕES]
```

`GET /v1/workspaces`

Lista os workspaces da organização que você pode abrir, com o seu papel em cada um — os mesmos do seletor do dashboard. Um token de máquina vê todos os workspaces da organização. Não exige `X-Workspace-Id`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | não | Trecho do nome do workspace, sem diferenciar maiúsculas. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de workspaces. |
| `404` | Recurso não encontrado na organização atual. |

## workspaces create

Criar workspace

```text
kobana dashboards workspaces create [OPÇÕES]
```

`POST /v1/workspaces`

Cria um workspace vazio na organização. A pessoa do token vira dona dele. Criado por um token de máquina, o workspace fica sem dono: os donos da organização o recebem no próximo login, e `PUT /dashboard_users/{id}/workspaces` o concede a outros membros.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do workspace, de 1 a 100 caracteres. |

Outros campos e objetos aninhados: `kobana schema dashboards.workspaces.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Workspace criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## workspaces import

Importar workspace

```text
kobana dashboards workspaces import [OPÇÕES]
```

`POST /v1/workspaces/import`

Restaura um arquivo de `GET /workspaces/{id}/export` em um workspace NOVO, cujo dono é a pessoa do token. Responde o que não pôde ser trazido de volta (`warnings`) para terminar à mão. Exige um token de pessoa (um token de máquina recebe 403) com as permissões de criar workspaces, configurar o portal e criar fontes de dados. As entidades do arquivo passam pelas mesmas validações de quando são criadas uma a uma (chaves, expressões, limite de campos do workspace). O corpo é limitado a 2 MB.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `data` | `object` | sim | O arquivo exportado, como retornado pela exportação. |
| `name` | `string` | não | Nome do novo workspace; por padrão, o do arquivo. |

Outros campos e objetos aninhados: `kobana schema dashboards.workspaces.import`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Workspace importado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | O arquivo não é uma exportação de workspace válida (`INVALID_WORKSPACE_FILE`). |

## workspaces get

Consultar workspace

```text
kobana dashboards workspaces get [ID] [OPÇÕES]
```

`GET /v1/workspaces/{id}`

Retorna um workspace que você pode abrir. Um workspace sem acesso concedido responde 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Workspace encontrado. |
| `404` | Recurso não encontrado na organização atual. |

## workspaces update

Renomear workspace

```text
kobana dashboards workspaces update [ID] [OPÇÕES]
```

`PATCH /v1/workspaces/{id}`

Renomeia o workspace. Só os donos do workspace podem (403 para um membro comum).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do workspace, de 1 a 100 caracteres. |

Outros campos e objetos aninhados: `kobana schema dashboards.workspaces.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Workspace renomeado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## workspaces delete

Excluir workspace

```text
kobana dashboards workspaces delete [ID] [OPÇÕES]
```

`DELETE /v1/workspaces/{id}`

Exclui o workspace (exclusão lógica), só para os donos dele: o portal sai do ar, os conectores param de sincronizar e os membros perdem o acesso. O último workspace da organização não pode ser excluído.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Workspace excluído. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | É o último workspace da organização (`WORKSPACE_LAST`). |

## workspaces export

Exportar workspace

```text
kobana dashboards workspaces export [ID] [OPÇÕES]
```

`GET /v1/workspaces/{id}/export`

Baixa a configuração do workspace (construtor do portal, acesso e configurações do portal, mapeamento de usuários, conectores e entidades) como um arquivo JSON, só para os donos dele. Segredos e dados ficam de fora. A resposta é o próprio arquivo, sem envelope `data`; envie-o como `data` em `POST /workspaces/import`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Arquivo de exportação (`Content-Disposition: attachment`). |
| `404` | Recurso não encontrado na organização atual. |

## workspaces usage

Contar o que a exclusão do workspace esconderia

```text
kobana dashboards workspaces usage [ID] [OPÇÕES]
```

`GET /v1/workspaces/{id}/usage`

Conta o que a exclusão do workspace esconderia — o mesmo resumo do diálogo de exclusão do dashboard.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Contagens do workspace. |
| `404` | Recurso não encontrado na organização atual. |
