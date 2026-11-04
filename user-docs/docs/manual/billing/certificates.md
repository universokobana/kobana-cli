---
id: certificates
title: "kobana billing certificates"
sidebar_label: "certificates"
---

# kobana billing certificates


Comandos de `kobana billing certificates`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`certificates list`](#certificates-list) | `GET /v1/certificates` | Listar certificados digitais |
| [`certificates create`](#certificates-create) | `POST /v1/certificates` | Enviar certificado digital |
| [`certificates get`](#certificates-get) | `GET /v1/certificates/{id}` | Obter certificado |
| [`certificates delete`](#certificates-delete) | `DELETE /v1/certificates/{id}` | Remover certificado |

## certificates list

Listar certificados digitais

```text
kobana billing certificates list [OPÇÕES]
```

`GET /v1/certificates`

Retorna uma lista paginada dos certificados digitais (A1) da organização atual. Por segurança, a resposta **nunca** inclui a chave privada nem a senha do PFX — apenas metadados extraídos do certificado.

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
| `status` | `string` | não | Filtra por status do certificado. `expiring_soon` retorna certificados ativos que vencem nos próximos 30 dias. Valores: `active`, `expired`, `revoked`, `expiring_soon`. |
| `search` | `string` | não | Busca textual em `name`, `common_name`, `document` ou `organization_name`. |
| `format` | `string` | não | Filtra pelo formato do certificado armazenado (`pfx`, `pem`). |
| `document` | `string` | não | Filtra pelo CPF ou CNPJ extraído do certificado (apenas dígitos). |
| `serial_number` | `string` | não | Filtra pelo número de série do certificado. |
| `not_after_from` | `string` | não | Retorna certificados cuja data de expiração (`not_after`) é maior ou igual à data informada (ISO 8601). |
| `not_after_to` | `string` | não | Retorna certificados cuja data de expiração (`not_after`) é menor ou igual à data informada (ISO 8601). |
| `not_before_from` | `string` | não | Retorna certificados cuja data de início de validade (`not_before`) é maior ou igual à data informada (ISO 8601). |
| `not_before_to` | `string` | não | Retorna certificados cuja data de início de validade (`not_before`) é menor ou igual à data informada (ISO 8601). |
| `created_after` | `string` | não | Retorna certificados criados a partir da data informada (ISO 8601). |
| `created_before` | `string` | não | Retorna certificados criados até a data informada (ISO 8601). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de certificados. |
| `404` | Recurso não encontrado na organização atual. |

## certificates create

Enviar certificado digital

```text
kobana billing certificates create [OPÇÕES]
```

`POST /v1/certificates`

Envia um certificado digital A1 via `multipart/form-data`. A chave privada e a senha são criptografadas em repouso e **nunca** retornadas pela API. O endpoint detecta duplicidade pelo `thumbprint` e associa automaticamente a um `person` pelo CNPJ/CPF extraído.

Valores aceitos para `type`:
- `pfx` — envia `file` (PFX/P12 binário) e `password`.
- `cert_key` — envia `cert_file` (PEM do certificado) e `key_file` (PEM da chave).
- `combined_pem` — envia `file` (PEM contendo certificado + chave).
- `cert_key_password` — envia `cert_file`, `key_file` e `password` (chave protegida por senha).

**Corpo da requisição**

Enviado como formulário `multipart/form-data`: os arquivos vão em `--file CAMPO=CAMINHO`.

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `type` | `string` | sim | Formato do envio: `pfx`, `cert_key`, `combined_pem` ou `cert_key_password`. Valores: `pfx`, `cert_key`, `combined_pem`, `cert_key_password`. |
| `name` | `string` | não | Nome amigável do certificado. Quando omitido, é derivado do `common_name` ou do documento. |
| `file` | `string (binary)` | não | Arquivo binário (PFX/P12) quando `type=pfx`, ou PEM combinado quando `type=combined_pem`. |
| `cert_file` | `string (binary)` | não | Arquivo PEM do certificado (somente para `type=cert_key` ou `type=cert_key_password`). |
| `key_file` | `string (binary)` | não | Arquivo PEM da chave privada (somente para `type=cert_key` ou `type=cert_key_password`). |
| `password` | `string` | não | Senha do PFX (`type=pfx`) ou da chave privada (`type=cert_key_password`). Armazenada criptografada e nunca retornada. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |

Outros campos e objetos aninhados: `kobana schema billing.certificates.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Certificado enviado e armazenado com sucesso. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## certificates get

Obter certificado

```text
kobana billing certificates get [ID] [OPÇÕES]
```

`GET /v1/certificates/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do certificado (sem dados sensíveis). |
| `404` | Recurso não encontrado na organização atual. |

## certificates delete

Remover certificado

```text
kobana billing certificates delete [ID] [OPÇÕES]
```

`DELETE /v1/certificates/{id}`

Remove o certificado por **soft delete**. Os dados criptografados permanecem no banco apenas para fins de auditoria.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Certificado removido. |
| `404` | Recurso não encontrado na organização atual. |
