---
id: installation
title: Instalación
sidebar_label: Instalación
sidebar_position: 1
description: Instala kobana en macOS, Linux o Windows — script, Homebrew, Nix, binario listo o compilación desde el código.
---

# Instalación

`kobana` es un único binario, sin dependencias. Elige una forma de instalarlo y verifica con `kobana --version`.

## Script de instalación (macOS y Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash
```

El script descarga el binario de la release más reciente en `~/.kobana/bin`, verifica el SHA-256 que publica GitHub y crea el enlace `~/.local/bin/kobana`. Confirma que `~/.local/bin` esté en tu `PATH`.

| Para | Haz |
|---|---|
| Instalar una versión específica | `curl -fsSL … \| bash -s -- v0.12.0` |
| Cambiar las carpetas | `KOBANA_INSTALL_DIR` (binario) y `KOBANA_BIN_DIR` (enlace) |
| Forzar el idioma de los mensajes | `KOBANA_LANG=pt`, `en` o `es` |

## Homebrew (macOS y Linux)

```bash
brew tap universokobana/tap
brew install kobana
```

Para actualizar: `brew update && brew upgrade kobana`.

## Binarios listos

Descarga el archivo de tu plataforma en la [página de Releases](https://github.com/universokobana/kobana-cli/releases/latest) y colócalo en el `PATH`.

| Plataforma | Archivo |
|---|---|
| Linux x86_64 | [`kobana-linux-amd64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-linux-amd64) |
| Linux ARM64 | [`kobana-linux-arm64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-linux-arm64) |
| macOS Intel | [`kobana-darwin-amd64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-darwin-amd64) |
| macOS Apple Silicon | [`kobana-darwin-arm64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-darwin-arm64) |
| Windows x86_64 | [`kobana-windows-amd64.exe`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-windows-amd64.exe) |

Los nombres son enlaces de descarga directa de la última versión. En macOS y Linux, haz el archivo ejecutable y muévelo al `PATH`:

```bash
chmod +x kobana-darwin-arm64
sudo mv kobana-darwin-arm64 /usr/local/bin/kobana
```

## Nix

```bash
# Ejecutar directo desde GitHub
nix run github:universokobana/kobana-cli

# Instalar en el perfil
nix profile install github:universokobana/kobana-cli
```

## Desde el código

```bash
git clone https://github.com/universokobana/kobana-cli.git
cd kobana-cli
cargo install --path crates/kobana-cli
```

Requiere [Rust](https://rustup.rs/) 1.86 o más reciente.

## Actualización

```bash
kobana update            # instala la versión más reciente
kobana update --check    # solo dice si hay una versión nueva
kobana update --check --json
```

En la instalación por script, `kobana update` descarga el binario de la release, verifica el SHA-256 y reemplaza el ejecutable. En Homebrew, Cargo y los demás casos muestra el comando de actualización correspondiente. Con `--json`, la salida incluye `current`, `latest`, `update_available`, `release_url`, `install_method` e `install_command`.

Los demás comandos avisan por stderr cuando hay una versión nueva, consultando GitHub como máximo una vez al día y con un timeout corto. `kobana update` y `kobana completions` no avisan.

## Autocompletado del shell

```bash
kobana completions bash > /etc/bash_completion.d/kobana
kobana completions zsh > ~/.zfunc/_kobana
kobana completions fish > ~/.config/fish/completions/kobana.fish
kobana completions powershell > kobana.ps1
```

Consulta [`kobana completions`](/manual/commands/completions).

## Siguiente paso

[Autenticación](/getting-started/authentication): cada producto tiene su propia credencial.
