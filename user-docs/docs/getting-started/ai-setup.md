---
id: ai-setup
title: Configurar com uma IA
sidebar_label: Configurar com uma IA
sidebar_position: 4
description: Deixe um agente de IA instalar o MCP da documentação, o CLI e as skills, e guiar o login.
---

# Configurar com uma IA

Deixe um agente de IA preparar o ambiente: instalar o MCP da documentação, o CLI e as skills, e guiar o login de cada produto. Funciona melhor com agentes que **rodam comandos na sua máquina**, porque instalar o CLI e fazer o login só acontece aí.

## Agentes com terminal (recomendado)

Rode o comando do seu agente. Ele lê esta página e faz as etapas, pedindo confirmação antes de instalar:

```bash
claude "Siga as instruções de https://cli.kobana.com.br/pt/getting-started/ai-setup.md para preparar meu ambiente do Kobana CLI."
codex "Siga as instruções de https://cli.kobana.com.br/pt/getting-started/ai-setup.md para preparar meu ambiente do Kobana CLI."
gemini "Siga as instruções de https://cli.kobana.com.br/pt/getting-started/ai-setup.md para preparar meu ambiente do Kobana CLI."
```

No **Cursor**, use o botão "Abrir no Cursor" da [página inicial](/). Em outros agentes (agente do Copilot no VS Code, Windsurf, Cline…), cole a mesma mensagem: "Siga as instruções deste endereço".

## Chats na web

ChatGPT, Claude e Gemini na web **não rodam comandos na sua máquina**: eles só explicam. Use-os para entender o CLI e tirar dúvidas — e, se quiser, conecte o [MCP da Documentação](/docs-mcp) como conector, para que respondam com base nestas páginas. A instalação em si, rode você mesmo com os comandos que eles passarem.

## Instruções para o agente

Você está ajudando uma pessoa a deixar o ambiente pronto para usar o Kobana CLI (o comando `kobana`, que acessa as APIs de todos os produtos Kobana). Siga as etapas na ordem. Antes de cada comando que instala ou altera algo, explique o que ele faz e peça confirmação. Se você não consegue executar comandos, passe os comandos para a pessoa rodar.

1. **MCP da documentação.** Conecte ao seu cliente o servidor MCP público e de leitura `https://cli.kobana.com.br/mcp`, para consultar a documentação. Instruções por cliente em [MCP da Documentação](/docs-mcp). Na dúvida: `npx add-mcp "https://cli.kobana.com.br/mcp"`.
2. **CLI.** No macOS e no Linux: `curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash` (ou `brew tap universokobana/tap && brew install kobana`). No Windows, use o binário da [página de Releases](https://github.com/universokobana/kobana-cli/releases/latest). Confirme com `kobana --version`.
3. **Skills de agente.** Pergunte quais produtos a pessoa usa (`banking`, `finance`, `billing`, `inbox`, `dunning`, `dashboards`, `kadu`) e instale a skill `kobana-shared` e as desses produtos (no banking, comece pela `kobana-banking`). Elas estão em [`skills/`](https://github.com/universokobana/kobana-cli/tree/main/skills); copie as pastas para a pasta de skills deste cliente. Veja [Skills para agentes](/guides/agent-skills).
4. **Autenticação.** Guie a pessoa no login de cada produto com `kobana <produto> auth login` (cada produto tem a sua credencial). Nunca peça tokens ou senhas no chat. Veja [Autenticação](/getting-started/authentication).
5. **Teste.** Para cada produto em que a pessoa fez login, rode `kobana <produto> me` (mostra a quem pertence o token, os escopos e a validade, sem expor o token) e diga se deu certo. Se falhar, use [Solução de problemas](/guides/troubleshooting).

Daqui em diante: use sempre `--dry-run` antes de qualquer comando que crie, altere ou exclua, e `--env sandbox` para testar. A documentação em texto está em [`/pt/llms.txt`](https://cli.kobana.com.br/pt/llms.txt).
