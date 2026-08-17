mod catalog;
mod help;

pub use catalog::*;
pub use help::{apply_translations, print_help};

use std::env;
use std::process::Command;
use std::sync::OnceLock;

static LANG: OnceLock<Lang> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Hu,
    De,
    Es,
    It,
    Zh,
    Ru,
    Uk,
}

impl Lang {
    fn parse(raw: &str) -> Option<Self> {
        let normalized = raw.trim().to_lowercase().replace('-', "_");
        let primary = normalized
            .split(['_', '.', '@'])
            .next()
            .unwrap_or("")
            .trim();

        match primary {
            "en" | "eng" | "english" => Some(Self::En),
            "hu" | "hun" | "hungarian" | "magyar" => Some(Self::Hu),
            "de" | "deu" | "ger" | "german" | "deutsch" => Some(Self::De),
            "es" | "spa" | "spanish" | "español" | "espanol" => Some(Self::Es),
            "it" | "ita" | "italian" | "italiano" => Some(Self::It),
            "zh" | "chi" | "zho" | "cn" | "chinese" => Some(Self::Zh),
            "ru" | "rus" | "russian" => Some(Self::Ru),
            "uk" | "ukr" | "ukrainian" | "ua" => Some(Self::Uk),
            _ => None,
        }
    }
}

/// Initialize UI language from `--lang` / `D_LANG` / system locale. Default: English.
pub fn init_from_args<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut forced: Option<Lang> = None;
    let args: Vec<String> = args.into_iter().map(|s| s.as_ref().to_string()).collect();
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        if let Some(value) = arg.strip_prefix("--lang=") {
            forced = Lang::parse(value);
            break;
        }
        if arg == "--lang" || arg == "-L" {
            if let Some(value) = args.get(i + 1) {
                forced = Lang::parse(value);
            }
            break;
        }
        i += 1;
    }

    let lang = forced
        .or_else(|| env::var("D_LANG").ok().and_then(|v| Lang::parse(&v)))
        .unwrap_or_else(detect_system_lang);

    let _ = LANG.set(lang);
}

pub fn lang() -> Lang {
    *LANG.get().unwrap_or(&Lang::En)
}

pub fn tr(catalog: &Catalog) -> &'static str {
    catalog.get(lang())
}

pub fn trf(catalog: &Catalog, pairs: &[(&str, &str)]) -> String {
    let mut text = tr(catalog).to_string();
    for (key, value) in pairs {
        text = text.replace(&format!("{{{key}}}"), value);
    }
    text
}

fn detect_system_lang() -> Lang {
    for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        let Ok(value) = env::var(var) else {
            continue;
        };
        if !is_meaningful_locale(&value) {
            continue;
        }
        if let Some(lang) = Lang::parse(&value) {
            return lang;
        }
    }

    if let Some(lang) = detect_macos_locale() {
        return lang;
    }

    Lang::En
}

fn is_meaningful_locale(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed != "C" && trimmed != "POSIX"
}

