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
#   KOBANA_LANG         idioma das mensagens: pt, en ou es (padrão: o do sistema)
#   NO_COLOR            desliga as cores

set -euo pipefail

REPO="universokobana/kobana-cli"
VERSION="${1:-${KOBANA_VERSION:-latest}}"
INSTALL_DIR="${KOBANA_INSTALL_DIR:-$HOME/.kobana}"
BIN_DIR="${KOBANA_BIN_DIR:-$HOME/.local/bin}"
ISSUES_URL="https://github.com/$REPO/issues"

# --- idioma ------------------------------------------------------------------

# Same precedence as gettext: LC_ALL, LC_MESSAGES, LANG. A bare "C"/"POSIX"
# locale says nothing about the user, so on macOS fall back to the system one.
detect_lang() {
  local locale="${KOBANA_LANG:-${LC_ALL:-${LC_MESSAGES:-${LANG:-}}}}"
  case "$locale" in
    "" | C | C.* | POSIX)
      if [ "$(uname -s)" = "Darwin" ]; then
        locale="$(defaults read -g AppleLocale 2>/dev/null || true)"
      fi
      ;;
  esac
  case "$locale" in
    pt* | PT*) printf 'pt' ;;
    es* | ES*) printf 'es' ;;
    *) printf 'en' ;;
  esac
}

