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
//!
//! On the output of a write that already went through, the filter runs in a
//! child process with a time limit ([`render_isolated`]): an expression that
//! overflows the stack or never ends must not turn a done write into a crash
//! that invites a retry.

use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use jaq_core::load::{self, Arena, File, Loader};
use jaq_core::native::{self, Fun};
use jaq_core::{compile, data, Compiler, Ctx, Vars};
use jaq_json::{Num, Val};
use kobana::error::KobanaError;
use kobana::{tr, trf};

type Data = data::JustLut<Val>;
type Filter = jaq_core::Filter<Data>;

/// The `--jq` of this run, set once by [`set`]
static EXPR: OnceLock<String> = OnceLock::new();

/// Check that `expr` compiles and keep it for this run. A bad expression is a
/// validation error (exit 3) before any request.
pub fn set(expr: &str) -> Result<(), KobanaError> {
    compiled(expr)?;
    let _ = EXPR.set(expr.to_string());
    Ok(())
}

/// The `--jq` of this run, if one was given
pub fn expr() -> Option<&'static str> {
    EXPR.get().map(String::as_str)
}

/// Native filters left out: `env` would reach outside the JSON; the others
/// are replaced by [`natives`]
const LEFT_OUT: &[&str] = &["env", "fromjson", "stderr_empty", "debug_empty"];

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

/// Replacements for jaq's own:
/// - `fromjson` parses with serde_json, whose nesting limit (128) turns a
///   hostile API string of thousands of `[` into an error `try` can catch,
///   where jaq's parser recursed until the stack overflowed (`tonumber` and
///   `toboolean` go through it too);
/// - `stderr` and `debug` write to stderr as jq does (jaq hands them to the
///   logger, where they showed up as `ERROR` log lines and in log files),
///   with control characters made harmless.
fn natives() -> [Fun<Data>; 3] {
    [
        native::run(("fromjson", native::v(0), |cv| native::bome(from_json(&cv.1)))),
        native::run(("stderr_empty", native::v(0), |cv| {
            to_stderr(&cv.1, false);
            Box::new(core::iter::empty())
        })),
        native::run(("debug_empty", native::v(0), |cv| {
            to_stderr(&cv.1, true);
            Box::new(core::iter::empty())
        })),
    ]
}

fn from_json(value: &Val) -> Result<Val, jaq_core::Error<Val>> {
    let Val::TStr(bytes) = value else {
        return Err(jaq_core::Error::typ(value.clone(), "string"));
    };
    let parsed: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|e| jaq_core::Error::str(format!("cannot parse {} as JSON: {e}", shorten(&String::from_utf8_lossy(bytes)))))?;
    to_val(&parsed).map_err(jaq_core::Error::str)
}

/// A JSON value as jaq's
fn to_val(value: &serde_json::Value) -> Result<Val, String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    jaq_json::read::parse_single(&bytes).map_err(|e| e.to_string())
}

/// jq's `stderr` (the value, compact) and `debug` (`["DEBUG:",<value>]`)
fn to_stderr(value: &Val, debug: bool) {
    let text = match (to_serde(value), debug) {
        (Ok(json), true) => serde_json::json!(["DEBUG:", json]).to_string() + "\n",
        (Ok(serde_json::Value::String(s)), false) => s,
        (Ok(json), false) => json.to_string(),
        (Err(_), _) => value.to_string(),
    };
    eprint!("{}", crate::ui::terminal_safe(&text));
}

/// At most 300 characters of `text`: a jq error message carries the whole
/// value it failed on, which may be a page of API records
fn shorten(text: &str) -> String {
    const MAX: usize = 300;
    match text.char_indices().nth(MAX) {
        Some((cut, _)) => format!("{}…", &text[..cut]),
        None => text.to_string(),
    }
}

thread_local! {
    /// The last expression compiled on this thread: `--page-all` runs the
    /// filter once per item
    static COMPILED: RefCell<Option<(String, Rc<Filter>)>> = const { RefCell::new(None) };
}

