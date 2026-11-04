---
id: holidays
title: "kobana billing holidays"
sidebar_label: "holidays"
---

# kobana billing holidays


Comandos de `kobana billing holidays`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`holidays list`](#holidays-list) | `GET /v1/holidays` | Listar feriados |
| [`holidays create`](#holidays-create) | `POST /v1/holidays` | Criar feriado |
| [`holidays sync`](#holidays-sync) | `POST /v1/holidays/sync` | Importar feriados nacionais |
| [`holidays get`](#holidays-get) | `GET /v1/holidays/{id}` | Consultar feriado |
| [`holidays update`](#holidays-update) | `PATCH /v1/holidays/{id}` | Atualizar feriado |
| [`holidays delete`](#holidays-delete) | `DELETE /v1/holidays/{id}` | Remover feriado |

## holidays list

Listar feriados

```text
kobana billing holidays list [OPÇÕES]
```

`GET /v1/holidays`

Retorna os feriados da organização — os dias que a contagem em dias úteis salta. Uma política de juros e multa com `day_count = business` lê exatamente esta tabela.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `year` | `integer` | não | Filtra por ano civil. |
| `date_from` | `string` | não | Data inicial, inclusiva, no formato `YYYY-MM-DD`. |
| `date_to` | `string` | não | Data final, inclusiva, no formato `YYYY-MM-DD`. |
| `type` | `string` | não | Filtra pela abrangência do feriado. Valores: `national`, `state`, `municipal`, `optional`, `custom`. |
| `include_deleted` | `string` | não | Inclui feriados removidos logicamente. Valores: `true`, `false`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de feriados. |
| `404` | Recurso não encontrado na organização atual. |

## holidays create

Criar feriado

```text
kobana billing holidays create [OPÇÕES]
```

`POST /v1/holidays`

Cria um feriado. Use para feriados estaduais, municipais e pontos facultativos — os nacionais chegam pela importação automática.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `date` | `string` | sim | Dia civil do feriado, no formato `YYYY-MM-DD`. Sem hora: um feriado é uma data no calendário, e aceitar um instante faria o dia gravado depender do fuso de quem enviou. |
| `name` | `string` | sim | Nome do feriado. Junto com a data, identifica a linha — um feriado municipal e um nacional podem cair no mesmo dia. |
| `type` | `string` | não | Abrangência: nacional, estadual, municipal, ponto facultativo (`optional`) ou próprio da organização (`custom`). Valores: `national`, `state`, `municipal`, `optional`, `custom`. |
| `notes` | `string` | não | Texto livre — a cidade de um feriado municipal, ou qualquer observação. |

Outros campos e objetos aninhados: `kobana schema billing.holidays.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Feriado criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## holidays sync

Importar feriados nacionais

```text
kobana billing holidays sync [OPÇÕES]
```

`POST /v1/holidays/sync`

Importa os feriados nacionais brasileiros da BrasilAPI para os anos informados (por omissão, o ano corrente e os dois seguintes). Idempotente: importa só o que falta e nunca ressuscita um feriado removido de propósito.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `years` | `array<integer>` | não | Anos a importar. Por omissão, o ano corrente e os dois seguintes. |

Outros campos e objetos aninhados: `kobana schema billing.holidays.sync`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da importação. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## holidays get

Consultar feriado

```text
kobana billing holidays get [ID] [OPÇÕES]
```

`GET /v1/holidays/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Feriado encontrado. |
| `404` | Recurso não encontrado na organização atual. |

## holidays update

Atualizar feriado

```text
kobana billing holidays update [ID] [OPÇÕES]
```

`PATCH /v1/holidays/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `date` | `string` | não | Dia civil do feriado, no formato `YYYY-MM-DD`. Sem hora: um feriado é uma data no calendário, e aceitar um instante faria o dia gravado depender do fuso de quem enviou. |
| `name` | `string` | não | Nome do feriado. Junto com a data, identifica a linha — um feriado municipal e um nacional podem cair no mesmo dia. |
| `type` | `string` | não | Abrangência: nacional, estadual, municipal, ponto facultativo (`optional`) ou próprio da organização (`custom`). Valores: `national`, `state`, `municipal`, `optional`, `custom`. |
| `notes` | `string` | não | Texto livre — a cidade de um feriado municipal, ou qualquer observação. |

Outros campos e objetos aninhados: `kobana schema billing.holidays.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Feriado atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## holidays delete

Remover feriado

```text
kobana billing holidays delete [ID] [OPÇÕES]
```

`DELETE /v1/holidays/{id}`

Remoção lógica. A linha permanece, e é isso que faz a importação respeitar a remoção: um feriado retirado de propósito não volta na próxima sincronização.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Feriado removido. |
| `404` | Recurso não encontrado na organização atual. |
