---
id: pix-keys
title: "kobana billing pix-keys"
sidebar_label: "pix-keys"
---

# kobana billing pix-keys


Comandos de `kobana billing pix-keys`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`pix-keys list`](#pix-keys-list) | `GET /v1/pix_keys` | Listar chaves Pix |
| [`pix-keys create`](#pix-keys-create) | `POST /v1/pix_keys` | Cadastrar chave Pix |
| [`pix-keys get`](#pix-keys-get) | `GET /v1/pix_keys/{id}` | Consultar chave Pix |
| [`pix-keys update`](#pix-keys-update) | `PATCH /v1/pix_keys/{id}` | Atualizar chave Pix |
| [`pix-keys delete`](#pix-keys-delete) | `DELETE /v1/pix_keys/{id}` | Excluir chave Pix |

## pix-keys list

Listar chaves Pix

```text
kobana billing pix-keys list [OPÇÕES]
```

`GET /v1/pix_keys`

Chaves Pix da organização para onde os clientes transferem numa cobrança `bank_transfer` (`POST /invoices/{id}/pay`).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `active` | `string` | não | Filtra por chaves ativas (`true`) ou inativas (`false`). Valores: `true`, `false`. |
| `key_type` | `string` | não | Filtra pelo tipo da chave. Valores: `cpf`, `cnpj`, `email`, `phone`, `random`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de chaves Pix. |
| `404` | Recurso não encontrado na organização atual. |

## pix-keys create

Cadastrar chave Pix

```text
kobana billing pix-keys create [OPÇÕES]
```

`POST /v1/pix_keys`

O tipo é identificado pela própria chave (CPF, CNPJ, e-mail, telefone com +55 ou aleatória); `key_type`, se enviado, precisa bater. A chave é guardada sem máscara e é única na organização.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | sim | A chave, sem máscara (documentos só com dígitos, telefone com `+55`, e-mail e aleatória em minúsculas). |
| `key_type` | `string` | não | Tipo da chave: `cpf`, `cnpj`, `email`, `phone` ou `random`. Valores: `cpf`, `cnpj`, `email`, `phone`, `random`. |
| `description` | `string` | não | Descrição livre para o operador (ex.: "Pix principal"). |
| `holder_name` | `string` | não | Titular mostrado ao cliente. Vazio usa a razão social da empresa emissora da fatura (ou da organização). |
| `holder_document` | `string` | não | CPF/CNPJ do titular, só dígitos. Vazio usa o documento da empresa emissora (ou da organização). |
| `company_id` | `string (uuid)` | não | Empresa emissora a que pertence (opcional). |
| `active` | `boolean` | não | Só destinos ativos podem ser escolhidos numa nova cobrança. |

Outros campos e objetos aninhados: `kobana schema billing.pix-keys.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Chave Pix criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## pix-keys get

Consultar chave Pix

```text
kobana billing pix-keys get [ID] [OPÇÕES]
```

`GET /v1/pix_keys/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Chave Pix encontrada. |
| `404` | Recurso não encontrado na organização atual. |

## pix-keys update

Atualizar chave Pix

```text
kobana billing pix-keys update [ID] [OPÇÕES]
```

`PATCH /v1/pix_keys/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | não | A chave, sem máscara (documentos só com dígitos, telefone com `+55`, e-mail e aleatória em minúsculas). |
| `key_type` | `string` | não | Tipo da chave: `cpf`, `cnpj`, `email`, `phone` ou `random`. Valores: `cpf`, `cnpj`, `email`, `phone`, `random`. |
| `description` | `string` | não | Descrição livre para o operador (ex.: "Pix principal"). |
| `holder_name` | `string` | não | Titular mostrado ao cliente. Vazio usa a razão social da empresa emissora da fatura (ou da organização). |
| `holder_document` | `string` | não | CPF/CNPJ do titular, só dígitos. Vazio usa o documento da empresa emissora (ou da organização). |
| `company_id` | `string (uuid)` | não | Empresa emissora a que pertence (opcional). |
| `active` | `boolean` | não | Só destinos ativos podem ser escolhidos numa nova cobrança. |

Outros campos e objetos aninhados: `kobana schema billing.pix-keys.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Chave Pix atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## pix-keys delete

Excluir chave Pix

```text
kobana billing pix-keys delete [ID] [OPÇÕES]
```

`DELETE /v1/pix_keys/{id}`

Exclusão lógica. Cobranças já enviadas continuam mostrando os dados que o cliente recebeu, e a chave pode ser cadastrada de novo.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Chave Pix excluída. |
| `404` | Recurso não encontrado na organização atual. |
