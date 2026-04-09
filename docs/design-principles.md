# Kobana CLI — Principios de Design

Principios derivados do artigo [Rewrite your CLI for AI Agents](https://justin.poehnelt.com/posts/rewrite-your-cli-for-ai-agents/) e da arquitetura do gws-cli.

---

## 1. Output JSON Estruturado por Default

Toda saida e JSON. Agentes de IA nao parsam tabelas formatadas.

```bash
# Default: JSON
kobana charge pix list

# Humanos podem pedir tabela
kobana charge pix list --output-format table
```

Erros tambem sao JSON no stdout:
```json
{
  "error": {
    "code": 422,
    "message": "amount nao pode ficar em branco",
    "fields": {"amount": ["nao pode ficar em branco"]}
  }
}
```

---

## 2. Field Masks para Proteger Context Windows

APIs financeiras retornam payloads grandes. `--fields` limita a resposta:

```bash
# Sem fields: resposta com 50+ campos por boleto
# Com fields: so o que o agente precisa
kobana v1 bank-billets list --fields "id,amount,status,due_at"
```

---

## 3. Payloads JSON como Input (nao flags individuais)

Em vez de dezenas de flags (`--amount`, `--expire-at`, `--customer-name`), aceitar o payload JSON direto via `--json`. Isso espelha a API 1:1 e elimina a camada de traducao.

```bash
# Bom: payload direto, mapeamento 1:1 com a API
kobana v1 bank-billets create --json '{"amount": 150, "expire_at": "2026-05-01"}'

# Evitar: flags individuais que duplicam a API
kobana v1 bank-billets create --amount 150 --expire-at 2026-05-01
```

---

## 4. Dry-Run para Mutacoes

Toda operacao que modifica estado (POST, PUT, PATCH, DELETE) deve suportar `--dry-run`:

```bash
kobana charge pix create --json '{...}' --dry-run
# Output: mostra a requisicao que seria feita, sem executar
```

---

## 5. Introspecao de Schema em Runtime

Agentes consultam o schema antes de montar payloads:

```bash
kobana schema charge.pix.create
# Retorna: parametros, campos obrigatorios, tipos, exemplos
```

O schema e derivado do OpenAPI spec embutido no binario.

---

## 6. Validacao Defensiva de Inputs

Agentes geram inputs que humanos nunca gerariam. Validar:

- **Path traversal**: rejeitar `../` em qualquer argumento de path
- **Caracteres de controle**: rejeitar ASCII < 0x20
- **Injection em URL**: rejeitar `?` e `#` em UIDs/IDs
- **Double-encoding**: rejeitar `%` em identificadores

---

## 7. Paginacao como NDJSON

`--page-all` emite uma linha JSON por pagina (Newline Delimited JSON), permitindo streaming sem buffering:

```bash
kobana v1 bank-billets list --page-all | jq -r '.[] | .id'
```

---

## 8. Auth via Env Vars para CI/Headless

O CLI deve funcionar sem interacao em ambientes CI:

```bash
export KOBANA_TOKEN=xxxxx
kobana v1 bank-billets list
# Funciona sem login interativo
```

---

## 9. Codigos de Saida Estruturados

Scripts precisam saber o tipo de falha sem parsear stderr:

| Codigo | Significado |
|--------|------------|
| 0 | Sucesso |
| 1 | Erro de API |
| 2 | Erro de auth |
| 3 | Erro de validacao |
| 4 | Erro de schema |
| 5 | Erro interno |

---

## 10. Help Contextual Derivado do Spec

`--help` em qualquer nivel mostra documentacao derivada do OpenAPI spec, incluindo campos obrigatorios e exemplos:

```bash
kobana charge pix create --help
# Mostra: descricao, campos obrigatorios, exemplo de --json
```

---

## 11. Separacao de Outputs

- **stdout**: dados (JSON)
- **stderr**: logs, hints, progresso, warnings

Agentes leem stdout. Humanos leem stderr.

---

## 12. Idempotency Key Automatica

Para operacoes de criacao, o CLI pode gerar automaticamente um `X-Idempotency-Key` header (UUID) para prevenir duplicacao acidental. Pode ser sobrescrito com `--idempotency-key`.