/// `expr` compiled, from the cache when it is the last one compiled
fn compiled(expr: &str) -> Result<Rc<Filter>, KobanaError> {
    if let Some(filter) = COMPILED.with_borrow(|c| c.as_ref().filter(|(e, _)| e == expr).map(|(_, f)| f.clone())) {
        return Ok(filter);
    }
    let tabular = load::parse(TABULAR, |p| p.defs()).expect("@csv and @tsv parse");
    let defs = jaq_core::defs()
        .chain(jaq_std::defs())
        .chain(jaq_json::defs())
        .chain(tabular);
    let funs = jaq_core::funs()
        .chain(jaq_std::funs())
        .chain(jaq_json::funs())
        .filter(|(name, _, _)| !LEFT_OUT.contains(name))
        .chain(natives());

    let arena = Arena::default();
    let modules = Loader::new(defs)
        .load(&arena, File { code: expr, path: () })
        .map_err(|errors| syntax_error(&errors))?;
    let filter = Rc::new(
        Compiler::default()
            .with_funs(funs)
            .compile(modules)
            .map_err(|errors| undefined_error(&errors))?,
    );
    COMPILED.set(Some((expr.to_string(), filter.clone())));
    Ok(filter)
}

/// Run `expr` on `input`
fn run(expr: &str, input: &serde_json::Value) -> Result<Vec<Val>, KobanaError> {
    let filter = compiled(expr)?;
    let input = to_val(input)
        .map_err(|e| KobanaError::Internal(trf!(pt: "--jq: JSON inválido: {e}", en: "--jq: invalid JSON: {e}", es: "--jq: JSON inválido: {e}")))?;

    let ctx = Ctx::<Data>::new(&filter.lut, Vars::new([]));
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
                let error = shorten(&error.to_string());
                return Err(KobanaError::Validation(trf!(
                    pt: "--jq falhou: {error}",
                    en: "--jq failed: {error}",
                    es: "--jq falló: {error}",
                )));
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

/// What jaq's parser expected, in the user's language. A keyword or symbol
/// (`then`, `|`) is shown as it is.
fn expected(token: &str) -> String {
    const KNOWN: &[(&str, [&str; 3])] = &[
        ("digit", ["um dígito", "a digit", "un dígito"]),
        ("identifier", ["um nome", "a name", "un nombre"]),
        ("closing parenthesis", ["')'", "')'", "')'"]),
        ("closing bracket", ["']'", "']'", "']'"]),
        ("closing brace", ["'}'", "'}'", "'}'"]),
        ("closing quote", ["'\"'", "'\"'", "'\"'"]),
        ("string escape sequence", ["uma sequência de escape", "an escape sequence", "una secuencia de escape"]),
        ("4-digit hexadecimal UTF-8 code point", ["um código \\uXXXX", "a \\uXXXX code", "un código \\uXXXX"]),
        ("token", ["um símbolo", "a token", "un símbolo"]),
        ("variable", ["uma variável", "a variable", "una variable"]),
        ("pattern", ["um padrão", "a pattern", "un patrón"]),
        ("else or end", ["else ou end", "else or end", "else o end"]),
        ("comma or right bracket", ["',' ou ']'", "',' or ']'", "',' o ']'"]),
        ("comma or right brace", ["',' ou '}'", "',' or '}'", "',' o '}'"]),
        ("semicolon or right parenthesis", ["';' ou ')'", "';' or ')'", "';' o ')'"]),
        ("term", ["um termo", "a term", "un término"]),
        ("key", ["uma chave", "a key", "una clave"]),
        ("argument", ["um argumento", "an argument", "un argumento"]),
        ("string", ["um texto entre aspas", "a quoted string", "un texto entre comillas"]),
        ("nothing", ["nada", "nothing", "nada"]),
    ];
    match KNOWN.iter().find(|(name, _)| *name == token) {
        Some((_, labels)) => kobana::i18n::pick(*labels).to_string(),
        None => format!("'{token}'"),
    }
}

/// Where in the expression parsing stopped, with a short piece of it
fn near(found: &str) -> String {
    let piece: String = found.chars().take(24).collect();
    if piece.is_empty() {
        tr!(pt: "no fim", en: "at the end", es: "al final").to_string()
    } else {
        trf!(pt: "perto de '{piece}'", en: "near '{piece}'", es: "cerca de '{piece}'")
    }
}

fn syntax_error(errors: &load::Errors<&str, ()>) -> KobanaError {
    let mut problems = Vec::new();
    for (_, error) in errors {
        let expected_near = |what: &str, found: &str| {
            let (what, found) = (expected(what), near(found));
            trf!(pt: "esperava {what} {found}", en: "expected {what} {found}", es: "esperaba {what} {found}")
        };
        match error {
            // The only file the loader could read is a module, and it reads none
            load::Error::Io(_) => problems.push(
                tr!(
                    pt: "módulos (import, include) não existem no --jq",
                    en: "modules (import, include) do not exist in --jq",
                    es: "los módulos (import, include) no existen en --jq",
                )
                .to_string(),
            ),
            load::Error::Lex(list) => problems.extend(list.iter().map(|(what, found)| expected_near(what.as_str(), found))),
            load::Error::Parse(list) => problems.extend(list.iter().map(|(what, found)| expected_near(what.as_str(), found))),
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
    let names: Vec<String> = errors
        .iter()
        .flat_map(|(_, list)| list.iter())
        .map(|(name, undefined)| match undefined {
            compile::Undefined::Filter(arity) => format!("{name}/{arity}"),
            // The name of a variable keeps its `$`
            _ => name.to_string(),
        })
        .collect();
    let mut message = trf!(
        pt: "a expressão --jq usa o que não existe: {}",
        en: "the --jq expression uses what does not exist: {}",
        es: "la expresión --jq usa lo que no existe: {}",
        names.join(", ")
    );
    if names.iter().any(|n| n.starts_with("env/") || n == "$ENV") {
        message.push_str(tr!(
            pt: " (env e $ENV ficam de fora, para não expor variáveis de ambiente)",
            en: " (env and $ENV are left out, so environment variables stay private)",
            es: " (env y $ENV quedan fuera, para no exponer variables de entorno)",
        ));
    }
    KobanaError::Validation(message)
}

/// A jaq value as JSON. jq prints NaN as `null` and infinities as the
/// largest float; byte strings and non-string keys have no JSON form.
fn to_serde(value: &Val) -> Result<serde_json::Value, String> {
    use serde_json::Value;
    Ok(match value {
        Val::Null => Value::Null,
        Val::Bool(b) => Value::Bool(*b),
        Val::Num(Num::Float(f)) if f.is_nan() => Value::Null,
        Val::Num(Num::Float(f)) if f.is_infinite() => Value::from(f64::MAX.copysign(*f)),
        Val::Num(n) => n.to_string().parse::<serde_json::Number>().map(Value::Number).map_err(|e| e.to_string())?,
        Val::TStr(bytes) => Value::String(String::from_utf8_lossy(bytes).into_owned()),
        Val::BStr(_) => return Err(not_json(tr!(pt: "bytes", en: "bytes", es: "bytes"))),
        Val::Arr(items) => Value::Array(items.iter().map(to_serde).collect::<Result<_, _>>()?),
        Val::Obj(entries) => {
            let mut object = serde_json::Map::new();
            for (key, value) in entries.iter() {
                let Val::TStr(key) = key else {
                    return Err(not_json(tr!(pt: "chave que não é texto", en: "a non-string key", es: "clave que no es texto")));
                };
                object.insert(String::from_utf8_lossy(key).into_owned(), to_serde(value)?);
            }
            Value::Object(object)
        }
    })
}

fn not_json(what: &str) -> String {
    trf!(
        pt: "o resultado tem {what}, que não existe em JSON",
        en: "the result has {what}, which JSON cannot hold",
        es: "el resultado tiene {what}, que no existe en JSON",
    )
}

/// The text `--jq` prints for `input`: one line per result, strings as is,
/// the rest as JSON (indented when `indent`, C1 controls escaped). Going to a
/// terminal (`to_terminal`), a string's control characters become U+FFFD:
/// API text is not trusted, and an escape sequence in it would reach the
/// screen raw.
pub fn render(expr: &str, input: &serde_json::Value, indent: bool, to_terminal: bool) -> Result<String, KobanaError> {
    let mut out = String::new();
    for value in run(expr, input)? {
        if let Val::TStr(bytes) = &value {
            let text = String::from_utf8_lossy(bytes);
            if to_terminal {
                out.push_str(&crate::ui::terminal_safe(&text));
            } else {
                out.push_str(&text);
            }
        } else {
            let json = to_serde(&value).map_err(|e| KobanaError::Validation(format!("--jq: {e}")))?;
            let text = if indent { serde_json::to_string_pretty(&json)? } else { serde_json::to_string(&json)? };
            out.push_str(&kobana::error::escape_c1(&text));
        }
        out.push('\n');
    }
    Ok(out)
}

// ---------------------------------------------------------------- isolated run

/// The hidden command a child process runs: `kobana __jq-child <expr>
/// <indent> <to_terminal> <lang>`, the JSON on stdin
pub const CHILD: &str = "__jq-child";

/// How long the child may take
const CHILD_TIME_LIMIT: Duration = Duration::from_secs(30);

/// [`render`] in a child process of this same binary, killed after
/// [`CHILD_TIME_LIMIT`]. A stack overflow aborts a whole process and a loop
/// never ends, so only another process can survive them. `Err` says why
/// there is no output; what the filter wrote to stderr (`debug`) is passed on.
pub fn render_isolated(expr: &str, input: &serde_json::Value, indent: bool, to_terminal: bool) -> Result<String, String> {
    let flag = |on: bool| if on { "1" } else { "0" };
    let mut child = std::process::Command::new(std::env::current_exe().map_err(|e| e.to_string())?)
        .args([CHILD, expr, flag(indent), flag(to_terminal), kobana::i18n::lang().code()])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    // Feed and drain on threads, so a full pipe never blocks the other side
    let input = serde_json::to_vec(input).map_err(|e| e.to_string())?;
    let mut stdin = child.stdin.take().expect("piped");
    let feeder = std::thread::spawn(move || stdin.write_all(&input));
    let drain = |mut pipe: Box<dyn Read + Send>| {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = pipe.read_to_end(&mut bytes);
            bytes
        })
    };
    let stdout = drain(Box::new(child.stdout.take().expect("piped")));
    let stderr = drain(Box::new(child.stderr.take().expect("piped")));

    let deadline = Instant::now() + CHILD_TIME_LIMIT;
    let status = loop {
        match child.try_wait().map_err(|e| e.to_string())? {
            Some(status) => break status,
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                let secs = CHILD_TIME_LIMIT.as_secs();
                return Err(trf!(
                    pt: "--jq passou de {secs} s e foi interrompido",
                    en: "--jq took over {secs} s and was stopped",
                    es: "--jq pasó de {secs} s y se interrumpió",
                ));
            }
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    };
    let _ = feeder.join();
    let out = stdout.join().unwrap_or_default();
    let err = String::from_utf8_lossy(&stderr.join().unwrap_or_default()).into_owned();

    if status.success() {
        eprint!("{err}");
        return String::from_utf8(out).map_err(|e| e.to_string());
    }
    // Exit 3 is the child's own message ("--jq falhou: …"); anything else is
    // a crash (a stack overflow aborts with the runtime's text)
    let err = err.split_whitespace().collect::<Vec<_>>().join(" ");
    Err(match (status.code(), err.is_empty()) {
        (Some(3), false) => shorten(&err),
        (_, true) => trf!(pt: "--jq terminou com {status}", en: "--jq ended with {status}", es: "--jq terminó con {status}"),
        (_, false) => {
            let err = shorten(&err);
            trf!(pt: "--jq falhou: {err}", en: "--jq failed: {err}", es: "--jq falló: {err}")
        }
    })
}

/// The child's side of [`render_isolated`]: the exit code
pub fn child_main(args: &[String]) -> i32 {
    let [expr, indent, to_terminal, lang] = args else {
        return 2;
    };
    if let Some(lang) = kobana::i18n::Lang::from_locale(lang) {
        kobana::i18n::set_lang(lang);
    }
    let mut input = Vec::new();
    if std::io::stdin().read_to_end(&mut input).is_err() {
        return 5;
    }
    match child_render(expr, &input, indent == "1", to_terminal == "1") {
        Ok(text) => {
            print!("{text}");
            0
        }
        Err(message) => {
            eprintln!("{}", crate::ui::terminal_safe(&message));
            3
        }
    }
}

fn child_render(expr: &str, input: &[u8], indent: bool, to_terminal: bool) -> Result<String, String> {
    let value: serde_json::Value = serde_json::from_slice(input).map_err(|e| e.to_string())?;
    render(expr, &value, indent, to_terminal).map_err(|e| match e {
        KobanaError::Validation(message) | KobanaError::Internal(message) => message,
        other => other.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn jq(expr: &str, input: serde_json::Value) -> Result<String, KobanaError> {
        render(expr, &input, false, false)
    }

    fn check(expr: &str) -> Result<(), KobanaError> {
        compiled(expr).map(|_| ())
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
    fn controls_never_reach_a_terminal() {
        let input = json!({"s": "a\u{1b}]8;;https://evil\u{7}b\u{85}\u{9b}2J\tc\nd"});
        let c1 = |text: &str| text.chars().any(|c| ('\u{80}'..='\u{9f}').contains(&c));
        let shown = render(".s", &input, false, true).unwrap();
        assert!(!shown.contains('\u{1b}') && !shown.contains('\u{7}') && !c1(&shown), "{shown:?}");
        assert!(shown.contains("\tc\nd"), "{shown:?}");
        let piped = render(".s", &input, false, false).unwrap();
        assert!(piped.contains('\u{1b}'), "a pipe gets the text as it is");
        // Inside JSON, C0 controls are escaped by serde and C1 ones by escape_c1
        for indent in [false, true] {
            let json = render(".", &input, indent, true).unwrap();
            assert!(!json.contains('\u{1b}') && !c1(&json), "{json:?}");
            assert!(json.contains("\\u009b2J"), "{json}");
        }
    }

    #[test]
    fn results_are_valid_json() {
        assert_eq!(jq("nan", json!(null)).unwrap(), "null\n");
        // As jq prints it
        assert_eq!(jq("infinite", json!(null)).unwrap(), "1.7976931348623157e+308\n");
        assert!(jq("\"a\" | tobytes", json!(null)).is_err());
        assert!(jq("{(1): 2}", json!(null)).is_err());
    }

    #[test]
    fn a_bad_expression_is_a_validation_error_before_anything_runs() {
        for bad in [".data[", "map(", ".a | foo", "\"unterminated", "if . then 1"] {
            let err = set(bad).unwrap_err();
            assert!(matches!(err, KobanaError::Validation(_)), "{bad}: {err}");
        }
        assert!(check(".data[] | .id").is_ok());
    }

    #[test]
    fn the_environment_and_files_are_out_of_reach() {
        for expr in ["env", "$ENV", "$ENV.HOME", "env.HOME", "input", "inputs", r#"import "x" as x; ."#, r#"include "x"; ."#] {
            let err = check(expr).unwrap_err();
            assert!(matches!(err, KobanaError::Validation(_)), "{expr}: {err}");
        }
        // The note about the environment only where it applies
        assert!(check("env").unwrap_err().to_string().contains("$ENV"));
        assert!(!check("tostream").unwrap_err().to_string().contains("$ENV"));
    }

    #[test]
    fn fromjson_cannot_be_made_to_overflow_the_stack() {
        let hostile = "[".repeat(100_000);
        let input = json!({"s": hostile});
        assert!(jq(".s | fromjson", input.clone()).is_err());
        assert_eq!(jq(".s | tonumber? // \"x\"", input).unwrap(), "x\n");
        assert_eq!(jq(r#""[1,{\"a\":2}]" | fromjson"#, json!(null)).unwrap(), "[1,{\"a\":2}]\n");
        assert_eq!(jq(r#""12" | tonumber"#, json!(null)).unwrap(), "12\n");
    }

    #[test]
    fn runtime_errors_and_halt() {
        let err = jq(".a[]", json!({"a": 1})).unwrap_err();
        assert!(matches!(err, KobanaError::Validation(ref m) if m.contains("--jq")), "{err}");
        assert_eq!(jq("1, halt, 2", json!(null)).unwrap(), "1\n");
        assert!(jq("\"x\" | halt_error(1)", json!(null)).is_err());
        // A long value in the message is cut short
        let page = json!({"data": vec!["x".repeat(50); 50]});
        let err = jq(".data | keys | .[0] | .[0]", page).unwrap_err().to_string();
        assert!(err.chars().count() < 450, "{err}");
    }

    #[test]
    fn the_child_renders_like_the_parent() {
        let input = serde_json::to_vec(&json!({"data": [{"id": 7}]})).unwrap();
        assert_eq!(child_render(".data[].id", &input, false, false).unwrap(), "7\n");
        assert!(child_render(".data[", &input, false, false).is_err());
        assert!(child_render(".", b"not json", false, false).is_err());
    }
}
