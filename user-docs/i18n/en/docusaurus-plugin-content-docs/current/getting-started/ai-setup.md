---
id: ai-setup
title: Set up with an AI
sidebar_label: Set up with an AI
sidebar_position: 4
description: Let an AI agent install the docs MCP, the CLI and the skills, and guide the login.
---

# Set up with an AI

Let an AI agent get the environment ready: install the docs MCP, the CLI and the skills, and guide the login to each product. It works best with agents that **run commands on your machine**, because installing the CLI and logging in only happens there.

## Agents with a terminal (recommended)

Run your agent's command. It reads this page and does the steps, asking for confirmation before installing:

```bash
claude "Follow the instructions at https://cli.kobana.com.br/en/getting-started/ai-setup.md to set up my Kobana CLI environment."
codex "Follow the instructions at https://cli.kobana.com.br/en/getting-started/ai-setup.md to set up my Kobana CLI environment."
gemini "Follow the instructions at https://cli.kobana.com.br/en/getting-started/ai-setup.md to set up my Kobana CLI environment."
```

In **Cursor**, use the "Open in Cursor" button on the [home page](/). In other agents (Copilot agent in VS Code, Windsurf, Cline…), paste the same message: "Follow the instructions at this address".

## Web chats

ChatGPT, Claude and Gemini on the web **do not run commands on your machine**: they only explain. Use them to understand the CLI and ask questions — and, if you want, connect the [Docs MCP](/docs-mcp) as a connector so they answer from these pages. Run the installation yourself with the commands they give you.

## Instructions for the agent

You are helping a person get their environment ready to use the Kobana CLI (the `kobana` command, which reaches the APIs of every Kobana product). Follow the steps in order. Before each command that installs or changes something, explain what it does and ask for confirmation. If you cannot run commands, give them to the person to run.

1. **Docs MCP.** Connect the public, read-only MCP server `https://cli.kobana.com.br/mcp` to your client to read the documentation. Per-client instructions in [Docs MCP](/docs-mcp). If unsure: `npx add-mcp "https://cli.kobana.com.br/mcp"`.
2. **CLI.** On macOS and Linux: `curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash` (or `brew tap universokobana/tap && brew install kobana`). On Windows, use the binary from the [Releases page](https://github.com/universokobana/kobana-cli/releases/latest). Check with `kobana --version`.
3. **Agent skills.** Ask which products the person uses (`banking`, `finance`, `billing`, `inbox`, `dunning`, `dashboards`, `kadu`) and install the `kobana-shared` skill and the ones for those products (for banking, start with `kobana-banking`). They live in [`skills/`](https://github.com/universokobana/kobana-cli/tree/main/skills); copy the folders into this client's skills folder. See [Skills for AI agents](/guides/agent-skills).
4. **Authentication.** Guide the person through logging in to each product with `kobana <product> auth login` (each product has its own credential). Never ask for tokens or passwords in the chat. See [Authentication](/getting-started/authentication).
5. **Test.** For each product the person logged in to, run `kobana <product> me` (it shows who the token belongs to, its scopes and expiry, without exposing the token) and say whether it worked. If it fails, use [Troubleshooting](/guides/troubleshooting).

From here on: always use `--dry-run` before any command that creates, changes or deletes, and `--env sandbox` to test. The plain-text documentation is at [`/en/llms.txt`](https://cli.kobana.com.br/en/llms.txt).
