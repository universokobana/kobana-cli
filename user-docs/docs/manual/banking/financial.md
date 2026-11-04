---
id: financial
title: "kobana banking financial"
sidebar_label: "financial"
description: "Financeiro (contas, saldos, extratos)"
---

# kobana banking financial

Financeiro (contas, saldos, extratos)

Comandos de `kobana banking financial`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`financial accounts balances list`](#financial-accounts-balances-list) | `GET /v2/financial/accounts/{financial_account_uid}/balances` | Listar saldos |
| [`financial accounts balances create`](#financial-accounts-balances-create) | `POST /v2/financial/accounts/{financial_account_uid}/balances` | Criar um saldo |
| [`financial accounts balances get`](#financial-accounts-balances-get) | `GET /v2/financial/accounts/{financial_account_uid}/balances/{balance_uid}` | Visualizar um Saldo |
| [`financial accounts commands get`](#financial-accounts-commands-get) | `GET /v2/financial/accounts/{financial_account_uid}/commands/{uid}` | Visualizar um Comando |
| [`financial accounts commands list`](#financial-accounts-commands-list) | `GET /v2/financial/accounts/{financial_account_uid}/commands` | Listar Comandos de uma Conta |
| [`financial accounts statement-transactions imports list`](#financial-accounts-statement-transactions-imports-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports` | Listar Importações de extrato |
| [`financial accounts statement-transactions imports create`](#financial-accounts-statement-transactions-imports-create) | `POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports` | Importar Extrato |
| [`financial accounts statement-transactions imports get`](#financial-accounts-statement-transactions-imports-get) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports/{uid}` | Visualizar Importação de Extrato |
| [`financial accounts statement-transactions summary`](#financial-accounts-statement-transactions-summary) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/summary` | Resumir Extrato |
| [`financial accounts statement-transactions syncs list`](#financial-accounts-statement-transactions-syncs-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs` | Listar Sincronizações de Extrato |
| [`financial accounts statement-transactions syncs get`](#financial-accounts-statement-transactions-syncs-get) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs/{uid}` | Consultar Sincronização de Extrato |
| [`financial accounts statement-transactions sync`](#financial-accounts-statement-transactions-sync) | `POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/sync` | Sincronizar Extrato |
| [`financial accounts statement-transactions list`](#financial-accounts-statement-transactions-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions` | Listar Transações |
| [`financial accounts list`](#financial-accounts-list) | `GET /v2/financial/accounts` | Listar Contas |
| [`financial accounts create`](#financial-accounts-create) | `POST /v2/financial/accounts` | Criar uma Conta |
| [`financial accounts get`](#financial-accounts-get) | `GET /v2/financial/accounts/{uid}` | Visualizar uma Conta |
| [`financial accounts update`](#financial-accounts-update) | `PUT /v2/financial/accounts/{uid}` | Atualizar Conta |
| [`financial accounts delete`](#financial-accounts-delete) | `DELETE /v2/financial/accounts/{uid}` | Excluir uma Conta |
| [`financial companies list`](#financial-companies-list) | `GET /v2/financial/companies` | Listar Empresas |
| [`financial companies create`](#financial-companies-create) | `POST /v2/financial/companies` | Criar uma Empresa |
| [`financial companies get`](#financial-companies-get) | `GET /v2/financial/companies/{uid}` | Consultar uma Empresa |
| [`financial companies update`](#financial-companies-update) | `PUT /v2/financial/companies/{uid}` | Atualizar uma Empresa |
| [`financial companies delete`](#financial-companies-delete) | `DELETE /v2/financial/companies/{uid}` | Excluir uma Empresa |
| [`financial providers`](#financial-providers) | `GET /v2/financial/providers` | Listar Provedores |
| [`financial statement-categories`](#financial-statement-categories) | `GET /v2/financial/statement_categories` | Listar Categorias de Extrato |

## financial accounts balances list

Listar saldos

```text
kobana banking financial accounts balances list [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/balances`

Lista todos os saldos de uma conta financeira específica. Retorna informações sobre valores disponíveis, bloqueados e investidos automaticamente.

**Argumentos do caminho:** `financial_account_uid`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Saldos encontrados |

## financial accounts balances create

Criar um saldo

```text
kobana banking financial accounts balances create [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`POST /v2/financial/accounts/{financial_account_uid}/balances`

Cria um novo saldo manual para uma conta financeira. Permite registrar saldos iniciais ou ajustes manuais de conciliação.

**Argumentos do caminho:** `financial_account_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `uid` | `string (uuid)` | não | UID do Saldo |
| `amount` | `number` | não | Valor do saldo |
| `blocked_amount` | `number` | não | Valor bloqueado |
| `automatically_invested_amount` | `number` | não | Valor automaticamente investido |
| `created_at` | `string (date-time)` | não | Data da criação. Formato iso8601 |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. Opcional para controle interno. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `tags` | `array<string>` | não | Tags associadas a importação. Opcional para controle interno. |

Outros campos e objetos aninhados: `kobana schema banking.financial.accounts.balances.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Saldo criado |

## financial accounts balances get

Visualizar um Saldo

```text
kobana banking financial accounts balances get [FINANCIAL_ACCOUNT_UID] [BALANCE_UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/balances/{balance_uid}`

Consulta informações específicas de uma Conta.

**Argumentos do caminho:** `financial_account_uid`, `balance_uid`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `field` | `string` | não | Campo para buscar o ID. Exemplo: `uid` ou `external_id` |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta encontrada |
| `404` | Conta não encontrada |

## financial accounts commands get

Visualizar um Comando

```text
kobana banking financial accounts commands get [FINANCIAL_ACCOUNT_UID] [UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/commands/{uid}`

Consulta informações detalhadas de um comando específico executado em uma conta financeira.

**Argumentos do caminho:** `financial_account_uid`, `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando encontrada |
| `404` | Comando não encontrada |

## financial accounts commands list

Listar Comandos de uma Conta

```text
kobana banking financial accounts commands list [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/commands`

Lista todos os comandos executados em uma conta financeira. Comandos representam operações assíncronas como sincronização de extrato e outras ações automatizadas.

**Argumentos do caminho:** `financial_account_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comandos encontrado |
| `404` | Conta não encontrada |

## financial accounts statement-transactions imports list

Listar Importações de extrato

```text
kobana banking financial accounts statement-transactions imports list [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports`

Lista todas as importações de extrato realizadas para uma conta financeira. Permite filtrar por status, data de processamento, tags e nome do arquivo importado.

**Argumentos do caminho:** `financial_account_uid`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `status` | `string` | não | Filtro por status da importação |
| `source_file_name` | `string` | não | Nome do arquivo importado |
| `processed_from` | `string` | não | Data inicial do filtro de data de processamento da importação |
| `processed_to` | `string` | não | Data final do filtro de data de processamento da importação |
| `created_from` | `string` | não | Data inicial do filtro de data de criação da importação |
| `created_to` | `string` | não | Data final do filtro de data de criação da importação |
| `tags` | `string` | não | Filtro por tags da importação |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Importações de Extrato encontradas |

## financial accounts statement-transactions imports create

Importar Extrato

```text
kobana banking financial accounts statement-transactions imports create [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports`

Importa um extrato bancário através de arquivo CNAB. O processamento é realizado de forma assíncrona e as transações são associadas à conta financeira.
> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de atualização para a Conta Financeira.

A Importação de Extrato de uma Conta Financeira é feita assincronamente, ou seja, ao receber a requisição,
a nossa API guarda os parâmetros da requisição e processa a importação em segundo plano.
Após a importação ser processada, as transações serão associadas à Conta Financeira.
Você recebe na resposta desta requisição o UID da Importação criada e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `import.created` é disparado.
O nosso servidor então começa o processamento da importação.
Após o fim do processamento, o evento `import.processed` é disparado.

**Argumentos do caminho:** `financial_account_uid`

**Corpo da requisição**

Enviado como formulário `multipart/form-data`: os arquivos vão em `--file CAMPO=CAMINHO`.

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `source` | `string (binary)` | sim | Arquivo para importação. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |

Outros campos e objetos aninhados: `kobana schema banking.financial.accounts.statement-transactions.imports.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Importação criada |

## financial accounts statement-transactions imports get

Visualizar Importação de Extrato

```text
kobana banking financial accounts statement-transactions imports get [FINANCIAL_ACCOUNT_UID] [UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports/{uid}`

Consulta informações específicas de uma Importação de Extrato.

**Argumentos do caminho:** `financial_account_uid`, `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Importação de Extrato encontrada |
| `404` | Conta não encontrada |

## financial accounts statement-transactions summary

Resumir Extrato

```text
kobana banking financial accounts statement-transactions summary [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/summary`

Agrega as transações do extrato em totais por grupo, em vez de devolver uma linha por transação.

Responde perguntas como "quanto gastei com tarifas em julho?" numa única requisição, com a
aritmética feita no banco — sem paginar o extrato inteiro.

> **A janela aqui pode chegar a 730 dias**, contra os 180 da listagem. O tamanho da resposta é
> limitado pela quantidade de grupos, não pelo histórico, então uma janela longa é segura.

Aceita os mesmos filtros da listagem. Se a combinação de `group_by` e janela gerar mais de
1000 grupos, a requisição é recusada com `422` em vez de devolver um resumo truncado — um total
cortado pela metade parece completo e engana silenciosamente.

A resposta também traz os blocos `filters` e `meta`, idênticos aos da listagem, com o acréscimo
de `filters.group_by` informando o agrupamento aplicado.

**Argumentos do caminho:** `financial_account_uid`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `group_by` | `string` | não | Dimensões de agrupamento separadas por vírgula: `day`, `week`, `month`, `kind`, `category`. Padrão: `month`. |
| `occurrence_date_from` | `string` | não | Data inicial da janela. Mesma regra de derivação da listagem, com limite de 730 dias. |
| `occurrence_date_to` | `string` | não | Data final da janela. |
| `kind` | `string` | não | Mesmo filtro da listagem. Valores: `credit`, `debit`. |
| `category` | `string` | não | Mesmo filtro da listagem. |
| `q` | `string` | não | Mesmo filtro da listagem. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resumo calculado |

## financial accounts statement-transactions syncs list

Listar Sincronizações de Extrato

```text
kobana banking financial accounts statement-transactions syncs list [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs`

Lista as sincronizações de extrato solicitadas para uma Conta Financeira, da mais recente para a mais antiga.

Cada item representa uma solicitação feita em
Sincronizar Extrato
e agrega os comandos criados para a janela pedida — janelas que cruzam mais de um mês geram um comando por mês.

O campo `commands_summary` traz a contagem de comandos por status, o que permite identificar quantos meses
falharam sem precisar abrir a solicitação. Para ver os comandos individualmente, use a consulta por `uid`.

> Apenas sincronizações solicitadas via API aparecem aqui. As sincronizações automáticas periódicas não geram registro.

**Argumentos do caminho:** `financial_account_uid`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `status` | `string` | não | Filtra por status agregado da sincronização. Valores: `pending`, `processing`, `done`, `partial`, `failed`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sincronizações encontradas |

## financial accounts statement-transactions syncs get

Consultar Sincronização de Extrato

```text
kobana banking financial accounts statement-transactions syncs get [FINANCIAL_ACCOUNT_UID] [UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs/{uid}`

Consulta o status de uma sincronização de extrato solicitada.

O status é recalculado a cada consulta a partir dos comandos da solicitação:

* `pending` - nenhum comando começou a ser processado
* `processing` - em andamento (parte dos comandos já terminou)
* `done` - todos os comandos foram confirmados e as transações já estão disponíveis em
  Listar Transações
* `partial` - todos terminaram, mas parte falhou. O array `commands` mostra qual janela falhou e por quê
* `failed` - todos terminaram e nenhum foi confirmado

O array `commands` traz cada comando com sua própria janela em `params.start_at`/`params.end_at` e a
mensagem de erro em `error_message`, quando houver. É por ele que se identifica **qual mês** falhou numa
sincronização multi-mês.

**Argumentos do caminho:** `financial_account_uid`, `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sincronização encontrada |
| `404` | Sincronização não encontrada |

## financial accounts statement-transactions sync

Sincronizar Extrato

```text
kobana banking financial accounts statement-transactions sync [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/sync`

Sincroniza o extrato bancário de uma conta financeira. Busca transações diretamente da instituição financeira através de API. Requer conexão ativa.
> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de sincronização de extrato para a Conta Financeira.

A Sincronização de Extrato de uma Conta Financeira é feita assincronamente, ou seja, ao receber a requisição, a nossa
API guarda os parametros da requisição e cria um Comando. Esse comando é processado em sgundo plano e depois é confirmado
e as transações serão importadas na Conta Financeira. Você recebe na resposta desta requisição o ID do comando criado
e pode consultá-lo a qualquer momento.

Veja mais em Listar Comandos de uma Conta Finaneira e
Visualizar um Comando.

Para que seja possível executar a Sincronização de Extrato, deve haver uma Conexão ativa (ou seja, validada e habilitada)
associada à Conta Financeira.

**Período da Sincronização**

Os campos `start_at` e `end_at` são opcionais e delimitam a janela a sincronizar.
Quando omitidos, a janela padrão vai do **primeiro dia do mês corrente até hoje**.

* informando apenas `start_at`, a data final assumida é hoje;
* informando apenas `end_at`, a data inicial assumida é o primeiro dia do mês de `end_at`;
* a data final não pode ser futura;
* o período máximo é de **2 anos** para trás a partir de hoje.

Janelas que cruzam mais de um mês-calendário são **fatiadas em um comando por mês**. Isso mantém cada
requisição à instituição financeira dentro de um volume que o banco consegue responder, e permite
reprocessar apenas o mês que eventualmente falhar. Uma janela dentro de um mesmo mês continua criando
um único comando.

**Acompanhando a Importação**

Além do comando em `data`, a resposta traz em `sync` o registro da importação solicitada, com um `uid`
próprio. Use esse `uid` para acompanhar o conjunto inteiro — especialmente em janelas multi-mês, onde
vários comandos são criados — em
Consultar Sincronização de Extrato.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `financial.account.statement_sync.requested` é disparado.
O nosso servidor então começa a comunicação com a API da instituição financeira para buscar transações.
Somente depois de confirmado na instituição financeira, aí então o extrato é atualizado.
Neste momento o evento `financial.account.statement_sync.confirmed` é disparado.

**Argumentos do caminho:** `financial_account_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `start_at` | `string (date)` | não | Data inicial da sincronização. Padrão: primeiro dia do mês corrente. Máximo de 2 anos atrás. |
| `end_at` | `string (date)` | não | Data final da sincronização. Padrão: hoje. Não pode ser futura. |

Outros campos e objetos aninhados: `kobana schema banking.financial.accounts.statement-transactions.sync`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Comando criado por mês para janela multi-mês |
| `422` | Conexão para Extrato não ativa |

## financial accounts statement-transactions list

Listar Transações

```text
kobana banking financial accounts statement-transactions list [FINANCIAL_ACCOUNT_UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions`

Lista as transações do extrato de uma conta financeira, filtrando por período, tipo, valor e descrição.

**Janela de datas — mudança agendada**

Hoje, uma requisição sem datas devolve o extrato inteiro da conta. Esse comportamento
**vai mudar em 05/02/2027**: a consulta passará a rodar sempre dentro de uma janela de no
máximo 180 dias, o que é o que mantém o endpoint rápido em contas com histórico longo.

Você pode adotar o comportamento novo desde já enviando `date_window=strict`, e deve fazê-lo
antes da data de corte. Com ele:

| Parâmetros enviados | Janela consultada |
|---|---|
| nenhum | Os últimos 180 dias |
| só `occurrence_date_from` | 180 dias a partir dela |
| só `occurrence_date_to` | 180 dias antes dela |
| ambos | O período informado, desde que não passe de 180 dias |

Um intervalo maior que 180 dias é recusado com `422` em vez de ser reduzido silenciosamente —
um extrato parcial com aparência de completo é pior do que um erro. Para períodos mais longos,
faça chamadas sucessivas de até 180 dias cada.

Enquanto `date_window=strict` não for enviado, nada muda. Requisições cujo resultado **seria
diferente** sob a janela estrita respondem com os cabeçalhos `Deprecation: true` e
`Sunset: Fri, 05 Feb 2027 00:00:00 GMT` — use-os para descobrir quais das suas integrações
precisam de ajuste.

**Argumentos do caminho:** `financial_account_uid`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página. Valores acima do máximo são reduzidos ao máximo. |
| `date_window` | `string` | não | Envie `strict` para adotar desde já a janela de 180 dias que passa a ser o padrão em 05/02/2027. Sem ele, o comportamento atual é preservado. Valores: `strict`. |
| `occurrence_date_from` | `string` | não | Data inicial do filtro de data de ocorrência. Com `date_window=strict`, se omitida é derivada de `occurrence_date_to` (180 dias antes) ou, na ausência das duas, de hoje. |
| `occurrence_date_to` | `string` | não | Data final do filtro de data de ocorrência. Com `date_window=strict`, se omitida é derivada de `occurrence_date_from` (180 dias depois) ou, na ausência das duas, é hoje. |
| `kind` | `string` | não | Filtra pelo tipo da transação. Valores: `credit`, `debit`. |
| `amount_from` | `number` | não | Valor mínimo da transação, em reais e com sinal — débitos são negativos, assim como no campo `amount` da resposta. Use ponto como separador decimal. |
| `amount_to` | `number` | não | Valor máximo da transação, em reais e com sinal — débitos são negativos, assim como no campo `amount` da resposta. Use ponto como separador decimal. |
| `amount_abs_from` | `number` | não | Valor mínimo em módulo, ignorando o sinal. Use para "acima de X, entrada ou saída". |
| `amount_abs_to` | `number` | não | Valor máximo em módulo, ignorando o sinal. |
| `category` | `string` | não | Código de categoria, ou lista separada por vírgula. Consulte o catálogo em `GET /v2/financial/statement_categories`. Códigos desconhecidos são ignorados. |
| `updated_since` | `string` | não | Retorna apenas transações alteradas a partir deste instante (ISO 8601). Serve para sincronização incremental. |
| `q` | `string` | não | Busca textual na descrição da transação enviada pelo banco (`description`). Casa qualquer trecho do texto, sem diferenciar maiúsculas, minúsculas ou acentos — `manutencao` encontra `MANUTENÇÃO`. |
| `view` | `string` | não | Use `compact` para receber apenas os campos essenciais de cada transação. Valores: `compact`. |
| `fields` | `string` | não | Lista de campos separados por vírgula a retornar em cada transação. Tem precedência sobre `view`. Um campo desconhecido resulta em `422`. |
| `pagination` | `string` | não | Use `keyset` para paginação por cursor, estável quando novas transações chegam durante a varredura. A resposta passa a trazer `next_cursor` no lugar das URLs. Valores: `keyset`. |
| `cursor` | `string` | não | Cursor devolvido em `pagination.next_cursor`. Só tem efeito com `pagination=keyset`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Transações encontradas |
| `422` | Intervalo de datas maior que 180 dias (com date_window=strict) |

## financial accounts list

Listar Contas

```text
kobana banking financial accounts list [OPÇÕES]
```

`GET /v2/financial/accounts`

Lista todas as contas financeiras cadastradas. Retorna informações sobre as contas bancárias incluindo provedor, tipo de conta, dados bancários e metadados personalizados.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Contas encontradas |

## financial accounts create

Criar uma Conta

```text
kobana banking financial accounts create [OPÇÕES]
```

`POST /v2/financial/accounts`

Cria uma nova conta financeira para gerenciamento de transações bancárias. Suporta contas correntes e poupança.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `account_number` | `string` | sim | Número da Conta |
| `account_digit` | `string` | sim | Dígito da Conta |
| `agency_number` | `string` | sim | Número da Agência |
| `financial_provider_slug` | `string` | sim | Provedor Financeiro |
| `person_info` | `object` | sim | Dados do Titular |
| `uid` | `string (uuid)` | não | UID da Conta Financeira |
| `kind` | `string` | não | Tipo da Conta `checking`: Conta Corrente `savings`: Conta Poupança `payment`: Conta Pagamento |
| `agency_digit` | `string` | não | Dígito da Agência. Opcional: será calculado automaticamente usando Módulo 11 quando o banco exigir e não for fornecido. |
| `bank_number` | `integer` | não | Número COMPE do Banco. Obrigatório se bank_id não for informado. |
| `created_at` | `string (date-time)` | não | Data da criação. Formato iso8601 |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. Opcional para controle interno. |
| `custom_name` | `string` | não | Nome da Conta Financeira para identificação dentro da Kobana. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `company_uid` | `string (uuid)` | não | UID da Empresa (`/v2/financial/companies`) à qual a conta pertence. UID inexistente, de outra conta ou malformado é recusado com 422 (`unresolvable_uid`). `null` ou vazio desliga a conta da empresa; sem o campo, a empresa não muda. |
| `payment_agreement_code` | `string` | não | Número do Convênio de Pagamento contratado junto ao Banco. |
| `tags` | `array<string>` | não | Tags associadas a importação. Opcional para controle interno. |
| `updated_at` | `string (date-time)` | não | Data da última atualização. Formato iso8601 |
| `service_accounts` | `array<object>` | não | Contas de serviço associadas a esta conta financeira. |

Outros campos e objetos aninhados: `kobana schema banking.financial.accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conta criada |
| `422` | Erro de validação |

## financial accounts get

Visualizar uma Conta

```text
kobana banking financial accounts get [UID] [OPÇÕES]
```

`GET /v2/financial/accounts/{uid}`

Consulta informações específicas de uma Conta.

**Argumentos do caminho:** `uid`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `field` | `string` | não | Campo para buscar o ID. Exemplo: `uid` ou `external_id` |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta encontrada |
| `404` | Conta não encontrada |

## financial accounts update

Atualizar Conta

```text
kobana banking financial accounts update [UID] [OPÇÕES]
```

`PUT /v2/financial/accounts/{uid}`

Atualiza uma Conta Financeira específica.

**Argumentos do caminho:** `uid`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `field` | `string` | não | Campo para buscar o ID. Exemplo: `uid` ou `external_id` |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `kind` | `string` | não | Tipo da Conta `checking`: Conta Corrente `savings`: Conta Poupança `payment`: Conta Pagamento |
| `account_number` | `string` | não | Número da Conta |
| `account_digit` | `string` | não | Dígito da Conta |
| `agency_number` | `string` | não | Número da Agência |
| `agency_digit` | `string` | não | Dígito da Agência. Opcional: será calculado automaticamente usando Módulo 11 quando o banco exigir e não for fornecido. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. Opcional para controle interno. |
| `custom_name` | `string` | não | Nome da Conta Financeira para identificação dentro da Kobana. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `company_uid` | `string (uuid)` | não | UID da Empresa (`/v2/financial/companies`) à qual a conta pertence. UID inexistente, de outra conta ou malformado é recusado com 422 (`unresolvable_uid`). `null` ou vazio desliga a conta da empresa; sem o campo, a empresa não muda. |
| `payment_agreement_code` | `string` | não | Número do Convênio de Pagamento contratado junto ao Banco. |
| `person_info` | `object` | não | Dados do Titular |
| `tags` | `array<string>` | não | Tags associadas a importação. Opcional para controle interno. |

Outros campos e objetos aninhados: `kobana schema banking.financial.accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta Financeira atualizada |
| `404` | Conta Financeira não encontrada |
| `422` | Falha na validação |

## financial accounts delete

Excluir uma Conta

```text
kobana banking financial accounts delete [UID] [OPÇÕES]
```

`DELETE /v2/financial/accounts/{uid}`

Exclui uma Conta Financeira. A exclusão é processada de forma assíncrona.

⚠️ Atenção: esta operação remove em cascata todos os registros
associados à Conta Financeira, incluindo carteiras de boleto,
transferências, pagamentos, extratos e demais recursos vinculados.
Essa ação não pode ser desfeita.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Conta enviada para exclusão |
| `404` | Conta Financeira não encontrada |

## financial companies list

Listar Empresas

```text
kobana banking financial companies list [OPÇÕES]
```

`GET /v2/financial/companies`

Lista as empresas (CNPJs) da conta, as mais recentes primeiro.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cnpj` | `string` | não | Filtra pelo CNPJ (com ou sem máscara). |
| `active` | `boolean` | não | Filtra por empresas ativas ou inativas. |
| `external_id` | `string` | não | Filtra pelo ID no seu sistema. |
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Empresas encontradas |

## financial companies create

Criar uma Empresa

```text
kobana banking financial companies create [OPÇÕES]
```

`POST /v2/financial/companies`

Cria uma empresa (CNPJ) na conta. Contatos no formato de Pessoa: listas de endereços, e-mails e telefones.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Razão social. |
| `cnpj` | `string` | sim | CNPJ sem máscara, em maiúsculas (aceita o CNPJ alfanumérico). |
| `trade_name` | `string` | não | Nome fantasia. |
| `active` | `boolean` | não | Empresa ativa. Só uma empresa ativa por CNPJ na conta. |
| `municipal_registration` | `string` | não | Inscrição municipal. |
| `tax_regime` | `string` | não | Regime tributário. Valores: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`, `null`. |
| `addresses` | `array<object>` | não |  |
| `emails` | `array<object>` | não |  |
| `phones` | `array<object>` | não |  |
| `external_id` | `string` | não | ID no seu sistema. Único por conta. |
| `tags` | `array<string>` | não |  |
| `custom_data` | `object` | não | Dados personalizados. |

Outros campos e objetos aninhados: `kobana schema banking.financial.companies.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Empresa criada |
| `422` | Falha na validação |

## financial companies get

Consultar uma Empresa

```text
kobana banking financial companies get [UID] [OPÇÕES]
```

`GET /v2/financial/companies/{uid}`

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Empresa encontrada |
| `404` | Empresa não encontrada |

## financial companies update

Atualizar uma Empresa

```text
kobana banking financial companies update [UID] [OPÇÕES]
```

`PUT /v2/financial/companies/{uid}`

Atualiza a empresa. As contas financeiras dela passam a ter o titular com os dados da empresa.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Razão social. |
| `trade_name` | `string` | não | Nome fantasia. |
| `cnpj` | `string` | não | CNPJ sem máscara, em maiúsculas (aceita o CNPJ alfanumérico). |
| `active` | `boolean` | não | Empresa ativa. Só uma empresa ativa por CNPJ na conta. |
| `municipal_registration` | `string` | não | Inscrição municipal. |
| `tax_regime` | `string` | não | Regime tributário. Valores: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`, `null`. |
| `addresses` | `array<object>` | não |  |
| `emails` | `array<object>` | não |  |
| `phones` | `array<object>` | não |  |
| `external_id` | `string` | não | ID no seu sistema. Único por conta. |
| `tags` | `array<string>` | não |  |
| `custom_data` | `object` | não | Dados personalizados. |

Outros campos e objetos aninhados: `kobana schema banking.financial.companies.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Empresa atualizada |

## financial companies delete

Excluir uma Empresa

```text
kobana banking financial companies delete [UID] [OPÇÕES]
```

`DELETE /v2/financial/companies/{uid}`

Exclui a empresa. As contas financeiras dela continuam, sem empresa.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Empresa excluída |

## financial providers

Listar Provedores

```text
kobana banking financial providers [OPÇÕES]
```

`GET /v2/financial/providers`

Esse endpoint é utilizado para receber os provedores de serviço que a Kobana está integrada.

Um provedor de serviço é qualquer instituição regulada pelo BACEN, seja um banco comercial, instituição financeira, instituição de pagamento, provedor de serviço de pagamento, etc.

O `slug` do provedor de serviço é utilizado para a criação de uma conta financeira.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Provedores encontradas |

## financial statement-categories

Listar Categorias de Extrato

```text
kobana banking financial statement-categories [OPÇÕES]
```

`GET /v2/financial/statement_categories`

Catálogo das categorias que uma transação de extrato pode carregar.

Necessário para usar o filtro `category` da listagem e do resumo: os códigos são um domínio
fechado, e sem o catálogo não há como descobrir que tarifas bancárias são o código `105`.

O catálogo tem duas naturezas: cerca de 50 códigos genéricos (`1xx` débitos, `2xx` créditos,
e `9999` para `outros`) e centenas de códigos específicos de banco, no formato
`1 + numero_do_banco + codigo_do_banco` (ex.: `107701`).

`kind` é derivado do slug e vem nulo quando a categoria não declara um lado — o que é o caso
da maioria dos códigos específicos de banco. Ele é uma conveniência, não uma classificação
exaustiva: para somar entradas e saídas, use o filtro `kind` da listagem ou o `group_by=kind`
do resumo, que operam sobre o tipo real da transação.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Catálogo de categorias |
