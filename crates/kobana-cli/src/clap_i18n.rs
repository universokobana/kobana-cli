//! clap speaks English only: "Usage:", "Options:", "Print help", and its
//! error messages are hard-coded. Every command is built through [`command`],
//! which swaps those for the user's language, and parse errors go through
//! [`exit_with`], which rewrites the common ones.

use clap::builder::styling::{AnsiColor, Style};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, ArgAction, Command};
use kobana::{tr, trf};

const HEADING: Style = Style::new().bold().underline();
const LITERAL: Style = Style::new().bold();
const ERROR: Style = AnsiColor::Red.on_default().bold();
const TIP: Style = AnsiColor::Green.on_default();

/// `[OPÇÕES]`, the placeholder for options in a usage line
pub fn options_placeholder() -> &'static str {
    tr!(pt: "[OPÇÕES]", en: "[OPTIONS]", es: "[OPCIONES]")
}

/// `<COMANDO>`, the placeholder for a subcommand in a usage line
pub fn command_placeholder() -> &'static str {
    tr!(pt: "<COMANDO>", en: "<COMMAND>", es: "<COMANDO>")
}

/// A `Command` whose help reads in the user's language. `usage` is the full
/// usage line after the heading, e.g. `kobana auth login [OPÇÕES]`.
pub fn command(name: impl Into<clap::builder::Str>, usage: String) -> Command {
    let usage_heading = tr!(pt: "Uso:", en: "Usage:", es: "Uso:");
    Command::new(name)
        .help_template(format!(
            "{{before-help}}{{about-with-newline}}\n{HEADING}{usage_heading}{HEADING:#} {{usage}}\n\n{{all-args}}{{after-help}}"
        ))
        .override_usage(format!("{LITERAL}{usage}{LITERAL:#}"))
        .subcommand_help_heading(tr!(pt: "Comandos", en: "Commands", es: "Comandos"))
        .subcommand_value_name(command_placeholder().trim_matches(['<', '>']))
        .next_help_heading(tr!(pt: "Opções", en: "Options", es: "Opciones"))
}

/// Heading for positional arguments
pub fn arguments_heading() -> &'static str {
    tr!(pt: "Argumentos", en: "Arguments", es: "Argumentos")
}

/// Settings for the root command: clap's own `-h`, `-V` and `help` subcommand
/// are replaced by localized ones (global settings, so they reach every
/// subcommand).
pub fn root(cmd: Command) -> Command {
    cmd.disable_help_flag(true)
        .disable_version_flag(true)
        .disable_help_subcommand(true)
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .global(true)
                .action(ArgAction::Help)
                .help(tr!(pt: "Mostra a ajuda", en: "Print help", es: "Muestra la ayuda")),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .action(ArgAction::Version)
                .help(tr!(pt: "Mostra a versão", en: "Print version", es: "Muestra la versión")),
        )
}

/// The `help` subcommand: `kobana help banking bank-billets` prints the help
/// of that command, like clap's own
pub fn help_subcommand() -> Command {
    command("help", format!("kobana help [{}]...", tr!(pt: "COMANDO", en: "COMMAND", es: "COMANDO")))
        .about(tr!(
            pt: "Mostra a ajuda do kobana ou de um comando",
            en: "Print the help of kobana or of a command",
            es: "Muestra la ayuda de kobana o de un comando",
        ))
        .arg(
            Arg::new("command")
                .num_args(0..)
                .value_name(tr!(pt: "COMANDO", en: "COMMAND", es: "COMANDO"))
                .help_heading(arguments_heading())
                .help(tr!(
                    pt: "O comando, como em `kobana help banking bank-billets`",
                    en: "The command, as in `kobana help banking bank-billets`",
                    es: "El comando, como en `kobana help banking bank-billets`",
                )),
        )
}

