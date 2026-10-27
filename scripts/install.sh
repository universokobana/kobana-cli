#!/usr/bin/env bash
# Instalador do Kobana CLI.
#
#   curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash
#
# Uma versão específica:
#
#   curl -fsSL .../install.sh | bash -s -- v0.5.1
#
# Variáveis:
#   KOBANA_VERSION      versão a instalar (padrão: a mais recente)
#   KOBANA_INSTALL_DIR  onde o binário fica (padrão: ~/.kobana)
#   KOBANA_BIN_DIR      onde fica o link no PATH (padrão: ~/.local/bin)
#   NO_COLOR            desliga as cores

set -euo pipefail

REPO="universokobana/kobana-cli"
VERSION="${1:-${KOBANA_VERSION:-latest}}"
INSTALL_DIR="${KOBANA_INSTALL_DIR:-$HOME/.kobana}"
BIN_DIR="${KOBANA_BIN_DIR:-$HOME/.local/bin}"
TERMS_URL="https://www.kobana.com.br/termos"
ISSUES_URL="https://github.com/$REPO/issues"

# --- saída -------------------------------------------------------------------

if [ -t 1 ] && [ -z "${NO_COLOR:-}" ]; then
  LIME=$'\033[38;2;211;253;84m'
  BOLD=$'\033[1m'
  LINK=$'\033[4;34m'
  RED=$'\033[31m'
  RESET=$'\033[0m'
else
  LIME="" BOLD="" LINK="" RED="" RESET=""
fi

say() { printf '%s %s\n' "$1" "$2"; }
blank() { printf '\n'; }
fail() {
  printf '%s❌ %s%s\n' "$RED" "$1" "$RESET" >&2
  exit 1
}

banner() {
  printf '%s' "$LIME"
  cat <<'ART'
    __ __  ____    ____    ___     _   __  ___       ______  __     ____
   / //_/ / __ \  / __ )  /   |   / | / / /   |     / ____/ / /    /  _/
  / ,<   / / / / / __  | / /| |  /  |/ / / /| |    / /     / /     / /
 / /| | / /_/ / / /_/ / / ___ | / /|  / / ___ |   / /___  / /___ _/ /
/_/ |_| \____/ /_____/ /_/  |_|/_/ |_/ /_/  |_|   \____/ /_____//___/
ART
  printf '%s' "$RESET"
}

# --- plataforma --------------------------------------------------------------

detect_asset() {
  local os arch
  case "$(uname -s)" in
    Darwin) os="darwin" ;;
    Linux) os="linux" ;;
    MINGW* | MSYS* | CYGWIN*)
      fail "No Windows, baixe kobana-windows-amd64.exe em https://github.com/$REPO/releases/latest"
      ;;
    *) fail "Sistema não suportado: $(uname -s)" ;;
  esac
  case "$(uname -m)" in
    x86_64 | amd64) arch="amd64" ;;
    arm64 | aarch64) arch="arm64" ;;
    *) fail "Arquitetura não suportada: $(uname -m)" ;;
  esac
  printf 'kobana-%s-%s' "$os" "$arch"
}

http_get() {
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$1"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO- "$1"
  else
    fail "É preciso ter curl ou wget para instalar."
  fi
}

download() {
  if command -v curl >/dev/null 2>&1; then
    curl -fL --progress-bar -o "$2" "$1"
  else
    wget -q --show-progress -O "$2" "$1"
  fi
}

sha256_of() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | cut -d' ' -f1
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | cut -d' ' -f1
  fi
}

# --- instalação --------------------------------------------------------------

