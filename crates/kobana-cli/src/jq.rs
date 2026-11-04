//! `--jq`: a jq expression applied to the JSON the CLI prints, as `gh --jq`
//! does. It runs in-process (jaq), so `jq` need not be installed.
//!
//! Each result goes on its own line: a string as is (like `jq -r`), anything
//! else as JSON, indented only on a terminal. The expression is checked
//! before anything is sent, so a typo never costs a request.
//!
//! The expression may come from an AI agent, so it gets no way out of the
//! JSON it is given: `env` and `$ENV` (which would print the tokens in the
//! environment) are left out, and so are modules (`import`, `include`) and
//! `input`/`inputs`.

use std::sync::OnceLock;

use jaq_core::load::{self, Arena, File, Loader};
use jaq_core::{compile, data, Compiler, Ctx, Vars};
use jaq_json::Val;
use kobana::error::KobanaError;
use kobana::trf;

/// The `--jq` of this run, set once by [`set`]
static EXPR: OnceLock<String> = OnceLock::new();

/// Check that `expr` compiles and keep it for this run. A bad expression is a
/// validation error (exit 3) before any request.
pub fn set(expr: &str) -> Result<(), KobanaError> {
    run(expr, None)?;
    let _ = EXPR.set(expr.to_string());
    Ok(())
}

/// The `--jq` of this run, if one was given
pub fn expr() -> Option<&'static str> {
    EXPR.get().map(String::as_str)
}

/// Native filters that would reach outside the JSON
const LEFT_OUT: &[&str] = &["env"];

/// `@csv` and `@tsv`, which jq has and jaq leaves to its file-format crate
/// (which also reads files): written here as jq's manual defines them
const TABULAR: &str = r#"
def @csv: if type == "array" then map(
    if type == "string" then "\"" + (split("\"") | join("\"\"")) + "\""
    elif type == "null" then ""
    elif type == "number" or type == "boolean" then tostring
    else error("\(type) (\(tojson)) is not valid in a csv row") end
  ) | join(",") else error("cannot csv-format \(type), only an array can be") end;
def @tsv: if type == "array" then map(
    if type == "string" then split("\\") | join("\\\\") | split("\t") | join("\\t") | split("\n") | join("\\n") | split("\r") | join("\\r")
    elif type == "null" then ""
    elif type == "number" or type == "boolean" then tostring
    else error("\(type) (\(tojson)) is not valid in a tsv row") end
  ) | join("\t") else error("cannot tsv-format \(type), only an array can be") end;
"#;

/// Compile `expr` and, given an input, run it. Compiling alone returns no
/// values.
fn run(expr: &str, input: Option<&serde_json::Value>) -> Result<Vec<Val>, KobanaError> {
    let tabular = load::parse(TABULAR, |p| p.defs()).expect("@csv and @tsv parse");
    let defs = jaq_core::defs()
        .chain(jaq_std::defs())
        .chain(jaq_json::defs())
        .chain(tabular);
    let funs = jaq_core::funs()
        .chain(jaq_std::funs())
        .chain(jaq_json::funs())
        .filter(|(name, _, _)| !LEFT_OUT.contains(name));

    let arena = Arena::default();
    let modules = Loader::new(defs)
        .load(&arena, File { code: expr, path: () })
        .map_err(|errors| syntax_error(&errors))?;
    let filter = Compiler::default()
        .with_funs(funs)
        .compile(modules)
        .map_err(|errors| undefined_error(&errors))?;

    let Some(input) = input else {
        return Ok(Vec::new());
    };
    let input = serde_json::to_vec(input)?;
    let input = jaq_json::read::parse_single(&input)
        .map_err(|e| KobanaError::Internal(trf!(pt: "--jq: JSON inválido: {e}", en: "--jq: invalid JSON: {e}", es: "--jq: JSON inválido: {e}")))?;

    let ctx = Ctx::<data::JustLut<Val>>::new(&filter.lut, Vars::new([]));
    let mut values = Vec::new();
    for result in filter.id.run((ctx, input)) {
        let exception = match result {
            Ok(value) => {
                values.push(value);
                continue;
            }
            Err(exception) => exception,
        };
        match exception.get_err() {
            Ok(error) => {
                return Err(KobanaError::Validation(trf!(
                    pt: "--jq falhou: {error}",
                    en: "--jq failed: {error}",
                    es: "--jq falló: {error}",
                )))
            }
            // `halt`, `halt_error`: the output ends there
            Err(exception) => match exception.get_halt() {
                Ok(0) => break,
                Ok(code) => {
                    return Err(KobanaError::Validation(trf!(
                        pt: "--jq parou com halt({code})",
                        en: "--jq stopped with halt({code})",
                        es: "--jq se detuvo con halt({code})",
                    )))
                }
                Err(_) => return Err(KobanaError::Internal("--jq: unexpected exception".into())),
            },
        }
    }
    Ok(values)
}

