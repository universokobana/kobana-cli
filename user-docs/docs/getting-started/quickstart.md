---
id: quickstart
title: Primeiros comandos
sidebar_label: Primeiros comandos
sidebar_position: 3
description: Do login à primeira chamada em cinco minutos.
---

# Primeiros comandos

## 1. Instale e entre

```bash
curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash
kobana billing auth login
```

Confira se deu certo com o `me`, que mostra a quem pertence o token (pessoa e organização), os escopos e a validade, sem expor o token:

```bash
kobana billing me
```

Se falhar, veja [Solução de problemas](/guides/troubleshooting).

## 2. Descubra o que existe

Toda a superfície aparece no `--help`, em qualquer nível:

```bash
kobana --help                     # os produtos
kobana billing --help             # os recursos do Faturamento Automático
kobana billing invoices --help    # os comandos de faturas
```

Tudo que o CLI aceita também está no [Manual](/manual).

## 3. Leia dados

```bash
kobana billing subscriptions list --fields "id,status,plan_id"
kobana billing invoices get INVOICE_ID
```

A saída é JSON por padrão. Para uma tabela: `--output-format table`. Veja [Saída](/usage/output).

## 4. Filtre e pagine

```bash
kobana billing subscriptions list --params '{"status": "active"}'
kobana billing subscriptions list --page-all --fields "id,status"
```

`--page-all` percorre todas as páginas e imprime uma linha JSON por registro. Veja [Paginação](/usage/pagination).

## 5. Altere com segurança

Antes de gravar, veja a requisição que seria enviada:

```bash
kobana billing invoices finalize INVOICE_ID --dry-run
```

Só depois tire o `--dry-run`. Veja [Alterações seguras](/usage/safe-writes).

## 6. Pergunte ao schema

```bash
kobana schema billing.subscriptions.create
```

Mostra parâmetros, campos obrigatórios e respostas, sem sair do terminal. Veja [Schema](/usage/schema).

## Próximos passos

- [Sintaxe dos comandos](/usage/syntax)
- [Scripts e agentes de IA](/usage/scripting)
- [Códigos de saída](/reference/exit-codes)
