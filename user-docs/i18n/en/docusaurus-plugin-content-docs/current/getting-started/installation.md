---
id: installation
title: Installation
sidebar_label: Installation
sidebar_position: 1
description: Install kobana on macOS, Linux or Windows — script, Homebrew, Nix, prebuilt binary or build from source.
---

# Installation

`kobana` is a single binary with no dependencies. Pick an install method and check it with `kobana --version`.

## Install script (macOS and Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash
```

The script downloads the binary from the latest release to `~/.kobana/bin`, checks the SHA-256 that GitHub publishes, and creates the `~/.local/bin/kobana` link. Make sure `~/.local/bin` is in your `PATH`.

| To | Do |
|---|---|
| Install a specific version | `curl -fsSL … \| bash -s -- v0.12.0` |
| Change the folders | `KOBANA_INSTALL_DIR` (binary) and `KOBANA_BIN_DIR` (link) |
| Force the message language | `KOBANA_LANG=pt`, `en` or `es` |

## Homebrew (macOS and Linux)

```bash
brew tap universokobana/tap
brew install kobana
```

To update: `brew update && brew upgrade kobana`.

## Prebuilt binaries

Download the file for your platform from the [Releases page](https://github.com/universokobana/kobana-cli/releases/latest) and put it in your `PATH`.

| Platform | File |
|---|---|
| Linux x86_64 | [`kobana-linux-amd64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-linux-amd64) |
| Linux ARM64 | [`kobana-linux-arm64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-linux-arm64) |
| macOS Intel | [`kobana-darwin-amd64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-darwin-amd64) |
| macOS Apple Silicon | [`kobana-darwin-arm64`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-darwin-arm64) |
| Windows x86_64 | [`kobana-windows-amd64.exe`](https://github.com/universokobana/kobana-cli/releases/latest/download/kobana-windows-amd64.exe) |

The names are direct download links for the latest version. On macOS and Linux, make the file executable and move it onto your `PATH`:

```bash
chmod +x kobana-darwin-arm64
sudo mv kobana-darwin-arm64 /usr/local/bin/kobana
```

## Nix

```bash
# Run straight from GitHub
nix run github:universokobana/kobana-cli

# Install into your profile
nix profile install github:universokobana/kobana-cli
```

## From source

```bash
git clone https://github.com/universokobana/kobana-cli.git
cd kobana-cli
cargo install --path crates/kobana-cli
```

Requires [Rust](https://rustup.rs/) 1.86 or newer.

## Updating

```bash
kobana update            # installs the latest version
kobana update --check    # only says whether there is a new version
kobana update --check --json
```

If you installed with the script, `kobana update` downloads the release binary, checks the SHA-256 and replaces the executable. With Homebrew, Cargo and the other methods, it prints the update command for each one. With `--json`, the output includes `current`, `latest`, `update_available`, `release_url`, `install_method` and `install_command`.

The other commands warn on stderr when there is a new version, checking GitHub at most once a day and with a short timeout. `kobana update` and `kobana completions` do not warn.

## Shell completion

```bash
kobana completions bash > /etc/bash_completion.d/kobana
kobana completions zsh > ~/.zfunc/_kobana
kobana completions fish > ~/.config/fish/completions/kobana.fish
kobana completions powershell > kobana.ps1
```

See [`kobana completions`](/manual/commands/completions).

## Next step

[Authentication](/getting-started/authentication): each product has its own credential.
