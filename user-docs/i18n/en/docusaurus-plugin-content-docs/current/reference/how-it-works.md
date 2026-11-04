---
id: how-it-works
title: How it works
sidebar_label: How it works
sidebar_position: 4
description: How kobana builds its commands from the OpenAPI specifications embedded in the binary.
---

# How it works

`kobana` **has no hand-written list of endpoints**. Each Kobana product publishes an OpenAPI 3.1 specification, and the binary loads those specifications and builds the command tree from them on every run.

## Two phases

1. **Discover the product.** The CLI reads the first argument (`billing`, `finance`…) and loads that product's specification, already embedded in the binary.
2. **Build and run.** The command tree comes from it: each path becomes a resource, each operation becomes a command. Only then are the remaining arguments read, the product credential resolved, and the request built and sent.

The result is that `--help` at any level, the [Manual](/manual) and [`kobana schema`](/usage/schema) all answer from the same source: the specification.

## What this means for you

- **No command is left out of the API.** When a product publishes a new endpoint, the next version of the binary already includes it — with no per-endpoint code.
- **No network needed to discover commands.** The specification is in the binary: `--help`, `schema` and `--dry-run` work offline.
- **The API version does not appear in the command.** `kobana banking bank-billets list` goes to `/v1/bank_billets` and `kobana banking charge pix list` goes to `/v2/charge/pix`.
- **Names follow how you speak.** A resource that only reads runs by its name (`kobana banking me`), and the route ids are arguments (`bank-billets get 123`). See [Syntax](/usage/syntax).
- **Old names do not break scripts.** When a naming rule changes, the old name keeps working as an alias.
- **All text comes translated.** The specification descriptions are replaced by the translation for the chosen language ([Languages](/usage/languages)).

## Helpers stay outside

The few hand-written commands are the [helpers](/usage/helpers) (`+emitir`, `+cobrar`…). They start with `+` precisely so they never collide with a generated command.

## Open source

The CLI is open source (MIT license) at [github.com/universokobana/kobana-cli](https://github.com/universokobana/kobana-cli), and is made of two parts: a library (HTTP client, errors, specification reading, input validation) and the binary (commands, authentication, output, pagination, helpers).
