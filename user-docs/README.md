# Kobana CLI — documentação

Site do `kobana` em **cli.kobana.com.br**, construído com [Docusaurus 3](https://docusaurus.io/), em pt-BR, en e es.

- **Documentação**: guia escrito à mão (`docs/getting-started`, `usage`, `reference`).
- **Manual**: todos os comandos, **gerados do binário** — não edite `docs/manual/<produto>`.

## Comandos

```bash
npm install
npm run gen-manual         # regenera o manual (precisa de ../target/release/kobana)
npm run gen-manual:check   # falha se o manual estiver desatualizado (CI)
npm run build              # build dos três idiomas em build/{pt,en,es} + seletor de idioma
npm run serve:prod         # http://localhost:3300 — use /pt/, /en/, /es/
npm run typecheck
```

`KOBANA_BIN=/caminho/kobana npm run gen-manual` usa outro binário.

Confira as mudanças com o **build de produção**, não com `npm start`: o servidor de desenvolvimento não gera os `.md` nem o `llms.txt`, e não tem o prefixo de idioma de produção.

## Publicação

Projeto Vercel `kobana-cli-docs` (Root Directory `user-docs`, domínio `cli.kobana.com.br`), com a branch `production` como branch de produção. Veja `user-docs-publish.md` nos standards.

O assistente (`api/assistant.ts`) usa o Vercel AI Gateway e precisa estar habilitado no projeto.