# Messages with %s take printf arguments
load_messages() {
  case "$1" in
    pt)
      TERMS_URL="https://www.kobana.com.br/termos"
      M_HELLO="Olá! Vamos instalar o..."
      M_WINDOWS="No Windows, baixe kobana-windows-amd64.exe em %s"
      M_UNSUPPORTED_OS="Sistema não suportado: %s"
      M_UNSUPPORTED_ARCH="Arquitetura não suportada: %s"
      M_NEED_CURL="É preciso ter curl ou wget para instalar."
      M_LOOKUP_LATEST="Procurando a versão mais recente do Kobana CLI..."
      M_LOOKUP_FAILED="Não foi possível consultar as releases no GitHub."
      M_LOOKUP_VERSION="Procurando a versão %s do Kobana CLI..."
      M_NO_VERSION="A versão %s não existe."
      M_NO_TAG="Não foi possível ler a versão da release."
      M_FOUND="Release %s encontrada! Baixando agora..."
      M_DOWNLOAD_FAILED="O download falhou."
      M_DOWNLOADED="Kobana CLI %s baixado em %s"
      M_SHA_MISMATCH="O SHA-256 do download não confere com o publicado no GitHub."
      M_SHA_OK="SHA-256 conferido com o publicado na release"
      M_INSTALLING="Instalando o binário em %s"
      M_LINKING="Criando o link simbólico %s → %s"
      M_OTHER="Já existe outro kobana no seu PATH: %s"
      M_OTHER_HINT="Ele roda no lugar deste. Remova-o (ex.: %s) ou ajuste o PATH."
      M_NOT_IN_PATH="%s não está no seu PATH. Adicione ao ~/.zshrc ou ~/.bashrc:"
      M_TERMS="O uso do Kobana CLI segue os Termos de Uso da Kobana:"
      M_FEEDBACK="Queremos saber como está sendo. De verdade. Tudo mesmo."
      M_FEEDBACK_LINK="Conte pra gente em %s"
      M_DONE="Pronto! O Kobana CLI está instalado (%s)."
      M_AUTH="Autentique no Gateway Bancário com \`%s\`, ou exporte \`%s\` do produto"
      M_EXPLORE="Veja os seis produtos com \`%s\` e cada endpoint com \`%s\`"
      M_UPDATE="Atualize depois com \`%s\`"
      ;;
    es)
      TERMS_URL="https://www.kobana.com.br/es/terminos"
      M_HELLO="¡Hola! Vamos a instalar el..."
      M_WINDOWS="En Windows, descarga kobana-windows-amd64.exe en %s"
      M_UNSUPPORTED_OS="Sistema no soportado: %s"
      M_UNSUPPORTED_ARCH="Arquitectura no soportada: %s"
      M_NEED_CURL="Se necesita curl o wget para instalar."
      M_LOOKUP_LATEST="Buscando la versión más reciente del Kobana CLI..."
      M_LOOKUP_FAILED="No se pudieron consultar las releases en GitHub."
      M_LOOKUP_VERSION="Buscando la versión %s del Kobana CLI..."
      M_NO_VERSION="La versión %s no existe."
      M_NO_TAG="No se pudo leer la versión de la release."
      M_FOUND="¡Release %s encontrada! Descargando ahora..."
      M_DOWNLOAD_FAILED="La descarga falló."
      M_DOWNLOADED="Kobana CLI %s descargado en %s"
      M_SHA_MISMATCH="El SHA-256 de la descarga no coincide con el publicado en GitHub."
      M_SHA_OK="SHA-256 verificado con el publicado en la release"
      M_INSTALLING="Instalando el binario en %s"
      M_LINKING="Creando el enlace simbólico %s → %s"
      M_OTHER="Ya existe otro kobana en tu PATH: %s"
      M_OTHER_HINT="Se ejecuta en lugar de este. Elimínalo (ej.: %s) o ajusta el PATH."
      M_NOT_IN_PATH="%s no está en tu PATH. Agrégalo a ~/.zshrc o ~/.bashrc:"
      M_TERMS="El uso del Kobana CLI se rige por los Términos de Uso de Kobana:"
      M_FEEDBACK="Queremos saber cómo te va. De verdad. Todo."
      M_FEEDBACK_LINK="Cuéntanos en %s"
      M_DONE="¡Listo! El Kobana CLI está instalado (%s)."
      M_AUTH="Autentícate en el Gateway Bancario con \`%s\`, o exporta \`%s\` del producto"
      M_EXPLORE="Mira los seis productos con \`%s\` y cada endpoint con \`%s\`"
      M_UPDATE="Actualiza después con \`%s\`"
      ;;
    *)
      TERMS_URL="https://www.kobana.com.br/en/terms"
      M_HELLO="Hi! Let's install the..."
      M_WINDOWS="On Windows, download kobana-windows-amd64.exe from %s"
      M_UNSUPPORTED_OS="Unsupported system: %s"
      M_UNSUPPORTED_ARCH="Unsupported architecture: %s"
      M_NEED_CURL="curl or wget is required to install."
      M_LOOKUP_LATEST="Looking up the latest version of the Kobana CLI..."
      M_LOOKUP_FAILED="Could not query the releases on GitHub."
      M_LOOKUP_VERSION="Looking up version %s of the Kobana CLI..."
      M_NO_VERSION="Version %s does not exist."
      M_NO_TAG="Could not read the release version."
      M_FOUND="Found release %s! Downloading now..."
      M_DOWNLOAD_FAILED="The download failed."
      M_DOWNLOADED="Kobana CLI %s downloaded to %s"
      M_SHA_MISMATCH="The download's SHA-256 does not match the one published on GitHub."
      M_SHA_OK="SHA-256 matches the one published in the release"
      M_INSTALLING="Installing the binary to %s"
      M_LINKING="Creating the symlink %s → %s"
      M_OTHER="There is another kobana in your PATH: %s"
      M_OTHER_HINT="It runs instead of this one. Remove it (e.g. %s) or adjust your PATH."
      M_NOT_IN_PATH="%s is not in your PATH. Add it to ~/.zshrc or ~/.bashrc:"
      M_TERMS="Use of the Kobana CLI is subject to Kobana's Terms of Use:"
      M_FEEDBACK="We want to know how it's going. Really. Everything."
      M_FEEDBACK_LINK="Tell us at %s"
      M_DONE="Done! The Kobana CLI is installed (%s)."
      M_AUTH="Authenticate with the Banking Gateway using \`%s\`, or export the product's \`%s\`"
      M_EXPLORE="See the six products with \`%s\` and every endpoint with \`%s\`"
      M_UPDATE="Update later with \`%s\`"
      ;;
  esac
}

load_messages "$(detect_lang)"

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
# shellcheck disable=SC2059 # the format is one of the M_* messages
msg() { local fmt="$1"; shift; printf "$fmt" "$@"; }
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
      fail "$(msg "$M_WINDOWS" "https://github.com/$REPO/releases/latest")"
      ;;
    *) fail "$(msg "$M_UNSUPPORTED_OS" "$(uname -s)")" ;;
  esac
  case "$(uname -m)" in
    x86_64 | amd64) arch="amd64" ;;
    arm64 | aarch64) arch="arm64" ;;
    *) fail "$(msg "$M_UNSUPPORTED_ARCH" "$(uname -m)")" ;;
  esac
  printf 'kobana-%s-%s' "$os" "$arch"
}

