---
id: admin
title: "kobana banking admin"
sidebar_label: "admin"
description: "Administração (subcontas, usuários, conexões)"
---

# kobana banking admin

Administração (subcontas, usuários, conexões)

Comandos de `kobana banking admin`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`admin certificates list`](#admin-certificates-list) | `GET /v2/admin/certificates` | Listar Certificados |
| [`admin certificates create`](#admin-certificates-create) | `POST /v2/admin/certificates` | Incluir um Certificado |
| [`admin certificates delete`](#admin-certificates-delete) | `DELETE /v2/admin/certificates/{uid}` | Excluir um Certificado |
| [`admin commands list`](#admin-commands-list) | `GET /v2/admin/commands` | Listar Comandos |
| [`admin commands get`](#admin-commands-get) | `GET /v2/admin/commands/{uid}` | Visualizar Comando |
| [`admin connections associations create`](#admin-connections-associations-create) | `POST /v2/admin/connections/{uid}/associations` | Conectar Conta de Serviço |
| [`admin connections associations delete`](#admin-connections-associations-delete) | `DELETE /v2/admin/connections/{uid}/associations` | Desconectar Conta de Serviço |
| [`admin connections list`](#admin-connections-list) | `GET /v2/admin/connections` | Listar Conexões |
| [`admin connections create`](#admin-connections-create) | `POST /v2/admin/connections` | Incluir uma Conexão |
| [`admin connections get`](#admin-connections-get) | `GET /v2/admin/connections/{uid}` | Informações da conexão |
| [`admin connections update`](#admin-connections-update) | `PUT /v2/admin/connections/{uid}` | Atualizar Conexão |
| [`admin connections delete`](#admin-connections-delete) | `DELETE /v2/admin/connections/{uid}` | Excluir uma Conexão |
| [`admin subaccounts list`](#admin-subaccounts-list) | `GET /v2/admin/subaccounts` | Listar Subcontas |
| [`admin subaccounts create`](#admin-subaccounts-create) | `POST /v2/admin/subaccounts` | Criar uma Subconta |
| [`admin subaccounts get`](#admin-subaccounts-get) | `GET /v2/admin/subaccounts/{uid}` | Visualizar uma Subconta |
| [`admin subaccounts update`](#admin-subaccounts-update) | `PUT /v2/admin/subaccounts/{uid}` | Alterar Subconta |
| [`admin users list`](#admin-users-list) | `GET /v2/admin/users` | Listar Usuários |
| [`admin users create`](#admin-users-create) | `POST /v2/admin/users` | Incluir um Usuário |
| [`admin users update`](#admin-users-update) | `PUT /v2/admin/users/{uid}` | Alterar Dados De Um Usuário |
| [`admin users delete`](#admin-users-delete) | `DELETE /v2/admin/users/{uid}` | Excluir um Usuário |

## admin certificates list

Listar Certificados

```text
kobana banking admin certificates list [OPÇÕES]
```

`GET /v2/admin/certificates`

Lista todos os certificados cadastrados na conta com filtros opcionais de paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Certificados encontrados |

## admin certificates create

Incluir um Certificado

```text
kobana banking admin certificates create [OPÇÕES]
```

`POST /v2/admin/certificates`

Incluir um certificado.

> **Tipo do Certificado:** Conforme o tipo do certificado informado é necessario enviar o objeto files com conteúdos diferentes. Se o tipo do certificado for `CRT` é obrigatório enviar o `crt_file` e o `crt_private_key_file`, se o tipo for `PFX` é obrigatório `pfx_file` e `pfx_password`.

**Corpo da requisição**

Enviado como formulário `multipart/form-data`: os arquivos vão em `--file CAMPO=CAMINHO`.

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim | Nome dado ao certificado |
| `cnpj_cpf` | `string` | sim | CPF/CNPJ |
| `type` | `string` | sim | Tipo do certificado: Se o tipo do certificado for `CRT` é obrigatório enviar o `crt_file` e o `crt_private_key_file`, se o tipo for `PFX` é obrigatório `pfx_file` e `pfx_password`. * `crt` CRT * `pfx` PFX Valores: `crt`, `pfx`. |
| `files` | `object` | sim | Arquivo do certificado e senha. Se o tipo do certificado for `CRT` é obrigatório enviar o `crt_file` e o `crt_private_key_file`, se o tipo for `PFX` é obrigatório `pfx_file` e `pfx_password`. |
| `subaccounts` | `boolean` | não | Indica se o certificado está liberado para uso nas subcontas |

Outros campos e objetos aninhados: `kobana schema banking.admin.certificates.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Certificado incluído |
| `422` | Dados do certificado inválido |

## admin certificates delete

Excluir um Certificado

```text
kobana banking admin certificates delete [UID] [OPÇÕES]
```

`DELETE /v2/admin/certificates/{uid}`

Remove permanentemente um certificado. Só é possível excluir certificados que não estejam vinculados a nenhuma conexão ativa e que não tenham sido criados pelo sistema.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Certificado excluído |
| `404` | Certificado não encontrado |
| `422` | Certificado criado pelo sistema não pode ser excluído |

## admin commands list

Listar Comandos

```text
kobana banking admin commands list [OPÇÕES]
```

`GET /v2/admin/commands`

Lista todos os Comandos da conta, com suporte a filtros por status, operação e origem de confirmação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Filtrar por status do Comando. |
| `operation` | `string` | não | Filtrar por tipo de operação. |
| `confirmation_via` | `string` | não | Filtrar por origem da confirmação. Valores: `api`, `edi`. |
| `created_from` | `string` | não | Filtrar por data de criação (início). Formato: YYYY-MM-DD. |
| `created_to` | `string` | não | Filtrar por data de criação (fim). Formato: YYYY-MM-DD. |
| `resource_type` | `string` | não | Filtrar por tipo de recurso (ex.: `BankBillet`, `Pix`, `Financial::Transfer`). |
| `resource_uid` | `string` | não | Filtrar Comandos de um recurso específico (boleto, Pix ou conta financeira) pelo seu uid. |
| `financial_provider_slug` | `string` | não | Filtrar pelo slug do provedor financeiro. |
| `bank_billet_account_uid` | `string` | não | Filtrar Comandos de Boletos de uma carteira (BankBilletAccount) específica, pelo seu uid. |
| `pix_account_uid` | `string` | não | Filtrar Comandos de Pix de uma Conta Pix (PixAccount) específica, pelo seu uid. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de Comandos |

## admin commands get

Visualizar Comando

```text
kobana banking admin commands get [UID] [OPÇÕES]
```

`GET /v2/admin/commands/{uid}`

Retorna os detalhes de um Comando específico pelo seu UID.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando encontrado |
| `404` | Comando não encontrado |

## admin connections associations create

Conectar Conta de Serviço

```text
kobana banking admin connections associations create [UID] [OPÇÕES]
```

`POST /v2/admin/connections/{uid}/associations`

Este endpoint é utilizado para associar uma Conta de Serviço a uma Conexão.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `resource` | `object` | sim |  |

Outros campos e objetos aninhados: `kobana schema banking.admin.connections.associations.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Associação criada |
| `422` | Dados da associação inválidos |

## admin connections associations delete

Desconectar Conta de Serviço

```text
kobana banking admin connections associations delete [UID] [OPÇÕES]
```

`DELETE /v2/admin/connections/{uid}/associations`

Este endpoint é utilizado para remover associação de uma Conexão com Conta de Serviço.

      A remoção não é bloqueada por operações em andamento, e a resposta não tem corpo — as
      consequências abaixo valem a partir do `204`.

      **Conta Pix.** Sem conexão a conta fica desativada: não registra novas cobranças, e as já
      registradas deixam de poder ser canceladas ou consultadas. Carteiras de boleto que emitem
      BolePix por essa conta passam a gerar boletos sem QR Code. Para voltar a operar, associe a
      conta a uma conexão com escopo Pix — a associação substitui o vínculo anterior, sem
      precisar removê-lo antes.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `resource` | `object` | sim |  |

Outros campos e objetos aninhados: `kobana schema banking.admin.connections.associations.delete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Conexão |
| `404` | Conexão não encontrada |

## admin connections list

Listar Conexões

```text
kobana banking admin connections list [OPÇÕES]
```

`GET /v2/admin/connections`

Lista todas as conexões cadastradas com filtros opcionais por provedor, certificado, status de habilitação e validação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `provider_slug` | `string` | não | Provedor financeiro |
| `certificate_uid` | `string` | não | UUID do Certificado |
| `enabled` | `boolean` | não | Conexões habilitadas |
| `validated` | `boolean` | não | Conexões validadas |
| `created_from` | `string` | não | Data inicial do filtro de data de criação da conexão. |
| `created_to` | `string` | não | Data final do filtro de data de criação da conexão. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conexões encontradas |

## admin connections create

Incluir uma Conexão

```text
kobana banking admin connections create [OPÇÕES]
```

`POST /v2/admin/connections`

Cadastra uma nova conexão com um provedor financeiro.

> **Processamento Assíncrono:** a resposta `201` é retornada antes de as credenciais serem validadas junto ao provedor. A conexão nasce com `validated_at` e `validate_failed_at` nulos, e a validação termina em segundo plano.

**Ciclo de validação**

| Estado | `validated_at` | `validate_failed_at` |
| --- | --- | --- |
| Validando | `null` | `null` |
| Validada | preenchido | `null` |
| Falhou | `null` | preenchido |

Quando a validação é bem-sucedida, `enabled_at` também é preenchido, mesmo que a conexão tenha sido criada com `enabled: false`.

**Eventos de Webhook**

- `integration.connection.db.created`: disparado nesta operação, com a conexão ainda não validada.
- `integration.connection.db.updated`: disparado quando a validação termina, com sucesso ou falha. Leia o desfecho em `object.validated_at` e `object.validate_failed_at`: a falha não aparece em `changes`.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.admin.connections.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conexão criada |
| `422` | Dados da conexão inválidos |

## admin connections get

Informações da conexão

```text
kobana banking admin connections get [UID] [OPÇÕES]
```

`GET /v2/admin/connections/{uid}`

Consulta uma conexão. Logo depois da criação, `validated_at` e `validate_failed_at` nulos indicam que a validação das credenciais ainda está em andamento (ver Incluir uma Conexão).

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conexão encontrada |
| `404` | Conexão não encontrada |

## admin connections update

Atualizar Conexão

```text
kobana banking admin connections update [UID] [OPÇÕES]
```

`PUT /v2/admin/connections/{uid}`

Atualiza uma conexão. Com `revalidate: true`, a revalidação das credenciais pode terminar em segundo plano: nesse caso a resposta traz `validation_status: pending` e o desfecho aparece em `validated_at` e `validate_failed_at`, como na criação.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não | Nome da Conexão |
| `environment` | `string` | não | Ambiente de conexão com o provedor financeiro. `production`: Produção. `homologation`: Homologação * Esse campo será ignorado no nosso ambiente de Sandbox Valores: `production`, `homologation`. |
| `apis` | `array<string>` | não | APIs permitidas para o provedor financeiro. `charge/bank_billet`: Boleto. `charge/pix`: Pix. `payment`: Pagamento. `transfer`: Transferência |
| `credentials` | `object` | não | Credenciais. |
| `certificate_uid` | `string (uuid)` | não | UUID do Certificado. |
| `keypair_uid` | `string (uuid)` | não | UUID do Par de Chaves. |
| `enabled` | `boolean` | não | Habilita conexão |
| `revalidate` | `boolean` | não | Revalida as credenciais no provedor financeiro. Se passado true, a conexão só será alterada se as credenciais e permissões forem válidas. |

Outros campos e objetos aninhados: `kobana schema banking.admin.connections.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conexão atualizada |
| `404` | Conexão não encontrada |
| `422` | Credenciais já utilizadas por outra conexão da conta |

## admin connections delete

Excluir uma Conexão

```text
kobana banking admin connections delete [UID] [OPÇÕES]
```

`DELETE /v2/admin/connections/{uid}`

Exclui uma conexão.
        A exclusão de uma conexão resultará na remoção de todas as suas associações, impossibilitando o uso da API anteriormente conectada.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Conexão |
| `404` | Conexão não encontrada |

## admin subaccounts list

Listar Subcontas

```text
kobana banking admin subaccounts list [OPÇÕES]
```

`GET /v2/admin/subaccounts`

Lista todas as subcontas da conta principal com filtros opcionais por email, CNPJ e data de criação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `email` | `string` | não | Filtro por email. Busca subconta por email |
| `business_cnpj` | `string` | não | Filtro por CNPJ. Busca subconta por CNPJ |
| `created_from` | `string` | não | Data inicial do filtro de data de criação da subconta |
| `created_to` | `string` | não | Data final do filtro de data de criação da subconta |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Subcontas encontradas |

## admin subaccounts create

Criar uma Subconta

```text
kobana banking admin subaccounts create [OPÇÕES]
```

`POST /v2/admin/subaccounts`

Criar uma subconta

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `nickname` | `string` | sim | Apelido |
| `id` | `integer` | não | ID da subconta (deprecated, use uid) |
| `uid` | `string` | não | UID da subconta |
| `parent_id` | `integer` | não | ID da conta principal (deprecated, use parent_uid) |
| `parent_uid` | `string (uuid)` | não | UID da conta principal |
| `email` | `string (email)` | não | E-mail |
| `business_name` | `string` | não | Nome Fantasia |
| `business_cnpj` | `string` | não | CNPJ da Empresa |
| `business_legal_name` | `string` | não | Razão Social |
| `configuration` | `string (json)` | não | Configuração de dados padrões da conta |
| `api_access_token` | `string` | não | Token de acesso da subconta |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas a Subconta |
| `created_via_api` | `boolean` | não | Indica se o registro foi criado pela API |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.admin.subaccounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Subconta criada |
| `422` | Dados da subconta inválidos |

## admin subaccounts get

Visualizar uma Subconta

```text
kobana banking admin subaccounts get [UID] [OPÇÕES]
```

`GET /v2/admin/subaccounts/{uid}`

Visualizar uma subconta

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Subconta encontrada |
| `404` | Subconta não encontrada |

## admin subaccounts update

Alterar Subconta

```text
kobana banking admin subaccounts update [UID] [OPÇÕES]
```

`PUT /v2/admin/subaccounts/{uid}`

Alterar Subconta. Não é possível alterar o CNPJ da subconta.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `enabled` | `integer` | não | Indica se a subconta está ativa ou inativa. Utilize `1` para ativar e `0` para desativar. Valores: `0`, `1`. |
| `id` | `integer` | não | ID da subconta (deprecated, use uid) |
| `uid` | `string` | não | UID da subconta |
| `parent_id` | `integer` | não | ID da conta principal (deprecated, use parent_uid) |
| `parent_uid` | `string (uuid)` | não | UID da conta principal |
| `email` | `string (email)` | não | E-mail |
| `business_cnpj` | `string` | não | CNPJ da Empresa |
| `nickname` | `string` | não | Apelido |
| `business_legal_name` | `string` | não | Razão Social |
| `account_type` | `string` | não | Tipo de Conta: * `individual` Pessoa Física * `juridical` Pessoa Jurídica Valores: `individual`, `juridical`. |
| `first_name` | `string` | não | Primeiro Nome do Responsável |
| `middle_name` | `string` | não | Nome do Meio do Responsável |
| `last_name` | `string` | não | Último Nome do Responsável |
| `full_name` | `string` | não | Nome completo do Responsável |
| `gender` | `string` | não | Gênero |
| `cpf` | `string` | não | CPF |
| `address_street_name` | `string` | não | Endereço |
| `address_state` | `string` | não | Estado |
| `address_neighborhood` | `string` | não | Bairro |
| `address_postal_code` | `string` | não | CEP |
| `address_number` | `string` | não | Número |
| `address_complement` | `string` | não | Complemento |
| `address_city_name` | `string` | não | Cidade |
| `phone_number` | `string` | não | Telefone |
| `date_of_birth` | `string (date)` | não | Data de Nascimento |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas a Subconta |
| `business_category` | `integer` | não | Categoria: \| Código da Categoria \| Descrição \| \|---\|---\| \| 1000 \| Arte e objetos de colecionadores \| \| 1001 \| Bebês \| \| 1002 \| Beleza e fragrâncias \| \| 1022 \| Brinquedos e hobbies \| \| 1015 \| Casa e jardim \| \| 1006 \| Computadores \| \| 1004 \| De empresa para empresa (B2B) \| \| 1007 \| Educação \| \| 1008 \| Eletrônicos e telecomunicações \| \| 1009 \| Entretenimento e mídia \| \| 1021 \| Esportes e atividades ao ar livre \| \| 1013 \| Governo \| \| 1003 \| Livros e revistas \| \| 1012 \| Presentes e flores \| \| 1018 \| Religião e espiritualidade (com fins lucrativos) \| \| 1014 \| Saúde e cuidados pessoais \| \| 1016 \| Sem fins lucrativos \| \| 1025 \| Serviço \| \| 1020 \| Serviços - Outro \| \| 1010 \| Serviços e produtos financeiros \| \| 1019 \| Varejo (não classificado em nenhum outro lugar) \| \| 1011 \| Varejo e serviços de alimentação \| \| 1024 \| Vendas de veículos \| \| 1005 \| Vestuário \| \| 1023 \| Viagem \| Valores: `null`, `1000`, `1001`, `1002`, `1022`, `1015`, `1006`, `1004`, `1007`, `1008`, `1009`, `1021`, `1013`, `1003`, `1012`, `1018`, `1014`, `1016`, `1025`, `1020`, `1010`, `1019`, `1011`, `1024`, `1005`, `1023`. |
| `business_subcategory` | `integer` | não | Subcategoria: \| Código da Categoria \| Código da Subcategoria \| Descrição \| \|---\|---\|---\| \| 1017 \| 2173 \| Animais de estimação especiais ou raros \| \| 1017 \| 2171 \| Medicamentos e suplementos \| \| 1017 \| 2172 \| Pet shops, alimentos e suprimentos para animais de estimação \| \| 1017 \| 2174 \| Serviços veterinários \| \| 1000 \| 2000 \| Antiguidades \| \| 1000 \| 2004 \| Arte digital \| \| 1000 \| 2007 \| Costura, bordado e tecidos \| \| 1000 \| 2003 \| Câmeras e suprimentos fotográficos \| \| 1000 \| 2005 \| Lembranças \| \| 1000 \| 2006 \| Loja de música - instrumentos e partituras \| \| 1000 \| 2002 \| Marchands e galerias \| \| 1000 \| 2009 \| Papel de carta, de impressão e escrita \| \| 1000 \| 2008 \| Selos e moedas \| \| 1000 \| 2001 \| Suprimentos de artesanato \| \| 1000 \| 2010 \| Vintage e artigos de colecionadores \| \| 1001 \| 2012 \| Móveis \| \| 1001 \| 2013 \| Produtos para bebês - Outro \| \| 1001 \| 2014 \| Segurança e saúde \| \| 1001 \| 2011 \| Vestuário \| \| 1002 \| 2015 \| Banho e corpo \| \| 1002 \| 2016 \| Fragrâncias e perfumes \| \| 1002 \| 2017 \| Maquiagem e cosméticos \| \| 1022 \| 2252 \| Artesanato \| \| 1022 \| 2253 \| Câmeras e suprimentos fotográficos \| \| 1022 \| 2255 \| Lembranças \| \| 1022 \| 2256 \| Loja de música - instrumentos e partituras \| \| 1022 \| 2254 \| Lojas de hobbies, brinquedos e jogos \| \| 1022 \| 2258 \| Papel de carta, de impressão e escrita \| \| 1022 \| 2257 \| Selos e moedas \| \| 1022 \| 2260 \| Videogames e sistemas \| \| 1022 \| 2259 \| Vintage e artigos de colecionadores \| \| 1015 \| 2145 \| Antiguidades \| \| 1015 \| 2146 \| Aparelhos \| \| 1015 \| 2148 \| Cama e banho \| \| 1015 \| 2150 \| Cortinas e tapeçaria \| \| 1015 \| 2157 \| Decoração residencial \| \| 1015 \| 2162 \| Equipamentos de segurança e vigilância \| \| 1015 \| 2152 \| Lareiras e telas para lareira \| \| 1015 \| 2147 \| Marchands e galerias \| \| 1015 \| 2149 \| Material de construção \| \| 1015 \| 2156 \| Máquinas e ferramentas \| \| 1015 \| 2153 \| Móveis \| \| 1015 \| 2160 \| Paisagismo \| \| 1015 \| 2163 \| Piscinas e spas \| \| 1015 \| 2151 \| Serviços de exterminação de pragas e desinfecção \| \| 1015 \| 2154 \| Suprimentos para jardim \| \| 1015 \| 2161 \| Tapetes e carpetes \| \| 1015 \| 2159 \| Utensílios de cozinha \| \| 1015 \| 2158 \| Utensílios domésticos \| \| 1015 \| 2155 \| Vidro, tinta e papel de parede \| \| 1006 \| 2057 \| Computadores e serviços de processamento de dados \| \| 1006 \| 2059 \| Conteúdo digital \| \| 1006 \| 2058 \| Desktops, laptops e notebooks \| \| 1006 \| 2069 \| Hospedagem e design para a Web \| \| 1006 \| 2064 \| Jogo online \| \| 1006 \| 2062 \| Monitores e projetores \| \| 1006 \| 2066 \| Periféricos \| \| 1006 \| 2065 \| Peças e acessórios \| \| 1006 \| 2063 \| Rede de comunicação \| \| 1006 \| 2060 \| Serviços de Comércio eletrônico \| \| 1006 \| 2061 \| Serviços de reparo e manutenção \| \| 1006 \| 2068 \| Serviços de treinamento \| \| 1006 \| 2067 \| Software \| \| 1004 \| 2027 \| Agrícola \| \| 1004 \| 2047 \| Atacado \| \| 1004 \| 2031 \| Construção \| \| 1004 \| 2025 \| Contabilidade \| \| 1004 \| 2043 \| Editoração e impressão \| \| 1004 \| 2045 \| Embalagem e envio \| \| 1004 \| 2030 \| Fotografia comercial, arte e gráficos \| \| 1004 \| 2038 \| Mala direta \| \| 1004 \| 2039 \| Marketing \| \| 1004 \| 2040 \| Marketing em vários níveis \| \| 1004 \| 2041 \| Móveis de escritório e comerciais \| \| 1004 \| 2029 \| Produtos químicos e assemelhados \| \| 1004 \| 2026 \| Propaganda \| \| 1004 \| 2028 \| Serviços de arquitetura, engenharia e prospecção \| \| 1004 \| 2032 \| Serviços de consultoria \| \| 1004 \| 2044 \| Serviços de cópia rápida e reprodução \| \| 1004 \| 2046 \| Serviços de estenografia e de secretaria \| \| 1004 \| 2034 \| Serviços de leasing e locação de equipamentos \| \| 1004 \| 2036 \| Serviços de recrutamento e seleção \| \| 1004 \| 2035 \| Serviços de reparo de equipamentos \| \| 1004 \| 2033 \| Serviços educacionais \| \| 1004 \| 2042 \| Suprimentos e equipamento de escritório \| \| 1004 \| 2037 \| Suprimentos industriais e de produção \| \| 1007 \| 2073 \| Academias, estúdios e escolas de dança \| \| 1007 \| 2070 \| Escolas comerciais e secretariais \| \| 1007 \| 2074 \| Escolas de ensino fundamental e médio \| \| 1007 \| 2075 \| Escolas vocacionais e de comércio \| \| 1007 \| 2072 \| Faculdades e universidades \| \| 1007 \| 2071 \| Serviços de cuidados infantis diários \| \| 1008 \| 2079 \| Acessórios eletrônicos em geral \| \| 1008 \| 2085 \| Cartões telefônicos \| \| 1008 \| 2076 \| Câmeras, camcorders e equipamentos \| \| 1008 \| 2081 \| Eletrônicos residenciais \| \| 1008 \| 2083 \| Equipamento de telecomunicações e vendas \| \| 1008 \| 2082 \| Segurança e vigilância \| \| 1008 \| 2084 \| Serviços de telecomunicação \| \| 1008 \| 2077 \| Som de carro e eletrônicos \| \| 1008 \| 2078 \| Telefones celulares, PDAs e pagers \| \| 1008 \| 2080 \| Áudio residencial \| \| 1009 \| 2097 \| Animadores de programas \| \| 1009 \| 2094 \| Brinquedos e jogos \| \| 1009 \| 2095 \| Caça níqueis \| \| 1009 \| 2096 \| Conteúdo digital \| \| 1009 \| 2091 \| Conteúdo digital adulto \| \| 1009 \| 2088 \| Filmes - DVDs, fitas de vídeo \| \| 1009 \| 2087 \| Ingressos de cinema \| \| 1009 \| 2093 \| Ingressos de teatro \| \| 1009 \| 2092 \| Ingressos para shows \| \| 1009 \| 2098 \| Jogo \| \| 1009 \| 2099 \| Jogos online \| \| 1009 \| 2086 \| Lembranças \| \| 1009 \| 2089 \| Música - CDs, cassetes e álbuns \| \| 1009 \| 2090 \| TV e rádio a cabo, por satélite e outros tipos pagos \| \| 1009 \| 2100 \| Videogames e sistemas \| \| 1021 \| 2241 \| Academias, estúdios e escolas de dança \| \| 1021 \| 2244 \| Acessórios de armas de fogo \| \| 1021 \| 2248 \| Armas de artes marciais \| \| 1021 \| 2245 \| Armas de fogo \| \| 1021 \| 2237 \| Calçados esportivos \| \| 1021 \| 2240 \| Camping e atividades ao ar livre \| \| 1021 \| 2246 \| Caça \| \| 1021 \| 2243 \| Engrenagem de relógio \| \| 1021 \| 2250 \| Equipamento esportivo \| \| 1021 \| 2242 \| Exercícios e condicionamento físico \| \| 1021 \| 2247 \| Facas \| \| 1021 \| 2249 \| Jogos e brinquedos esportivos \| \| 1021 \| 2238 \| Loja, serviço e reparo de bicicletas \| \| 1021 \| 2251 \| Piscinas e spas \| \| 1021 \| 2239 \| Remo, vela e acessórios \| \| 1013 \| 2137 \| Serviços governamentais (não classificado em nenhum outro lugar) \| \| 1003 \| 2019 \| Conteúdo digital \| \| 1003 \| 2023 \| Editoração e impressão \| \| 1003 \| 2021 \| Ficção e não ficção \| \| 1003 \| 2020 \| Livros de texto e educativos \| \| 1003 \| 2018 \| Livros em áudio \| \| 1003 \| 2024 \| Livros raros e usados \| \| 1003 \| 2022 \| Revistas \| \| 1012 \| 2134 \| Comidas de gourmet \| \| 1012 \| 2132 \| Floristas \| \| 1012 \| 2133 \| Lojas de presentes, cartões, novidades e lembranças \| \| 1012 \| 2136 \| Suprimentos para festas \| \| 1012 \| 2135 \| Viveiro de plantas e flores \| \| 1018 \| 2176 \| Mercadoria \| \| 1018 \| 2177 \| Serviços (não classificados em nenhum outro lugar) \| \| 1018 \| 2175 \| Serviços de associação \| \| 1014 \| 2141 \| Assistência médica \| \| 1014 \| 2140 \| Cuidados dentários \| \| 1014 \| 2142 \| Equipamentos e suprimentos médicos \| \| 1014 \| 2138 \| Farmácia (exceto prescrição de medicamentos) \| \| 1014 \| 2139 \| Farmácia (inclusive prescrição de medicamentos) \| \| 1014 \| 2143 \| Oftalmologia \| \| 1014 \| 2144 \| Vitaminas e suplementos \| \| 1016 \| 2169 \| Educacional \| \| 1016 \| 2164 \| Entidade assistencial \| \| 1016 \| 2167 \| Outro \| \| 1016 \| 2168 \| Pessoal \| \| 1016 \| 2165 \| Político \| \| 1016 \| 2166 \| Religioso \| \| 1025 \| 2297 \| Acessórios \| \| 1025 \| 2296 \| Aluguel de caminhões e trailers utilitários \| \| 1025 \| 2293 \| Aluguel de trailers e veículos recreativos \| \| 1025 \| 2291 \| Aluguel e leasing de barcos \| \| 1025 \| 2294 \| Ferramentas e equipamentos \| \| 1025 \| 2292 \| Lavagem de carros \| \| 1025 \| 2288 \| Locação de carros \| \| 1025 \| 2284 \| Peças e suprimentos novos - veículo motor \| \| 1025 \| 2285 \| Peças usadas - veículo motor \| \| 1025 \| 2287 \| Reparo e pintura de autos \| \| 1025 \| 2295 \| Serviço de guincho \| \| 1025 \| 2289 \| Serviços de autos \| \| 1025 \| 2290 \| Suprimento e serviços de pneus automotivos \| \| 1025 \| 2286 \| Áudio e vídeo \| \| 1020 \| 2232 \| Ajustes e alterações \| \| 1020 \| 2216 \| Assistência médica \| \| 1020 \| 2188 \| Carpintaria \| \| 1020 \| 2217 \| Clubes e organizações de associados \| \| 1020 \| 2192 \| Computadores e serviços de processamento de dados \| \| 1020 \| 2225 \| Conserto de rádios, televisores e estéreo \| \| 1020 \| 2226 \| Corretor de imóveis \| \| 1020 \| 2197 \| Cuidados dentários \| \| 1020 \| 2205 \| Desenho gráfico e comercial \| \| 1020 \| 2230 \| Embalagem e envio \| \| 1020 \| 2204 \| Empreiteiros em geral \| \| 1020 \| 2220 \| Encontros online \| \| 1020 \| 2200 \| Entretenimento \| \| 1020 \| 2222 \| Estúdios fotográficos - retratos \| \| 1020 \| 2221 \| Fotoacabamento \| \| 1020 \| 2191 \| Fotografia comercial \| \| 1020 \| 2227 \| Gerenciamento de locação de imóvel \| \| 1020 \| 2207 \| Identidades, certidões e passaportes \| \| 1020 \| 2208 \| Importação e exportação \| \| 1020 \| 2203 \| Jogo \| \| 1020 \| 2190 \| Limpeza e manutenção \| \| 1020 \| 2215 \| Loterias e concursos \| \| 1020 \| 2218 \| Misc. publicação e impressão \| \| 1020 \| 2219 \| Mudança e depósito \| \| 1020 \| 2235 \| Oftalmologia \| \| 1020 \| 2212 \| Paisagismo e horticultura \| \| 1020 \| 2202 \| Planejamento de eventos e casamentos \| \| 1020 \| 2185 \| Propaganda \| \| 1020 \| 2199 \| Reparo de dispositivos pequenos e elétricos \| \| 1020 \| 2228 \| Reparos de móveis e estofamento \| \| 1020 \| 2236 \| Reparos de relógios e joias \| \| 1020 \| 2210 \| Seguros - auto e residência \| \| 1020 \| 2211 \| Seguros - vida e anuidade \| \| 1020 \| 2214 \| Serviço de entrega local \| \| 1020 \| 2233 \| Serviço de telecomunicação \| \| 1020 \| 2229 \| Serviços (não classificados em nenhum outro lugar) \| \| 1020 \| 2198 \| Serviços de Comércio eletrônico \| \| 1020 \| 2195 \| Serviços de aconselhamento \| \| 1020 \| 2187 \| Serviços de carreira \| \| 1020 \| 2194 \| Serviços de consultoria \| \| 1020 \| 2189 \| Serviços de cuidados com crianças \| \| 1020 \| 2224 \| Serviços de cópia rápida e reprodução \| \| 1020 \| 2196 \| Serviços de entregas \| \| 1020 \| 2201 \| Serviços de leasing e locação de equipamentos \| \| 1020 \| 2231 \| Serviços de piscinas \| \| 1020 \| 2223 \| Serviços de proteção e segurança \| \| 1020 \| 2209 \| Serviços de recuperação de informações \| \| 1020 \| 2193 \| Serviços de rede de computador \| \| 1020 \| 2186 \| Serviços e clubes de compras \| \| 1020 \| 2213 \| Serviços jurídicos e advogados \| \| 1020 \| 2206 \| Spas de saúde e beleza \| \| 1020 \| 2234 \| Utilitários \| \| 1010 \| 2122 \| Agentes e corretores de títulos \| \| 1010 \| 2115 \| Agentes ou corretores de hipotecas \| \| 1010 \| 2103 \| Bolsa de commodities e futuros \| \| 1010 \| 2118 \| Cartões de valores pré-pagos e armazenados \| \| 1010 \| 2109 \| Companhia financeira \| \| 1010 \| 2110 \| Consultoria financeira e de investimentos \| \| 1010 \| 2101 \| Contabilidade \| \| 1010 \| 2106 \| Cooperativa de crédito \| \| 1010 \| 2119 \| Corretor de imóveis \| \| 1010 \| 2107 \| Corretores e câmbio \| \| 1010 \| 2108 \| Custódia de títulos \| \| 1010 \| 2114 \| Empresa de serviços financeiros \| \| 1010 \| 2104 \| Empresas de informação de crédito de consumidor \| \| 1010 \| 2120 \| Entrega \| \| 1010 \| 2102 \| Escritório de cobrança \| \| 1010 \| 2121 \| Gerenciamento de locação de imóvel \| \| 1010 \| 2113 \| Investimentos - geral \| \| 1010 \| 2116 \| Moeda de jogo online \| \| 1010 \| 2117 \| Pagamento antecipado ou empréstimo contra salário \| \| 1010 \| 2111 \| Seguros - auto e residência \| \| 1010 \| 2112 \| Seguros - vida e anuidade \| \| 1010 \| 2105 \| Serviço de orientação sobre débitos \| \| 1010 \| 2123 \| Transferência eletrônica e ordem de pagamento \| \| 1019 \| 2181 \| Bens duráveis \| \| 1019 \| 2182 \| Bens não duráveis \| \| 1019 \| 2179 \| Loja de departamentos \| \| 1019 \| 2180 \| Loja de descontos \| \| 1019 \| 2183 \| Loja de usados e segunda mão \| \| 1019 \| 2184 \| Loja de variedades \| \| 1019 \| 2178 \| Produtos químicos e assemelhados \| \| 1011 \| 2124 \| Bebidas alcoólicas \| \| 1011 \| 2126 \| Café e chá \| \| 1011 \| 2127 \| Comidas de gourmet \| \| 1011 \| 2130 \| Fumo \| \| 1011 \| 2128 \| Lojas de alimentos variados e especialidades \| \| 1011 \| 2129 \| Restaurante \| \| 1011 \| 2125 \| Serviços de bufê \| \| 1011 \| 2131 \| Vitaminas e suplementos \| \| 1024 \| 2277 \| Aviação \| \| 1024 \| 2283 \| Vintage e artigos de colecionadores \| \| 1024 \| 2275 \| vendedor de autos - novos e usados \| \| 1024 \| 2276 \| vendedor de autos - somente usados \| \| 1024 \| 2278 \| vendedor de barcos \| \| 1024 \| 2280 \| vendedor de motocicletas \| \| 1024 \| 2279 \| vendedor de trailers \| \| 1024 \| 2281 \| vendedor de trailers utilitários e recreativos \| \| 1024 \| 2282 \| vendedor de veículos recreativos \| \| 1005 \| 2053 \| Acessórios \| \| 1005 \| 2055 \| Atacado - pedras e metais preciosos \| \| 1005 \| 2056 \| Joalheria da moda \| \| 1005 \| 2050 \| Roupas femininas \| \| 1005 \| 2048 \| Roupas infantis \| \| 1005 \| 2049 \| Roupas masculinas \| \| 1005 \| 2051 \| Sapatos \| \| 1005 \| 2052 \| Uniformes de serviço militar e civil \| \| 1005 \| 2054 \| Varejo - joalheria fina e relógios \| \| 1023 \| 2268 \| Acampamento de esportes e recreação \| \| 1023 \| 2274 \| Agência de viagens \| \| 1023 \| 2264 \| Cruzeiros \| \| 1023 \| 2265 \| Hospedagem e acomodações \| \| 1023 \| 2261 \| Linha aérea \| \| 1023 \| 2263 \| Linha de ônibus \| \| 1023 \| 2262 \| Locação de carros \| \| 1023 \| 2266 \| Malas e mercadorias em couro \| \| 1023 \| 2272 \| Parques ou acampamentos de trailers \| \| 1023 \| 2270 \| Propriedades de lazer compartilhado \| \| 1023 \| 2267 \| Serviços de recreação \| \| 1023 \| 2273 \| Serviços de transporte - outros \| \| 1023 \| 2269 \| Táxis e limusines \| \| 1023 \| 2271 \| Viagens \| Valores: `null`, `2173`, `2171`, `2172`, `2174`, `2000`, `2004`, `2007`, `2003`, `2005`, `2006`, `2002`, `2009`, `2008`, `2001`, `2010`, `2012`, `2013`, `2014`, `2011`, `2015`, `2016`, `2017`, `2252`, `2253`, `2255`, `2256`, `2254`, `2258`, `2257`, `2260`, `2259`, `2145`, `2146`, `2148`, `2150`, `2157`, `2162`, `2152`, `2147`, `2149`, `2156`, `2153`, `2160`, `2163`, `2151`, `2154`, `2161`, `2159`, `2158`, `2155`, `2057`, `2059`, `2058`, `2069`, `2064`, `2062`, `2066`, `2065`, `2063`, `2060`, `2061`, `2068`, `2067`, `2027`, `2047`, `2031`, `2025`, `2043`, `2045`, `2030`, `2038`, `2039`, `2040`, `2041`, `2029`, `2026`, `2028`, `2032`, `2044`, `2046`, `2034`, `2036`, `2035`, `2033`, `2042`, `2037`, `2073`, `2070`, `2074`, `2075`, `2072`, `2071`, `2079`, `2085`, `2076`, `2081`, `2083`, `2082`, `2084`, `2077`, `2078`, `2080`, `2097`, `2094`, `2095`, `2096`, `2091`, `2088`, `2087`, `2093`, `2092`, `2098`, `2099`, `2086`, `2089`, `2090`, `2100`, `2241`, `2244`, `2248`, `2245`, `2237`, `2240`, `2246`, `2243`, `2250`, `2242`, `2247`, `2249`, `2238`, `2251`, `2239`, `2137`, `2019`, `2023`, `2021`, `2020`, `2018`, `2024`, `2022`, `2134`, `2132`, `2133`, `2136`, `2135`, `2176`, `2177`, `2175`, `2141`, `2140`, `2142`, `2138`, `2139`, `2143`, `2144`, `2169`, `2164`, `2167`, `2168`, `2165`, `2166`, `2297`, `2296`, `2293`, `2291`, `2294`, `2292`, `2288`, `2284`, `2285`, `2287`, `2295`, `2289`, `2290`, `2286`, `2232`, `2216`, `2188`, `2217`, `2192`, `2225`, `2226`, `2197`, `2205`, `2230`, `2204`, `2220`, `2200`, `2222`, `2221`, `2191`, `2227`, `2207`, `2208`, `2203`, `2190`, `2215`, `2218`, `2219`, `2235`, `2212`, `2202`, `2185`, `2199`, `2228`, `2236`, `2210`, `2211`, `2214`, `2233`, `2229`, `2198`, `2195`, `2187`, `2194`, `2189`, `2224`, `2196`, `2201`, `2231`, `2223`, `2209`, `2193`, `2186`, `2213`, `2206`, `2234`, `2122`, `2115`, `2103`, `2118`, `2109`, `2110`, `2101`, `2106`, `2119`, `2107`, `2108`, `2114`, `2104`, `2120`, `2102`, `2121`, `2113`, `2116`, `2117`, `2111`, `2112`, `2105`, `2123`, `2181`, `2182`, `2179`, `2180`, `2183`, `2184`, `2178`, `2124`, `2126`, `2127`, `2130`, `2128`, `2129`, `2125`, `2131`, `2277`, `2283`, `2275`, `2276`, `2278`, `2280`, `2279`, `2281`, `2282`, `2053`, `2055`, `2056`, `2050`, `2048`, `2049`, `2051`, `2052`, `2054`, `2268`, `2274`, `2264`, `2265`, `2261`, `2263`, `2262`, `2266`, `2272`, `2270`, `2267`, `2273`, `2269`, `2271`. |
| `business_website` | `string` | não | Website |
| `business_name` | `string` | não | Nome Fantasia |
| `business_type` | `string` | não | Tipo da Empresa: * `mei` Microempreendedor Individual * `individual` Micro Empresa/Empresa de pequeno porte * `proprietorship` Sociedade Empresária Limitada * `partnership` Sociedade Anônima de Capital Fechado * `corporation` Sociedade Anônima de Capital Aberto * `nonprofit` Sem fins lucrativos * `government` Governamental Valores: `null`, `mei`, `individual`, `proprietorship`, `partnership`, `corporation`, `nonprofit`, `government`. |
| `mother_name` | `string` | não | Nome da Mãe |
| `father_name` | `string` | não | Nome do Pai |
| `configuration` | `string (json)` | não | Configuração de dados padrões para boleto |
| `api_access_token` | `string` | não | Token de acesso da subconta |
| `created_via_api` | `boolean` | não | Indica se o registro foi criado pela API |
| `billing_email` | `string` | não | E-mail de cobrança |
| `simple_opting` | `boolean` | não | Optante pelo simples |
| `rate_limits` | `object` | não | Limites de requisições - Hash com chave e valor no formato JSON. |

Outros campos e objetos aninhados: `kobana schema banking.admin.subaccounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Subconta alterada |
| `404` | Subconta não encontrado |
| `422` | Parâmetros inválidos |

## admin users list

Listar Usuários

```text
kobana banking admin users list [OPÇÕES]
```

`GET /v2/admin/users`

Lista todos os usuários cadastrados na conta, com opção de filtrar por email. Suporta paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `email` | `string` | não | Filtro por email. Busca usuário por email |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Usuários encontrados |

## admin users create

Incluir um Usuário

```text
kobana banking admin users create [OPÇÕES]
```

`POST /v2/admin/users`

Incluir um usuário

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `id` | `integer` | não | ID do usuário (deprecated, use uid) |
| `uid` | `string` | não | UID público do usuário |
| `intercom_id` | `string` | não | Public ID do usuário usado como external_id no Intercom. null quando a integração com o Intercom está desabilitada |
| `email` | `string (email)` | não | E-mail do usuário |
| `email_verified` | `boolean` | não | Indica se o e-mail do usuário já foi confirmado (escopo email) |
| `phone` | `string` | não | Telefone cadastrado (escopo phone) |
| `first_name` | `string` | não | Primeiro nome |
| `middle_name` | `string` | não | Nome do meio |
| `last_name` | `string` | não | Último nome |
| `full_name` | `string` | não | Nome completo |
| `created_via_api` | `boolean` | não | Indica se foi criado via API |
| `permissions` | `array<string>` | não | Permissões do usuário |
| `created_at` | `string (date-time)` | não | Data de criação |
| `updated_at` | `string (date-time)` | não | Data de atualização |
| `enabled_at` | `string (date-time)` | não | Data de habilitação |
| `legal_data` | `object` | não | Dados legais (nome civil e CPF) |
| `profile_photo_url` | `string (uri)` | não | URL da foto de perfil (ActiveStorage) ou Gravatar como fallback |
| `gender` | `string` | não | Identidade de gênero (undisclosed, woman, man, non_binary, other_identity ou valor livre — escopo profile.identity) |
| `birthday` | `string (date)` | não | Data de nascimento (escopo profile.basic) |
| `languages` | `array<string>` | não | Idiomas preferidos de comunicação do usuário (escopo profile.communication) |
| `email_unsubscribed_at` | `string (date-time)` | não | Data em que o usuário cancelou todas as comunicações por e-mail (opt-out geral), null se nunca cancelou (escopo profile.communication) |
| `timezone` | `string` | não | Fuso horário do usuário (escopo profile.preferences) |
| `accessibility_resources` | `array<string>` | não | Recursos de acessibilidade usados na plataforma (escopo profile.accessibility) |
| `disability_disclosure` | `array<string>` | não | Deficiências informadas pelo usuário, opcional (escopo profile.accessibility) |

Outros campos e objetos aninhados: `kobana schema banking.admin.users.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Usuário incluído |
| `422` | Dados do usuário inválido |

## admin users update

Alterar Dados De Um Usuário

```text
kobana banking admin users update [UID] [OPÇÕES]
```

`PUT /v2/admin/users/{uid}`

Atualiza as informações de um usuário existente, incluindo nome, email e permissões.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `id` | `integer` | não | ID do usuário (deprecated, use uid) |
| `uid` | `string` | não | UID público do usuário |
| `intercom_id` | `string` | não | Public ID do usuário usado como external_id no Intercom. null quando a integração com o Intercom está desabilitada |
| `email` | `string (email)` | não | E-mail do usuário |
| `email_verified` | `boolean` | não | Indica se o e-mail do usuário já foi confirmado (escopo email) |
| `phone` | `string` | não | Telefone cadastrado (escopo phone) |
| `first_name` | `string` | não | Primeiro nome |
| `middle_name` | `string` | não | Nome do meio |
| `last_name` | `string` | não | Último nome |
| `full_name` | `string` | não | Nome completo |
| `created_via_api` | `boolean` | não | Indica se foi criado via API |
| `permissions` | `array<string>` | não | Permissões do usuário |
| `created_at` | `string (date-time)` | não | Data de criação |
| `updated_at` | `string (date-time)` | não | Data de atualização |
| `enabled_at` | `string (date-time)` | não | Data de habilitação |
| `legal_data` | `object` | não | Dados legais (nome civil e CPF) |
| `profile_photo_url` | `string (uri)` | não | URL da foto de perfil (ActiveStorage) ou Gravatar como fallback |
| `gender` | `string` | não | Identidade de gênero (undisclosed, woman, man, non_binary, other_identity ou valor livre — escopo profile.identity) |
| `birthday` | `string (date)` | não | Data de nascimento (escopo profile.basic) |
| `languages` | `array<string>` | não | Idiomas preferidos de comunicação do usuário (escopo profile.communication) |
| `email_unsubscribed_at` | `string (date-time)` | não | Data em que o usuário cancelou todas as comunicações por e-mail (opt-out geral), null se nunca cancelou (escopo profile.communication) |
| `timezone` | `string` | não | Fuso horário do usuário (escopo profile.preferences) |
| `accessibility_resources` | `array<string>` | não | Recursos de acessibilidade usados na plataforma (escopo profile.accessibility) |
| `disability_disclosure` | `array<string>` | não | Deficiências informadas pelo usuário, opcional (escopo profile.accessibility) |

Outros campos e objetos aninhados: `kobana schema banking.admin.users.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Usuário Alterado |
| `404` | Usuário não encontrado |
| `422` | Parâmetros inválidos |

## admin users delete

Excluir um Usuário

```text
kobana banking admin users delete [UID] [OPÇÕES]
```

`DELETE /v2/admin/users/{uid}`

Remove permanentemente um usuário do sistema.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Usuário excluído |
| `404` | Usuário não encontrado |
