# Kobana CLI Demo
# Run: vhs docs/demo.tape
#
# Comentarios digitados ficam em ASCII puro de proposito: o Type do VHS
# descarta caracteres acentuados ("binario" sai como "bin rio").
# A saida dos comandos aceita acentos normalmente.

Output docs/demo.gif

Require kobana

Set Shell "bash"
Set FontSize 14
Set Width 1500
Set Height 750
Set Padding 20
Set Theme "Catppuccin Mocha"
Set TypingSpeed 30ms

# ── Intro ──
Type "# Kobana CLI: as APIs dos quatro produtos Kobana" Enter
Sleep 1.5s

# ── Produtos ──
Type "kobana --help" Enter
Sleep 4s

# ── Um comando por produto ──
Type "# Primeiro segmento do comando: o produto" Enter
Sleep 1.2s
Type "kobana banking bank-billets list --dry-run" Enter
Sleep 2.5s
Type "kobana finance payables list --dry-run" Enter
Sleep 2.5s
Type "kobana billing subscriptions list --dry-run" Enter
Sleep 2.5s
Type "kobana inbox emails list --dry-run" Enter
Sleep 2.5s

# ── Dry-run com corpo ──
Type "# Dry-run: mostra o request sem executar" Enter
Sleep 1.2s
Type `kobana banking charge pix create --dry-run --json '{"amount": 99.90, "pix_account_uid": "abc-123"}'` Enter
Sleep 3.5s

# ── Schema ──
Type "# Schema de qualquer endpoint, de qualquer produto" Enter
Sleep 1.2s
Type "kobana schema inbox.emails.reprocess | head -14" Enter
Sleep 3.5s

# ── Help contextual ──
Type "# Help contextual em qualquer ponto" Enter
Sleep 1.2s
Type "kobana finance payables --help" Enter
Sleep 3.5s

# ── Fim ──
Type "# Comece por: kobana --help" Enter
Sleep 2s
