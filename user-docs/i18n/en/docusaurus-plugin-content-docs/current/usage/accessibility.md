---
id: accessibility
title: Accessibility
sidebar_label: Accessibility
sidebar_position: 9
description: Colors you can turn off or set to high contrast, a screen reader mode, simple prompts and script-friendly output — on the GitHub CLI model.
---

# Accessibility

`kobana` follows the accessibility practices of the [GitHub CLI](https://cli.github.com/manual/gh_help_accessibility) (`gh accessibility`) for people who use a screen reader, a braille display or high contrast, or are sensitive to motion. Everything is turned on by an option or an environment variable; to keep a choice, put the variable in the configuration folder's `.env` (`~/.config/kobana/.env`).

```bash
kobana accessibility    # or: kobana a11y — the options and what is on in this terminal
```

## Colors

Colors only reinforce what the text already says: every colored message has a word that says the same (`error:`, `Warning:`, `tip:`). The CLI only uses the terminal's 16 ANSI colors — which you redefine in its preferences — never greys or 24-bit colors.

| How | Effect |
|---|---|
| `--no-color`, `NO_COLOR=1` (any value) or `CLICOLOR=0` | No colors or styles |
| `CLICOLOR_FORCE=1` (any value) | Colors even when the output is not a terminal (for example, `\| less -R`) |
| `KOBANA_ACCESSIBLE_COLORS=1` | High contrast: bold and underline in your theme's text color, no hues — the contrast is the one your theme guarantees, light or dark |

Outside a terminal (pipe, file) there are no colors, unless `CLICOLOR_FORCE` asks for them; on a terminal, neither with `TERM=dumb` or no `TERM`. The order is `--no-color`, `NO_COLOR`, `CLICOLOR_FORCE`, `CLICOLOR` — the same rules as the help, so everything agrees.

## Screen reader

```bash
export KOBANA_SCREEN_READER=1
```

Leaves the output with static text only, which a screen reader or a braille display reads as it was written:

- **No decorative symbols** (such as `✓` and `└─`), which would be read by name.
- **Tables without borders** (`--output-format table`), one line per record: borders are not spelled out, and a long cell does not split the record into two lines.
- **Simple prompts** (see below).

`kobana` has no spinners: progress is always a new line of text, never a redrawn line. Its one animation is Kadu's (`kobana art anim` and `kobana oi`), which only runs if you ask; with `KOBANA_SCREEN_READER=1`, the art becomes a sentence and nothing moves.

## Prompts

`kobana auth login` asks which scopes to request, and `kobana update` whether it may install. By default they are interactive lists, redrawn at every keystroke. With `KOBANA_ACCESSIBLE_PROMPTER=1` (or `KOBANA_SCREEN_READER=1`), the question is written once and read on a single line:

```text
Access levels (apply to every chosen resource)
  1. read: Read
  2. write: Create and update
  3. delete: Delete
  4. import: Bulk import
  5. export: Bulk export
Type the numbers separated by commas (e.g. 1, 3, 5-7) or "all". Enter keeps: 1. 1, 2
Chosen: read, write
```

To never be asked anything, set `KOBANA_PROMPT_DISABLED=1`.

## Scripts and AI agents

Without a person at the terminal — `stdin`, `stdout` and `stderr` are not all a terminal, as in a pipe, in CI or in an AI agent — or with `KOBANA_PROMPT_DISABLED=1`, `kobana` asks nothing and goes by the command line:

- `auth login` requests every published resource with read access (`--scopes` picks others);
- `update` only shows the command that would install;
- the new-version notice does not show (nor query the network); `KOBANA_NO_UPDATE_NOTIFIER=1` turns it off in the terminal too.

Output is JSON (the default). `--fields` picks the fields and `--jq` filters with a [jq](https://jqlang.github.io/jq/manual/) expression, with no `jq` installed — see [Output](/usage/output#filter-with---jq).

## Summary

| Option or variable | `gh` equivalent | Effect |
|---|---|---|
| `--no-color`, `NO_COLOR`, `CLICOLOR=0` | `NO_COLOR`, `CLICOLOR=0` | No colors |
| `CLICOLOR_FORCE` | `CLICOLOR_FORCE` | Colors outside the terminal |
| `KOBANA_ACCESSIBLE_COLORS` | `GH_ACCESSIBLE_COLORS` | High contrast, no hues |
| `KOBANA_SCREEN_READER` | `GH_ACCESSIBLE_PROMPTER` + `GH_SPINNER_DISABLED` | Static text: no decorative symbols, tables without borders, simple prompts |
| `KOBANA_ACCESSIBLE_PROMPTER` | `GH_ACCESSIBLE_PROMPTER` | The simple prompts alone |
| `KOBANA_PROMPT_DISABLED` | `GH_PROMPT_DISABLED` | Never asks |
| `KOBANA_NO_UPDATE_NOTIFIER` | `GH_NO_UPDATE_NOTIFIER` | No new-version notice |
| `--jq <EXPRESSION>` | `--jq` | Filters the JSON output |

The variables apply when set to an enabled value: any value except empty, `0`, `false`, `no` and `off`. `NO_COLOR`, as its standard says, applies with any non-empty value.