/// Print the help of the command named by `path` under `root`
pub fn print_help_for(mut root: Command, path: &[String]) -> Result<(), kobana::error::KobanaError> {
    root.build();
    let mut cmd = &mut root;
    for name in path {
        cmd = match cmd.find_subcommand_mut(name) {
            Some(sub) => sub,
            None => {
                return Err(kobana::error::KobanaError::Validation(trf!(
                    pt: "comando desconhecido '{}'",
                    en: "unrecognized command '{}'",
                    es: "comando desconocido '{}'",
                    path.join(" ")
                )))
            }
        };
    }
    cmd.print_help()?;
    Ok(())
}

/// Print a clap parse error in the user's language and exit. Help and version
/// requests print as usual (their text is already localized).
pub fn exit_with(err: clap::Error) -> ! {
    match err.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayVersion
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => err.exit(),
        _ => {}
    }

    let Some(message) = describe(&err) else {
        // A kind the CLI never produces in practice: keep clap's own text
        err.exit()
    };

    let styled = std::io::IsTerminal::is_terminal(&std::io::stderr());
    let paint = |style: Style, text: &str| {
        if styled {
            format!("{style}{text}{style:#}")
        } else {
            text.to_string()
        }
    };

    let mut out = format!(
        "{} {message}\n",
        paint(ERROR, tr!(pt: "erro:", en: "error:", es: "error:"))
    );
    if let Some(tip) = suggestion(&err) {
        out.push_str(&format!("\n  {} {tip}\n", paint(TIP, tr!(pt: "dica:", en: "tip:", es: "sugerencia:"))));
    }
    if let Some(ContextValue::StyledStr(usage)) = err.get(ContextKind::Usage) {
        let usage = if styled { usage.ansi().to_string() } else { usage.to_string() };
        // clap renders it with its own "Usage:" heading
        let usage = usage.split_once(' ').map_or(usage.as_str(), |(_, rest)| rest);
        out.push_str(&format!(
            "\n{} {usage}\n",
            paint(HEADING, tr!(pt: "Uso:", en: "Usage:", es: "Uso:"))
        ));
    }
    out.push_str(&format!(
        "\n{}\n",
        tr!(
            pt: "Para mais informações, use '--help'.",
            en: "For more information, try '--help'.",
            es: "Para más información, usa '--help'.",
        )
    ));

    eprint!("{out}");
    std::process::exit(2)
}

