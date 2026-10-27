//! The language of everything the CLI says to a person: help text, prompts,
//! progress lines and error messages. Machine-facing output (JSON keys, exit
//! codes, command and flag names) never changes with it.
//!
//! The language comes from `KOBANA_LANG`, then the usual locale variables
//! (`LC_ALL`, `LC_MESSAGES`, `LANG`), then the operating system. Portuguese and
//! Spanish locales get their language; everything else gets English.
//!
//! Strings are written in place, all three side by side:
//!
//! ```ignore
//! eprintln!("{}", tr!(pt: "Pronto", en: "Done", es: "Listo"));
//! let msg = trf!(pt: "Página {page}", en: "Page {page}", es: "Página {page}");
//! ```

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Pt,
    En,
    Es,
}

impl Lang {
    /// The language a locale name points to (`pt_BR.UTF-8`, `es-AR`, `en`),
    /// or `None` when it says nothing about the person (`C`, `POSIX`, empty).
    pub fn from_locale(locale: &str) -> Option<Lang> {
        let locale = locale.trim();
        if locale.is_empty() || locale == "C" || locale.starts_with("C.") || locale == "POSIX" {
            return None;
        }
        let lower = locale.to_ascii_lowercase();
        Some(if lower.starts_with("pt") {
            Lang::Pt
        } else if lower.starts_with("es") {
            Lang::Es
        } else {
            Lang::En
        })
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::Pt => "pt",
            Lang::En => "en",
            Lang::Es => "es",
        }
    }
}

static LANG: OnceLock<Lang> = OnceLock::new();

/// The language for this run. Detected once, on first use.
pub fn lang() -> Lang {
    *LANG.get_or_init(detect)
}

/// Fix the language for this run (e.g. from `--lang`). Only the first call —
/// or the first [`lang()`] — counts, so call it before anything is printed.
pub fn set_lang(lang: Lang) {
    let _ = LANG.set(lang);
}

/// Same precedence as gettext: the first variable that is set decides. A bare
/// `C`/`POSIX` locale says nothing about the person, so it falls through to
/// the operating system's language.
pub fn detect() -> Lang {
    detect_from(|var| std::env::var(var).ok(), sys_locale::get_locale)
}

fn detect_from(
    env: impl Fn(&str) -> Option<String>,
    system: impl FnOnce() -> Option<String>,
) -> Lang {
    for var in ["KOBANA_LANG", "LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Some(value) = env(var).filter(|v| !v.is_empty()) {
            if let Some(lang) = Lang::from_locale(&value) {
                return lang;
            }
            break;
        }
    }
    system().and_then(|l| Lang::from_locale(&l)).unwrap_or(Lang::En)
}

/// One of three texts — Portuguese, English, Spanish — in the current language.
pub fn pick<T: Copy>(texts: [T; 3]) -> T {
    match lang() {
        Lang::Pt => texts[0],
        Lang::En => texts[1],
        Lang::Es => texts[2],
    }
}

/// A `&'static str` in the current language.
#[macro_export]
macro_rules! tr {
    (pt: $pt:literal, en: $en:literal, es: $es:literal $(,)?) => {
        match $crate::i18n::lang() {
            $crate::i18n::Lang::Pt => $pt,
            $crate::i18n::Lang::En => $en,
            $crate::i18n::Lang::Es => $es,
        }
    };
}

/// A `String` in the current language, formatted like `format!`.
#[macro_export]
macro_rules! trf {
    (pt: $pt:literal, en: $en:literal, es: $es:literal $(, $arg:expr)* $(,)?) => {
        match $crate::i18n::lang() {
            $crate::i18n::Lang::Pt => format!($pt $(, $arg)*),
            $crate::i18n::Lang::En => format!($en $(, $arg)*),
            $crate::i18n::Lang::Es => format!($es $(, $arg)*),
        }
    };
}

/// Translations of an OpenAPI spec's human text: each Portuguese `summary`
/// or `description` mapped to its translation. Built by
/// `scripts/build-spec-translations.rb` from the product's localized specs.
pub type SpecTranslations = HashMap<String, String>;

/// Replace every `summary` and `description` string in a spec with its
/// translation. Text without one stays in Portuguese.
pub fn translate_spec(spec: &mut serde_json::Value, translations: &SpecTranslations) {
    match spec {
        serde_json::Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                match value {
                    serde_json::Value::String(text) if key == "summary" || key == "description" => {
                        if let Some(translated) = translations.get(text.as_str()) {
                            *text = translated.clone();
                        }
                    }
                    other => translate_spec(other, translations),
                }
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                translate_spec(item, translations);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env<'a>(vars: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| vars.iter().find(|(k, _)| *k == name).map(|(_, v)| v.to_string())
    }

    #[test]
    fn reads_the_language_from_locale_names() {
        assert_eq!(Lang::from_locale("pt_BR.UTF-8"), Some(Lang::Pt));
        assert_eq!(Lang::from_locale("pt-PT"), Some(Lang::Pt));
        assert_eq!(Lang::from_locale("es_AR.UTF-8"), Some(Lang::Es));
        assert_eq!(Lang::from_locale("en_US"), Some(Lang::En));
        assert_eq!(Lang::from_locale("de_DE.UTF-8"), Some(Lang::En));
        assert_eq!(Lang::from_locale("C"), None);
        assert_eq!(Lang::from_locale("C.UTF-8"), None);
        assert_eq!(Lang::from_locale("POSIX"), None);
        assert_eq!(Lang::from_locale(""), None);
    }

    #[test]
    fn kobana_lang_wins_over_the_locale() {
        let vars = [("KOBANA_LANG", "es"), ("LANG", "pt_BR.UTF-8")];
        assert_eq!(detect_from(env(&vars), || None), Lang::Es);
    }

    #[test]
    fn the_first_locale_variable_set_decides() {
        let vars = [("LC_ALL", "en_US.UTF-8"), ("LANG", "pt_BR.UTF-8")];
        assert_eq!(detect_from(env(&vars), || None), Lang::En);
        let vars = [("LANG", "pt_BR.UTF-8")];
        assert_eq!(detect_from(env(&vars), || Some("es-ES".into())), Lang::Pt);
    }

    #[test]
    fn a_c_locale_falls_back_to_the_system_language() {
        let vars = [("LANG", "C")];
        assert_eq!(detect_from(env(&vars), || Some("pt-BR".into())), Lang::Pt);
        assert_eq!(detect_from(env(&[]), || Some("es-MX".into())), Lang::Es);
        assert_eq!(detect_from(env(&[]), || None), Lang::En);
    }

    #[test]
    fn translates_summaries_and_descriptions_only() {
        let mut spec = serde_json::json!({
            "paths": {"/v1/x": {"get": {
                "summary": "Listar",
                "description": "Lista tudo",
                "operationId": "Listar",
                "parameters": [{"name": "description", "description": "Listar"}]
            }}},
            "components": {"schemas": {"X": {"properties": {
                "description": {"type": "string", "description": "Lista tudo"}
            }}}}
        });
        let translations: SpecTranslations = [("Listar", "List"), ("Lista tudo", "Lists everything")]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        translate_spec(&mut spec, &translations);

        let op = &spec["paths"]["/v1/x"]["get"];
        assert_eq!(op["summary"], "List");
        assert_eq!(op["description"], "Lists everything");
        assert_eq!(op["operationId"], "Listar");
        assert_eq!(op["parameters"][0]["name"], "description");
        assert_eq!(op["parameters"][0]["description"], "List");
        let prop = &spec["components"]["schemas"]["X"]["properties"]["description"];
        assert_eq!(prop["description"], "Lists everything");
    }
}
