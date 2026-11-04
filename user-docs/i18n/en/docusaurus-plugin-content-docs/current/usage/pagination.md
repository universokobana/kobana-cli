---
id: pagination
title: Pagination
sidebar_label: Pagination
sidebar_position: 3
description: Walk through all the pages of a listing with --page-all and receive NDJSON.
---

# Pagination

A listing returns one page per call. With `--page-all` the CLI walks through the pages and prints **one record per line**, in [NDJSON](https://github.com/ndjson/ndjson-spec):

```bash
kobana billing invoices list --page-all --fields "id,status,total"
```

Each line is a complete JSON object, so the output can be processed as it arrives:

```bash
kobana kadu executions list --page-all | jq -r 'select(.status == "failed") | .id'
```

## How it paginates

- If the endpoint declares a `cursor` parameter, the CLI follows `meta.next_cursor`.
- Otherwise, it uses `page` and `per_page`.

## Limits

| Flag | Default | What for |
|---|---|---|
| `--page-limit <N>` | `10` | Maximum number of pages fetched |
| `--page-delay <MS>` | `100` | Interval between pages, in milliseconds |

Raise `--page-limit` on purpose: it exists so that a huge listing does not turn into thousands of calls without you noticing.

```bash
kobana banking charge pix list --page-all --page-limit 50 --page-delay 250
```
