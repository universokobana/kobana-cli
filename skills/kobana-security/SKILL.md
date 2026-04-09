---
name: kobana-security
description: "Kobana Segurança: Gerenciamento de tokens de acesso à API."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana security --help"
---

# security — Tokens de Acesso

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana security <resource> <method> [flags]
```

## API Resources

### access-tokens
- `create` — Criar um Token de Acesso
- `update` — Atualizar Token de Acesso. Params: `uid`
- `delete` — Excluir Token de Acesso. Params: `uid`
- `enable` — Habilitar Token de Acesso. Params: `uid`
- `disable` — Desabilitar Token de Acesso. Params: `uid`
- `renew` — Renovar Token de Acesso. Params: `uid`
- `revoke` — Revogar Token de Acesso. Params: `uid`

## Discovering Commands

```bash
kobana security --help
kobana security access-tokens --help
kobana schema security.access-tokens.create
```
