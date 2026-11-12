//! How the CLI presents itself in a terminal, and the accessibility settings
//! that change it. Modelled on the GitHub CLI (`gh help environment`,
//! `gh accessibility`):
//!
//! | Setting | Effect |
//! |---|---|
//! | `--no-color`, `NO_COLOR` (any value), `CLICOLOR=0` | no ANSI color or style |
//! | `CLICOLOR_FORCE` (any non-empty value, as clap reads it) | color even when the output is not a terminal |
//! | `KOBANA_ACCESSIBLE_COLORS` | high contrast: bold and underline in the terminal's own text color, no hues |
//! | `KOBANA_SCREEN_READER` | static text only: no decorative symbols, tables without borders, simple prompts |
//! | `KOBANA_ACCESSIBLE_PROMPTER` | the simple prompts alone |
//! | `KOBANA_PROMPT_DISABLED` | never ask; take what a script would get |
//!
//! Color never carries meaning alone: every colored piece of text sits next to
//! a word saying the same (`erro:`, `Aviso:`, `dica:`). The CLI has no
//! spinner, and its one animation is a picture you ask for (`kobana art anim`,
//! `kobana oi`, see [`crate::art`]), so nothing ever redraws a line a screen
//! reader is reading; keep it that way, or make it static under
//! [`screen_reader`].

use std::io::{BufRead, IsTerminal, Write};
use std::sync::atomic::{AtomicBool, Ordering};

use clap::builder::styling::{AnsiColor, Style, Styles};
use kobana::{tr, trf};

/// `--no-color`, read from the raw arguments before clap (parse errors are
/// printed before any match exists) and confirmed from the matches after.
static NO_COLOR_FLAG: AtomicBool = AtomicBool::new(false);

/// Windows only: virtual terminal processing could not be turned on, so a
/// console may print escapes raw. The CLI's own colors stay off on
/// terminals then; a forced color into a pipe is not the console's business.
static NO_ANSI_CONSOLE: AtomicBool = AtomicBool::new(false);

/// Read `--no-color` from the raw arguments. Call before anything is printed.
pub fn init(args: &[String]) {
    set_no_color_flag(no_color_in_args(args));
    // clap's output goes through anstream, which turns on virtual terminal
    // processing itself; the CLI's own messages need it too (no-op elsewhere)
    if anstyle_query::windows::enable_ansi_colors() == Some(false) {
        NO_ANSI_CONSOLE.store(true, Ordering::Relaxed);
    }
    if !color(Stream::Stderr) || accessible_colors() {
        // inquire honours NO_COLOR alone, and its default colors include a
        // dark grey that is hard to read on a dark background
        inquire::set_global_render_config(inquire::ui::RenderConfig::empty());
    }
}

/// The parsed `--no-color` (an argument after `--`, or a value that looked
/// like the flag, is not one)
pub fn set_no_color_flag(on: bool) {
    NO_COLOR_FLAG.store(on, Ordering::Relaxed);
}

fn no_color_in_args(args: &[String]) -> bool {
    args.iter().take_while(|arg| *arg != "--").any(|arg| arg == "--no-color")
}

/// gh's notion of an enabled setting: set, and not empty, `0`, `false`, `no`
/// or `off` (in any case)
pub fn truthy(value: Option<&str>) -> bool {
    value.is_some_and(|v| !matches!(v.trim().to_ascii_lowercase().as_str(), "" | "0" | "false" | "no" | "off"))
}

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

fn env_flag(name: &str) -> bool {
    truthy(env(name).as_deref())
}

/// `KOBANA_SCREEN_READER`: static text for screen readers and braille
/// displays — no decorative symbols, tables one line per record, simple prompts
pub fn screen_reader() -> bool {
    env_flag("KOBANA_SCREEN_READER")
}

/// Prompts that print their options once and read one line, instead of a
/// list redrawn under the cursor: `KOBANA_ACCESSIBLE_PROMPTER`, or the
/// screen-reader mode
pub fn accessible_prompter() -> bool {
    screen_reader() || env_flag("KOBANA_ACCESSIBLE_PROMPTER")
}