main() {
  say "👋" "Olá! Vamos instalar o..."
  blank
  banner
  blank

  local asset release_json tag
  asset="$(detect_asset)"

  if [ "$VERSION" = "latest" ]; then
    say "🔍" "Procurando a versão mais recente do Kobana CLI..."
    release_json="$(http_get "https://api.github.com/repos/$REPO/releases/latest")" \
      || fail "Não foi possível consultar as releases no GitHub."
  else
    case "$VERSION" in v*) ;; *) VERSION="v$VERSION" ;; esac
    say "🔍" "Procurando a versão $VERSION do Kobana CLI..."
    release_json="$(http_get "https://api.github.com/repos/$REPO/releases/tags/$VERSION")" \
      || fail "A versão $VERSION não existe."
  fi

  tag="$(printf '%s' "$release_json" | grep -m1 '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"
  [ -n "$tag" ] || fail "Não foi possível ler a versão da release."

  # O GitHub publica o SHA-256 de cada arquivo da release (campo `digest`)
  local expected_sha
  expected_sha="$(printf '%s' "$release_json" | tr ',' '\n' | awk -v asset="\"$asset\"" '
    $0 ~ "\"name\": *" asset { found = 1; next }
    found && /"digest"/ { if (match($0, /sha256:[0-9a-f]+/)) print substr($0, RSTART + 7, RLENGTH - 7); exit }
    found && /"browser_download_url"/ { exit }
  ')"

  local url="https://github.com/$REPO/releases/download/$tag/$asset"
  local tmp="$INSTALL_DIR/kobana.download"

  say "💾" "Release $tag encontrada! Baixando agora..."
  blank
  printf '  %s%s%s\n' "$LINK" "$url" "$RESET"
  blank

  mkdir -p "$INSTALL_DIR/bin" "$BIN_DIR"
  download "$url" "$tmp" || fail "O download falhou."
  blank

  say "💾" "Kobana CLI $tag baixado em $tmp"

  if [ -n "$expected_sha" ]; then
    local actual_sha
    actual_sha="$(sha256_of "$tmp")"
    if [ -n "$actual_sha" ]; then
      [ "$actual_sha" = "$expected_sha" ] || {
        rm -f "$tmp"
        fail "O SHA-256 do download não confere com o publicado no GitHub."
      }
      say "🔒" "SHA-256 conferido com o publicado na release"
    fi
  fi

  chmod 755 "$tmp"
  mv -f "$tmp" "$INSTALL_DIR/bin/kobana"
  say "📦" "Instalando o binário em $INSTALL_DIR/bin/kobana"

  ln -sf "$INSTALL_DIR/bin/kobana" "$BIN_DIR/kobana"
  say "🔗" "Criando o link simbólico $BIN_DIR/kobana → $INSTALL_DIR/bin/kobana"

  # Outro kobana antes no PATH (Homebrew, cargo) roda no lugar deste
  local found
  found="$(command -v kobana 2>/dev/null || true)"
  if [ -n "$found" ] && [ "$found" != "$BIN_DIR/kobana" ]; then
    blank
    say "⚠️ " "Já existe outro kobana no seu PATH: $found"
    say "  " "Ele roda no lugar deste. Remova-o (ex.: ${BOLD}brew uninstall kobana${RESET}) ou ajuste o PATH."
  fi

  case ":$PATH:" in
    *":$BIN_DIR:"*) ;;
    *)
      blank
      say "🧭" "$BIN_DIR não está no seu PATH. Adicione ao ~/.zshrc ou ~/.bashrc:"
      # shellcheck disable=SC2016 # $PATH is meant literally, for the rc file
      printf '     %sexport PATH="%s:$PATH"%s\n' "$BOLD" "$BIN_DIR" "$RESET"
      ;;
  esac

  blank
  say "📄" "O uso do Kobana CLI segue os Termos de Uso da Kobana:"
  say "🏛️ " "${LINK}${TERMS_URL}${RESET}"
  blank
  say "💚" "Queremos saber como está sendo. De verdade. Tudo mesmo."
  say "✨" "Conte pra gente em ${LINK}${ISSUES_URL}${RESET}"
  blank
  say "💻" "Pronto! O Kobana CLI está instalado ($("$INSTALL_DIR/bin/kobana" --version 2>/dev/null || printf '%s' "$tag"))."
  say "🔐" "Autentique no Gateway Bancário com \`${BOLD}kobana auth login${RESET}\`, ou exporte \`${BOLD}KOBANA_TOKEN${RESET}\` do produto"
  say "🧾" "Veja os seis produtos com \`${BOLD}kobana --help${RESET}\` e cada endpoint com \`${BOLD}kobana schema --list${RESET}\`"
  say "🔄" "Atualize depois com \`${BOLD}kobana update${RESET}\`"
}

main "$@"