/// Where in the expression parsing stopped, with a short piece of it
fn near(found: &str) -> String {
    let piece: String = found.chars().take(24).collect();
    if piece.is_empty() {
        kobana::tr!(pt: "no fim", en: "at the end", es: "al final").to_string()
    } else {
        trf!(pt: "perto de '{piece}'", en: "near '{piece}'", es: "cerca de '{piece}'")
    }
}

fn syntax_error(errors: &load::Errors<&str, ()>) -> KobanaError {
    let mut problems = Vec::new();
    for (_, error) in errors {
        match error {
            load::Error::Io(list) => problems.extend(list.iter().map(|(path, e)| format!("{path}: {e}"))),
            load::Error::Lex(list) => problems.extend(list.iter().map(|(expected, found)| {
                let (expected, found) = (expected.as_str(), near(found));
                trf!(pt: "esperava {expected} {found}", en: "expected {expected} {found}", es: "esperaba {expected} {found}")
            })),
            load::Error::Parse(list) => problems.extend(list.iter().map(|(expected, found)| {
                let (expected, found) = (expected.as_str(), near(found));
                trf!(pt: "esperava {expected} {found}", en: "expected {expected} {found}", es: "esperaba {expected} {found}")
            })),
        }
    }
    let problems = problems.join("; ");
    KobanaError::Validation(trf!(
        pt: "expressão --jq inválida: {problems}",
        en: "invalid --jq expression: {problems}",
        es: "expresión --jq inválida: {problems}",
    ))
}

fn undefined_error(errors: &compile::Errors<&str, ()>) -> KobanaError {
    let names = errors
        .iter()
        .flat_map(|(_, list)| list.iter())
        .map(|(name, undefined)| match undefined {
            compile::Undefined::Filter(arity) => format!("{name}/{arity}"),
            // The name of a variable keeps its `$`
            _ => name.to_string(),
        })
        .collect::<Vec<_>>()
        .join(", ");
    KobanaError::Validation(trf!(
        pt: "expressão --jq usa o que não existe: {names} (env e $ENV ficam de fora, para não expor variáveis de ambiente)",
        en: "the --jq expression uses what does not exist: {names} (env and $ENV are left out, so environment variables stay private)",
        es: "la expresión --jq usa lo que no existe: {names} (env y $ENV quedan fuera, para no exponer variables de entorno)",
    ))
}

