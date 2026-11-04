---
id: accessibility
title: Accesibilidad
sidebar_label: Accesibilidad
sidebar_position: 9
description: Colores que se pueden apagar o poner en alto contraste, modo lector de pantalla, preguntas simples y salida para scripts — con el modelo de GitHub CLI.
---

# Accesibilidad

`kobana` sigue las prácticas de accesibilidad de [GitHub CLI](https://cli.github.com/manual/gh_help_accessibility) (`gh accessibility`) para quien usa lector de pantalla, línea braille o alto contraste, o es sensible al movimiento. Todo se activa con una opción o una variable de entorno; para mantener la elección, pon la variable en el `.env` de la carpeta de configuración (`~/.config/kobana/.env`).

```bash
kobana accessibility    # o: kobana a11y — las opciones y lo que está activo en este terminal
```

## Colores

Los colores solo refuerzan lo que el texto ya dice: todo mensaje con color tiene una palabra que dice lo mismo (`error:`, `Aviso:`, `sugerencia:`). El CLI usa solo los 16 colores ANSI del terminal — que redefines en sus preferencias —, nunca grises ni colores de 24 bits.

| Cómo | Efecto |
|---|---|
| `--no-color`, `NO_COLOR=1` (cualquier valor) o `CLICOLOR=0` | Sin colores ni estilos |
| `CLICOLOR_FORCE=1` | Colores aunque la salida no sea un terminal (por ejemplo, `\| less -R`) |
| `KOBANA_ACCESSIBLE_COLORS=1` | Alto contraste: negrita y subrayado en el color del texto de tu tema, sin matices — el contraste es el que tu tema garantiza, claro u oscuro |

Fuera de un terminal (pipe, archivo, CI) no hay colores, salvo que `CLICOLOR_FORCE` los pida. El orden es el mismo de `gh`: `--no-color`, `NO_COLOR`, `CLICOLOR_FORCE`, `CLICOLOR`.

## Lector de pantalla

```bash
export KOBANA_SCREEN_READER=1
```

Deja la salida solo con texto estático, que un lector de pantalla o una línea braille lee tal como se escribió:

- **Sin símbolos decorativos** (como `✓` y `└─`), que se leerían por su nombre.
- **Tablas sin bordes** (`--output-format table`), una línea por registro: ni los bordes se deletrean, ni una celda larga parte el registro en dos líneas.
- **Preguntas simples** (ver abajo).

`kobana` no tiene spinners ni animaciones: el progreso es siempre una línea de texto nueva, nunca una línea redibujada.

## Preguntas

`kobana auth login` pregunta qué alcances pedir, y `kobana update`, si puede instalar. Por defecto son listas interactivas, redibujadas con cada tecla. Con `KOBANA_ACCESSIBLE_PROMPTER=1` (o `KOBANA_SCREEN_READER=1`), la pregunta se escribe una vez y se lee en una sola línea:

```text
Niveles de acceso (valen para todos los recursos elegidos)
  1. read: Lectura
  2. write: Creación y modificación
  3. delete: Eliminación
  4. import: Importación masiva
  5. export: Exportación masiva
Escribe los números separados por comas (ej.: 1, 3, 5-7) o "todos". Enter mantiene: 1. 1, 2
Elegidos: read, write
```

Para que no pregunte nada, define `KOBANA_PROMPT_DISABLED=1`.

## Scripts y agentes de IA

Sin una persona en el terminal — `stdin`, `stdout` y `stderr` no son todos un terminal, como en un pipe, en CI o en un agente de IA — o con `KOBANA_PROMPT_DISABLED=1`, `kobana` no pregunta nada y usa lo que dice la línea de comandos:

- `auth login` pide todos los recursos publicados con acceso de lectura (`--scopes` elige otros);
- `update` solo muestra el comando que instalaría;
- el aviso de versión nueva no aparece (ni consulta la red); `KOBANA_NO_UPDATE_NOTIFIER=1` lo apaga también en el terminal.

La salida es JSON (el predeterminado). `--fields` elige los campos y `--jq` filtra con una expresión [jq](https://jqlang.github.io/jq/manual/), sin tener `jq` instalado — ver [Salida](/usage/output#filtrar-con---jq).

## Resumen

| Opción o variable | Equivalente en `gh` | Efecto |
|---|---|---|
| `--no-color`, `NO_COLOR`, `CLICOLOR=0` | `NO_COLOR`, `CLICOLOR=0` | Sin colores |
| `CLICOLOR_FORCE` | `CLICOLOR_FORCE` | Colores fuera del terminal |
| `KOBANA_ACCESSIBLE_COLORS` | `GH_ACCESSIBLE_COLORS` | Alto contraste, sin matices |
| `KOBANA_SCREEN_READER` | `GH_ACCESSIBLE_PROMPTER` + `GH_SPINNER_DISABLED` | Texto estático: sin símbolos decorativos, tablas sin bordes, preguntas simples |
| `KOBANA_ACCESSIBLE_PROMPTER` | `GH_ACCESSIBLE_PROMPTER` | Solo las preguntas simples |
| `KOBANA_PROMPT_DISABLED` | `GH_PROMPT_DISABLED` | Nunca pregunta |
| `KOBANA_NO_UPDATE_NOTIFIER` | `GH_NO_UPDATE_NOTIFIER` | Sin aviso de versión nueva |
| `--jq <EXPRESIÓN>` | `--jq` | Filtra el JSON de la salida |

Las variables valen cuando tienen un valor activo: cualquiera, menos vacío, `0`, `false`, `no` y `off`. `NO_COLOR`, como dice su estándar, vale con cualquier valor no vacío.
