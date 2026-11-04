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

## Filter with `--jq`

`--jq` applies a [jq](https://jqlang.github.io/jq/manual/) expression to the JSON output, like `gh`'s `--jq` — with no `jq` installed. Each result goes on its own line: a string as is (like `jq -r`), the rest as JSON, indented only on a terminal.

```bash
kobana banking bank-billets list --jq '.[].id'
kobana billing invoices list --page-all --jq 'select(.status == "open") | [.id, .total] | @tsv'
kobana banking auth status --jq .authenticated
```

- It applies to every JSON output: API responses (including each `--page-all` item), `--dry-run`, `kobana schema`, `auth status` and `kobana update` (which, with `--jq`, only reports).
- It runs after `--fields`, and does not go with `--output-format table` or `csv`.
- The expression is checked before any request: a syntax error exits with code `3` without sending anything.
- If the expression fails on the response of a write (`create`, `update`, `delete`, helpers), the CLI warns on stderr and prints the response unfiltered, with code `0`: the operation went through, and repeating it would not be safe.
- On a terminal, a string's control characters become `�`, so that text from the API cannot send escape sequences to the screen.
- `env` and `$ENV` do not exist in `--jq`, so that an expression — written by an agent, for example — cannot read the tokens in the environment; neither do modules (`import`, `include`) and `input`.

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
