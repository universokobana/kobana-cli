---
id: schema
title: Schema
sidebar_label: Schema
sidebar_position: 5
description: Consulte parâmetros, corpo e respostas de qualquer endpoint sem sair do terminal.
---

# Schema

`kobana schema` mostra, em JSON, tudo que a especificação diz sobre um endpoint: método e caminho HTTP, parâmetros de caminho, de consulta e de cabeçalho, corpo e respostas.

```bash
# Todos os produtos e recursos
kobana schema --list

# Um endpoint — o caminho tem a forma do comando
kobana schema banking.charge.pix.create
kobana schema finance.payables.list
kobana schema kadu.tasks.run
```

O caminho é `<produto>.<recurso>.<método>`. Para comandos que rodam pelo nome, o método pode ser omitido: `kobana schema banking.me`.

## O que sai

| Campo | Conteúdo |
|---|---|
| `method`, `path` | Verbo e caminho HTTP, com a versão da API |
| `summary`, `description` | Texto da especificação, no idioma do CLI |
| `path_params` | Identificadores do caminho, na ordem dos argumentos |
| `parameters` | Parâmetros de consulta, com tipo, obrigatoriedade e valores aceitos |
| `header_params` | Cabeçalhos, como `X-Workspace-Id` |
| `request_body` | Esquema do corpo |
| `responses` | Códigos de resposta e seus esquemas |
| `deprecated` | `true` quando o endpoint está aposentado |

## Para agentes

O schema é a forma mais curta de um agente descobrir como chamar um endpoint sem ler a documentação inteira. Veja [Scripts e agentes de IA](/usage/scripting). As mesmas informações, já formatadas, estão no [Manual](/manual).
