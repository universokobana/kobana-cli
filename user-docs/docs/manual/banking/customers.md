---
id: customers
title: "kobana banking customers"
sidebar_label: "customers"
---

# kobana banking customers


Comandos de `kobana banking customers`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`customers cnpj-cpf`](#customers-cnpj-cpf) | `GET /v1/customers/cnpj_cpf` | Busca Cliente por CNPJ/CPF |
| [`customers email`](#customers-email) | `GET /v1/customers/email` | Busca Cliente por E-mail |
| [`customers people-sync`](#customers-people-sync) | `PUT /v1/customers/people_sync` | Habilitar ou desabilitar o espelho Cliente &lt;-> Pessoa |
| [`customers list`](#customers-list) | `GET /v1/customers` | Listar Clientes |
| [`customers create`](#customers-create) | `POST /v1/customers` | Criar um Cliente |
| [`customers get`](#customers-get) | `GET /v1/customers/{id}` | Visualizar o Cliente |
| [`customers update`](#customers-update) | `PUT /v1/customers/{id}` | Atualizar Cliente |

## customers cnpj-cpf

Busca Cliente por CNPJ/CPF

```text
kobana banking customers cnpj-cpf [OPÇÕES]
```

`GET /v1/customers/cnpj_cpf`

Visualizar um cliente buscando por cnpj ou cpf.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `q` | `string` | sim | CNPJ/CPF do cliente |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cliente encontrado |
| `404` | Cliente não encontrado |

## customers email

Busca Cliente por E-mail

```text
kobana banking customers email [OPÇÕES]
```

`GET /v1/customers/email`

Visualizar um cliente buscando por e-mail.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `q` | `string` | sim | E-mail do cliente |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cliente encontrado |
| `404` | Cliente não encontrado |

## customers people-sync

Habilitar ou desabilitar o espelho Cliente &lt;-> Pessoa

```text
kobana banking customers people-sync [OPÇÕES]
```

`PUT /v1/customers/people_sync`

Liga ou desliga, para a conta, o espelho automático entre Clientes e Pessoas (cada Cliente ganha uma Pessoa equivalente e alterações se propagam nos dois sentidos). Só altera a configuração: Clientes já existentes não ganham Pessoa até o backfill rodar. Numa subconta, `enabled` na resposta reflete também a herança da conta principal.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `enabled` | `boolean` | sim | true liga o espelho; false desliga. |

Outros campos e objetos aninhados: `kobana schema banking.customers.people-sync`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Configuração atualizada |
| `422` | Parâmetro enabled ausente ou inválido |

## customers list

Listar Clientes

```text
kobana banking customers list [OPÇÕES]
```

`GET /v1/customers`

Lista clientes.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de clientes |

## customers create

Criar um Cliente

```text
kobana banking customers create [OPÇÕES]
```

`POST /v1/customers`

Cria um novo cliente para associação com cobranças, boletos e assinaturas.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_name` | `string` | sim | Nome ou Razão Social |
| `address` | `string` | sim | Endereço |
| `neighborhood` | `string` | sim | Bairro |
| `zipcode` | `string` | sim | CEP (formato 99999999) |
| `city_name` | `string` | sim | Cidade (Nome deve estar correto e completo) |
| `id` | `integer` | não | ID |
| `external_code` | `string` | não | Código Externo |
| `cnpj_cpf` | `string` | não | CPF/CNPJ |
| `contact_person` | `string` | não | Pessoa de Contato (quando Empresa) |
| `nickname` | `string` | não | Apelido ou Nome Fantasia do Pagador |
| `birthday` | `string (date)` | não | Data de Nascimento |
| `email` | `string (email)` | não | E-mail principal |
| `email_cc` | `string (email)` | não | E-mail alternativo |
| `address_complement` | `string` | não | Complemento do Endereço |
| `address_number` | `string` | não | Número |
| `state` | `string` | não | Estado |
| `phone_number` | `string` | não | Telefone (com DDD, DDI é opcional) |
| `mobile_local_code` | `string` | não | DDD do Celular |
| `mobile_number` | `string` | não | Celular |
| `ignore_email` | `boolean` | não | Nunca enviar e-mail para este cliente |
| `ignore_sms` | `boolean` | não | Nunca enviar SMS para este cliente |
| `ignore_whatsapp` | `boolean` | não | Nunca enviar Whatsapp para este cliente |
| `notes` | `string` | não | Observações |
| `tags` | `array<string>` | não | Tags associadas ao registro |
| `tag_list` | `string` | não | Tags associadas ao registro |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `created_via_api` | `boolean` | não | Indica se o registro foi criado por API |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.customers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Cliente criado |
| `422` | Cliente inválido |

## customers get

Visualizar o Cliente

```text
kobana banking customers get [ID] [OPÇÕES]
```

`GET /v1/customers/{id}`

Consulta informações específicas de um Cliente.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cliente encontrado |
| `404` | Cliente não encontrado |

## customers update

Atualizar Cliente

```text
kobana banking customers update [ID] [OPÇÕES]
```

`PUT /v1/customers/{id}`

Atualiza informações específicas de um Cliente.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_name` | `string` | sim | Nome ou Razão Social |
| `address` | `string` | sim | Endereço |
| `neighborhood` | `string` | sim | Bairro |
| `zipcode` | `string` | sim | CEP (formato 99999999) |
| `city_name` | `string` | sim | Cidade (Nome deve estar correto e completo) |
| `id` | `integer` | não | ID |
| `external_code` | `string` | não | Código Externo |
| `contact_person` | `string` | não | Pessoa de Contato (quando Empresa) |
| `nickname` | `string` | não | Apelido ou Nome Fantasia do Pagador |
| `birthday` | `string (date)` | não | Data de Nascimento |
| `email` | `string (email)` | não | E-mail principal |
| `email_cc` | `string (email)` | não | E-mail alternativo |
| `address_complement` | `string` | não | Complemento do Endereço |
| `address_number` | `string` | não | Número |
| `state` | `string` | não | Estado |
| `phone_number` | `string` | não | Telefone (com DDD, DDI é opcional) |
| `mobile_local_code` | `string` | não | DDD do Celular |
| `mobile_number` | `string` | não | Celular |
| `ignore_email` | `boolean` | não | Nunca enviar e-mail para este cliente |
| `ignore_sms` | `boolean` | não | Nunca enviar SMS para este cliente |
| `ignore_whatsapp` | `boolean` | não | Nunca enviar Whatsapp para este cliente |
| `notes` | `string` | não | Observações |
| `tags` | `array<string>` | não | Tags associadas ao registro |
| `tag_list` | `string` | não | Tags associadas ao registro |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `created_via_api` | `boolean` | não | Indica se o registro foi criado por API |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.customers.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Cliente atualizado |
| `404` | Cliente não encontrado |
