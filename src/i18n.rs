use std::collections::HashMap;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

/// Supported languages for statutory business rates calculations and UI rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    En,
    Cy,
    Sco,
    Gd,
}

impl Language {
    /// Parses language code from string ("en", "cy", "sco", "gd", "welsh", "scots", "gaelic")
    pub fn from_code(s: &str) -> Self {
        let trimmed = s.trim().to_lowercase();
        if trimmed.starts_with("cy") || trimmed == "welsh" || trimmed == "cymraeg" {
            Language::Cy
        } else if trimmed.starts_with("sco") || trimmed == "scots" {
            Language::Sco
        } else if trimmed.starts_with("gd")
            || trimmed == "gaelic"
            || trimmed == "gla"
            || trimmed == "gàidhlig"
            || trimmed == "gaidhlig"
            || trimmed == "scottish gaelic"
        {
            Language::Gd
        } else {
            Language::En
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Cy => "cy",
            Language::Sco => "sco",
            Language::Gd => "gd",
        }
    }
}

pub static EN_JSON: &str = include_str!("../locales/en.json");
pub static CY_JSON: &str = include_str!("../locales/cy.json");
pub static SCO_JSON: &str = include_str!("../locales/sco.json");
pub static GD_JSON: &str = include_str!("../locales/gd.json");

static EN_BUNDLE: OnceLock<LocaleBundle> = OnceLock::new();
static CY_BUNDLE: OnceLock<LocaleBundle> = OnceLock::new();
static SCO_BUNDLE: OnceLock<LocaleBundle> = OnceLock::new();
static GD_BUNDLE: OnceLock<LocaleBundle> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
pub struct LocaleBundle {
    pub narrative: NarrativeLocale,
    #[serde(default)]
    pub relief_names: HashMap<String, String>,
    #[serde(default)]
    pub errors: ErrorLocale,
    #[serde(default)]
    pub reference_codes: HashMap<String, HashMap<String, ReferenceCodeLocale>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ErrorLocale {
    pub invalid_occupation_period: String,
    #[serde(default)]
    pub overlapping_occupation_periods: String,
    #[serde(default)]
    pub rateable_value_missing_for_period: String,
    #[serde(default)]
    pub missing_multiplier_for_period: String,
    pub payload_deserialization: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReferenceCodeLocale {
    pub label: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NarrativeLocale {
    pub header_title: String,
    pub divider: String,
    pub frameworks: FrameworksLocale,
    pub framework_intro: String,
    pub liability_period: String,
    pub financial_year_header: String,
    pub occupancy_status: OccupancyStatusLocale,
    pub lines: NarrativeLinesLocale,
    pub holiday_let: HolidayLetLocale,
    pub total_net_payable: String,
    #[serde(default)]
    pub disqualifications: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrameworksLocale {
    pub england: String,
    pub wales: String,
    pub scotland: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OccupancyStatusLocale {
    pub occupied: String,
    pub unoccupied: String,
    pub part_occupied: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NarrativeLinesLocale {
    pub occupancy_status: String,
    pub rateable_value: String,
    pub multiplier: String,
    pub segment_duration: String,
    pub gross_liability: String,
    pub transitional_adjustment: String,
    #[serde(default)]
    pub transitional_relief_label: Option<String>,
    #[serde(default)]
    pub transitional_premium_label: Option<String>,
    pub applied_reliefs_none: String,
    pub applied_relief_item: String,
    pub audit_header: String,
    pub audit_item: String,
    pub statutory_penalty_item: String,
    pub net_segment_payable: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HolidayLetLocale {
    pub header: String,
    pub disqualified_actual: String,
    pub disqualified_status: String,
    pub qualified_actual: String,
    pub qualified_status: String,
    pub default_actual: String,
    pub default_status: String,
}

/// Returns the static LocaleBundle reference for the requested language
pub fn get_locale(lang: Language) -> &'static LocaleBundle {
    match lang {
        Language::En => EN_BUNDLE.get_or_init(|| serde_json::from_str(EN_JSON).expect("Failed to parse en.json")),
        Language::Cy => CY_BUNDLE.get_or_init(|| serde_json::from_str(CY_JSON).expect("Failed to parse cy.json")),
        Language::Sco => SCO_BUNDLE.get_or_init(|| serde_json::from_str(SCO_JSON).expect("Failed to parse sco.json")),
        Language::Gd => GD_BUNDLE.get_or_init(|| serde_json::from_str(GD_JSON).expect("Failed to parse gd.json")),
    }
}

/// Formats a template string by replacing `{key}` with `val` for each pair in `vars`
pub fn format_template(template: &str, vars: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (k, v) in vars {
        let pattern = format!("{{{}}}", k);
        result = result.replace(&pattern, v);
    }
    result
}

/// Translates statutory reference codes, categories, and descriptions to Welsh, Scots, Scottish Gaelic, or English
pub fn localize_reference_code(
    category: &str,
    code: &str,
    en_label: &str,
    en_desc: &str,
    lang: Language,
) -> (String, String) {
    let bundle = get_locale(lang);
    if let Some(cat_map) = bundle.reference_codes.get(category) {
        if let Some(entry) = cat_map.get(code) {
            return (entry.label.clone(), entry.description.clone());
        }
    }
    (en_label.to_string(), en_desc.to_string())
}

/// Translates itemized statutory relief names to Welsh, Scots, Scottish Gaelic, or English
pub fn localize_relief_name(name: &str, lang: Language) -> String {
    let bundle = get_locale(lang);
    if let Some(trans) = bundle.relief_names.get(name) {
        return trans.clone();
    }
    name.to_string()
}