/// `KOBANA_ACCESSIBLE_COLORS`: the high-contrast palette
pub fn accessible_colors() -> bool {
    env_flag("KOBANA_ACCESSIBLE_COLORS")
}

/// `KOBANA_PROMPT_DISABLED`: never ask anything
pub fn prompt_disabled() -> bool {
    env_flag("KOBANA_PROMPT_DISABLED")
}

/// Whether the CLI may ask the person a question: never with
/// `KOBANA_PROMPT_DISABLED`, and only when stdin, stdout and stderr are all
/// a terminal. A pipe, CI or an AI agent gets the defaults a flag would
/// give, instead of a question nobody is there to answer (gh's `CanPrompt`).
pub fn can_prompt() -> bool {
    !prompt_disabled()
        && std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal()
        && std::io::stderr().is_terminal()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stream {
    Stdout,
    Stderr,
}

impl Stream {
    fn is_terminal(self) -> bool {
        match self {
            Self::Stdout => std::io::stdout().is_terminal(),
            Self::Stderr => std::io::stderr().is_terminal(),
        }
    }
}

/// Whether text written to `stream` gets ANSI styles
pub fn color(stream: Stream) -> bool {
    let terminal = stream.is_terminal();
    if terminal && NO_ANSI_CONSOLE.load(Ordering::Relaxed) {
        return false;
    }
    color_with(NO_COLOR_FLAG.load(Ordering::Relaxed), env, terminal, cfg!(windows))
}

/// `--no-color`, then exactly what clap's anstream decides, so the help and
/// the CLI's own messages always agree: `NO_COLOR` (any non-empty value) is
/// off; `CLICOLOR_FORCE` (any non-empty value) is on, even piped;
/// `CLICOLOR=0` is off; otherwise a terminal whose `TERM` takes color (set
/// and not `dumb`; any on Windows), or one with `CLICOLOR` on or in `CI`.
fn color_with(flag: bool, env: impl Fn(&str) -> Option<String>, terminal: bool, windows: bool) -> bool {
    let non_empty = |name: &str| env(name).is_some_and(|v| !v.is_empty());
    if flag || non_empty("NO_COLOR") {
        return false;
    }
    if non_empty("CLICOLOR_FORCE") {
        return true;
    }
    let clicolor = env("CLICOLOR").map(|v| v != "0");
    if clicolor == Some(false) {
        return false;
    }
    let term_takes_color = windows || env("TERM").is_some_and(|t| t != "dumb");
    terminal && (term_takes_color || clicolor == Some(true) || env("CI").is_some())
}

/// `text` safe to write to a terminal: control characters (C0 but line
/// breaks and tabs, DEL, C1) become U+FFFD. API text is not trusted, and an
/// escape sequence in it could move the cursor, clear the screen or plant a
/// link.
pub fn terminal_safe(text: &str) -> String {
    text.chars()
        .map(|c| {
            let control = c.is_control() || ('\u{80}'..='\u{9f}').contains(&c);
            if control && c != '\n' && c != '\t' { '\u{FFFD}' } else { c }
        })
        .collect()
}

/// What a piece of styled text means. The palette has only the 16 ANSI
/// colors, which every terminal lets the person redefine, and no greys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Error,
    Warning,
    Tip,
    Heading,
}

fn style(role: Role, accessible: bool) -> Style {
    match (role, accessible) {
        (Role::Heading, _) => Style::new().bold().underline(),
        // High contrast: the terminal's own text color, whose contrast the
        // person's theme guarantees, light or dark. Bold sets it apart.
        (_, true) => Style::new().bold(),
        (Role::Error, false) => AnsiColor::Red.on_default().bold(),
        (Role::Warning, false) => AnsiColor::Yellow.on_default().bold(),
        (Role::Tip, false) => AnsiColor::Green.on_default(),
    }
}

/// `text` in the style of `role`, when `stream` takes color
pub fn paint(role: Role, text: &str, stream: Stream) -> String {
    if !color(stream) {
        return text.to_string();
    }
    let style = style(role, accessible_colors());
    format!("{style}{text}{style:#}")
}

