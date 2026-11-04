---
id: quickstart
title: First commands
sidebar_label: First commands
sidebar_position: 3
description: From login to your first call in five minutes.
---

# First commands

## 1. Install and sign in

```bash
curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash
kobana billing auth login
```

Check that it worked with `me`, which shows who the token belongs to (person and organization), its scopes and expiry, without exposing the token:

```bash
kobana billing me
```

If it fails, see [Troubleshooting](/guides/troubleshooting).

## 2. Find out what exists

The whole surface shows up in `--help`, at any level:

```bash
kobana --help                     # the products
kobana billing --help             # the Automatic Billing resources
kobana billing invoices --help    # the invoice commands
```

Everything the CLI accepts is also in the [Manual](/manual).

## 3. Read data

```bash
kobana billing subscriptions list --fields "id,status,plan_id"
kobana billing invoices get INVOICE_ID
```

Output is JSON by default. For a table: `--output-format table`. See [Output](/usage/output).

## 4. Filter and paginate

```bash
kobana billing subscriptions list --params '{"status": "active"}'
kobana billing subscriptions list --page-all --fields "id,status"
```

`--page-all` walks through all the pages and prints one JSON line per record. See [Pagination](/usage/pagination).

## 5. Change data safely

Before writing, look at the request that would be sent:

```bash
kobana billing invoices finalize INVOICE_ID --dry-run
```

Only then remove `--dry-run`. See [Safe writes](/usage/safe-writes).

## 6. Ask the schema

```bash
kobana schema billing.subscriptions.create
```

It shows parameters, required fields and responses, without leaving the terminal. See [Schema](/usage/schema).

## Next steps

- [Command syntax](/usage/syntax)
- [Scripts and AI agents](/usage/scripting)
- [Exit codes](/reference/exit-codes)
