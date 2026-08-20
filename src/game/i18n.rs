use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HeaderStrings {
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StatsStrings {
    pub turn: String,
    pub walls: String,
    pub biomass: String,
    pub max: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ModalStrings {
    pub containment_complete: String,
    pub congratulations: String,
    pub all_sectors_contained: String,
    pub containment_breached: String,
    pub sector_cleared_template: String,
    pub facility_secured_template: String,
    pub all_threats_neutralized: String,
    pub defeat_reason: String,
    pub retry_level: String,
    pub replay_level: String,
    pub next_level: String,
    pub skip_level: String,
    pub start_from_first: String,
}

impl ModalStrings {
    pub fn format_sector_cleared(&self, turns: usize) -> String {
        self.sector_cleared_template
            .replace("{turns}", &turns.to_string())
    }

    pub fn format_facility_secured(&self, turns: usize) -> String {
        self.facility_secured_template
            .replace("{turns}", &turns.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LocaleStrings {
    pub locale: String,
    pub language_name: String,
    pub header: HeaderStrings,
    pub stats: StatsStrings,
    pub levels: Vec<String>,
    pub modal: ModalStrings,
}

static LOCALES: OnceLock<Vec<LocaleStrings>> = OnceLock::new();

fn load_all_locales() -> Vec<LocaleStrings> {
    const LOCALES_JSON: &[&str] = &[
        include_str!("../../assets/locales/en-US.json"),
        include_str!("../../assets/locales/ru-RU.json"),
        include_str!("../../assets/locales/es-ES.json"),
        include_str!("../../assets/locales/de-DE.json"),
        include_str!("../../assets/locales/fr-FR.json"),
        include_str!("../../assets/locales/ja-JP.json"),
        include_str!("../../assets/locales/zh-CN.json"),
        include_str!("../../assets/locales/ko-KR.json"),
    ];

    LOCALES_JSON
        .iter()
        .map(|raw| serde_json::from_str(raw).expect("Failed to deserialize embedded locale JSON"))
        .collect()
}

fn get_locales_list() -> &'static [LocaleStrings] {
    LOCALES.get_or_init(load_all_locales).as_slice()
}

/// Normalizes locale tags with various separators ('-', '_', '+') to lowercased hyphenated format.
/// e.g. "pt_BR", "pt+BR", "PT-BR" -> "pt-br"
pub fn normalize_locale_tag(tag: &str) -> String {
    tag.trim().replace(['_', '+'], "-").to_ascii_lowercase()
}

/// Resolves a requested locale tag against available translations.
/// 1. Exact normalized match (e.g. "ru-ru" matches "ru-RU", "ja-jp" matches "ja-JP")
/// 2. Shorthand alias match (e.g. "jp" -> "ja", "cn" -> "zh", "kr" -> "ko")
/// 3. Language prefix match (e.g. "ru" or "ru-kz" matches "ru-RU", "ja" matches "ja-JP", "zh-hans" matches "zh-CN")
/// 4. Fallback to default English ("en-US")
pub fn resolve_locale(tag: &str) -> &'static LocaleStrings {
    let locales = get_locales_list();
    let norm = normalize_locale_tag(tag);
    let raw_prefix = norm.split('-').next().unwrap_or("");
    let base_lang = match raw_prefix {
        "jp" => "ja",
        "cn" => "zh",
        "kr" => "ko",
        other => other,
    };

    locales
        .iter()
        .find(|l| normalize_locale_tag(&l.locale) == norm)
        .or_else(|| {
            (!base_lang.is_empty()).then(|| {
                locales.iter().find(|l| {
                    normalize_locale_tag(&l.locale)
                        .split('-')
                        .next()
                        .is_some_and(|prefix| prefix == base_lang)
                })
            })?
        })
        .unwrap_or(&locales[0])
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
extern "C" {
    fn biomass_get_system_language() -> i32;
}

#[cfg(target_arch = "wasm32")]
pub fn detect_locale_tag() -> String {
    match unsafe { biomass_get_system_language() } {
        1 => "ru-RU".to_string(),
        2 => "es-ES".to_string(),
        3 => "de-DE".to_string(),
        4 => "fr-FR".to_string(),
        5 => "ja-JP".to_string(),
        6 => "zh-CN".to_string(),
        7 => "ko-KR".to_string(),
        _ => "en-US".to_string(),
    }
}

#[cfg(target_os = "android")]
fn query_android_jni_locale() -> Option<String> {
    unsafe {
        let env = macroquad::miniquad::native::android::attach_jni_env();
        if env.is_null() {
            return None;
        }

        let find_class = (**env).FindClass?;
        let get_static_method_id = (**env).GetStaticMethodID?;
        let call_static_object_method = (**env).CallStaticObjectMethod?;
        let get_method_id = (**env).GetMethodID?;
        let call_object_method = (**env).CallObjectMethod?;
        let get_string_utf_chars = (**env).GetStringUTFChars?;
        let release_string_utf_chars = (**env).ReleaseStringUTFChars?;

        let locale_class_name = std::ffi::CString::new("java/util/Locale").ok()?;
        let locale_class = find_class(env, locale_class_name.as_ptr());
        if locale_class.is_null() {
            return None;
        }

        let get_default_sig = std::ffi::CString::new("()Ljava/util/Locale;").ok()?;
        let get_default_name = std::ffi::CString::new("getDefault").ok()?;
        let get_default_mid = get_static_method_id(
            env,
            locale_class,
            get_default_name.as_ptr(),
            get_default_sig.as_ptr(),
        );
        if get_default_mid.is_null() {
            return None;
        }

        let default_locale = call_static_object_method(env, locale_class, get_default_mid);
        if default_locale.is_null() {
            return None;
        }

        let to_lang_tag_sig = std::ffi::CString::new("()Ljava/lang/String;").ok()?;
        let to_lang_tag_name = std::ffi::CString::new("toLanguageTag").ok()?;
        let to_lang_tag_mid = get_method_id(
            env,
            locale_class,
            to_lang_tag_name.as_ptr(),
            to_lang_tag_sig.as_ptr(),
        );
        if to_lang_tag_mid.is_null() {
            return None;
        }

        let jstr = call_object_method(env, default_locale, to_lang_tag_mid);
        if jstr.is_null() {
            return None;
        }

        let cstr_ptr = get_string_utf_chars(env, jstr as _, std::ptr::null_mut());
        if cstr_ptr.is_null() {
            return None;
        }

        let result = std::ffi::CStr::from_ptr(cstr_ptr)
            .to_str()
            .ok()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);

        release_string_utf_chars(env, jstr as _, cstr_ptr);

        result
    }
}

#[cfg(target_os = "android")]
fn query_android_sys_prop_locale() -> Option<String> {
    extern "C" {
        fn __system_property_get(
            name: *const std::os::raw::c_char,
            value: *mut std::os::raw::c_char,
        ) -> std::os::raw::c_int;
    }

    const PROPS: &[&[u8]] = &[
        b"persist.sys.locale\0",
        b"ro.product.locale\0",
        b"persist.sys.language\0",
    ];

    PROPS.iter().find_map(|&prop| {
        let mut buf = [0u8; 128];
        let len =
            unsafe { __system_property_get(prop.as_ptr() as *const _, buf.as_mut_ptr() as *mut _) };
        (len > 0).then(|| {
            std::str::from_utf8(&buf[..len as usize])
                .ok()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })?
    })
}

#[cfg(target_os = "android")]
pub fn detect_locale_tag() -> String {
    query_android_jni_locale()
        .or_else(query_android_sys_prop_locale)
        .unwrap_or_else(|| "en-US".to_string())
}

/// Extracts locale tag from CLI arguments (--lang <tag>, --lang=<tag>, -l <tag>).
#[allow(dead_code)]
pub fn parse_cli_locale(args: impl IntoIterator<Item = impl AsRef<str>>) -> Option<String> {
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        let s = arg.as_ref();
        if s == "--lang" || s == "-l" {
            if let Some(val) = iter.next() {
                let v = val.as_ref().trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        } else if let Some(val) = s
            .strip_prefix("--lang=")
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            return Some(val.to_string());
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn detect_macos_locale() -> Option<String> {
    extern "C" {
        fn CFLocaleCopyCurrent() -> *const std::ffi::c_void;
        fn CFLocaleGetIdentifier(loc: *const std::ffi::c_void) -> *const std::ffi::c_void;
        fn CFStringGetCString(
            str_ref: *const std::ffi::c_void,
            buf: *mut std::os::raw::c_char,
            size: isize,
            enc: u32,
        ) -> bool;
        fn CFRelease(cf: *const std::ffi::c_void);
    }
    unsafe {
        let loc = CFLocaleCopyCurrent();
        if loc.is_null() {
            return None;
        }
        let ident = CFLocaleGetIdentifier(loc);
        let mut buf = [0u8; 64];
        let ok = !ident.is_null()
            && CFStringGetCString(ident, buf.as_mut_ptr() as _, buf.len() as isize, 0x08000100);
        CFRelease(loc);
        ok.then(|| {
            std::ffi::CStr::from_bytes_until_nul(&buf)
                .ok()?
                .to_str()
                .ok()
                .map(str::to_string)
        })?
    }
}

#[cfg(target_os = "windows")]
fn detect_windows_locale() -> Option<String> {
    extern "system" {
        fn GetUserDefaultLocaleName(buf: *mut u16, len: i32) -> i32;
    }
    let mut buf = [0u16; 85];
    let len = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
    (len > 1).then(|| String::from_utf16(&buf[..(len as usize - 1)]).ok())?
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
fn detect_env_locale() -> Option<String> {
    ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"]
        .into_iter()
        .find_map(|var| {
            let val = std::env::var(var).ok()?;
            let clean = val.trim().split(['.', ':']).next().unwrap_or("");
            (!clean.is_empty() && clean != "C" && clean != "POSIX").then(|| clean.to_string())
        })
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
fn detect_os_locale() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        detect_macos_locale()
    }
    #[cfg(target_os = "windows")]
    {
        detect_windows_locale()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
pub fn detect_locale_tag() -> String {
    parse_cli_locale(std::env::args().skip(1))
        .or_else(detect_os_locale)
        .or_else(detect_env_locale)
        .unwrap_or_else(|| "en-US".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_locales_load_and_have_10_levels() {
        let locales = get_locales_list();
        assert_eq!(locales.len(), 8);

        for loc in locales {
            assert!(!loc.locale.is_empty());
            assert!(!loc.language_name.is_empty());
            assert!(!loc.header.title.is_empty());
            assert!(!loc.header.subtitle.is_empty());
            assert!(!loc.stats.turn.is_empty());
            assert!(!loc.stats.walls.is_empty());
            assert!(!loc.stats.biomass.is_empty());
            assert!(!loc.stats.max.is_empty());
            assert_eq!(
                loc.levels.len(),
                10,
                "Locale {} must have 10 levels",
                loc.locale
            );
            for (idx, lvl) in loc.levels.iter().enumerate() {
                assert!(
                    !lvl.is_empty(),
                    "Level {} in {} is empty",
                    idx + 1,
                    loc.locale
                );
            }
            assert!(!loc.modal.containment_complete.is_empty());
            assert!(!loc.modal.congratulations.is_empty());
            assert!(!loc.modal.all_sectors_contained.is_empty());
            assert!(!loc.modal.containment_breached.is_empty());
            assert!(!loc.modal.defeat_reason.is_empty());
            assert!(!loc.modal.retry_level.is_empty());
            assert!(!loc.modal.replay_level.is_empty());
            assert!(!loc.modal.next_level.is_empty());
            assert!(!loc.modal.skip_level.is_empty());
            assert!(!loc.modal.start_from_first.is_empty());

            // Test template placeholders
            let cleared = loc.modal.format_sector_cleared(4);
            assert!(
                cleared.contains('4'),
                "Cleared message in {} missing turn: {}",
                loc.locale,
                cleared
            );

            let secured = loc.modal.format_facility_secured(8);
            assert!(
                secured.contains('8'),
                "Secured message in {} missing turn: {}",
                loc.locale,
                secured
            );
        }
    }

    #[test]
    fn test_locale_normalization_and_resolution() {
        assert_eq!(resolve_locale("ru-RU").locale, "ru-RU");
        assert_eq!(resolve_locale("ru_RU").locale, "ru-RU");
        assert_eq!(resolve_locale("ru+RU").locale, "ru-RU");
        assert_eq!(resolve_locale("RU").locale, "ru-RU");
        assert_eq!(resolve_locale("ru_KZ").locale, "ru-RU");

        assert_eq!(resolve_locale("es-ES").locale, "es-ES");
        assert_eq!(resolve_locale("es_MX").locale, "es-ES");
        assert_eq!(resolve_locale("es").locale, "es-ES");

        assert_eq!(resolve_locale("de-DE").locale, "de-DE");
        assert_eq!(resolve_locale("de_AT").locale, "de-DE");
        assert_eq!(resolve_locale("de").locale, "de-DE");

        assert_eq!(resolve_locale("fr-FR").locale, "fr-FR");
        assert_eq!(resolve_locale("fr_CA").locale, "fr-FR");
        assert_eq!(resolve_locale("fr").locale, "fr-FR");

        assert_eq!(resolve_locale("ja-JP").locale, "ja-JP");
        assert_eq!(resolve_locale("ja_JP").locale, "ja-JP");
        assert_eq!(resolve_locale("ja").locale, "ja-JP");
        assert_eq!(resolve_locale("jp").locale, "ja-JP");
        assert_eq!(resolve_locale("jp-JP").locale, "ja-JP");

        assert_eq!(resolve_locale("zh-CN").locale, "zh-CN");
        assert_eq!(resolve_locale("zh_CN").locale, "zh-CN");
        assert_eq!(resolve_locale("zh").locale, "zh-CN");
        assert_eq!(resolve_locale("cn").locale, "zh-CN");
        assert_eq!(resolve_locale("zh-hans").locale, "zh-CN");

        assert_eq!(resolve_locale("ko-KR").locale, "ko-KR");
        assert_eq!(resolve_locale("ko_KR").locale, "ko-KR");
        assert_eq!(resolve_locale("ko").locale, "ko-KR");
        assert_eq!(resolve_locale("kr").locale, "ko-KR");

        assert_eq!(resolve_locale("en-US").locale, "en-US");
        assert_eq!(resolve_locale("en_GB").locale, "en-US");
        assert_eq!(resolve_locale("en").locale, "en-US");

        // Unknown locale falls back to en-US
        assert_eq!(resolve_locale("it-IT").locale, "en-US");
        assert_eq!(resolve_locale("unknown").locale, "en-US");
    }

    #[test]
    fn test_parse_cli_locale() {
        assert_eq!(
            parse_cli_locale(&["--lang", "ru-RU"]),
            Some("ru-RU".to_string())
        );
        assert_eq!(
            parse_cli_locale(&["-l", "es-ES"]),
            Some("es-ES".to_string())
        );
        assert_eq!(
            parse_cli_locale(&["--lang=de-DE"]),
            Some("de-DE".to_string())
        );
        assert_eq!(parse_cli_locale(&["--lang=fr"]), Some("fr".to_string()));
        assert_eq!(
            parse_cli_locale(&["--other", "val", "-l", "pt+BR"]),
            Some("pt+BR".to_string())
        );
        assert_eq!(parse_cli_locale(&["--other", "val"]), None);
    }

    #[test]
    fn test_detect_locale_tag() {
        let tag = detect_locale_tag();
        assert!(!tag.is_empty(), "Detected locale tag should not be empty");
        let resolved = resolve_locale(&tag);
        assert!(!resolved.locale.is_empty());
    }
}
