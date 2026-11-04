---
id: installation
title: Instalação
sidebar_label: Instalação
sidebar_position: 1
description: Instale o kobana no macOS, Linux ou Windows — script, Homebrew, Nix, binário pronto ou build a partir do código.
---

# Instalação

O `kobana` é um único binário, sem dependências. Escolha um jeito de instalar e confira com `kobana --version`.

## Script de instalação (macOS e Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash
```

O script baixa o binário da release mais recente para `~/.kobana/bin`, confere o SHA-256 que o GitHub publica e cria o link `~/.local/bin/kobana`. Confirme que `~/.local/bin` está no seu `PATH`.

| Para | Faça |
|---|---|
| Instalar uma versão específica | `curl -fsSL … \| bash -s -- v0.12.0` |
| Trocar as pastas | `KOBANA_INSTALL_DIR` (binário) e `KOBANA_BIN_DIR` (link) |
| Forçar o idioma das mensagens | `KOBANA_LANG=pt`, `en` ou `es` |

## Homebrew (macOS e Linux)

```bash
brew tap universokobana/tap
brew install kobana
```

Para atualizar: `brew update && brew upgrade kobana`.

## Binários prontos

Baixe o arquivo da sua plataforma na [página de Releases](https://github.com/universokobana/kobana-cli/releases/latest) e coloque-o no `PATH`.

| Plataforma | Arquivo |
|---|---|
| Linux x86_64 | [`kobana-linux-amd64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-linux-amd64) |
| Linux ARM64 | [`kobana-linux-arm64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-linux-arm64) |
| macOS Intel | [`kobana-darwin-amd64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-darwin-amd64) |
| macOS Apple Silicon | [`kobana-darwin-arm64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-darwin-arm64) |
| Windows x86_64 | [`kobana-windows-amd64.exe`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-windows-amd64.exe) |

Os nomes são links de download direto da versão mais recente. No macOS e no Linux, torne o arquivo executável e mova-o para o `PATH`:

```bash
chmod +x kobana-darwin-arm64
sudo mv kobana-darwin-arm64 /usr/local/bin/kobana
```

## Nix

```bash
# Rodar direto do GitHub
nix run github:universokobana/kobana-cli

# Instalar no perfil
nix profile install github:universokobana/kobana-cli
```

## A partir do código

```bash
git clone https://github.com/universokobana/kobana-cli.git
cd kobana-cli
cargo install --path crates/kobana-cli
```

Exige [Rust](https://rustup.rs/) 1.86 ou mais novo.

## Atualização

```bash
kobana update            # instala a versão mais recente
kobana update --check    # só diz se há versão nova
kobana update --check --json
```

Na instalação pelo script, `kobana update` baixa o binário da release, confere o SHA-256 e troca o executável. No Homebrew, no Cargo e nos outros casos ele mostra o comando de atualização de cada um. Com `--json`, a saída traz `current`, `latest`, `update_available`, `release_url`, `install_method` e `install_command`.

Os outros comandos avisam no stderr quando há versão nova, consultando o GitHub no máximo uma vez por dia e com timeout curto. `kobana update` e `kobana completions` não avisam.

## Autocompletar do shell

```bash
kobana completions bash > /etc/bash_completion.d/kobana
kobana completions zsh > ~/.zfunc/_kobana
kobana completions fish > ~/.config/fish/completions/kobana.fish
kobana completions powershell > kobana.ps1
```

Veja [`kobana completions`](/manual/commands/completions).

## Próximo passo

[Autenticação](/getting-started/authentication): cada produto tem a sua credencial.