/// clap's styles for help and parse errors: its defaults, or the
/// high-contrast palette (no hues) under `KOBANA_ACCESSIBLE_COLORS`
pub fn clap_styles() -> Styles {
    if !accessible_colors() {
        return Styles::default();
    }
    let strong = Style::new().bold();
    Styles::styled()
        .header(style(Role::Heading, true))
        .usage(style(Role::Heading, true))
        .literal(strong)
        .placeholder(Style::new())
        .error(strong)
        .valid(strong)
        .invalid(strong.underline())
        .context(Style::new())
        .context_value(strong)
}

/// clap's color choice: never with `--no-color`; otherwise clap decides per
/// stream with the same rules as [`color`]
pub fn clap_color_choice() -> clap::ColorChoice {
    if NO_COLOR_FLAG.load(Ordering::Relaxed) {
        clap::ColorChoice::Never
    } else {
        clap::ColorChoice::Auto
    }
}

/// A warning on stderr, labelled by a word and not by its color alone. The
/// message may quote API text (a `--jq` error does), so it goes through
/// [`terminal_safe`].
pub fn warn(message: &str) {
    let label = paint(Role::Warning, tr!(pt: "Aviso:", en: "Warning:", es: "Aviso:"), Stream::Stderr);
    eprintln!("{label} {}", terminal_safe(message));
}

/// A symbol that only decorates a line whose words already say it all
/// (`✓ Atualizado`). Screen readers read it out by name, so the
/// screen-reader mode drops it.
pub fn decoration(symbol: &'static str) -> &'static str {
    if screen_reader() {
        ""
    } else {
        symbol
    }
}

// ---------------------------------------------------------------- prompts

/// A yes/no question; Enter takes `default`. Accepts the answer in any of the
/// three languages (s/sim, y/yes, n/não). `Err` carries why it was not
/// answered (cancelled, end of input).
pub fn confirm(question: &str, default: bool) -> Result<bool, String> {
    if accessible_prompter() {
        let stdin = std::io::stdin();
        return ask_yes_no(&mut stdin.lock(), &mut std::io::stderr(), question, default);
    }
    let (yes, no) = (tr!(pt: "Sim", en: "Yes", es: "Sí"), tr!(pt: "Não", en: "No", es: "No"));
    let hint = yes_no_hint(default);
    let parser = |answer: &str| parse_yes_no(answer).ok_or(());
    inquire::Confirm::new(question)
        .with_default(default)
        // inquire puts the default in parentheses itself
        .with_default_value_formatter(&|_| hint.trim_matches(['(', ')']).to_string())
        .with_formatter(&|answer| if answer { yes } else { no }.to_string())
        .with_parser(&parser)
        .with_error_message(answer_yes_or_no())
        .prompt()
        .map_err(|e| e.to_string())
}

fn yes_no_hint(default: bool) -> &'static str {
    if default {
        tr!(pt: "(S/n)", en: "(Y/n)", es: "(S/n)")
    } else {
        tr!(pt: "(s/N)", en: "(y/N)", es: "(s/N)")
    }
}

fn answer_yes_or_no() -> &'static str {
    tr!(pt: "Responda s ou n", en: "Answer y or n", es: "Responde s o n")
}

fn parse_yes_no(answer: &str) -> Option<bool> {
    match answer.trim().to_lowercase().as_str() {
        "s" | "sim" | "si" | "sí" | "y" | "yes" => Some(true),
        "n" | "nao" | "não" | "no" => Some(false),
        _ => None,
    }
}

fn cancelled() -> String {
    tr!(pt: "entrada encerrada sem resposta", en: "input ended without an answer", es: "la entrada terminó sin respuesta").to_string()
}

/// One line of input, without its line break; `None` at the end of input
fn read_answer(input: &mut impl BufRead) -> Result<Option<String>, String> {
    let mut line = String::new();
    match input.read_line(&mut line) {
        Ok(0) => Ok(None),
        Ok(_) => Ok(Some(line.trim_end_matches(['\n', '\r']).to_string())),
        Err(e) => Err(e.to_string()),
    }
}

