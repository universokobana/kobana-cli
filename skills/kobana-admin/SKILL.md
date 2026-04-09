---
name: kobana-admin
description: "Kobana Administração: Subcontas, usuários, conexões, certificados."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana admin --help"
---

# admin — Administração

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana admin <resource> <method> [flags]
```

## API Resources

### certificates
- `list` — Listar Certificados
- `create` — Incluir um Certificado

### connections
- `list` — Listar Conexões
- `create` — Incluir uma Conexão
- `get` — Informações da Conexão. Params: `uid`
- `update` — Atualizar Conexão. Params: `uid`
- `delete` — Excluir uma Conexão. Params: `uid`
- `associations` — Conectar/Desconectar Conta de Serviço. Params: `uid`

### subaccounts
- `list` — Listar Subcontas
- `create` — Criar uma Subconta
- `get` — Visualizar uma Subconta. Params: `uid`
- `update` — Alterar Subconta. Params: `uid`

### users
- `list` — Listar Usuários
- `create` — Incluir um Usuário
- `update` — Alterar Dados de um Usuário. Params: `uid`
- `delete` — Excluir um Usuário. Params: `uid`

## Discovering Commands

```bash
kobana admin --help
kobana admin subaccounts --help
kobana schema admin.subaccounts.create
```
