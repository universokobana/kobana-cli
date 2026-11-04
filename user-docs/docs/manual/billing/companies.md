---
id: companies
title: "kobana billing companies"
sidebar_label: "companies"
---

# kobana billing companies


Comandos de `kobana billing companies`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`companies fiscal`](#companies-fiscal) | `GET /v1/companies/{id}/fiscal` | Obter perfil fiscal |
| [`companies fiscal update`](#companies-fiscal-update) | `PATCH /v1/companies/{id}/fiscal` | Atualizar perfil fiscal |
| [`companies fiscal-profile`](#companies-fiscal-profile) | `GET /v1/companies/{id}/fiscal-profile` | Obter perfil fiscal (alias) |
| [`companies fiscal-profile update`](#companies-fiscal-profile-update) | `PATCH /v1/companies/{id}/fiscal-profile` | Atualizar perfil fiscal (alias) |
| [`companies matriz`](#companies-matriz) | `GET /v1/companies/matriz` | Listar matrizes |
| [`companies list`](#companies-list) | `GET /v1/companies` | Listar empresas |
| [`companies create`](#companies-create) | `POST /v1/companies` | Criar empresa |
| [`companies update`](#companies-update) | `PUT /v1/companies` | Criar ou atualizar empresa |
| [`companies get`](#companies-get) | `GET /v1/companies/{id}` | Obter empresa |
| [`companies delete`](#companies-delete) | `DELETE /v1/companies/{id}` | Excluir empresa |
| [`companies deactivate`](#companies-deactivate) | `POST /v1/companies/{id}/deactivate` | Desativar empresa |
| [`companies reactivate`](#companies-reactivate) | `POST /v1/companies/{id}/reactivate` | Reativar empresa |
| [`companies set-default`](#companies-set-default) | `POST /v1/companies/{id}/set-default` | Definir empresa padrão |

## companies fiscal

Obter perfil fiscal

```text
kobana billing companies fiscal [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/companies/{id}/fiscal`

Retorna o perfil fiscal da empresa, ou `null` quando ainda não foi configurado.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Perfil fiscal da empresa (ou `null`). |
| `404` | Recurso não encontrado na organização atual. |

## companies fiscal update

Atualizar perfil fiscal

```text
kobana billing companies fiscal update [ID] [OPÇÕES]
```

`PATCH /v1/companies/{id}/fiscal`

Cria ou atualiza (upsert) o perfil fiscal da empresa. Inclui regime tributário, alíquotas (ISS, PIS, COFINS, CSLL, IRPJ) e configurações de CBS/IBS.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `taxation_regime` | `string` | sim | Regime tributário: `simple_national`, `presumed_profit`, `real_profit` ou `mei`. Valores: `simple_national`, `presumed_profit`, `real_profit`, `mei`. |
| `simples_annex` | `string` | não | Anexo do Simples Nacional: `annex_iii`, `annex_iv` ou `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `rbt12` | `number` | não | Receita bruta acumulada nos últimos 12 meses, em reais (float, até 4 casas decimais). |
| `factor_r` | `number` | não | Fator R (folha de pagamento ÷ receita), usado no enquadramento do Simples Nacional. |
| `cpp_inside_das` | `boolean` | não | Indica se a CPP (contribuição previdenciária patronal) está embutida no DAS. |
| `presumption_rate` | `number` | não | Alíquota de presunção (lucro presumido), em decimal (ex.: 0.32). |
| `iss_municipality` | `string` | não | Município de incidência do ISS. |
| `iss_rate` | `number` | não | Alíquota do ISS, em decimal. |
| `pis_rate` | `number` | não | Alíquota do PIS, em decimal. |
| `cofins_rate` | `number` | não | Alíquota da COFINS, em decimal. |
| `csll_rate` | `number` | não | Alíquota da CSLL, em decimal. |
| `irpj_rate` | `number` | não | Alíquota do IRPJ, em decimal. |
| `cbs_ibs_enabled` | `boolean` | não | Habilita os campos CBS/IBS (Reforma Tributária). |
| `cbs_rate` | `number` | não | Alíquota da CBS, em decimal. |
| `ibs_rate` | `number` | não | Alíquota do IBS, em decimal. |
| `cbs_ibs_mode` | `string` | não | Modo de cálculo CBS/IBS: `informative` (apenas demonstrativo) ou `compositive` (compõe o preço). Valores: `informative`, `compositive`. |

Outros campos e objetos aninhados: `kobana schema billing.companies.fiscal.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Perfil fiscal atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## companies fiscal-profile

Obter perfil fiscal (alias)

```text
kobana billing companies fiscal-profile [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/companies/{id}/fiscal-profile`

Alias de `GET /companies/{id}/fiscal` — mesma representação e comportamento.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Perfil fiscal da empresa (ou `null`). |
| `404` | Recurso não encontrado na organização atual. |

## companies fiscal-profile update

Atualizar perfil fiscal (alias)

```text
kobana billing companies fiscal-profile update [ID] [OPÇÕES]
```

`PATCH /v1/companies/{id}/fiscal-profile`

Alias de `PATCH /companies/{id}/fiscal` — mesma representação e comportamento.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `taxation_regime` | `string` | sim | Regime tributário: `simple_national`, `presumed_profit`, `real_profit` ou `mei`. Valores: `simple_national`, `presumed_profit`, `real_profit`, `mei`. |
| `simples_annex` | `string` | não | Anexo do Simples Nacional: `annex_iii`, `annex_iv` ou `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `rbt12` | `number` | não | Receita bruta acumulada nos últimos 12 meses, em reais (float, até 4 casas decimais). |
| `factor_r` | `number` | não | Fator R (folha de pagamento ÷ receita), usado no enquadramento do Simples Nacional. |
| `cpp_inside_das` | `boolean` | não | Indica se a CPP (contribuição previdenciária patronal) está embutida no DAS. |
| `presumption_rate` | `number` | não | Alíquota de presunção (lucro presumido), em decimal (ex.: 0.32). |
| `iss_municipality` | `string` | não | Município de incidência do ISS. |
| `iss_rate` | `number` | não | Alíquota do ISS, em decimal. |
| `pis_rate` | `number` | não | Alíquota do PIS, em decimal. |
| `cofins_rate` | `number` | não | Alíquota da COFINS, em decimal. |
| `csll_rate` | `number` | não | Alíquota da CSLL, em decimal. |
| `irpj_rate` | `number` | não | Alíquota do IRPJ, em decimal. |
| `cbs_ibs_enabled` | `boolean` | não | Habilita os campos CBS/IBS (Reforma Tributária). |
| `cbs_rate` | `number` | não | Alíquota da CBS, em decimal. |
| `ibs_rate` | `number` | não | Alíquota do IBS, em decimal. |
| `cbs_ibs_mode` | `string` | não | Modo de cálculo CBS/IBS: `informative` (apenas demonstrativo) ou `compositive` (compõe o preço). Valores: `informative`, `compositive`. |

Outros campos e objetos aninhados: `kobana schema billing.companies.fiscal-profile.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Perfil fiscal atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## companies matriz

Listar matrizes

```text
kobana billing companies matriz [OPÇÕES]
```

`GET /v1/companies/matriz`

Lista as empresas matrizes (sem `parent_company_id`) ativas da organização. Útil para selecionar a matriz ao criar uma filial. Apenas empresas com CNPJ aparecem aqui.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de empresas matrizes. |
| `404` | Recurso não encontrado na organização atual. |

## companies list

Listar empresas

```text
kobana billing companies list [OPÇÕES]
```

`GET /v1/companies`

Retorna uma lista paginada das empresas (matrizes e filiais) da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `person_id` | `string` | não | Filtra por UUID da pessoa associada. |
| `person_document_number` | `string` | não | Filtra por CPF/CNPJ da pessoa (exato). |
| `person_name_contains` | `string` | não | Filtra por nome da pessoa (busca parcial, sem distinção de maiúsculas). |
| `status` | `string` | não | Filtra por status: `active` ou `inactive`. Valores: `active`, `inactive`. |
| `search` | `string` | não | Busca textual por nome, razão social, e-mail ou documento. |
| `is_default` | `string` | não | Filtra apenas a empresa marcada como padrão da organização. Valores: `true`, `false`. |
| `parent_company_id` | `string` | não | Filtra filiais por matriz. Use o UUID da matriz para listar suas filiais. |
| `email_layout_id` | `string` | não | Filtra empresas pelo UUID do layout de e-mail associado. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de empresas. |
| `404` | Recurso não encontrado na organização atual. |

## companies create

Criar empresa

```text
kobana billing companies create [OPÇÕES]
```

`POST /v1/companies`

Cria uma empresa (matriz ou filial). Para criar uma filial, informe `parent_company_id` — o CNPJ deve compartilhar o mesmo prefixo da matriz. Apenas empresas com CNPJ podem ter filiais. A primeira empresa criada na organização é marcada como `is_default` automaticamente.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome fantasia da empresa. |
| `document_type` | `string` | sim | Tipo de documento: `cpf` ou `cnpj`. Apenas `cnpj` pode ter filiais. Valores: `cpf`, `cnpj`. |
| `document_number` | `string` | sim | CPF ou CNPJ (com ou sem máscara — será normalizado para apenas dígitos). |
| `email` | `string (email)` | sim | E-mail principal da empresa. |
| `parent_company_id` | `string (uuid)` | não | UUID da matriz ao criar uma filial. Omita para criar uma matriz. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `legal_name` | `string` | não | Razão social. |
| `phone` | `string` | não | Telefone principal da empresa. Quando informado, é adicionado ao array `phones`. |
| `addresses` | `array<object>` | não | Lista de endereços da empresa. |
| `emails` | `array<object>` | não | Lista de e-mails adicionais. Se omitida, o `email` principal é usado. |
| `phones` | `array<object>` | não | Lista de telefones da empresa. |
| `websites` | `array<object>` | não | Lista de sites da empresa. |
| `is_default` | `boolean` | não | Quando `true`, marca esta empresa como padrão da organização. Apenas matrizes podem ser padrão. |
| `custom_metadata` | `object` | não | Metadados customizados (chave/valor livre). |

Outros campos e objetos aninhados: `kobana schema billing.companies.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Empresa criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## companies update

Criar ou atualizar empresa

```text
kobana billing companies update [OPÇÕES]
```

`PUT /v1/companies`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `parent_company_id` | `string (uuid)` | não | UUID da matriz ao criar uma filial. Omita para criar uma matriz. |
| `name` | `string` | não | Nome fantasia da empresa. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `legal_name` | `string` | não | Razão social. |
| `document_type` | `string` | não | Tipo de documento: `cpf` ou `cnpj`. Apenas `cnpj` pode ter filiais. Valores: `cpf`, `cnpj`. |
| `document_number` | `string` | não | CPF ou CNPJ (com ou sem máscara — será normalizado para apenas dígitos). |
| `email` | `string (email)` | não | E-mail principal da empresa. |
| `phone` | `string` | não | Telefone principal da empresa. Quando informado, é adicionado ao array `phones`. |
| `addresses` | `array<object>` | não | Lista de endereços da empresa. |
| `emails` | `array<object>` | não | Lista de e-mails adicionais. Se omitida, o `email` principal é usado. |
| `phones` | `array<object>` | não | Lista de telefones da empresa. |
| `websites` | `array<object>` | não | Lista de sites da empresa. |
| `is_default` | `boolean` | não | Quando `true`, marca esta empresa como padrão da organização. Apenas matrizes podem ser padrão. |
| `custom_metadata` | `object` | não | Metadados customizados (chave/valor livre). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.companies.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## companies get

Obter empresa

```text
kobana billing companies get [ID] [OPÇÕES]
```

`GET /v1/companies/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da empresa. |
| `404` | Recurso não encontrado na organização atual. |

## companies delete

Excluir empresa

```text
kobana billing companies delete [ID] [OPÇÕES]
```

`DELETE /v1/companies/{id}`

Exclui a empresa (soft delete: `deleted_at`, ela some de toda leitura; faturas e NFS-e emitidas seguem citando-a). Exige o nível de acesso `delete`. Não é possível excluir a única empresa ativa nem uma empresa com assinaturas ativas ou em período de teste. Até outubro de 2026 esta rota só desativava a empresa — desativar é `POST /v1/companies/{id}/deactivate`, reversível.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Empresa excluída. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## companies deactivate

Desativar empresa

```text
kobana billing companies deactivate [ID] [OPÇÕES]
```

`POST /v1/companies/{id}/deactivate`

Desativa a empresa. Não é possível desativar a única empresa ativa, nem uma empresa com assinaturas ativas ou em período de teste. Se a empresa desativada era a padrão, outra empresa ativa é promovida automaticamente.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Empresa desativada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## companies reactivate

Reativar empresa

```text
kobana billing companies reactivate [ID] [OPÇÕES]
```

`POST /v1/companies/{id}/reactivate`

Reativa uma empresa previamente desativada. Retorna **422** quando a empresa já está ativa.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Empresa reativada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## companies set-default

Definir empresa padrão

```text
kobana billing companies set-default [ID] [OPÇÕES]
```

`POST /v1/companies/{id}/set-default`

Marca a empresa como padrão da organização. A empresa padrão anterior é desmarcada na mesma transação.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Empresa definida como padrão. |
| `404` | Recurso não encontrado na organização atual. |