/// The text `--jq` prints for `input`: one line per result, strings as is,
/// the rest as JSON (indented when `indent`). Going to a terminal
/// (`to_terminal`), a string's control characters become U+FFFD: API text is
/// not trusted, and an escape sequence in it would reach the screen raw.
pub fn render(expr: &str, input: &serde_json::Value, indent: bool, to_terminal: bool) -> Result<String, KobanaError> {
    let mut out = String::new();
    for value in run(expr, Some(input))? {
        match &value {
            Val::TStr(bytes) => {
                let text = String::from_utf8_lossy(bytes);
                if to_terminal {
                    out.extend(text.chars().map(|c| {
                        let keep = c == '\n' || c == '\t' || !(c.is_control() || ('\u{80}'..='\u{9f}').contains(&c));
                        if keep { c } else { '\u{FFFD}' }
                    }));
                } else {
                    out.push_str(&text);
                }
            }
            other => {
                let pp = jaq_json::write::Pp {
                    indent: indent.then(|| "  ".to_string()),
                    sep_space: indent,
                    ..Default::default()
                };
                let mut buf = Vec::new();
                jaq_json::write::write(&mut buf, &pp, 0, other)?;
                out.push_str(&String::from_utf8_lossy(&buf));
            }
        }
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn jq(expr: &str, input: serde_json::Value) -> Result<String, KobanaError> {
        render(expr, &input, false, false)
    }

    #[test]
    fn strings_print_raw_and_the_rest_as_json() {
        let data = json!({"data": [{"id": 1, "name": "Ana"}, {"id": 2, "name": "Bia"}]});
        assert_eq!(jq(".data[].name", data.clone()).unwrap(), "Ana\nBia\n");
        assert_eq!(jq(".data[].id", data.clone()).unwrap(), "1\n2\n");
        assert_eq!(jq(".data[0]", data.clone()).unwrap(), "{\"id\":1,\"name\":\"Ana\"}\n");
        assert_eq!(jq("[.data[].id] | length", data.clone()).unwrap(), "2\n");
        assert_eq!(jq("empty", data.clone()).unwrap(), "");
        // The standard library is there: @tsv, map, select, test
        assert_eq!(jq(r#".data[] | select(.name | test("^B")) | [.id, .name] | @tsv"#, data).unwrap(), "2\tBia\n");
    }

    #[test]
    fn csv_and_tsv_follow_jq() {
        let row = json!(["a\"b", 1.5, null, true, "x\ty\nz\\w"]);
        assert_eq!(jq("@csv", row.clone()).unwrap(), "\"a\"\"b\",1.5,,true,\"x\ty\nz\\w\"\n");
        assert_eq!(jq("@tsv", row).unwrap(), "a\"b\t1.5\t\ttrue\tx\\ty\\nz\\\\w\n");
        assert!(jq("@csv", json!({"a": 1})).is_err());
        assert!(jq("@tsv", json!([[1]])).is_err());
        assert_eq!(jq("@base64", json!("hi")).unwrap(), "aGk=\n");
    }

    #[test]
    fn indented_output_is_for_terminals() {
        let text = render(".", &json!({"a": [1]}), true, true).unwrap();
        assert_eq!(text, "{\n  \"a\": [\n    1\n  ]\n}\n");
    }

    #[test]
    fn strings_lose_their_control_characters_on_a_terminal_only() {
        let input = json!({"s": "a\u{1b}]8;;https://evil\u{7}b\u{85}\tc\nd"});
        let shown = render(".s", &input, false, true).unwrap();
        assert!(!shown.contains('\u{1b}') && !shown.contains('\u{7}') && !shown.contains('\u{85}'), "{shown:?}");
        assert!(shown.contains("\tc\nd"), "{shown:?}");
        let piped = render(".s", &input, false, false).unwrap();
        assert!(piped.contains('\u{1b}'), "a pipe gets the bytes as they are");
        // Inside JSON, controls are escaped anyway
        assert!(!render(".", &input, false, true).unwrap().contains('\u{1b}'));
    }

    #[test]
    fn a_bad_expression_is_a_validation_error_before_anything_runs() {
        for bad in [".data[", "map(", ".a | foo", "\"unterminated", "if . then 1"] {
            let err = set(bad).unwrap_err();
            assert!(matches!(err, KobanaError::Validation(_)), "{bad}: {err}");
        }
        assert!(run(".data[] | .id", None).is_ok());
    }

    #[test]
    fn the_environment_and_files_are_out_of_reach() {
        for expr in ["env", "$ENV", "$ENV.HOME", "env.HOME", "input", "inputs", r#"import "x" as x; ."#, r#"include "x"; ."#] {
            let err = run(expr, None).unwrap_err();
            assert!(matches!(err, KobanaError::Validation(_)), "{expr}: {err}");
        }
    }

    #[test]
    fn runtime_errors_and_halt() {
        let err = jq(".a[]", json!({"a": 1})).unwrap_err();
        assert!(matches!(err, KobanaError::Validation(ref m) if m.contains("--jq")), "{err}");
        assert_eq!(jq("1, halt, 2", json!(null)).unwrap(), "1\n");
        assert!(jq("\"x\" | halt_error(1)", json!(null)).is_err());
    }
}