fn text(err: &clap::Error, kind: ContextKind) -> Option<String> {
    match err.get(kind)? {
        ContextValue::String(s) => Some(s.clone()),
        ContextValue::Strings(list) => Some(list.join(", ")),
        ContextValue::StyledStr(s) => Some(s.to_string()),
        ContextValue::StyledStrs(list) => Some(
            list.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "),
        ),
        ContextValue::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn describe(err: &clap::Error) -> Option<String> {
    Some(match err.kind() {
        ErrorKind::InvalidSubcommand => {
            let name = text(err, ContextKind::InvalidSubcommand)?;
            trf!(
                pt: "comando desconhecido '{name}'",
                en: "unrecognized subcommand '{name}'",
                es: "comando desconocido '{name}'",
            )
        }
        ErrorKind::UnknownArgument => {
            let arg = text(err, ContextKind::InvalidArg)?;
            trf!(
                pt: "argumento inesperado '{arg}'",
                en: "unexpected argument '{arg}' found",
                es: "argumento inesperado '{arg}'",
            )
        }
        ErrorKind::InvalidValue => {
            let value = text(err, ContextKind::InvalidValue)?;
            let arg = text(err, ContextKind::InvalidArg)?;
            let mut msg = if value.is_empty() {
                trf!(
                    pt: "'{arg}' precisa de um valor",
                    en: "a value is required for '{arg}' but none was supplied",
                    es: "'{arg}' necesita un valor",
                )
            } else {
                trf!(
                    pt: "valor inválido '{value}' para '{arg}'",
                    en: "invalid value '{value}' for '{arg}'",
                    es: "valor inválido '{value}' para '{arg}'",
                )
            };
            if let Some(valid) = text(err, ContextKind::ValidValue) {
                msg.push_str(&trf!(
                    pt: "\n  [valores possíveis: {valid}]",
                    en: "\n  [possible values: {valid}]",
                    es: "\n  [valores posibles: {valid}]",
                ));
            }
            msg
        }
        ErrorKind::MissingRequiredArgument => {
            let args = match err.get(ContextKind::InvalidArg)? {
                ContextValue::Strings(list) => list.join("\n  "),
                _ => text(err, ContextKind::InvalidArg)?,
            };
            trf!(
                pt: "faltam argumentos obrigatórios:\n  {args}",
                en: "the following required arguments were not provided:\n  {args}",
                es: "faltan argumentos obligatorios:\n  {args}",
            )
        }
        ErrorKind::ArgumentConflict => {
            let arg = text(err, ContextKind::InvalidArg)?;
            let prior = text(err, ContextKind::PriorArg)?;
            if arg == prior {
                trf!(
                    pt: "'{arg}' não pode ser usado mais de uma vez",
                    en: "the argument '{arg}' cannot be used multiple times",
                    es: "'{arg}' no se puede usar más de una vez",
                )
            } else {
                trf!(
                    pt: "'{arg}' não pode ser usado com '{prior}'",
                    en: "the argument '{arg}' cannot be used with '{prior}'",
                    es: "'{arg}' no se puede usar con '{prior}'",
                )
            }
        }
        ErrorKind::MissingSubcommand => {
            let name = text(err, ContextKind::InvalidSubcommand)?;
            trf!(
                pt: "'{name}' precisa de um comando, mas nenhum foi informado",
                en: "'{name}' requires a subcommand but one was not provided",
                es: "'{name}' necesita un comando, pero no se indicó ninguno",
            )
        }
        _ => return None,
    })
}

fn suggestion(err: &clap::Error) -> Option<String> {
    if let Some(sub) = text(err, ContextKind::SuggestedSubcommand) {
        return Some(trf!(
            pt: "existe um comando parecido: '{sub}'",
            en: "a similar subcommand exists: '{sub}'",
            es: "existe un comando parecido: '{sub}'",
        ));
    }
    if let Some(arg) = text(err, ContextKind::SuggestedArg) {
        return Some(trf!(
            pt: "existe um argumento parecido: '{arg}'",
            en: "a similar argument exists: '{arg}'",
            es: "existe un argumento parecido: '{arg}'",
        ));
    }
    if let Some(value) = text(err, ContextKind::SuggestedValue) {
        return Some(trf!(
            pt: "talvez você quis dizer '{value}'",
            en: "did you mean '{value}'?",
            es: "quizás quisiste decir '{value}'",
        ));
    }
    if matches!(err.get(ContextKind::TrailingArg), Some(ContextValue::Bool(true))) {
        let arg = text(err, ContextKind::InvalidArg)?;
        return Some(trf!(
            pt: "para passar '{arg}' como valor, use '-- {arg}'",
            en: "to pass '{arg}' as a value, use '-- {arg}'",
            es: "para pasar '{arg}' como valor, usa '-- {arg}'",
        ));
    }
    None
}

/// `--lang` must take effect before any command is built, so it is read from
/// the raw arguments ahead of clap. Returns the language it names, if any.
pub fn lang_from_args(args: &[String]) -> Option<kobana::i18n::Lang> {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--" {
            break;
        }
        let value = if arg == "--lang" {
            iter.next().map(String::as_str)
        } else {
            arg.strip_prefix("--lang=")
        };
        if let Some(value) = value {
            return kobana::i18n::Lang::from_locale(value);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use kobana::i18n::Lang;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn reads_lang_from_the_raw_arguments() {
        assert_eq!(lang_from_args(&args(&["kobana", "--lang", "es", "auth"])), Some(Lang::Es));
        assert_eq!(lang_from_args(&args(&["kobana", "auth", "--lang=en"])), Some(Lang::En));
        assert_eq!(lang_from_args(&args(&["kobana", "auth"])), None);
        // After `--` it is a value, not the flag
        assert_eq!(lang_from_args(&args(&["kobana", "--", "--lang", "es"])), None);
    }
}