fn ask_yes_no(input: &mut impl BufRead, out: &mut impl Write, question: &str, default: bool) -> Result<bool, String> {
    let io = |e: std::io::Error| e.to_string();
    loop {
        write!(out, "{question} {} ", yes_no_hint(default)).map_err(io)?;
        out.flush().map_err(io)?;
        let Some(answer) = read_answer(input)? else {
            return Err(cancelled());
        };
        if answer.trim().is_empty() {
            return Ok(default);
        }
        match parse_yes_no(&answer) {
            Some(yes) => return Ok(yes),
            None => writeln!(out, "{}.", answer_yes_or_no()).map_err(io)?,
        }
    }
}

/// An option of [`multi_select`]: a name and what it means (may be empty)
#[derive(Clone)]
pub struct Choice {
    pub name: String,
    pub description: String,
    /// Width of the name column in the list, so descriptions line up
    width: usize,
}

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, description) = (terminal_safe(&self.name), terminal_safe(&self.description));
        if description.is_empty() {
            return write!(f, "{name}");
        }
        write!(f, "{name:<width$}  {description}", width = self.width)
    }
}

/// Options laid out in two columns. What is shown goes through
/// [`terminal_safe`]: the OAuth scopes come from a server's discovery
/// document. The returned names stay as they came.
pub fn choices(items: Vec<(String, String)>) -> Vec<Choice> {
    let width = items.iter().map(|(name, _)| name.chars().count()).max().unwrap_or(0);
    items
        .into_iter()
        .map(|(name, description)| Choice { name, description, width })
        .collect()
}

fn multi_select_help() -> &'static str {
    tr!(
        pt: "↑↓ Navegar · Espaço Selecionar · →/← Todos/Nenhum · Digite para filtrar · Enter Confirmar · Esc Cancelar",
        en: "↑↓ Move · Space Select · →/← All/None · Type to filter · Enter Confirm · Esc Cancel",
        es: "↑↓ Navegar · Espacio Seleccionar · →/← Todos/Ninguno · Escribe para filtrar · Enter Confirmar · Esc Cancelar",
    )
}

/// Pick any of `options`, with `defaults` (indexes) chosen up front. Returns
/// the indexes picked, in the order of `options`. The accessible prompter
/// prints a numbered list once and reads the numbers on one line; otherwise
/// it is an interactive list.
pub fn multi_select(message: &str, options: Vec<Choice>, defaults: &[usize]) -> Result<Vec<usize>, String> {
    if accessible_prompter() {
        let stdin = std::io::stdin();
        return ask_many(&mut stdin.lock(), &mut std::io::stderr(), message, &options, defaults);
    }
    let names_only = |picked: &[inquire::list_option::ListOption<&Choice>]| {
        picked.iter().map(|o| o.value.name.as_str()).collect::<Vec<_>>().join(", ")
    };
    inquire::MultiSelect::new(message, options)
        .with_default(defaults)
        .with_page_size(20)
        .with_formatter(&names_only)
        .with_help_message(multi_select_help())
        .raw_prompt()
        .map(|picked| picked.into_iter().map(|o| o.index).collect())
        .map_err(|e| e.to_string())
}

fn ask_many(
    input: &mut impl BufRead,
    out: &mut impl Write,
    message: &str,
    options: &[Choice],
    defaults: &[usize],
) -> Result<Vec<usize>, String> {
    let io = |e: std::io::Error| e.to_string();
    writeln!(out, "{message}").map_err(io)?;
    for (i, option) in options.iter().enumerate() {
        let (name, description) = (terminal_safe(&option.name), terminal_safe(&option.description));
        if description.is_empty() {
            writeln!(out, "  {}. {name}", i + 1).map_err(io)?;
        } else {
            writeln!(out, "  {}. {name}: {description}", i + 1).map_err(io)?;
        }
    }
    let kept = if defaults.is_empty() {
        tr!(pt: "nenhum", en: "none", es: "ninguno").to_string()
    } else {
        defaults.iter().map(|i| (i + 1).to_string()).collect::<Vec<_>>().join(", ")
    };
    let instruction = trf!(
        pt: "Digite os números separados por vírgula (ex.: 1, 3, 5-7) ou \"todos\". Enter mantém: {kept}.",
        en: "Type the numbers separated by commas (e.g. 1, 3, 5-7) or \"all\". Enter keeps: {kept}.",
        es: "Escribe los números separados por comas (ej.: 1, 3, 5-7) o \"todos\". Enter mantiene: {kept}.",
    );
    loop {
        write!(out, "{instruction} ").map_err(io)?;
        out.flush().map_err(io)?;
        let Some(answer) = read_answer(input)? else {
            return Err(cancelled());
        };
        match parse_selection(&answer, options.len(), defaults) {
            Ok(picked) => {
                let names: Vec<String> = picked.iter().map(|&i| terminal_safe(&options[i].name)).collect();
                let names = if names.is_empty() { tr!(pt: "nenhum", en: "none", es: "ninguno").to_string() } else { names.join(", ") };
                writeln!(out, "{} {names}", tr!(pt: "Escolhidos:", en: "Chosen:", es: "Elegidos:")).map_err(io)?;
                return Ok(picked);
            }
            Err(problem) => writeln!(out, "{problem}").map_err(io)?,
        }
    }
}