http_get() {
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$1"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO- "$1"
  else
    fail "$M_NEED_CURL"
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
  say "👋" "$M_HELLO"
  blank
  banner
  blank

  local asset release_json tag
  asset="$(detect_asset)"

  if [ "$VERSION" = "latest" ]; then
    say "🔍" "$M_LOOKUP_LATEST"
    release_json="$(http_get "https://api.github.com/repos/$REPO/releases/latest")" \
      || fail "$M_LOOKUP_FAILED"
  else
    case "$VERSION" in v*) ;; *) VERSION="v$VERSION" ;; esac
    say "🔍" "$(msg "$M_LOOKUP_VERSION" "$VERSION")"
    release_json="$(http_get "https://api.github.com/repos/$REPO/releases/tags/$VERSION")" \
      || fail "$(msg "$M_NO_VERSION" "$VERSION")"
  fi

  tag="$(printf '%s' "$release_json" | grep -m1 '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"
  [ -n "$tag" ] || fail "$M_NO_TAG"

  # O GitHub publica o SHA-256 de cada arquivo da release (campo `digest`)
  local expected_sha
  expected_sha="$(printf '%s' "$release_json" | tr ',' '\n' | awk -v asset="\"$asset\"" '
    $0 ~ "\"name\": *" asset { found = 1; next }
    found && /"digest"/ { if (match($0, /sha256:[0-9a-f]+/)) print substr($0, RSTART + 7, RLENGTH - 7); exit }
    found && /"browser_download_url"/ { exit }
  ')"

  local url="https://github.com/$REPO/releases/download/$tag/$asset"
  local tmp="$INSTALL_DIR/kobana.download"

  say "💾" "$(msg "$M_FOUND" "$tag")"
  blank
  printf '  %s%s%s\n' "$LINK" "$url" "$RESET"
  blank

  mkdir -p "$INSTALL_DIR/bin" "$BIN_DIR"
  download "$url" "$tmp" || fail "$M_DOWNLOAD_FAILED"
  blank

  say "💾" "$(msg "$M_DOWNLOADED" "$tag" "$tmp")"

  if [ -n "$expected_sha" ]; then
    local actual_sha
    actual_sha="$(sha256_of "$tmp")"
    if [ -n "$actual_sha" ]; then
      [ "$actual_sha" = "$expected_sha" ] || {
        rm -f "$tmp"
        fail "$M_SHA_MISMATCH"
      }
      say "🔒" "$M_SHA_OK"
    fi
  fi

  chmod 755 "$tmp"
  mv -f "$tmp" "$INSTALL_DIR/bin/kobana"
  say "📦" "$(msg "$M_INSTALLING" "$INSTALL_DIR/bin/kobana")"

  ln -sf "$INSTALL_DIR/bin/kobana" "$BIN_DIR/kobana"
  say "🔗" "$(msg "$M_LINKING" "$BIN_DIR/kobana" "$INSTALL_DIR/bin/kobana")"

  # Outro kobana antes no PATH (Homebrew, cargo) roda no lugar deste
  local found
  found="$(command -v kobana 2>/dev/null || true)"
  if [ -n "$found" ] && [ "$found" != "$BIN_DIR/kobana" ]; then
    blank
    say "⚠️ " "$(msg "$M_OTHER" "$found")"
    say "  " "$(msg "$M_OTHER_HINT" "${BOLD}brew uninstall kobana${RESET}")"
  fi

  case ":$PATH:" in
    *":$BIN_DIR:"*) ;;
    *)
      blank
      say "🧭" "$(msg "$M_NOT_IN_PATH" "$BIN_DIR")"
      # shellcheck disable=SC2016 # $PATH is meant literally, for the rc file
      printf '     %sexport PATH="%s:$PATH"%s\n' "$BOLD" "$BIN_DIR" "$RESET"
      ;;
  esac

  blank
  say "📄" "$M_TERMS"
  say "🏛️ " "${LINK}${TERMS_URL}${RESET}"
  blank
  say "💚" "$M_FEEDBACK"
  say "✨" "$(msg "$M_FEEDBACK_LINK" "${LINK}${ISSUES_URL}${RESET}")"
  blank
  say "💻" "$(msg "$M_DONE" "$("$INSTALL_DIR/bin/kobana" --version 2>/dev/null || printf '%s' "$tag")")"
  say "🔐" "$(msg "$M_AUTH" "${BOLD}kobana auth login${RESET}" "${BOLD}KOBANA_TOKEN${RESET}")"
  say "🧾" "$(msg "$M_EXPLORE" "${BOLD}kobana --help${RESET}" "${BOLD}kobana schema --list${RESET}")"
  say "🔄" "$(msg "$M_UPDATE" "${BOLD}kobana update${RESET}")"
}

main "$@"
