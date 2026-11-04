---
id: portal-settings
title: "kobana dashboards portal-settings"
sidebar_label: "portal-settings"
description: "Obter configurações do portal"
---

# kobana dashboards portal-settings

Obter configurações do portal

Comandos de `kobana dashboards portal-settings`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`portal-settings`](#portal-settings) | `GET /v1/portal_settings` | Obter configurações do portal |
| [`portal-settings domain check-slug`](#portal-settings-domain-check-slug) | `GET /v1/portal_settings/domain/check_slug` | Verificar disponibilidade do endereço |
| [`portal-settings domain update`](#portal-settings-domain-update) | `PATCH /v1/portal_settings/domain` | Definir endereço do portal |
| [`portal-settings logo create`](#portal-settings-logo-create) | `POST /v1/portal_settings/logo` | Enviar imagem do portal |
| [`portal-settings logo delete`](#portal-settings-logo-delete) | `DELETE /v1/portal_settings/logo` | Remover imagem do portal |
| [`portal-settings update`](#portal-settings-update) | `PATCH /v1/portal_settings` | Atualizar configurações do portal |
| [`portal-settings verify-sender`](#portal-settings-verify-sender) | `POST /v1/portal_settings/verify_sender` | Verificar o e-mail remetente das notificações |

## portal-settings

Obter configurações do portal

```text
kobana dashboards portal-settings [OPÇÕES] [COMANDO]
```

`GET /v1/portal_settings`

Retorna a configuração do portal do workspace: marca, idioma da página, cores, endereço, opções de login e SSO, notificações e remetente, e configurações do site (cabeçalhos de segurança, SEO, rastreadores, chat e código próprio).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Configuração do portal do workspace, com os valores padrão aplicados. |
| `404` | Recurso não encontrado na organização atual. |

## portal-settings domain check-slug

Verificar disponibilidade do endereço

```text
kobana dashboards portal-settings domain check-slug [OPÇÕES]
```

`GET /v1/portal_settings/domain/check_slug`

Informa se o `slug` está livre, sob o `domain`, para o portal deste workspace dentro da organização (o endereço atual dele conta como livre).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `slug` | `string` | sim | Slug do workspace; o endereço é `{slug_da_organizacao}-{slug}.{domain}` (um rótulo só, até 63 caracteres): 3 a 50 letras minúsculas, números e hífens (sem hífen nas pontas). `null` para remover. |
| `domain` | `string` | sim | Um dos domínios da plataforma (`available_domains`). `null` para remover. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da verificação. |
| `400` | Requisição inválida (parâmetro ausente ou fora do formato). |
| `404` | Recurso não encontrado na organização atual. |

## portal-settings domain update

Definir endereço do portal

```text
kobana dashboards portal-settings domain update [OPÇÕES]
```

`PATCH /v1/portal_settings/domain`

Define o endereço do portal na plataforma, `https://{slug_da_organizacao}-{slug}.{domain}` (o slug da organização é fixo aqui), ou o remove quando os dois são `null`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `slug` | `string` | sim | Slug do workspace; o endereço é `{slug_da_organizacao}-{slug}.{domain}` (um rótulo só, até 63 caracteres): 3 a 50 letras minúsculas, números e hífens (sem hífen nas pontas). `null` para remover. |
| `domain` | `string` | sim | Um dos domínios da plataforma (`available_domains`). `null` para remover. |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-settings.domain.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Endereço atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Recusado por uma regra de negócio (endereço já em uso ou reservado, arquivo grande demais ou de tipo não aceito…). |

## portal-settings logo create

Enviar imagem do portal

```text
kobana dashboards portal-settings logo create [OPÇÕES]
```

`POST /v1/portal_settings/logo`

Envia o logo, o logo para fundos escuros ou o favicon do portal, em base64 no corpo JSON. Substitui a imagem anterior.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `filename` | `string` | sim |  |
| `content_type` | `string` | sim | `image/png`, `image/jpeg` ou `image/webp`; o favicon aceita também `image/x-icon`. SVG não é aceito. |
| `content_base64` | `string` | sim | Conteúdo do arquivo em base64 (até 500 KB depois de decodificado). |
| `variant` | `string` | não | Qual imagem: `logo` (padrão), `logo_light` (logo para fundos escuros) ou `favicon`. Valores: `logo`, `logo_light`, `favicon`. |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-settings.logo.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Imagem salva; `url` é o endereço público. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Recusado por uma regra de negócio (endereço já em uso ou reservado, arquivo grande demais ou de tipo não aceito…). |

## portal-settings logo delete

Remover imagem do portal

```text
kobana dashboards portal-settings logo delete [OPÇÕES]
```

`DELETE /v1/portal_settings/logo`

Remove o logo, o logo para fundos escuros ou o favicon; o portal volta ao padrão.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `variant` | `string` | não | Qual imagem: `logo` (padrão), `logo_light` (logo para fundos escuros) ou `favicon`. Valores: `logo`, `logo_light`, `favicon`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Imagem removida. |
| `400` | Requisição inválida (parâmetro ausente ou fora do formato). |
| `404` | Recurso não encontrado na organização atual. |

## portal-settings update

Atualizar configurações do portal

```text
kobana dashboards portal-settings update [OPÇÕES]
```

`PATCH /v1/portal_settings`

Atualiza a configuração do portal do workspace. Parcial: só os campos enviados mudam. Retorna a configuração completa.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_name` | `string` | não |  |
| `title` | `string` | não |  |
| `subtitle` | `string` | não |  |
| `html_lang` | `string` | não | O idioma do portal: vai no `lang` do `<html>` das páginas e traduz as telas de login. Valores: `pt-BR`, `en-US`, `es-ES`. |
| `light_colors` | `object` | não | Cores do tema, em HSL sem o `hsl()` (ex.: `210 40% 98%`). Chaves omitidas usam o padrão. |
| `dark_colors` | `object` | não | Cores do tema, em HSL sem o `hsl()` (ex.: `210 40% 98%`). Chaves omitidas usam o padrão. |
| `enable_login` | `boolean` | não |  |
| `enable_email_login` | `boolean` | não |  |
| `allow_password_change` | `boolean` | não |  |
| `sso_enabled` | `boolean` | não |  |
| `sso_public_key` | `string` | não |  |
| `sso_algorithm` | `string` | não | Valores: `RS256`, `RS384`, `RS512`, `ES256`, `ES384`, `ES512`, `PS256`, `PS384`, `PS512`. |
| `sso_issuer` | `string` | não |  |
| `notification_type_statuses` | `object` | não | Situação de cada notificação do portal: `ACTIVE` (envia), `INACTIVE` (silenciada) ou `DISABLED` (indisponível). |
| `notification_bcc_emails` | `array<object>` | não | Substitui a lista inteira (até 20). |
| `notification_sender_name` | `string` | não |  |
| `notification_sender_email` | `string` | não | Um endereço novo inicia a verificação no AWS SES, que envia um link de confirmação para ele. `""` volta ao remetente da plataforma. |
| `notification_reply_to_email` | `string` | não |  |
| `hsts_include_subdomains` | `boolean` | não |  |
| `hsts_preload` | `boolean` | não |  |
| `secure_frame_headers` | `boolean` | não |  |
| `seo_indexing_enabled` | `boolean` | não |  |
| `seo_robots_txt` | `string` | não |  |
| `seo_sitemap_enabled` | `boolean` | não |  |
| `seo_google_site_verification` | `string` | não | Apenas o conteúdo da meta tag de verificação do Google. |
| `seo_canonical_url` | `string` | não |  |
| `tracking_integrations` | `object` | não | Rastreadores por provedor (`google_tag_manager`, `google_analytics`, …): `{ provedor: { enabled, ...ids } }`. Provedores e campos desconhecidos são descartados. |
| `chat_integrations` | `object` | não | Widgets de chat por provedor (`intercom`, `hubspot`, `zendesk`, …), no mesmo formato de `tracking_integrations`. |
| `custom_code` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-settings.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Configuração do portal do workspace, com os valores padrão aplicados. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Recusado por uma regra de negócio (endereço já em uso ou reservado, arquivo grande demais ou de tipo não aceito…). |

## portal-settings verify-sender

Verificar o e-mail remetente das notificações

```text
kobana dashboards portal-settings verify-sender [OPÇÕES]
```

`POST /v1/portal_settings/verify_sender`

Pede ou reconfere a verificação no AWS SES do e-mail remetente das notificações do portal. Quando o endereço nunca foi enviado ao SES ou a verificação falhou, pede a verificação (o SES envia ao endereço um link); senão, consulta o estado atual. Retorna `status` (`pending`, `active` ou `failed`) e `verification_requested`. Substitui `POST /portal_settings/sender_identity`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Situação da verificação. |
| `404` | Recurso não encontrado na organização atual. |