/// The indexes a typed answer picks: numbers from 1 and ranges (`5-7`),
/// separated by commas or spaces; `todos`/`all`; empty keeps `defaults`.
fn parse_selection(answer: &str, count: usize, defaults: &[usize]) -> Result<Vec<usize>, String> {
    let answer = answer.trim();
    if answer.is_empty() {
        return Ok(defaults.to_vec());
    }
    if matches!(answer.to_lowercase().as_str(), "todos" | "all" | "*") {
        return Ok((0..count).collect());
    }
    let invalid = |part: &str| {
        trf!(
            pt: "\"{part}\" não é um número de 1 a {count}.",
            en: "\"{part}\" is not a number from 1 to {count}.",
            es: "\"{part}\" no es un número de 1 a {count}.",
        )
    };
    let number = |part: &str| part.trim().parse::<usize>().ok().filter(|n| (1..=count).contains(n));
    let mut picked = Vec::new();
    for part in answer.split([',', ' ', ';']).filter(|p| !p.trim().is_empty()) {
        let (first, last) = match part.split_once('-') {
            Some((a, b)) => (number(a), number(b)),
            None => (number(part), number(part)),
        };
        match (first, last) {
            (Some(a), Some(b)) if a <= b => picked.extend(a - 1..b),
            _ => return Err(invalid(part)),
        }
    }
    picked.sort_unstable();
    picked.dedup();
    Ok(picked)
}

// ---------------------------------------------------------------- `kobana accessibility`

