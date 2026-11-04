---
id: environment-variables
title: Environment variables
sidebar_label: Environment variables
sidebar_position: 2
description: All the variables kobana reads — credentials, configuration and logs — and the rules for the .env file.
---

# Environment variables

## Authentication

| Variable | Description |
|---|---|
| `KOBANA_<PRODUTO>_TOKEN` | A product's (Bearer) token: `KOBANA_BANKING_TOKEN`, `KOBANA_FINANCE_TOKEN`… It has the highest priority. Kadu also reads `KADU_TOKEN` |
| `KOBANA_CREDENTIALS_FILE` | Path to a credentials JSON (the `auth export` format). It needs the `product` field; it applies only to that product and to the file's `environment` (without it, any environment) |
| `KOBANA_CLIENT_ID` | Login Client ID (default: `kobana-cli`) |
| `KOBANA_CLIENT_SECRET` | Client secret. Like `--client-secret`, it selects the Client Credentials flow (`banking` only); a person's login (PKCE) does not use a secret |
| `KOBANA_ACCOUNT_URL` | Address of the Kobana Account used for login (default: the environment's one) |
| `KOBANA_WORKSPACE_ID` | Workspace sent in `X-Workspace-Id` by `finance` (required), `dashboards` and `dunning`. A value in `--params` wins |

## Configuration

| Variable | Description |
|---|---|
| `KOBANA_CONFIG_DIR` | Configuration folder (default: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `production` (default), `sandbox` or `development`; any other value is an error |
| `KOBANA_LANG` | `pt`, `en` or `es`. `--lang` wins |

## Accessibility and terminal

See [Accessibility](/usage/accessibility). The `KOBANA_*` ones apply with any value except empty, `0`, `false`, `no` and `off`.

| Variable | Description |
|---|---|
| `NO_COLOR` | Any non-empty value turns colors off (like `--no-color`) |
| `CLICOLOR` | `0` turns colors off |
| `CLICOLOR_FORCE` | Other than `0`: colors even outside a terminal (loses to `NO_COLOR` and `--no-color`) |
| `KOBANA_ACCESSIBLE_COLORS` | High contrast: bold and underline in the theme's text color, no hues |
| `KOBANA_SCREEN_READER` | Screen reader mode: no decorative symbols, tables without borders, simple prompts |
| `KOBANA_ACCESSIBLE_PROMPTER` | The simple prompts alone (numbered list, answer on one line) |
| `KOBANA_PROMPT_DISABLED` | Never asks; the command line decides. Without a terminal (pipe, CI, agent), the CLI does not ask anyway |
| `KOBANA_NO_UPDATE_NOTIFIER` | No new-version notice (which never shows outside a terminal or with `CI`) |

## Logs

| Variable | Description |
|---|---|
| `KOBANA_LOG` | Log level on stderr, for example `kobana=debug`. Off by default |
| `KOBANA_LOG_FILE` | Folder for JSON logs with daily rotation. Off by default |

## `.env` file

Any variable can also live in a `.env`: the one in the current folder (never the ones in parent folders) and the one in the configuration folder. A variable already set in the environment wins over both.

> **The `.env` in the current folder does not define credentials.** It may come from a third-party repository, which could send the login to another server, store credentials inside it or make the CLI act with someone else's token. That is why `KOBANA_ACCOUNT_URL`, `KOBANA_CONFIG_DIR`, `KOBANA_CREDENTIALS_FILE`, `KOBANA_CLIENT_ID`, `KOBANA_CLIENT_SECRET` and all tokens (`KOBANA_*_TOKEN`, `KADU_TOKEN`) are ignored in it — the CLI warns you. Set those in the environment or in the `.env` of the configuration folder.
