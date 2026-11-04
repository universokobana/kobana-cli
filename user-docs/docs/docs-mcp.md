---
id: docs-mcp
title: MCP da Documentação
sidebar_label: MCP da Documentação
sidebar_position: 3
slug: /docs-mcp
---

# MCP da Documentação

Esta documentação tem um **servidor MCP próprio**. Com ele, um assistente de IA
(Claude, ChatGPT, Cursor, VS Code, Claude Code) busca e lê as páginas deste
site sem que você precise copiar e colar nada. Assim, as respostas sobre o
comandos e flags do `kobana` saem do conteúdo publicado, no idioma que você escolher.

```text
https://cli.kobana.com.br/mcp
```


## Como funciona

- **Público e só de leitura:** não pede login nem token. Expõe o mesmo conteúdo
  que qualquer pessoa vê no site.
- **Sempre atualizado:** lê a versão publicada da documentação. Uma página nova
  ou alterada aparece no MCP assim que o site é publicado.
- **Três idiomas:** português (`pt`), inglês (`en`) e espanhol (`es`). Sem
  idioma informado, usa o português.
- **Transporte:** Streamable HTTP, sem sessão. Qualquer cliente MCP que conecte
  a um servidor remoto por URL funciona.

## Tools

| Tool | O que faz | Parâmetros |
|---|---|---|
| `search_docs` | Busca páginas por palavras-chave, no título, no caminho e no resumo, sem diferenciar acento nem maiúsculas. Devolve as melhores páginas com caminho e resumo | `query` (obrigatório), `locale`, `limit` (1 a 25, padrão 8) |
| `get_page` | Devolve o conteúdo completo de uma página em Markdown | `path`: o caminho que `search_docs` ou `list_pages` devolveu (por exemplo, `/pt/getting-started/installation`) ou a URL completa |
| `list_pages` | Lista todas as páginas, com caminho e resumo | `locale` |

O assistente decide sozinho quando chamar cada tool. Normalmente ele busca com
`search_docs` e lê com `get_page` as páginas que interessam.

## Como instalar

O jeito mais rápido é pelo menu **Copiar página**, no topo de cada página
deste site. Em **MCP da Documentação**, você encontra:

- **Conectar ao Cursor** e **Conectar ao VS Code:** instalam o servidor com um
  clique.
- **Copiar config do MCP:** copia o bloco JSON para clientes configurados por
  arquivo.
- **Copiar comando do MCP:** copia o comando de instalação para o terminal.

Se preferir configurar à mão, os passos de cada cliente estão abaixo.

### Qualquer cliente, pelo terminal

O [`add-mcp`](https://www.npmjs.com/package/add-mcp) detecta os clientes
instalados na máquina e adiciona o servidor a eles:

```bash
npx add-mcp "https://cli.kobana.com.br/mcp"
```

### Claude Code

```bash
claude mcp add --transport http kobana-cli-docs https://cli.kobana.com.br/mcp
```

### Cursor

Use **Conectar ao Cursor** no menu da página, ou adicione ao
`~/.cursor/mcp.json` (para todos os projetos) ou ao `.cursor/mcp.json` do
projeto:

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

Use **Conectar ao VS Code** no menu da página, ou crie o `.vscode/mcp.json` no
projeto. No VS Code a chave é `servers`, não `mcpServers`:

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

### Claude e ChatGPT

Em **Configurações › Conectores**, adicione um conector personalizado e cole a
URL `https://cli.kobana.com.br/mcp`. Não há login a fazer.

### Outros clientes configurados por arquivo

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

## Exemplos de perguntas

Com o servidor conectado, pergunte normalmente. O assistente consulta a
documentação antes de responder:

- "Como instalo o kobana e me autentico no Faturamento Automático?"
- "Como listo todas as páginas de um recurso e salvo em CSV?"
- "Quais são os códigos de saída e como uso num script?"
- "Monte um comando para criar uma cobrança Pix com `--dry-run`."

## Testar sem cliente

```bash
curl -s https://cli.kobana.com.br/mcp \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"search_docs","arguments":{"query":"webhook","locale":"pt"}}}'
```

## Limites

- Os parâmetros de cada endpoint ficam no [Manual](/manual); para a API em si, use a documentação de cada produto (`docs.<produto>.kobana.com.br`).
- A busca é por palavras-chave, não semântica. Se não encontrar, tente outros
  termos ou peça ao assistente para usar `list_pages`.
- Também dá para ler qualquer página como Markdown acrescentando `.md` à URL,
  ou baixar a documentação inteira em [`llms-full.txt`](https://cli.kobana.com.br/pt/llms-full.txt).