/// What `kobana accessibility` prints: the settings, as `gh accessibility`
/// does, and which of them are on in this terminal
pub fn accessibility_text() -> String {
    let on_off = |on: bool| if on { tr!(pt: "sim", en: "yes", es: "sí") } else { tr!(pt: "não", en: "no", es: "no") };
    let heading = |text: &str| paint(Role::Heading, text, Stream::Stdout);
    let body = tr!(
        pt: "O kobana segue as práticas de acessibilidade do GitHub CLI (gh accessibility) para \
             quem usa leitor de tela, linha braille ou alto contraste, ou tem sensibilidade a movimento.",
        en: "kobana follows the accessibility practices of the GitHub CLI (gh accessibility) for \
             people who use a screen reader, a braille display or high contrast, or are sensitive to motion.",
        es: "kobana sigue las prácticas de accesibilidad de GitHub CLI (gh accessibility) para \
             quien usa lector de pantalla, línea braille o alto contraste, o es sensible al movimiento.",
    );
    let colors = tr!(
        pt: "  As cores só reforçam o que o texto já diz (erro:, Aviso:, dica:) e usam as 16 cores\n  \
             ANSI do terminal, que você ajusta nas preferências dele.\n  \
             - Sem cores: --no-color, NO_COLOR=1 ou CLICOLOR=0\n  \
             - Cores mesmo fora de um terminal: CLICOLOR_FORCE=1\n  \
             - Alto contraste (negrito e sublinhado na cor do texto do seu tema): KOBANA_ACCESSIBLE_COLORS=1",
        en: "  Colors only reinforce what the text already says (error:, Warning:, tip:) and use the\n  \
             terminal's 16 ANSI colors, which you adjust in its preferences.\n  \
             - No colors: --no-color, NO_COLOR=1 or CLICOLOR=0\n  \
             - Colors even outside a terminal: CLICOLOR_FORCE=1\n  \
             - High contrast (bold and underline in your theme's text color): KOBANA_ACCESSIBLE_COLORS=1",
        es: "  Los colores solo refuerzan lo que el texto ya dice (error:, Aviso:, sugerencia:) y usan los\n  \
             16 colores ANSI del terminal, que ajustas en sus preferencias.\n  \
             - Sin colores: --no-color, NO_COLOR=1 o CLICOLOR=0\n  \
             - Colores incluso fuera de un terminal: CLICOLOR_FORCE=1\n  \
             - Alto contraste (negrita y subrayado en el color del texto de tu tema): KOBANA_ACCESSIBLE_COLORS=1",
    );
    let reader = tr!(
        pt: "  KOBANA_SCREEN_READER=1 deixa só texto estático: sem símbolos decorativos, tabelas sem\n  \
             bordas (uma linha por registro) e perguntas simples, numeradas, que não redesenham a tela.\n  \
             O kobana não usa spinners; a única animação é a do Kadu (kobana art anim, kobana oi), que só\n  \
             roda se você pedir e aqui vira texto. Só as perguntas simples: KOBANA_ACCESSIBLE_PROMPTER=1",
        en: "  KOBANA_SCREEN_READER=1 leaves static text only: no decorative symbols, tables without\n  \
             borders (one line per record) and simple numbered prompts that do not redraw the screen.\n  \
             kobana has no spinners; its one animation is Kadu's (kobana art anim, kobana oi), which only\n  \
             runs if you ask and here becomes text. The simple prompts alone: KOBANA_ACCESSIBLE_PROMPTER=1",
        es: "  KOBANA_SCREEN_READER=1 deja solo texto estático: sin símbolos decorativos, tablas sin\n  \
             bordes (una línea por registro) y preguntas simples, numeradas, que no redibujan la pantalla.\n  \
             kobana no usa spinners; su única animación es la de Kadu (kobana art anim, kobana oi), que solo\n  \
             corre si la pides y aquí se vuelve texto. Solo las preguntas simples: KOBANA_ACCESSIBLE_PROMPTER=1",
    );
    let scripts = tr!(
        pt: "  Sem terminal (pipe, CI, agente de IA) ou com KOBANA_PROMPT_DISABLED=1, o kobana não pergunta\n  \
             nada: usa o que as opções da linha de comando dizem. A saída é JSON; --fields escolhe os\n  \
             campos e --jq filtra com uma expressão jq, sem precisar do jq instalado.",
        en: "  Without a terminal (pipe, CI, AI agent) or with KOBANA_PROMPT_DISABLED=1, kobana asks\n  \
             nothing: it goes by the command-line options. Output is JSON; --fields picks the fields\n  \
             and --jq filters it with a jq expression, with no jq installed.",
        es: "  Sin terminal (pipe, CI, agente de IA) o con KOBANA_PROMPT_DISABLED=1, kobana no pregunta\n  \
             nada: usa lo que dicen las opciones de la línea de comandos. La salida es JSON; --fields elige\n  \
             los campos y --jq la filtra con una expresión jq, sin tener jq instalado.",
    );
    let dotenv = crate::config::config_dir().join(".env");
    let dotenv = dotenv.display();
    let keep = trf!(
        pt: "Para manter uma escolha, ponha a variável no .env da pasta de configuração ({dotenv}).",
        en: "To keep a choice, put the variable in the config folder's .env ({dotenv}).",
        es: "Para mantener una elección, pon la variable en el .env de la carpeta de configuración ({dotenv}).",
    );
    let more = tr!(
        pt: "Saiba mais: https://cli.kobana.com.br/pt/usage/accessibility",
        en: "Learn more: https://cli.kobana.com.br/en/usage/accessibility",
        es: "Más información: https://cli.kobana.com.br/es/usage/accessibility",
    );
    let state = [
        (tr!(pt: "Cores", en: "Colors", es: "Colores"), on_off(color(Stream::Stdout))),
        (tr!(pt: "Alto contraste", en: "High contrast", es: "Alto contraste"), on_off(accessible_colors())),
        (tr!(pt: "Leitor de tela", en: "Screen reader", es: "Lector de pantalla"), on_off(screen_reader())),
        (tr!(pt: "Perguntas simples", en: "Simple prompts", es: "Preguntas simples"), on_off(accessible_prompter())),
        (tr!(pt: "Pode perguntar", en: "May prompt", es: "Puede preguntar"), on_off(can_prompt())),
    ]
    .iter()
    .map(|(name, value)| format!("  {name}: {value}"))
    .collect::<Vec<_>>()
    .join("\n");

    format!(
        "{body}\n\n{}\n{colors}\n\n{}\n{reader}\n\n{}\n{scripts}\n\n{}\n{state}\n\n{keep}\n{more}\n",
        heading(tr!(pt: "Cores", en: "Colors", es: "Colores")),
        heading(tr!(pt: "Leitor de tela", en: "Screen reader", es: "Lector de pantalla")),
        heading(tr!(pt: "Scripts e agentes", en: "Scripts and agents", es: "Scripts y agentes")),
        heading(tr!(pt: "Neste terminal", en: "In this terminal", es: "En este terminal")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(list: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |name| list.iter().find(|(k, _)| *k == name).map(|(_, v)| v.to_string())
    }

    #[test]
    fn truthy_follows_gh() {
        for on in ["1", "true", "yes", "enabled", "TRUE", "on"] {
            assert!(truthy(Some(on)), "{on}");
        }
        for off in ["", " ", "0", "false", "FALSE", "no", "off"] {
            assert!(!truthy(Some(off)), "{off:?}");
        }
        assert!(!truthy(None));
    }

    #[test]
    fn color_precedence_matches_clap() {
        let unix = |flag: bool, env: &'static [(&'static str, &'static str)], terminal: bool| {
            color_with(flag, vars(env), terminal, false)
        };
        let term = &[("TERM", "xterm-256color")];
        assert!(unix(false, term, true));
        assert!(!unix(false, term, false), "not a terminal");
        assert!(!unix(true, term, true), "--no-color");
        assert!(!unix(false, &[("NO_COLOR", "1"), ("TERM", "xterm")], true));
        // NO_COLOR is any non-empty value, `0` included
        assert!(!unix(false, &[("NO_COLOR", "0"), ("TERM", "xterm")], true));
        assert!(unix(false, &[("NO_COLOR", ""), ("TERM", "xterm")], true));
        assert!(!unix(false, &[("CLICOLOR", "0"), ("TERM", "xterm")], true));
        assert!(!unix(false, &[("TERM", "dumb")], true));
        assert!(!unix(false, &[], true), "no TERM");
        // CLICOLOR on or CI make up for TERM, on a terminal only
        assert!(unix(false, &[("CLICOLOR", "1")], true));
        assert!(unix(false, &[("CI", "true"), ("TERM", "dumb")], true));
        assert!(!unix(false, &[("CI", "true")], false));
        // Forced (any non-empty value, as anstream reads it): even piped, but
        // never over NO_COLOR or --no-color
        assert!(unix(false, &[("CLICOLOR_FORCE", "1")], false));
        assert!(unix(false, &[("CLICOLOR_FORCE", "0")], false));
        assert!(!unix(false, &[("CLICOLOR_FORCE", "")], false));
        assert!(!unix(false, &[("CLICOLOR_FORCE", "1"), ("NO_COLOR", "1")], true));
        assert!(!unix(true, &[("CLICOLOR_FORCE", "1")], true));
        // Windows consoles take color without TERM
        assert!(color_with(false, vars(&[]), true, true));
    }

    #[test]
    fn terminal_safe_keeps_text_and_line_breaks_only() {
        let text = "a\u{1b}[31mb\u{9b}2J\u{7f}\r\tc\nd ação";
        assert_eq!(terminal_safe(text), "a\u{FFFD}[31mb\u{FFFD}2J\u{FFFD}\u{FFFD}\tc\nd ação");
    }

    #[test]
    fn no_color_is_read_before_the_end_of_options_only() {
        let args = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(no_color_in_args(&args(&["kobana", "schema", "--no-color"])));
        assert!(!no_color_in_args(&args(&["kobana", "--", "--no-color"])));
        assert!(!no_color_in_args(&args(&["kobana", "--no-colors"])));
    }

    #[test]
    fn the_high_contrast_palette_has_no_hues() {
        for role in [Role::Error, Role::Warning, Role::Tip, Role::Heading] {
            assert_eq!(style(role, true).get_fg_color(), None, "{role:?}");
            assert!(style(role, true).get_effects().contains(clap::builder::styling::Effects::BOLD), "{role:?}");
        }
        // The default palette sticks to the 16 ANSI colors
        for role in [Role::Error, Role::Warning, Role::Tip] {
            assert!(matches!(style(role, false).get_fg_color(), Some(clap::builder::styling::Color::Ansi(_))), "{role:?}");
        }
    }

    #[test]
    fn a_selection_takes_numbers_ranges_and_all() {
        assert_eq!(parse_selection("", 5, &[0, 1]), Ok(vec![0, 1]));
        assert_eq!(parse_selection("  ", 5, &[]), Ok(vec![]));
        assert_eq!(parse_selection("3, 1", 5, &[]), Ok(vec![0, 2]));
        assert_eq!(parse_selection("1 2;2", 5, &[]), Ok(vec![0, 1]));
        assert_eq!(parse_selection("2-4", 5, &[]), Ok(vec![1, 2, 3]));
        assert_eq!(parse_selection("todos", 3, &[]), Ok(vec![0, 1, 2]));
        assert_eq!(parse_selection("ALL", 2, &[]), Ok(vec![0, 1]));
        for bad in ["0", "6", "x", "4-2", "1-", "-1", "1.5"] {
            assert!(parse_selection(bad, 5, &[]).is_err(), "{bad}");
        }
    }

    #[test]
    fn the_simple_multi_select_lists_once_and_asks_again_on_a_bad_answer() {
        let options = choices(vec![("read".into(), "Ler".into()), ("write".into(), String::new())]);
        let mut input = std::io::Cursor::new("9\n2\n");
        let mut out = Vec::new();
        let picked = ask_many(&mut input, &mut out, "Níveis", &options, &[0]).unwrap();
        assert_eq!(picked, vec![1]);
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains("  1. read: Ler\n") && text.contains("  2. write\n"), "{text}");
        // The list is printed once; nothing redraws it
        assert_eq!(text.matches("1. read").count(), 1, "{text}");
        assert!(!text.contains('\u{1b}') && !text.contains('\r'), "{text:?}");
        assert!(text.contains("\"9\""), "{text}");

        // End of input is not an answer
        let mut input = std::io::Cursor::new("");
        assert!(ask_many(&mut input, &mut Vec::new(), "x", &options, &[0]).is_err());
    }

    #[test]
    fn option_text_cannot_drive_the_terminal() {
        let options = choices(vec![("a\u{1b}[2Jb".into(), "c\u{9b}d".into())]);
        let mut out = Vec::new();
        ask_many(&mut std::io::Cursor::new("\n"), &mut out, "x", &options, &[0]).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(!text.contains('\u{1b}') && !text.contains('\u{9b}'), "{text:?}");
        assert!(!options[0].to_string().contains('\u{1b}'));
        // The picked name itself is untouched
        assert_eq!(options[0].name, "a\u{1b}[2Jb");
    }

    #[test]
    fn the_simple_confirm_takes_the_default_on_enter() {
        let ask = |answers: &str, default: bool| {
            ask_yes_no(&mut std::io::Cursor::new(answers.to_string()), &mut Vec::new(), "Seguir?", default)
        };
        assert_eq!(ask("\n", true), Ok(true));
        assert_eq!(ask("\n", false), Ok(false));
        assert_eq!(ask("talvez\nn\n", true), Ok(false));
        assert_eq!(ask("Sim\n", false), Ok(true));
        assert!(ask("", true).is_err(), "end of input");
    }
}
