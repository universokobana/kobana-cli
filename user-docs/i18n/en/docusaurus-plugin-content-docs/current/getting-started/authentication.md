---
id: authentication
title: Authentication
sidebar_label: Authentication
sidebar_position: 2
description: Sign in to each product with browser login (OAuth + PKCE) or with a token in an environment variable.
---

# Authentication

> **A token works for one product only:** products do not share credentials. A Banking Gateway token does not authenticate on Automatic Billing, and the other way around. Authenticate on each product you use.

Tokens come from the **Kobana Account** (`account.kobana.com.br`). The `aud` of each token is the product's API URL — that is why one product's token does not work for another.

## Which method to use

| Situation | Use |
|---|---|
| You, in your terminal | [Browser login](#login-in-the-browser-oauth--pkce) |
| A script or an AI agent on your machine | The login you already did (saved credentials work) or `KOBANA_<PRODUCT>_TOKEN` |
| A server, container or CI, without a browser | [Exported credentials](#servers-and-ci) or `KOBANA_<PRODUCT>_TOKEN` |
| An application, no person involved (`banking` only) | Client credentials: `--client-id` and `--client-secret` |

## Login in the browser (OAuth + PKCE)

```bash
kobana billing auth login
kobana kadu auth login
```

Login follows the same form as every command, `kobana <product> auth login`. The browser opens on the Kobana Account, you approve, and the CLI stores the token. When it expires, the CLI renews it on its own with the refresh token.

The command lists the **scopes the product publishes** (read at login time) and then the access levels, which apply to all the resources you choose:

| Level | Allows |
|---|---|
| `read` | Reading |
| `write` | Creating and changing |
| `delete` | Deleting |
| `import` | Bulk import |
| `export` | Bulk export |

One level does not imply another: `DELETE` requires `delete`, and importing requires `import`.

```bash
# Scopes without prompting (outside a terminal, the default is all resources + read)
kobana billing auth login --scopes "billing/customers,billing/plans,read,write"

# Client credentials — banking only (application token, no person involved)
kobana banking auth login --client-id <ID> --client-secret <SECRET>

kobana billing auth status      # who am I, until when it is valid
kobana billing auth export      # prints the credentials JSON
kobana billing auth logout
```

Credentials are stored one per product and environment (`~/.config/kobana/credentials-<product>-<environment>.enc`), encrypted with AES-256-GCM. The key lives in the system keyring (Keychain on macOS, for example), with a file as fallback.

## Token in an environment variable

For scripts and CI, export the product's token:

```bash
export KOBANA_FINANCE_TOKEN=...
kobana finance payables list
```

| Product | Variable |
|---|---|
| `banking` | `KOBANA_BANKING_TOKEN` |
| `finance` | `KOBANA_FINANCE_TOKEN` |
| `billing` | `KOBANA_BILLING_TOKEN` |
| `inbox` | `KOBANA_INBOX_TOKEN` |
| `dunning` | `KOBANA_DUNNING_TOKEN` |
| `dashboards` | `KOBANA_DASHBOARDS_TOKEN` |
| `kadu` | `KOBANA_KADU_TOKEN` or `KADU_TOKEN` |

On `banking`, the API token from *Integrations › API › API Token* in the Kobana interface also works.

## Servers and CI

Where there is no browser, log in on a machine that has one and carry the credentials over:

```bash
# On your machine
kobana billing auth login
kobana billing auth export > credentials.json

# On the server
export KOBANA_CREDENTIALS_FILE=/path/credentials.json
kobana billing invoices list
```

The file is what `auth export` prints, it needs the `product` field, and it is valid only for that product (and for the file's `environment`, if present). Treat it as a secret: keep it in the CI vault, not in the repository. If the token expires, the CLI renews it with the refresh token in the file.

Even simpler, when you already have the token: store it as a CI secret and export `KOBANA_<PRODUCT>_TOKEN` in the job.

```yaml
# GitHub Actions
- run: kobana billing invoices list --page-all --fields "id,status"
  env:
    KOBANA_BILLING_TOKEN: ${{ secrets.KOBANA_BILLING_TOKEN }}
```

## Resolution order

| Priority | Source | Setting |
|---|---|---|
| 1 | Product token | `KOBANA_<PRODUCT>_TOKEN` |
| 2 | Credentials file | `KOBANA_CREDENTIALS_FILE` |
| 3 | Saved credentials | `kobana <product> auth login` |

## Workspace (`X-Workspace-Id`)

On `finance`, every route acts on **one** workspace, chosen by the `X-Workspace-Id` header; without it the API responds `400 WORKSPACE_REQUIRED`. On `dashboards` and `dunning` the header is optional while the organization has a single workspace.

The CLI fills in the header from `KOBANA_WORKSPACE_ID`. To switch workspace for a single call, pass it in `--params`, which takes precedence:

```bash
export KOBANA_WORKSPACE_ID=workspace-uuid
kobana finance payables list
kobana finance payables list --params '{"X-Workspace-Id": "other-uuid"}'
```

## Environments

The default is **production**. Switch with `--env` or `KOBANA_ENVIRONMENT`:

```bash
kobana banking charge pix list --env sandbox
export KOBANA_ENVIRONMENT=sandbox
kobana billing auth login --env sandbox
```

Tokens are different in each environment. The hosts for each product are in [Products](/reference/products). `--env sandbox` on a product without a sandbox ends with an error, so a test is never sent to production.
