---
id: imports
title: "kobana banking imports"
sidebar_label: "imports"
---

# kobana banking imports


Comandos de `kobana banking imports`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports/{collection_name}` | Listar Importações |
| [`imports create`](#imports-create) | `POST /v1/imports/{collection_name}` | Importar |
| [`imports get`](#imports-get) | `GET /v1/imports/{collection_name}/{id}` | Visualizar Importação |

## imports list

Listar Importações

```text
kobana banking imports list [COLLECTION_NAME] [OPÇÕES]
```

`GET /v1/imports/{collection_name}`

Lista as importações realizadas para um tipo específico de recurso (clientes, assinaturas, boletos ou carnês).

**Argumentos do caminho:** `collection_name`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Importações encontradas |

## imports create

Importar

```text
kobana banking imports create [COLLECTION_NAME] [OPÇÕES]
```

`POST /v1/imports/{collection_name}`

Importa recursos (clientes, assinaturas, boletos ou carnês) através de um arquivo CSV. O processamento é realizado de forma assíncrona.

> **Processamento Assíncrono:** Este endpoint é processado assincronamente.

A Importação é feita assincronamente, ou seja, ao receber a requisição,
a nossa API guarda os parâmetros da requisição e processa a importação em segundo plano.
Após a importação ser processada, os registros serão associados à sua Conta.
Você recebe na resposta desta requisição o ID da Importação criada e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `import.created` é disparado.
O nosso servidor então começa o processamento da importação.
Após o fim do processamento, o evento `import.processed` é disparado.

**Argumentos do caminho:** `collection_name`

**Corpo da requisição**

Enviado como formulário `multipart/form-data`: os arquivos vão em `--file CAMPO=CAMINHO`.

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `source` | `string (binary)` | sim | Arquivo para importação. |

Outros campos e objetos aninhados: `kobana schema banking.imports.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Importação criada |
| `422` | Já existe uma importação em andamento ou concluída para este arquivo |
| `503` | Não foi possível verificar duplicidade da importação no momento — tente novamente |

## imports get

Visualizar Importação

```text
kobana banking imports get [COLLECTION_NAME] [ID] [OPÇÕES]
```

`GET /v1/imports/{collection_name}/{id}`

Consulta informações específicas de uma Importação.

**Argumentos do caminho:** `collection_name`, `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Importação encontrada |
| `404` | Conta não encontrada |
