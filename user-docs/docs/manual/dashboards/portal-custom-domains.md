---
id: portal-custom-domains
title: "kobana dashboards portal-custom-domains"
sidebar_label: "portal-custom-domains"
---

# kobana dashboards portal-custom-domains


Comandos de `kobana dashboards portal-custom-domains`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`portal-custom-domains list`](#portal-custom-domains-list) | `GET /v1/portal_custom_domains` | Listar domínios próprios |
| [`portal-custom-domains create`](#portal-custom-domains-create) | `POST /v1/portal_custom_domains` | Adicionar domínio próprio |
| [`portal-custom-domains delete`](#portal-custom-domains-delete) | `DELETE /v1/portal_custom_domains/{id}` | Remover domínio próprio |
| [`portal-custom-domains dns-status`](#portal-custom-domains-dns-status) | `GET /v1/portal_custom_domains/{id}/dns_status` | Consultar DNS do domínio próprio |
| [`portal-custom-domains verify`](#portal-custom-domains-verify) | `POST /v1/portal_custom_domains/{id}/verify` | Verificar e provisionar domínio próprio |

## portal-custom-domains list

Listar domínios próprios

```text
kobana dashboards portal-custom-domains list [OPÇÕES]
```

`GET /v1/portal_custom_domains`

Lista os domínios próprios do portal do workspace (conjunto completo, sem paginação). Vazia onde domínios próprios não são suportados.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Domínios próprios do portal. |
| `404` | Recurso não encontrado na organização atual. |

## portal-custom-domains create

Adicionar domínio próprio

```text
kobana dashboards portal-custom-domains create [OPÇÕES]
```

`POST /v1/portal_custom_domains`

Registra um domínio próprio como `pending_verification`, sem acionar a AWS, e retorna os registros DNS a publicar. Depois chame `POST /portal_custom_domains/{id}/verify`. O registro é só uma reivindicação: o domínio fica com o primeiro portal que publicar o TXT de verificação (as outras reivindicações não comprovadas são liberadas), e uma reivindicação não comprovada expira em 7 dias. Cada portal tem no máximo 10 domínios. Domínios da plataforma (`kobana.com.br`, `kobana.tech` e os domínios compartilhados de portal) não são aceitos.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `hostname` | `string` | sim | Nome completo do domínio, ex.: `portal.exemplo.com.br`. |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-custom-domains.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | O domínio próprio. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `409` | Este domínio já está registrado (comprovado por outro portal, ou já reivindicado por este). |
| `422` | Domínio reservado da plataforma (`hostname_reserved`) ou limite de domínios do portal atingido (`domain_limit_reached`). |

## portal-custom-domains delete

Remover domínio próprio

```text
kobana dashboards portal-custom-domains delete [ID] [OPÇÕES]
```

`DELETE /v1/portal_custom_domains/{id}`

Remove o domínio próprio e desfaz o tenant da CloudFront e o certificado, quando provisionados. O portal deixa de responder nesse endereço.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Removido. |
| `404` | Recurso não encontrado na organização atual. |

## portal-custom-domains dns-status

Consultar DNS do domínio próprio

```text
kobana dashboards portal-custom-domains dns-status [ID] [OPÇÕES]
```

`GET /v1/portal_custom_domains/{id}/dns_status`

Consulta somente leitura dos CNAMEs exigidos, registro a registro, sem alterar nada na AWS.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Situação do DNS. |
| `404` | Recurso não encontrado na organização atual. |

## portal-custom-domains verify

Verificar e provisionar domínio próprio

```text
kobana dashboards portal-custom-domains verify [ID] [OPÇÕES]
```

`POST /v1/portal_custom_domains/{id}/verify`

Enquanto pendente, confere o DNS e provisiona o domínio na AWS (certificado e tenant da CloudFront); depois de provisionado, atualiza a situação. Retorna o domínio com o novo `status`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O domínio próprio. |
| `404` | Recurso não encontrado na organização atual. |
