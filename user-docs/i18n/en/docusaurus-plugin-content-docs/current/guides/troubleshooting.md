---
id: troubleshooting
title: Troubleshooting
sidebar_label: Troubleshooting
sidebar_position: 3
description: The most common kobana errors — authentication, permissions, environment, workspace — and how to fix them.
---

# Troubleshooting

API, authentication and validation errors come out as JSON on stdout, with `error.code` and `error.message`; the process [exit code](/reference/exit-codes) tells you the category. Run with `--verbose` to see the request and the response (without the token).

## Authentication (exit code 2)

### `No authentication configured for <product>`

There is no token for that product. Each product has its own:

```bash
kobana billing auth login            # or
export KOBANA_BILLING_TOKEN=...
kobana billing auth status           # checks what is in effect
```

`auth status` exits with code `0` even without a credential: check the `authenticated` field in the output. A token from another product does not work — see [Authentication](/getting-started/authentication).

### API `401`

The token exists, but it is expired, belongs to another product or to another environment. Sandbox and production tokens are different. Log in again (`kobana <product> auth login`) in the right environment (`--env`).

### API `403`

The token is valid, but it lacks permission. Common causes:

- **Missing access level.** `DELETE` requires `delete`; import requires `import`; writing requires `write`. Log in again choosing the levels, or use `--scopes`.
- **Missing resource scope.** The login lists the product's scopes (`billing/customers`, `inbox/emails`…); a resource outside them is refused.

### The login opens the browser in the wrong place

The login uses the Kobana Account of the environment (`account.kobana.com.br`, `account.sandbox.kobana.com.br`). `KOBANA_ACCOUNT_URL` changes the address; check that no old variable is exported.

## Environment

### `<product> has no sandbox environment`

`inbox`, `dashboards` and `kadu` have no sandbox, and the CLI refuses instead of sending the test to production. Run without `--env sandbox`, carefully, or use `--dry-run`.

## Workspace

### `400 WORKSPACE_REQUIRED` on `finance`

Every `finance` route needs `X-Workspace-Id`:

```bash
export KOBANA_WORKSPACE_ID=workspace-uuid
# or, in a single call:
kobana finance payables list --params '{"X-Workspace-Id": "uuid"}'
```

`kobana dunning workspaces list` shows the workspaces the token can reach.

## Command line (exit code 3)

### `unrecognized subcommand` or `unexpected argument`

The command or the flag does not exist. See what is available at that level:

```bash
kobana billing --help
kobana billing plans --help
```

Command names changed in older versions, but the old names still work as aliases. If a command "disappeared", look for it in the [Manual](/manual).

### `Validation error: id cannot contain …`

The CLI refuses identifiers containing `/`, `\`, `..`, `?`, `#`, `%` or control characters before calling the API. This is on purpose: it protects against path injection. Pass only the id.

### Invalid JSON in `--params` or `--json`

Use single quotes outside and double quotes inside (`--json '{"amount": 99.90}'`). In PowerShell, escape the inner double quotes or use a file.

## Variables and `.env`

### A token in the `.env` was ignored

The `.env` in the current folder cannot define tokens or credentials, because it may come from a third-party repository. The CLI warns about it. Set them in the environment or in the `.env` of the configuration folder (`~/.config/kobana`). See [Environment variables](/reference/environment-variables).

## Output

### Huge response

Use `--fields` to limit the columns and `--page-limit` to limit the pages. For sums, use the `aggregates` endpoints.

### `--output` refused

The path cannot contain `..`, be a symbolic link, a folder or a device.

### The "New version available" notice

It is a notice on stderr, once a day. `kobana update` installs it. It affects neither stdout nor the exit code.

## Still not solved?

Run the command with `--verbose` and `KOBANA_LOG=kobana=debug`, and open an [issue](https://github.com/universokobana/kobana-cli/issues) with the command (without tokens), the output and `kobana --version`.
