---
id: security
title: "kobana banking security"
sidebar_label: "security"
description: "Segurança (tokens de acesso)"
---

# kobana banking security

Segurança (tokens de acesso)

Comandos de `kobana banking security`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`security access-tokens create`](#security-access-tokens-create) | `POST /v2/security/access_tokens` | Criar um Token de Acesso |
| [`security access-tokens update`](#security-access-tokens-update) | `PUT /v2/security/access_tokens/{uid}` | Atualizar Token de Acesso |
| [`security access-tokens delete`](#security-access-tokens-delete) | `DELETE /v2/security/access_tokens/{uid}` | Excluir Token de Acesso |
| [`security access-tokens disable`](#security-access-tokens-disable) | `PUT /v2/security/access_tokens/{uid}/disable` | Desabilitar Token de Acesso |
| [`security access-tokens enable`](#security-access-tokens-enable) | `PUT /v2/security/access_tokens/{uid}/enable` | Habilitar Token de Acesso |
| [`security access-tokens renew`](#security-access-tokens-renew) | `PUT /v2/security/access_tokens/{uid}/renew` | Renovar Token de Acesso |
| [`security access-tokens revoke`](#security-access-tokens-revoke) | `PUT /v2/security/access_tokens/{uid}/revoke` | Revogar Token de Acesso |

## security access-tokens create

Criar um Token de Acesso

```text
kobana banking security access-tokens create [OPÇÕES]
```

`POST /v2/security/access_tokens`

Cria um novo token de acesso (access token) para autenticação na API. O token criado pode ter escopos limitados e data de expiração.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `expires_at` | `string (date)` | sim | Data de expiração do token. Obrigatório para todos os tokens v2 (leitura e escrita). Máximo 1 ano a partir de hoje. |
| `reminder_emails` | `array<object>` | sim | E-mails para notificações de expiração. Obrigatório, com pelo menos 1 endereço. |
| `external_id` | `string` | não | ID externo do token |
| `description` | `string` | não | Descrição do token |
| `scope_type` | `string` | não | Tipo de escopo: Todos (all) ou Limitado (limited) Valores: `all`, `limited`. |
| `limited_scopes` | `array<string>` | não | Escopos disponíveis (quando scope_type = limited). Selecione um ou mais valores: \| Escopo \| Descrição \| \|--------\|-----------\| \| `admin.subaccounts` \| Gerenciar subcontas da conta principal \| \| `admin.users` \| Gerenciar usuários e permissões \| \| `automation.email_accounts` \| Gerenciar contas de e-mail para automação \| \| `automation.email_deliveries` \| Visualizar e gerenciar envios de e-mail \| \| `automation.sms_accounts` \| Gerenciar contas de SMS \| \| `automation.sms_deliveries` \| Visualizar e gerenciar envios de SMS \| \| `automation.webhook_deliveries` \| Visualizar e gerenciar entregas de webhooks \| \| `automation.webhooks` \| Gerenciar webhooks e notificações automatizadas \| \| `billing.transactions` \| Visualizar transações de cobrança e faturamento \| \| `charge.automatic_pix.locations` \| Gerenciar localizações de Pix automático \| \| `charge.automatic_pix.pix` \| Gerenciar cobranças Pix automáticas \| \| `charge.automatic_pix.recurrences` \| Gerenciar recorrências de Pix automático \| \| `charge.automatic_pix.requests` \| Gerenciar solicitações de Pix automático \| \| `charge.bank_billet_accounts` \| Gerenciar contas de boletos bancários \| \| `charge.bank_billet_payments` \| Visualizar e registrar pagamentos de boletos \| \| `charge.bank_billet_registrations` \| Gerenciar registros de boletos nos bancos \| \| `charge.bank_billets` \| Gerenciar boletos bancários \| \| `charge.customer_subscriptions` \| Gerenciar assinaturas de clientes \| \| `charge.installments` \| Gerenciar parcelamentos \| \| `charge.payments` \| Visualizar e gerenciar pagamentos recebidos \| \| `charge.pix` \| Gerenciar cobranças Pix \| \| `charge.pix_accounts` \| Gerenciar contas Pix \| \| `core.providers` \| Visualizar provedores disponíveis \| \| `crm.customers` \| Gerenciar clientes e informações comerciais \| \| `crm.people` \| Gerenciar pessoas e contatos \| \| `data.bank_billet_queries` \| Consultar informações de boletos bancários \| \| `data.pix_qrcode_queries` \| Consultar QR Codes Pix (copia e cola) \| \| `financial.accounts` \| Gerenciar contas financeiras \| \| `financial.balances` \| Visualizar saldos e movimentações financeiras \| \| `financial.providers` \| Gerenciar provedores financeiros e integrações bancárias \| \| `financial.statement_transactions` \| Visualizar transações e extratos financeiros \| \| `integration.certificates` \| Gerenciar certificados digitais para integrações \| \| `integration.connections` \| Gerenciar conexões com bancos e provedores \| \| `integration.discharges` \| Gerenciar arquivos de retorno bancário (baixas) \| \| `integration.edi_boxes` \| Gerenciar caixas postais EDI para troca de arquivos \| \| `integration.remittances` \| Gerenciar arquivos de remessa bancária \| \| `mailbox.entries` \| Gerenciar caixas postais para recebimento de arquivos \| \| `mailbox.files` \| Visualizar e gerenciar arquivos nas caixas postais \| \| `partner.bank_contracts` \| Gerenciar contratos bancários com parceiros \| \| `payment.accounts` \| Gerenciar contas de pagamento (incluindo habilitação de remessa EDI) \| \| `payment.bank_billets` \| Efetuar pagamentos de boletos bancários \| \| `payment.batches` \| Gerenciar lotes de pagamentos \| \| `payment.dda_accounts` \| Gerenciar contas DDA \| \| `payment.dda.bank_billets` \| Gerenciar boletos DDA \| \| `payment.darfs` \| Efetuar pagamentos de DARFs (tributos federais) \| \| `payment.payments` \| Gerenciar todos os tipos de pagamentos através da API \| \| `payment.pix` \| Efetuar pagamentos via Pix \| \| `payment.taxes` \| Efetuar pagamentos de impostos e taxas \| \| `payment.utilities` \| Efetuar pagamentos de contas de consumo (água, luz, etc) \| \| `security.access_tokens` \| Gerenciar tokens de acesso e autenticação \| \| `system.events` \| Visualizar eventos e logs do sistema \| \| `system.imports` \| Gerenciar importações de dados \| \| `system.reports` \| Gerar e visualizar relatórios do sistema \| \| `transfer.accounts` \| Gerenciar contas de transferência (incluindo habilitação de remessa EDI) \| \| `transfer.batches` \| Gerenciar lotes de transferências \| \| `transfer.internal` \| Realizar transferências internas entre contas \| \| `transfer.pix` \| Realizar transferências via Pix \| \| `transfer.ted` \| Realizar transferências TED \| \| `transfer.transfers` \| Gerenciar todos os tipos de transferências (Pix, TED, Internas) via endpoint unificado \| |
| `access_type` | `string` | não | Tipo de acesso: Leitura (read) ou Leitura e escrita (write) Valores: `read`, `write`. |
| `tags` | `array<string>` | não | Tags do token |
| `custom_data` | `object` | não | Dados customizados |

Outros campos e objetos aninhados: `kobana schema banking.security.access-tokens.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Token criado |
| `422` | Dados do token inválidos |

## security access-tokens update

Atualizar Token de Acesso

```text
kobana banking security access-tokens update [UID] [OPÇÕES]
```

`PUT /v2/security/access_tokens/{uid}`

Atualiza as informações de um token de acesso existente, incluindo descrição, escopos, tags e data de expiração. **Nota:** A data de expiração só pode ser definida se ainda não estava definida. Uma vez definida, não pode mais ser alterada.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `external_id` | `string` | não | ID externo do token |
| `description` | `string` | não | Descrição do token |
| `scope_type` | `string` | não | Tipo de escopo (all ou limited) Valores: `all`, `limited`. |
| `limited_scopes` | `array<string>` | não | Escopos disponíveis (quando scope_type = limited). Selecione um ou mais valores: \| Escopo \| Descrição \| \|--------\|-----------\| \| `admin.subaccounts` \| Gerenciar subcontas da conta principal \| \| `admin.users` \| Gerenciar usuários e permissões \| \| `automation.email_accounts` \| Gerenciar contas de e-mail para automação \| \| `automation.email_deliveries` \| Visualizar e gerenciar envios de e-mail \| \| `automation.sms_accounts` \| Gerenciar contas de SMS \| \| `automation.sms_deliveries` \| Visualizar e gerenciar envios de SMS \| \| `automation.webhook_deliveries` \| Visualizar e gerenciar entregas de webhooks \| \| `automation.webhooks` \| Gerenciar webhooks e notificações automatizadas \| \| `billing.transactions` \| Visualizar transações de cobrança e faturamento \| \| `charge.automatic_pix.locations` \| Gerenciar localizações de Pix automático \| \| `charge.automatic_pix.pix` \| Gerenciar cobranças Pix automáticas \| \| `charge.automatic_pix.recurrences` \| Gerenciar recorrências de Pix automático \| \| `charge.automatic_pix.requests` \| Gerenciar solicitações de Pix automático \| \| `charge.bank_billet_accounts` \| Gerenciar contas de boletos bancários \| \| `charge.bank_billet_payments` \| Visualizar e registrar pagamentos de boletos \| \| `charge.bank_billet_registrations` \| Gerenciar registros de boletos nos bancos \| \| `charge.bank_billets` \| Gerenciar boletos bancários \| \| `charge.customer_subscriptions` \| Gerenciar assinaturas de clientes \| \| `charge.installments` \| Gerenciar parcelamentos \| \| `charge.payments` \| Visualizar e gerenciar pagamentos recebidos \| \| `charge.pix` \| Gerenciar cobranças Pix \| \| `charge.pix_accounts` \| Gerenciar contas Pix \| \| `core.providers` \| Visualizar provedores disponíveis \| \| `crm.customers` \| Gerenciar clientes e informações comerciais \| \| `crm.people` \| Gerenciar pessoas e contatos \| \| `data.bank_billet_queries` \| Consultar informações de boletos bancários \| \| `data.pix_qrcode_queries` \| Consultar QR Codes Pix (copia e cola) \| \| `financial.accounts` \| Gerenciar contas financeiras \| \| `financial.balances` \| Visualizar saldos e movimentações financeiras \| \| `financial.providers` \| Gerenciar provedores financeiros e integrações bancárias \| \| `financial.statement_transactions` \| Visualizar transações e extratos financeiros \| \| `integration.certificates` \| Gerenciar certificados digitais para integrações \| \| `integration.connections` \| Gerenciar conexões com bancos e provedores \| \| `integration.discharges` \| Gerenciar arquivos de retorno bancário (baixas) \| \| `integration.edi_boxes` \| Gerenciar caixas postais EDI para troca de arquivos \| \| `integration.remittances` \| Gerenciar arquivos de remessa bancária \| \| `mailbox.entries` \| Gerenciar caixas postais para recebimento de arquivos \| \| `mailbox.files` \| Visualizar e gerenciar arquivos nas caixas postais \| \| `partner.bank_contracts` \| Gerenciar contratos bancários com parceiros \| \| `payment.accounts` \| Gerenciar contas de pagamento (incluindo habilitação de remessa EDI) \| \| `payment.bank_billets` \| Efetuar pagamentos de boletos bancários \| \| `payment.batches` \| Gerenciar lotes de pagamentos \| \| `payment.dda_accounts` \| Gerenciar contas DDA \| \| `payment.dda.bank_billets` \| Gerenciar boletos DDA \| \| `payment.darfs` \| Efetuar pagamentos de DARFs (tributos federais) \| \| `payment.payments` \| Gerenciar todos os tipos de pagamentos através da API \| \| `payment.pix` \| Efetuar pagamentos via Pix \| \| `payment.taxes` \| Efetuar pagamentos de impostos e taxas \| \| `payment.utilities` \| Efetuar pagamentos de contas de consumo (água, luz, etc) \| \| `security.access_tokens` \| Gerenciar tokens de acesso e autenticação \| \| `system.events` \| Visualizar eventos e logs do sistema \| \| `system.imports` \| Gerenciar importações de dados \| \| `system.reports` \| Gerar e visualizar relatórios do sistema \| \| `transfer.accounts` \| Gerenciar contas de transferência (incluindo habilitação de remessa EDI) \| \| `transfer.batches` \| Gerenciar lotes de transferências \| \| `transfer.internal` \| Realizar transferências internas entre contas \| \| `transfer.pix` \| Realizar transferências via Pix \| \| `transfer.ted` \| Realizar transferências TED \| \| `transfer.transfers` \| Gerenciar todos os tipos de transferências (Pix, TED, Internas) via endpoint unificado \| |
| `access_type` | `string` | não | Tipo de acesso Valores: `read`, `write`. |
| `expires_at` | `string (date)` | não | Data de expiração (máximo 1 ano). **Atenção:** Só pode ser definida se ainda não estava definida. Uma vez definida, não pode mais ser alterada. |
| `tags` | `array<string>` | não | Tags do token |
| `custom_data` | `object` | não | Dados customizados |
| `reminder_emails` | `array<object>` | não | E-mails para notificações de expiração |

Outros campos e objetos aninhados: `kobana schema banking.security.access-tokens.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Token atualizado |
| `404` | Token não encontrado |

## security access-tokens delete

Excluir Token de Acesso

```text
kobana banking security access-tokens delete [UID] [OPÇÕES]
```

`DELETE /v2/security/access_tokens/{uid}`

Remove permanentemente um token de acesso do sistema. Apenas tokens revogados ou expirados podem ser excluídos.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Token excluído |
| `404` | Token não encontrado |

## security access-tokens disable

Desabilitar Token de Acesso

```text
kobana banking security access-tokens disable [UID] [OPÇÕES]
```

`PUT /v2/security/access_tokens/{uid}/disable`

Desabilita um token de acesso habilitado, impedindo temporariamente seu uso para autenticação.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Token desabilitado |

## security access-tokens enable

Habilitar Token de Acesso

```text
kobana banking security access-tokens enable [UID] [OPÇÕES]
```

`PUT /v2/security/access_tokens/{uid}/enable`

Habilita um token de acesso desabilitado, permitindo seu uso para autenticação.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Token habilitado |

## security access-tokens renew

Renovar Token de Acesso

```text
kobana banking security access-tokens renew [UID] [OPÇÕES]
```

`PUT /v2/security/access_tokens/{uid}/renew`

Cria um novo token de acesso com as mesmas configurações do token atual e revoga o token antigo. O novo token é retornado na resposta.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Token renovado |

## security access-tokens revoke

Revogar Token de Acesso

```text
kobana banking security access-tokens revoke [UID] [OPÇÕES]
```

`PUT /v2/security/access_tokens/{uid}/revoke`

Revoga permanentemente um token de acesso. Tokens v2 devem estar desabilitados antes de serem revogados. Tokens v1 podem ser revogados diretamente.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Token revogado |
