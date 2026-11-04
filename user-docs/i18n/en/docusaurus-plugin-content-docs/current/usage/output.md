---
id: output
title: Output
sidebar_label: Output
sidebar_position: 2
description: JSON by default, table and CSV when you want, field selection and saving to a file.
---

# Output

The response goes to **stdout**; messages, warnings and `--verbose` go to **stderr**. This way, `kobana … | jq` and `kobana … > file` work without noise.

## Format

```bash
kobana banking bank-billets list                         # JSON (default)
kobana banking bank-billets list --output-format table   # table
kobana banking bank-billets list --output-format csv     # CSV
```

## Only the fields that matter

`--fields` limits the response to a comma-separated list. It is the cheapest way to read data — and to avoid filling an agent's context window:

```bash
kobana banking bank-billets list --fields "id,amount,status,due_at"
```

## Save to a file

```bash
kobana banking bank-billets get 12345 --output boleto.json
```

`--output` refuses paths with `..`, symbolic links, folders and devices.

## See the conversation with the API

`--verbose` prints the request and response to stderr, without the token:

```bash
kobana billing plans list --verbose
```

For structured logging, use `KOBANA_LOG=kobana=debug` (stderr) or `KOBANA_LOG_FILE=./logs` (JSON files with daily rotation). See [Environment variables](/reference/environment-variables).
