---
id: authentication
title: Autenticação
sidebar_label: Autenticação
sidebar_position: 2
description: Entre em cada produto com login pelo navegador (OAuth + PKCE) ou com um token em variável de ambiente.
---

# Autenticação

> **Um token vale para um produto só:** os produtos não compartilham credenciais. Um token do Gateway Bancário não autentica no Faturamento Automático, e o contrário também. Autentique-se em cada produto que for usar.

Os tokens vêm da **Conta Kobana** (`account.kobana.com.br`). O `aud` de cada um é a URL da API do produto — por isso o token de um não serve para outro.

## Qual método usar

| Situação | Use |
|---|---|
| Você, no seu terminal | [Login pelo navegador](#login-pelo-navegador-oauth--pkce) |
| Um script ou um agente de IA na sua máquina | O login já feito (as credenciais salvas valem) ou `KOBANA_<PRODUTO>_TOKEN` |
| Um servidor, contêiner ou CI, sem navegador | [Credenciais exportadas](#servidores-e-ci) ou `KOBANA_<PRODUTO>_TOKEN` |
| Uma aplicação, sem pessoa (só no `banking`) | Client credentials: `--client-id` e `--client-secret` |

## Login pelo navegador (OAuth + PKCE)

```bash
kobana billing auth login
kobana kadu auth login
```

O login segue a forma de todo comando, `kobana <produto> auth login`. O navegador abre na Conta Kobana, você aprova, e o CLI guarda o token. Quando ele vence, o CLI o renova sozinho com o refresh token.

O comando lista os **escopos que o produto publica** (lidos na hora do login) e, em seguida, os níveis de acesso, que valem para todos os recursos escolhidos:

| Nível | Libera |
|---|---|
| `read` | Leitura |
| `write` | Criação e alteração |
| `delete` | Exclusão |
| `import` | Importação em volume |
| `export` | Exportação em volume |

Um nível não implica outro: `DELETE` exige `delete`, e importar exige `import`.

```bash
# Escopos sem perguntar (fora de um terminal, o padrão é todos os recursos + read)
kobana billing auth login --scopes "billing/customers,billing/plans,read,write"

# Client credentials — só no banking (token de aplicação, sem pessoa)
kobana banking auth login --client-id <ID> --client-secret <SECRET>

kobana billing auth status      # quem sou eu, até quando vale
kobana billing auth export      # imprime o JSON das credenciais
kobana billing auth logout
```

As credenciais ficam uma por produto e ambiente (`~/.config/kobana/credentials-<produto>-<ambiente>.enc`), criptografadas com AES-256-GCM. A chave fica no keyring do sistema (Keychain no macOS, por exemplo), com arquivo como alternativa.

## Token por variável de ambiente

Para scripts e CI, exporte o token do produto:

```bash
export KOBANA_FINANCE_TOKEN=...
kobana finance payables list
```

| Produto | Variável |
|---|---|
| `banking` | `KOBANA_BANKING_TOKEN` |
| `finance` | `KOBANA_FINANCE_TOKEN` |
| `billing` | `KOBANA_BILLING_TOKEN` |
| `inbox` | `KOBANA_INBOX_TOKEN` |
| `dunning` | `KOBANA_DUNNING_TOKEN` |
| `dashboards` | `KOBANA_DASHBOARDS_TOKEN` |
| `kadu` | `KOBANA_KADU_TOKEN` ou `KADU_TOKEN` |

No `banking`, o token de API de *Integrações › API › Token de API*, na interface da Kobana, também serve.

## Servidores e CI

Onde não há navegador, faça o login numa máquina que tenha um e leve as credenciais:

```bash
# Na sua máquina
kobana billing auth login
kobana billing auth export > credentials.json

# No servidor
export KOBANA_CREDENTIALS_FILE=/caminho/credentials.json
kobana billing invoices list
```

O arquivo é o que `auth export` imprime, precisa do campo `product` e vale só para esse produto (e para o `environment` do arquivo, se houver). Trate-o como um segredo: guarde-o no cofre do CI, não no repositório. Se o token vencer, o CLI o renova com o refresh token do arquivo.

Mais simples ainda, quando você já tem o token: guarde-o como segredo do CI e exporte `KOBANA_<PRODUTO>_TOKEN` no job.

```yaml
# GitHub Actions
- run: kobana billing invoices list --page-all --fields "id,status"
  env:
    KOBANA_BILLING_TOKEN: ${{ secrets.KOBANA_BILLING_TOKEN }}
```

## Ordem de resolução

| Prioridade | Fonte | Configuração |
|---|---|---|
| 1 | Token do produto | `KOBANA_<PRODUTO>_TOKEN` |
| 2 | Arquivo de credenciais | `KOBANA_CREDENTIALS_FILE` |
| 3 | Credenciais salvas | `kobana <produto> auth login` |

## Espaço de trabalho (`X-Workspace-Id`)

No `finance`, toda rota age sobre **um** espaço de trabalho, escolhido pelo cabeçalho `X-Workspace-Id`; sem ele a API responde `400 WORKSPACE_REQUIRED`. No `dashboards` e no `dunning` o cabeçalho é opcional enquanto a organização tem um único workspace.

O CLI preenche o cabeçalho com `KOBANA_WORKSPACE_ID`. Para trocar de espaço numa chamada só, passe-o em `--params`, que tem precedência:

```bash
export KOBANA_WORKSPACE_ID=uuid-do-espaco
kobana finance payables list
kobana finance payables list --params '{"X-Workspace-Id": "outro-uuid"}'
```

## Ambientes

O padrão é **produção**. Troque com `--env` ou `KOBANA_ENVIRONMENT`:

```bash
kobana banking charge pix list --env sandbox
export KOBANA_ENVIRONMENT=sandbox
kobana billing auth login --env sandbox
```

Os tokens são diferentes em cada ambiente. Os hosts de cada produto estão em [Produtos](/reference/products). `--env sandbox` num produto sem sandbox termina com erro, para nunca mandar um teste para produção.
