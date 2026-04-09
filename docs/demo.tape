# Kobana CLI Demo
# Run: vhs docs/demo.tape

Output docs/demo.gif

Require kobana

Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 700
Set Padding 20
Set Theme "Catppuccin Mocha"
Set TypingSpeed 40ms

# ── Intro ──
Type "# Kobana CLI — acesso completo à API Kobana v1 e v2" Enter
Sleep 1.5s

# ── Help ──
Type "kobana --help" Enter
Sleep 4s

# ── Listar serviços via Schema ──
Type "# Introspecção de schema" Enter
Sleep 1s
Type "kobana schema charge.pix.create" Enter
Sleep 4s

# ── Dry-run: criar cobrança Pix ──
Type "# Dry-run: validar requisição sem executar" Enter
Sleep 1s
Type `kobana charge pix create --dry-run --json '{"amount": 99.90, "pix_account_uid": "abc-123"}'` Enter
Sleep 4s

# ── Dry-run: criar boleto ──
Type "# Dry-run: emitir boleto via API v1" Enter
Sleep 1s
Type `kobana v1 bank-billets create --dry-run --json '{"amount": 150.50, "expire_at": "2026-05-01", "customer_person_name": "Maria Silva", "customer_cnpj_cpf": "012.345.678-90", "bank_billet_account_id": 1}'` Enter
Sleep 4s

# ── Helper: +cobrar ──
Type "# Helpers: atalhos para operações comuns" Enter
Sleep 1s
Type "kobana +cobrar --dry-run --valor 250.00 --conta-pix abc-123 --nome João --cpf-cnpj 012.345.678-90" Enter
Sleep 4s

# ── Help contextual ──
Type "# Help contextual em qualquer nível" Enter
Sleep 1s
Type "kobana charge pix --help" Enter
Sleep 4s

# ── Auth status ──
Type "# Status de autenticação" Enter
Sleep 1s
Type "kobana auth status" Enter
Sleep 3s

# ── Fim ──
Type "# Pronto! kobana --help para mais informações" Enter
Sleep 2s
