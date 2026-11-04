---
id: docs-mcp
title: Docs MCP
sidebar_label: Docs MCP
sidebar_position: 3
slug: /docs-mcp
---

# Docs MCP

This documentation has **its own MCP server**. It lets an AI assistant (Claude,
ChatGPT, Cursor, VS Code, Claude Code) search and read the pages of this site
without you copying and pasting anything, so answers about the `kobana` commands and
flags come from the published content, in the language you choose.

```text
https://cli.kobana.com.br/mcp
```


## How it works

- **Public and read-only:** no sign-in, no token. It exposes the same content
  anyone sees on the site.
- **Always current:** it reads the published version of the documentation. A
  new or changed page shows up in the MCP as soon as the site is published.
- **Three languages:** Portuguese (`pt`), English (`en`) and Spanish (`es`).
  Without a language, it uses Portuguese.
- **Transport:** Streamable HTTP, sessionless. Any MCP client that connects to a
  remote server by URL works.

## Tools

| Tool | What it does | Parameters |
|---|---|---|
| `search_docs` | Searches pages by keywords in the title, path and summary, ignoring accents and case. Returns the best pages with path and summary | `query` (required), `locale`, `limit` (1 to 25, default 8) |
| `get_page` | Returns the full content of a page as Markdown | `path`: the path returned by `search_docs` or `list_pages` (e.g. `/en/getting-started/installation`) or the full URL |
| `list_pages` | Lists every page, with path and summary | `locale` |

The assistant decides on its own when to call each tool. It usually searches
with `search_docs` and then reads the relevant pages with `get_page`.

## Installing

The quickest way is the **Copy page** menu at the top of every page of this
site. Under **Docs MCP** you'll find:

- **Connect to Cursor** and **Connect to VS Code:** install the server in one
  click.
- **Copy MCP config:** copies the JSON block for clients configured by file.
- **Copy MCP command:** copies the install command for the terminal.

If you'd rather configure it by hand, the steps for each client are below.

### Any client, from the terminal

[`add-mcp`](https://www.npmjs.com/package/add-mcp) detects the clients
installed on your machine and adds the server to them:

```bash
npx add-mcp "https://cli.kobana.com.br/mcp"
```

### Claude Code

```bash
claude mcp add --transport http kobana-cli-docs https://cli.kobana.com.br/mcp
```

### Cursor

Use **Connect to Cursor** in the page menu, or add this to `~/.cursor/mcp.json`
(all projects) or the project's `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "kobana-cli-docs": {
      "url": "https://cli.kobana.com.br/mcp"
    }
  }
}
```

### VS Code

Use **Connect to VS Code** in the page menu, or create `.vscode/mcp.json` in the
project. In VS Code the key is `servers`, not `mcpServers`:

```json
{
  "servers": {
    "kobana-cli-docs": {
      "type": "http",
      "url": "https://cli.kobana.com.br/mcp"
    }
  }
}
```

### Claude and ChatGPT

In **Settings › Connectors**, add a custom connector and paste the URL
`https://cli.kobana.com.br/mcp`. There is no sign-in.

### Other clients configured by file

```json
{
  "mcpServers": {
    "kobana-cli-docs": {
      "type": "http",
      "url": "https://cli.kobana.com.br/mcp"
    }
  }
}
```

## Example questions

With the server connected, just ask. The assistant checks the documentation
before answering:

- "How do I install kobana and authenticate to Automatic Billing?"
- "How do I fetch every page of a listing and save it as CSV?"
- "What are the exit codes and how do I use them in a script?"
- "Build a command that creates a Pix charge with `--dry-run`."

## Testing without a client

```bash
curl -s https://cli.kobana.com.br/mcp \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"search_docs","arguments":{"query":"webhook","locale":"en"}}}'
```

## Limits

- The parameters of each endpoint are in the [Manual](/manual); for the API itself, use each product's documentation (`docs.<product>.kobana.com.br`).
- Search is by keywords, not semantic. If nothing comes up, try other terms or
  ask the assistant to use `list_pages`.
- You can also read any page as Markdown by appending `.md` to its URL, or
  download the whole documentation at
  [`llms-full.txt`](https://cli.kobana.com.br/en/llms-full.txt).