fn detect_macos_locale() -> Option<Lang> {
    let output = Command::new("defaults")
        .args(["read", "-g", "AppleLocale"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8_lossy(&output.stdout);
    Lang::parse(value.trim())
}

/// Whether an answer means yes in the active language (plus universal y/yes).
pub fn is_yes(answer: &str) -> bool {
    let a = answer.trim().to_lowercase();
    matches!(
        a.as_str(),
        "y" | "yes"
            | "true"
            | "1"
            | "i"
            | "igen"
            | "ja"
            | "sí"
            | "si"
            | "sì"
            | "oui"
            | "да"
            | "так"
            | "是"
            | "好"
            | "要"
    )
}

/// Whether an answer means no in the active language (plus universal n/no).
pub fn is_no(answer: &str) -> bool {
    let a = answer.trim().to_lowercase();
    matches!(
        a.as_str(),
        "n" | "no"
            | "false"
            | "0"
            | "nem"
            | "nein"
            | "nope"
            | "non"
            | "нет"
            | "ні"
            | "否"
            | "不"
            | "不要"
    )
}

pub fn is_public(answer: &str) -> bool {
    let a = answer.trim().to_lowercase();
    matches!(
        a.as_str(),
        "public"
            | "pub"
            | "nyilvános"
            | "nyilvanos"
            | "öffentlich"
            | "offentlich"
            | "público"
            | "publico"
            | "pubblico"
            | "公开"
            | "公開"
            | "публичный"
            | "публічний"
    )
}

pub fn is_private(answer: &str) -> bool {
    let a = answer.trim().to_lowercase();
    matches!(
        a.as_str(),
        "private"
            | "priv"
            | "privát"
            | "privat"
            | "privado"
            | "privato"
            | "私有"
            | "приватный"
            | "приватний"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_parse_accepts_aliases_and_locales() {
        assert_eq!(Lang::parse("en"), Some(Lang::En));
        assert_eq!(Lang::parse("EN"), Some(Lang::En));
        assert_eq!(Lang::parse("en_US.UTF-8"), Some(Lang::En));
        assert_eq!(Lang::parse("english"), Some(Lang::En));
        assert_eq!(Lang::parse("hu_HU"), Some(Lang::Hu));
        assert_eq!(Lang::parse("magyar"), Some(Lang::Hu));
        assert_eq!(Lang::parse("de-DE"), Some(Lang::De));
        assert_eq!(Lang::parse("español"), Some(Lang::Es));
        assert_eq!(Lang::parse("it_IT.UTF-8"), Some(Lang::It));
        assert_eq!(Lang::parse("zh_CN"), Some(Lang::Zh));
        assert_eq!(Lang::parse("cn"), Some(Lang::Zh));
        assert_eq!(Lang::parse("ru_RU"), Some(Lang::Ru));
        assert_eq!(Lang::parse("uk_UA"), Some(Lang::Uk));
        assert_eq!(Lang::parse("ua"), Some(Lang::Uk));
        assert_eq!(Lang::parse("pt"), None);
        assert_eq!(Lang::parse(""), None);
        assert_eq!(Lang::parse("   "), None);
    }

    #[test]
    fn is_meaningful_locale_rejects_c_and_posix() {
        assert!(is_meaningful_locale("en_US.UTF-8"));
        assert!(is_meaningful_locale("hu"));
        assert!(!is_meaningful_locale(""));
        assert!(!is_meaningful_locale("   "));
        assert!(!is_meaningful_locale("C"));
        assert!(!is_meaningful_locale("POSIX"));
    }

    #[test]
    fn is_yes_and_is_no_recognize_common_answers() {
        assert!(is_yes("y"));
        assert!(is_yes(" YES "));
        assert!(is_yes("igen"));
        assert!(is_yes("是"));
        assert!(is_yes("да"));
        assert!(!is_yes("maybe"));

        assert!(is_no("n"));
        assert!(is_no("nem"));
        assert!(is_no("nein"));
        assert!(is_no("否"));
        assert!(!is_no("maybe"));
    }

    #[test]
    fn is_public_and_is_private_recognize_localized_labels() {
        assert!(is_public("public"));
        assert!(is_public("nyilvános"));
        assert!(is_public("公开"));
        assert!(!is_public("private"));

        assert!(is_private("private"));
        assert!(is_private("privát"));
        assert!(is_private("私有"));
        assert!(!is_private("public"));
    }

    #[test]
    fn trf_replaces_placeholders() {
        assert_eq!(trf(&ERR_EXIT_CODE, &[("code", "42")]), "exit code: 42");
        assert_eq!(
            trf(
                &ERR_START_PROGRAM,
                &[("program", "git"), ("error", "not found")]
            ),
            "failed to start git: not found"
        );
        assert_eq!(
            trf(&ERR_EXIT_CODE, &[("missing", "x")]),
            "exit code: {code}"
        );
    }

    #[test]
    fn git_update_hungarian_matches_english_meaning() {
        assert_eq!(
            CMD_GIT_UPDATE.get(Lang::Hu),
            "Az aktuális branch legfrissebb változásainak letöltése"
        );
    }
}
