use std::sync::Arc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::calculation::{MultiplierInfo, ParameterVersionIds};
use crate::models::enums::Jurisdiction;
use crate::error::AppError;

/// Represents a row in the param_multiplier table (1990–2027)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamMultiplier {
    pub parameter_id: String,
    pub financial_year: String,
    pub jurisdiction: Jurisdiction,
    pub multiplier_type: String,
    pub rv_threshold_min: Decimal,
    pub rv_threshold_max: Option<Decimal>,
    pub multiplier_rate: Decimal,
    pub effective_from: NaiveDate,
    pub effective_to: NaiveDate,
}

/// Represents a row in the param_sbrr_sbbs_threshold table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSbrrSbbsThreshold {
    pub parameter_id: String,
    pub financial_year: String,
    pub jurisdiction: Jurisdiction,
    pub min_rv: Decimal,
    pub max_rv: Decimal,
    pub relief_percentage: Decimal,
    pub is_tapered: bool,
    pub effective_from: NaiveDate,
    pub effective_to: NaiveDate,
}

/// Represents a row in the param_sector_relief table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSectorRelief {
    pub parameter_id: String,
    pub financial_year: String,
    pub jurisdiction: Jurisdiction,
    pub relief_code: String,
    pub discount_percentage: Decimal,
    pub cash_cap_gbp: Option<Decimal>,
    pub effective_from: NaiveDate,
    pub effective_to: NaiveDate,
}

/// Resolution result for Small Business Rate Relief / SBBS
#[derive(Debug, Clone)]
pub struct SbrrResolution {
    pub relief_code: String,
    pub relief_name: String,
    pub statutory_authority: String,
    pub discount_percentage: Decimal,
}

/// Resolution result for sector-specific reliefs (e.g. Retail, Hospitality & Leisure)
#[derive(Debug, Clone)]
pub struct SectorReliefResolution {
    pub relief_code: String,
    pub relief_name: String,
    pub statutory_authority: String,
    pub discount_percentage: Decimal,
    pub cash_cap_gbp: Option<Decimal>,
}

/// Cap mechanism for Scottish Small Business Bonus Scheme Transitional Relief (SBBS TR)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SbbsTrCapType {
    /// Fixed cash increase limit in GBP (e.g. £600 in 2023/24, £1,200 in 2024/25, £1,800 in 2025/26 under SSI 2023/31)
    FixedCash(Decimal),
    /// Percentage of the unmitigated bill increase payable by ratepayer (e.g. 25% in 2026/27 under SSI 2026)
    PercentageOfIncrease(Decimal),
}

/// Resolution result for Scottish SBBS Transitional Relief
#[derive(Debug, Clone)]
pub struct SbbsTrResolution {
    pub relief_code: String,
    pub relief_name: String,
    pub statutory_authority: String,
    pub cap_type: SbbsTrCapType,
}

/// Resolution result for English Supporting Small Business Relief (SSBR)
#[derive(Debug, Clone)]
pub struct SsbrResolution {
    pub relief_code: String,
    pub relief_name: String,
    pub statutory_authority: String,
    pub annual_cap: Decimal,
}

/// Represents a row in the param_transitional_cap table (Suite 7)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamTransitionalCap {
    pub parameter_id: String,
    pub revaluation_list_year: i32,
    pub financial_year: String,
    pub jurisdiction: Jurisdiction,
    pub property_tier: String,
    pub rv_threshold_min: Decimal,
    pub rv_threshold_max: Option<Decimal>,
    pub upward_cap_percentage: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_greater_london: Option<bool>,
}

/// Represents an allowable statutory reference code and its legal description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamReferenceCode {
    pub category: String,
    pub code: String,
    pub label: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    pub display_order: i32,
}

/// Statutory rateable value exemption threshold for unoccupied property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamEmptyPropertyThreshold {
    pub parameter_id: String,
    pub jurisdiction: Jurisdiction,
    pub threshold_rv: Decimal,
    pub statutory_authority: String,
    pub effective_from: NaiveDate,
    pub effective_to: NaiveDate,
}

/// Statutory letting requirements for self-catering holiday accommodation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamLettingTest {
    pub parameter_id: String,
    pub jurisdiction: Jurisdiction,
    pub min_actual_let_days: i64,
    pub min_available_days: i64,
    pub statutory_authority: String,
    pub effective_from: NaiveDate,
    pub effective_to: NaiveDate,
}

/// Statutory scheme thresholds, caps, and rates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSchemeCap {
    pub parameter_id: String,
    pub scheme_code: String,
    pub financial_year: Option<String>,
    pub jurisdiction: Option<Jurisdiction>,
    pub cap_value: Decimal,
    pub secondary_cap_value: Option<Decimal>,
    pub cap_unit: String,
    pub statutory_authority: String,
    pub effective_from: Option<NaiveDate>,
    pub effective_to: Option<NaiveDate>,
}

/// In-memory parameter repository storage
#[derive(Clone, Debug)]
pub struct RepoData {
    multipliers: Vec<ParamMultiplier>,
    sbrr_thresholds: Vec<ParamSbrrSbbsThreshold>,
    sector_reliefs: Vec<ParamSectorRelief>,
    transitional_caps: Vec<ParamTransitionalCap>,
    reference_codes: Vec<ParamReferenceCode>,
    empty_property_thresholds: Vec<ParamEmptyPropertyThreshold>,
    letting_tests: Vec<ParamLettingTest>,
    scheme_caps: Vec<ParamSchemeCap>,
    versions: ParameterVersionIds,
}

/// In-memory parameter repository seeded with GB rating parameters (1990–2027).
/// Cheaply clonable via internal Arc pointer.
#[derive(Clone, Debug)]
pub struct ParameterRepository {
    data: Arc<RepoData>,
}

impl std::ops::Deref for ParameterRepository {
    type Target = RepoData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Default for ParameterRepository {
    fn default() -> Self {
        Self::new_seeded()
    }
}

impl ParameterRepository {
    pub fn new_mock() -> Self {
        Self::new_seeded()
    }

    pub fn new_seeded() -> Self {
        Self {
            data: Arc::new(RepoData::new_seeded()),
        }
    }

    /// Extends the repository with an additional multiplier (useful for future year testing)
    pub fn with_additional_multiplier(self, multiplier: ParamMultiplier) -> Self {
        let mut data = (*self.data).clone();
        data.multipliers.push(multiplier);
        Self {
            data: Arc::new(data),
        }
    }

    pub fn get_versions(&self) -> &ParameterVersionIds {
        &self.versions
    }

    pub fn all_multipliers(&self) -> &[ParamMultiplier] {
        &self.multipliers
    }

    pub fn all_sbrr_thresholds(&self) -> &[ParamSbrrSbbsThreshold] {
        &self.sbrr_thresholds
    }

    pub fn all_sector_reliefs(&self) -> &[ParamSectorRelief] {
        &self.sector_reliefs
    }

    pub fn all_transitional_caps(&self) -> &[ParamTransitionalCap] {
        &self.transitional_caps
    }

    pub fn all_empty_property_thresholds(&self) -> &[ParamEmptyPropertyThreshold] {
        &self.empty_property_thresholds
    }

    pub fn all_letting_tests(&self) -> &[ParamLettingTest] {
        &self.letting_tests
    }

    pub fn all_scheme_caps(&self) -> &[ParamSchemeCap] {
        &self.scheme_caps
    }

    pub fn all_reference_codes(&self) -> &[ParamReferenceCode] {
        &self.reference_codes
    }

    pub fn all_reference_codes_localized(&self, lang: crate::i18n::Language) -> Vec<ParamReferenceCode> {
        self.reference_codes.iter().map(|rc| {
            let (label, desc) = crate::i18n::localize_reference_code(
                &rc.category,
                &rc.code,
                &rc.label,
                &rc.description,
                lang,
            );
            ParamReferenceCode {
                category: rc.category.clone(),
                code: rc.code.clone(),
                label,
                description: desc,
                jurisdiction: rc.jurisdiction.clone(),
                display_order: rc.display_order,
            }
        }).collect()
    }

    pub fn reference_codes_by_category(&self, category: &str) -> Vec<&ParamReferenceCode> {
        self.reference_codes
            .iter()
            .filter(|r| r.category.eq_ignore_ascii_case(category))
            .collect()
    }

    /// Resolves the statutory transitional phasing cap factor under LGFA 1988 s.57A / NDRA 2023
    pub fn resolve_transitional_cap(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> Option<&ParamTransitionalCap> {
        self.resolve_transitional_cap_ex(jurisdiction, financial_year, rv, false)
    }

    /// Resolves the statutory transitional phasing cap factor with optional Greater London threshold qualification (S.I. 2022/1403 reg. 10(5))
    pub fn resolve_transitional_cap_ex(
        &self,
        jurisdiction: Jurisdiction,
        financial_year: &str,
        rv: Decimal,
        is_greater_london: bool,
    ) -> Option<&ParamTransitionalCap> {
        self.transitional_caps.iter()
            .find(|c| {
                c.jurisdiction == jurisdiction 
                    && c.financial_year == financial_year 
                    && (c.is_greater_london == Some(is_greater_london) || c.is_greater_london.is_none())
                    && rv >= c.rv_threshold_min 
                    && c.rv_threshold_max.is_none_or(|max| rv <= max)
            })
    }

    /// Dynamically resolves the statutory multiplier by querying in-memory parameter tables
    pub fn resolve_multiplier(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> Result<MultiplierInfo, AppError> {
        self.resolve_multiplier_ex(jurisdiction, financial_year, rv, false, false, false)
    }

    /// Dynamically resolves the statutory multiplier, accounting for retail and RHL sector qualifications and higher multiplier exceptions
    pub fn resolve_multiplier_ex(
        &self,
        jurisdiction: Jurisdiction,
        financial_year: &str,
        rv: Decimal,
        is_rhl: bool,
        is_retail: bool,
        is_higher_multiplier_excepted: bool,
    ) -> Result<MultiplierInfo, AppError> {
        let matches: Vec<&ParamMultiplier> = self.multipliers.iter()
            .filter(|m| m.jurisdiction == jurisdiction && m.financial_year == financial_year)
            .collect();

        if matches.is_empty() {
            return Err(AppError::MissingMultiplierForPeriod {
                financial_year: financial_year.to_string(),
                jurisdiction,
            });
        }

        // 1. High-Value Multiplier (NDRA 2023 - England 2026/27, RV >= £500,000)
        if let Some(m) = matches.iter().find(|m| m.multiplier_type == "HIGH_VALUE" && rv >= m.rv_threshold_min) {
            return Ok(MultiplierInfo {
                multiplier_type: m.multiplier_type.clone(),
                rate: m.multiplier_rate,
            });
        }

        // 1b. Wales Differential Multipliers (Local Government Finance (Wales) Act 2024 - 2026/27 & 2027/28)
        if jurisdiction == Jurisdiction::WALES && (financial_year == "2026/27" || financial_year == "2027/28") {
            if rv > dec!(100000.00) && !is_higher_multiplier_excepted {
                if let Some(m) = matches.iter().find(|m| m.multiplier_type.starts_with("HIGHER")) {
                    return Ok(MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    });
                }
            } else if is_retail && rv < dec!(51000.00) {
                if let Some(m) = matches.iter().find(|m| m.multiplier_type.starts_with("RETAIL")) {
                    return Ok(MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    });
                }
            } else {
                if let Some(m) = matches.iter().find(|m| m.multiplier_type.starts_with("STANDARD")) {
                    return Ok(MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    });
                }
            }
        }

        // 2. Retail, Hospitality & Leisure (RHL) permanent lower multipliers (England 2026/27)
        if is_rhl {
            if let Some(m) = matches.iter().find(|m| {
                (m.multiplier_type == "SMALL_BUSINESS_RHL" || m.multiplier_type == "STANDARD_RHL")
                    && rv >= m.rv_threshold_min
                    && m.rv_threshold_max.is_none_or(|max| rv <= max)
            }) {
                return Ok(MultiplierInfo {
                    multiplier_type: m.multiplier_type.clone(),
                    rate: m.multiplier_rate,
                });
            }
        }

        // 3. Small Business / Basic rate threshold check
        for m in &matches {
            if m.multiplier_type == "SMALL_BUSINESS" || m.multiplier_type == "BASIC" {
                let min_ok = rv >= m.rv_threshold_min;
                let max_ok = m.rv_threshold_max.is_none_or(|max| rv <= max);
                if min_ok && max_ok {
                    return Ok(MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    });
                }
            }
        }

        // 4. Intermediate / Welsh Differential check
        for m in &matches {
            if m.multiplier_type == "INTERMEDIATE" || m.multiplier_type == "WELSH_DIFFERENTIAL" {
                let min_ok = rv >= m.rv_threshold_min;
                let max_ok = m.rv_threshold_max.is_none_or(|max| rv <= max);
                if min_ok && max_ok {
                    return Ok(MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    });
                }
            }
        }

        // 4b. Higher / High-Value rate threshold check (e.g. Scotland Higher Multiplier for RV > £100,000, Wales HIGHER, England HIGH_VALUE)
        for m in &matches {
            if m.multiplier_type == "HIGHER" || m.multiplier_type == "HIGH_VALUE" {
                let min_ok = rv >= m.rv_threshold_min;
                let max_ok = m.rv_threshold_max.is_none_or(|max| rv <= max);
                if min_ok && max_ok {
                    return Ok(MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    });
                }
            }
        }

        // 5. Fallback to Standard or first available multiplier in matches
        let standard = matches.iter()
            .find(|m| m.multiplier_type == "STANDARD" || m.multiplier_type == "WELSH_DIFFERENTIAL")
            .unwrap_or(&matches[0]);

        Ok(MultiplierInfo {
            multiplier_type: standard.multiplier_type.clone(),
            rate: standard.multiplier_rate,
        })
    }

    /// Resolves SBRR / SBBS discount percentage and statutory authority
    pub fn resolve_sbrr(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> Option<SbrrResolution> {
        if jurisdiction == Jurisdiction::SCOTLAND && financial_year >= "2023/24" {
            return self.resolve_scottish_sbbs(financial_year, rv, false, None);
        }

        let records: Vec<&ParamSbrrSbbsThreshold> = self.sbrr_thresholds.iter()
            .filter(|s| s.jurisdiction == jurisdiction && s.financial_year == financial_year)
            .collect();

        for record in records {
            if rv >= record.min_rv && rv <= record.max_rv {
                let pct = if record.is_tapered {
                    let range = record.max_rv - record.min_rv;
                    if range.is_zero() {
                        record.relief_percentage
                    } else {
                        ((record.max_rv - rv) / range * record.relief_percentage).round_dp(2)
                    }
                } else {
                    record.relief_percentage
                };

                let (code, name, authority) = match jurisdiction {
                    Jurisdiction::ENGLAND => (
                        "SBRR",
                        "Small Business Rate Relief",
                        if financial_year >= "2023/24" {
                            "LGFA 1988 s.43(4B) / NDRA 2023"
                        } else if financial_year >= "2017/18" {
                            "LGFA 1988 s.43(4B) / SI 2017/411"
                        } else if financial_year >= "2010/11" {
                            "LGFA 1988 s.43(4B) / SI 2010/1959"
                        } else {
                            "LGFA 1988 s.43(4B) / SI 2004/3315"
                        },
                    ),
                    Jurisdiction::WALES => (
                        "SBRR_WALES",
                        "Welsh Small Business Rates Relief",
                        if financial_year >= "2018/19" {
                            "LGFA 1988 Sched 4ZA para 4 / W.S.I. 2017/1229"
                        } else if financial_year >= "2010/11" {
                            "WSI 2010/248 (W.34)"
                        } else {
                            "WSI 2008/707 (W.75)"
                        },
                    ),
                    Jurisdiction::SCOTLAND => (
                        "SBBS",
                        "Small Business Bonus Scheme",
                        if financial_year >= "2023/24" {
                            "Council Tax (Dwellings) (Scotland) Regs 2026"
                        } else if financial_year >= "2017/18" {
                            "SSI 2017/36"
                        } else if financial_year >= "2010/11" {
                            "SSI 2010/37"
                        } else {
                            "SSI 2008/106"
                        },
                    ),
                };

                return Some(SbrrResolution {
                    relief_code: code.to_string(),
                    relief_name: name.to_string(),
                    statutory_authority: authority.to_string(),
                    discount_percentage: pct,
                });
            }
        }
        None
    }

    /// Statutory divisor E for the SBRR taper band, where the chargeable amount is (A x M) / E.
    /// - England from 2017/18, RV £12,000.01-£15,000: E = 3000 / (A - 12000) to three decimal places, adding one
    ///   thousandth only where the remainder is more than five ten-thousandths (S.I. 2017/102 art. 3(3), (5);
    ///   S.I. 2023/1250 reg 3(3)-(4) from 2024/25)
    /// - Wales from 2018/19, RV £6,000.01-£12,000 (rateable value conditions): E = 6000 / (RV - 6000) to three decimal
    ///   places, rounding up or down as appropriate (W.S.I. 2017/1229 art. 11(3))
    ///
    /// Returns None outside the taper band, where the relief percentage applies directly.
    pub fn sbrr_taper_divisor(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal, is_childcare: bool, is_post_office: bool) -> Option<Decimal> {
        use rust_decimal::RoundingStrategy;
        if is_childcare || is_post_office {
            return None;
        }

        // Statutory divisor E was introduced in England from 2017/18 (S.I. 2017/102 art. 3)
        // and in Wales from 2018/19 (W.S.I. 2017/1229 art. 11(3))
        let divisor_applies = match jurisdiction {
            Jurisdiction::ENGLAND => financial_year >= "2017/18",
            Jurisdiction::WALES => financial_year >= "2018/19",
            _ => false,
        };
        if !divisor_applies {
            return None;
        }

        let record = self.sbrr_thresholds.iter().find(|s| {
            s.jurisdiction == jurisdiction
                && s.financial_year == financial_year
                && s.is_tapered
                && rv >= s.min_rv
                && rv <= s.max_rv
        })?;

        let base = record.min_rv.floor();
        let span = record.max_rv - base;
        let diff = rv - base;
        if diff.is_zero() {
            return None;
        }

        match jurisdiction {
            Jurisdiction::ENGLAND => {
                Some((span / diff).round_dp_with_strategy(3, RoundingStrategy::MidpointTowardZero))
            }
            Jurisdiction::WALES => {
                Some((span / diff).round_dp_with_strategy(3, RoundingStrategy::MidpointAwayFromZero))
            }
            _ => None,
        }
    }

    /// Resolves Welsh Small Business Rates Relief, including Childcare and Post Office rules
    /// under The Non-Domestic Rating (Small Business Relief) (Wales) Order 2017 (W.S.I. 2017/1229)
    pub fn resolve_welsh_sbrr(
        &self,
        financial_year: &str,
        rv: Decimal,
        is_childcare: bool,
        is_post_office: bool,
    ) -> Option<SbrrResolution> {
        if is_childcare && financial_year >= "2019/20" {
            // W.S.I. 2017/1229 art. 8 as amended by W.S.I. 2018/1193:
            // 100% relief for premises wholly used for child minding or day care up to £100,000 RV (from 2019/20)
            if let Some(cap) = self.scheme_caps.iter().find(|c| c.scheme_code == "WELSH_SBRR_CHILDCARE") {
                if rv <= cap.cap_value {
                    return Some(SbrrResolution {
                        relief_code: "SBRR_WALES".to_string(),
                        relief_name: "Welsh Small Business Rates Relief (Registered Childcare)".to_string(),
                        statutory_authority: cap.statutory_authority.clone(),
                        discount_percentage: cap.secondary_cap_value.unwrap_or(dec!(100.00)),
                    });
                } else {
                    return None;
                }
            }
        }

        if is_post_office {
            // W.S.I. 2017/1229 art. 9 & 11(2):
            // 100% relief up to £9,000 RV, 50% relief for £9,001 to £12,000 RV
            if let Some(cap_full) = self.scheme_caps.iter().find(|c| c.scheme_code == "WELSH_SBRR_POST_OFFICE_FULL") {
                if rv <= cap_full.cap_value {
                    return Some(SbrrResolution {
                        relief_code: "SBRR_WALES".to_string(),
                        relief_name: "Welsh Small Business Rates Relief (Post Office)".to_string(),
                        statutory_authority: cap_full.statutory_authority.clone(),
                        discount_percentage: cap_full.secondary_cap_value.unwrap_or(dec!(100.00)),
                    });
                }
            }
            if let Some(cap_half) = self.scheme_caps.iter().find(|c| c.scheme_code == "WELSH_SBRR_POST_OFFICE_HALF") {
                if rv <= cap_half.cap_value {
                    return Some(SbrrResolution {
                        relief_code: "SBRR_WALES".to_string(),
                        relief_name: "Welsh Small Business Rates Relief (Post Office)".to_string(),
                        statutory_authority: cap_half.statutory_authority.clone(),
                        discount_percentage: cap_half.secondary_cap_value.unwrap_or(dec!(50.00)),
                    });
                }
            }
            return None;
        }

        self.resolve_sbrr(Jurisdiction::WALES, financial_year, rv)
    }

    /// Resolves Scottish Small Business Bonus Scheme (SBBS) relief percentage and statutory authority
    /// under S.S.I. 2023/30, S.S.I. 2024/4, S.S.I. 2025/38, and S.S.I. 2026/68.
    pub fn resolve_scottish_sbbs(
        &self,
        financial_year: &str,
        rv: Decimal,
        is_multiple: bool,
        cumulative_rv: Option<Decimal>,
    ) -> Option<SbrrResolution> {
        let is_multi = is_multiple || cumulative_rv.is_some();
        let authority = match financial_year {
            "2023/24" => if is_multi { "S.S.I. 2023/30 reg. 4" } else { "S.S.I. 2023/30 reg. 3" },
            "2024/25" => if is_multi { "S.S.I. 2024/4 reg. 4" } else { "S.S.I. 2024/4 reg. 3" },
            "2025/26" => if is_multi { "S.S.I. 2025/38 reg. 4" } else { "S.S.I. 2025/38 reg. 3" },
            "2026/27" => if is_multi { "S.S.I. 2026/68 reg. 4" } else { "S.S.I. 2026/68 reg. 3" },
            _ => if is_multi { "S.S.I. 2026/68 reg. 4" } else { "S.S.I. 2026/68 reg. 3" },
        };

        if is_multi {
            // Reg 4: Multiple entries with cumulative RV <= £35,000
            let cum = cumulative_rv.unwrap_or(rv);
            if cum > dec!(35000.00) {
                return None;
            }
            let pct = if cum <= dec!(12000.00) {
                dec!(100.00)
            } else if rv <= dec!(15000.00) {
                dec!(25.00)
            } else if rv <= dec!(20000.00) {
                ((dec!(20000.00) - rv) * dec!(25.00) / dec!(5000.00)).round_dp(2)
            } else {
                return None;
            };

            return Some(SbrrResolution {
                relief_code: "SBBS".to_string(),
                relief_name: "Small Business Bonus Scheme".to_string(),
                statutory_authority: authority.to_string(),
                discount_percentage: pct,
            });
        }

        // Reg 3: Single entry in the roll (2023 list and 2026/27)
        if financial_year >= "2023/24" {
            let pct = if rv <= dec!(12000.00) {
                dec!(100.00)
            } else if rv <= dec!(15000.00) {
                // Straight taper from 100% down to 25%:
                // 100 - 75 * (1 - (15,000 - RV) / 3,000) = 25 + 75 * (15,000 - RV) / 3,000
                (dec!(25.00) + (dec!(15000.00) - rv) * dec!(75.00) / dec!(3000.00)).round_dp(2)
            } else if rv <= dec!(20000.00) {
                // Straight taper from 25% down to 0%:
                // 25 * (20,000 - RV) / 5,000
                ((dec!(20000.00) - rv) * dec!(25.00) / dec!(5000.00)).round_dp(2)
            } else {
                return None;
            };

            return Some(SbrrResolution {
                relief_code: "SBBS".to_string(),
                relief_name: "Small Business Bonus Scheme".to_string(),
                statutory_authority: authority.to_string(),
                discount_percentage: pct,
            });
        }

        // Pre-2023 lists: use database table matching
        let records: Vec<&ParamSbrrSbbsThreshold> = self.sbrr_thresholds.iter()
            .filter(|s| s.jurisdiction == Jurisdiction::SCOTLAND && s.financial_year == financial_year)
            .collect();
        for record in records {
            if rv >= record.min_rv && rv <= record.max_rv {
                let pct = if record.is_tapered {
                    let range = record.max_rv - record.min_rv;
                    if range.is_zero() {
                        record.relief_percentage
                    } else {
                        ((record.max_rv - rv) / range * record.relief_percentage).round_dp(2)
                    }
                } else {
                    record.relief_percentage
                };
                let authority = if financial_year >= "2017/18" {
                    "SSI 2017/36"
                } else if financial_year >= "2010/11" {
                    "SSI 2010/37"
                } else {
                    "SSI 2008/106"
                };
                return Some(SbrrResolution {
                    relief_code: "SBBS".to_string(),
                    relief_name: "Small Business Bonus Scheme".to_string(),
                    statutory_authority: authority.to_string(),
                    discount_percentage: pct,
                });
            }
        }
        None
    }

    /// Resolves Sector Relief (e.g. Retail, Hospitality & Leisure)
    pub fn resolve_sector_relief(
        &self,
        jurisdiction: Jurisdiction,
        financial_year: &str,
        is_rhl: bool,
        is_island: bool,
        is_hospitality: bool,
    ) -> Option<SectorReliefResolution> {
        if !is_rhl {
            return None;
        }

        let record = self.sector_reliefs.iter().find(|s| {
            if s.jurisdiction != jurisdiction || s.financial_year != financial_year {
                return false;
            }
            if jurisdiction == Jurisdiction::SCOTLAND {
                if is_island {
                    s.relief_code.contains("ISLANDS")
                } else if is_hospitality {
                    s.relief_code.contains("HOSPITALITY")
                } else {
                    s.relief_code.contains("MAINLAND")
                }
            } else {
                s.relief_code.starts_with("RHL")
            }
        })?;

        let authority = match jurisdiction {
            Jurisdiction::ENGLAND => "LGFA 1988 s.47".to_string(),
            Jurisdiction::WALES => "LGFA 1988 s.47 / LGF(W)A 2024".to_string(),
            Jurisdiction::SCOTLAND => {
                if is_island {
                    "Non-Domestic Rates (Retail, Hospitality and Leisure Relief) (Scotland) Regs 2026 (SSI 2026/69) reg. 5 / SSI 2026/135 / Islands (Scotland) Act 2018".to_string()
                } else if is_hospitality {
                    "Non-Domestic Rates (Retail, Hospitality and Leisure Relief) (Scotland) Regs 2026 (SSI 2026/69) reg. 4A / SSI 2026/135".to_string()
                } else {
                    "Non-Domestic Rates (Retail, Hospitality and Leisure Relief) (Scotland) Regs 2026 (SSI 2026/69) reg. 4".to_string()
                }
            }
        };

        let relief_name = match jurisdiction {
            Jurisdiction::SCOTLAND => {
                if is_island {
                    "Scottish Islands Retail, Hospitality & Leisure Relief".to_string()
                } else if is_hospitality {
                    "Scottish Hospitality Rates Relief".to_string()
                } else {
                    "Scottish Retail, Hospitality & Leisure Relief".to_string()
                }
            }
            Jurisdiction::WALES => {
                if financial_year >= "2026/27" {
                    "Welsh Food & Drink Hospitality Relief".to_string()
                } else {
                    "Retail, Leisure & Hospitality Relief".to_string()
                }
            }
            _ => "Retail, Hospitality & Leisure Relief".to_string(),
        };

        Some(SectorReliefResolution {
            relief_code: record.relief_code.clone(),
            relief_name,
            statutory_authority: authority,
            discount_percentage: record.discount_percentage,
            cash_cap_gbp: record.cash_cap_gbp,
        })
    }

    /// Resolves Scottish Small Business Bonus Scheme Transitional Relief (SBBS TR)
    pub fn resolve_sbbs_tr(&self, financial_year: &str) -> Option<SbbsTrResolution> {
        let cap = self.scheme_caps.iter().find(|c| {
            c.scheme_code == "SBBS_TR"
                && c.financial_year.as_deref() == Some(financial_year)
                && c.jurisdiction == Some(Jurisdiction::SCOTLAND)
        })?;

        let cap_type = match cap.cap_unit.as_str() {
            "PERCENTAGE_INCREASE" => SbbsTrCapType::PercentageOfIncrease(cap.cap_value),
            _ => SbbsTrCapType::FixedCash(cap.cap_value),
        };

        Some(SbbsTrResolution {
            relief_code: "SBBS_TR".to_string(),
            relief_name: "Small Business Bonus Scheme Transitional Relief".to_string(),
            statutory_authority: cap.statutory_authority.clone(),
            cap_type,
        })
    }

    /// Resolves English Supporting Small Business Relief (SSBR) under LGFA 1988 s.47
    pub fn resolve_ssbr(&self, financial_year: &str) -> Option<SsbrResolution> {
        let cap = self.scheme_caps.iter().find(|c| {
            c.scheme_code == "SSBR"
                && c.financial_year.as_deref() == Some(financial_year)
                && c.jurisdiction == Some(Jurisdiction::ENGLAND)
        })?;

        Some(SsbrResolution {
            relief_code: "SSBR".to_string(),
            relief_name: "Supporting Small Business Relief".to_string(),
            statutory_authority: cap.statutory_authority.clone(),
            annual_cap: cap.cap_value,
        })
    }

    /// Resolves Welsh Transitional Relief scheme parameters under W.S.I. 2025/1371 / W.S.I. 2022/1350 / W.S.I. 2016/1247
    pub fn resolve_welsh_transitional_relief_rate(&self, financial_year: &str) -> Option<(Decimal, String)> {
        self.scheme_caps.iter()
            .find(|c| {
                c.scheme_code == "WALES_TR"
                    && c.financial_year.as_deref() == Some(financial_year)
                    && c.jurisdiction == Some(Jurisdiction::WALES)
            })
            .map(|c| (c.cap_value, c.statutory_authority.clone()))
    }

    /// Resolves the statutory rateable value exemption threshold for unoccupied property (LGFA 1988 s.45 / S.I. 2008/386 / WSI 2008/2499 / SSI 2018/77)
    pub fn resolve_empty_property_threshold(&self, jurisdiction: Jurisdiction, date: NaiveDate) -> Option<(Decimal, String)> {
        self.empty_property_thresholds.iter()
            .find(|t| t.jurisdiction == jurisdiction && date >= t.effective_from && date <= t.effective_to)
            .map(|t| (t.threshold_rv, t.statutory_authority.clone()))
    }

    /// Resolves self-catering holiday accommodation statutory letting tests (minimum actual let days, minimum available days, statutory authority)
    /// (LGFA 1988 s.66(2B)/(2BB) / W.S.I. 2022/563 / S.S.I. 2021/489 reg 2(4) / S.I. 2022/217)
    pub fn resolve_holiday_let_thresholds(&self, jurisdiction: Jurisdiction, date: NaiveDate) -> (i64, i64, String) {
        self.letting_tests.iter()
            .find(|t| t.jurisdiction == jurisdiction && date >= t.effective_from && date <= t.effective_to)
            .map(|t| (t.min_actual_let_days, t.min_available_days, t.statutory_authority.clone()))
            .unwrap_or((0, 140, "LGFA 1988 s.66(2B)".to_string()))
    }

    /// Resolves rural settlement relief rateable value ceiling for given SCAT code (LGFA 1988 s.43(6B) / LGFA 1997 / 1975 Act Sched 1A)
    pub fn resolve_rural_settlement_cap(&self, scat_code: &str) -> (Decimal, String) {
        let is_pub_or_petrol = scat_code == "209" || scat_code == "211";
        let target_code = if is_pub_or_petrol { "RURAL_PUB_PETROL" } else { "RURAL_GENERAL" };
        self.scheme_caps.iter()
            .find(|c| c.scheme_code == target_code)
            .map(|c| (c.cap_value, c.statutory_authority.clone()))
            .unwrap_or_else(|| (if is_pub_or_petrol { dec!(12500.00) } else { dec!(8500.00) }, "LGFA 1988 s.43(6B) / LGFA 1997".to_string()))
    }

    /// Resolves Scottish renewable energy generation relief percentage for rateable value (SSI 2010/336)
    pub fn resolve_renewable_energy_relief_pct(&self, rv: Decimal) -> (Decimal, String) {
        let matching = self.scheme_caps.iter()
            .filter(|c| c.scheme_code == "RENEWABLE_ENERGY_SCOTLAND" && rv <= c.cap_value)
            .min_by_key(|c| c.cap_value);
        if let Some(c) = matching {
            (c.secondary_cap_value.unwrap_or(dec!(0.00)), c.statutory_authority.clone())
        } else {
            (dec!(0.00), "Non-Domestic Rates (Renewable Energy Generation Relief) (Scotland) Regs 2010 (SSI 2010/336)".to_string())
        }
    }

    /// Resolves consecutive occupied days required to reset empty property rate-free period (NDRA 2023 s.15 / LGFA 1988 s.45A)
    pub fn resolve_occupancy_reset_days(&self, jurisdiction: Jurisdiction, date: NaiveDate) -> (i64, String) {
        self.scheme_caps.iter()
            .find(|c| {
                c.scheme_code == "EMPTY_OCCUPIED_RESET"
                    && (c.jurisdiction.is_none() || c.jurisdiction == Some(jurisdiction))
                    && c.effective_from.is_none_or(|f| date >= f)
                    && c.effective_to.is_none_or(|t| date <= t)
            })
            .map(|c| (c.cap_value.to_string().parse::<i64>().unwrap_or(42), c.statutory_authority.clone()))
            .unwrap_or((42, "LGFA 1988 s.45A / WSI 2008/2499 / SSI 2008/77".to_string()))
    }

    /// Resolves Transitional Relief Supplement (TRS) addition (S.I. 2026/81 reg. 16)
    pub fn resolve_transitional_supplement(&self, jurisdiction: Jurisdiction, financial_year: &str) -> Option<(Decimal, String)> {
        self.scheme_caps.iter()
            .find(|c| {
                c.scheme_code == "TRANSITIONAL_RELIEF_SUPPLEMENT"
                    && c.jurisdiction == Some(jurisdiction)
                    && c.financial_year.as_deref() == Some(financial_year)
            })
            .map(|c| (c.cap_value, c.statutory_authority.clone()))
    }

    /// Resolves Welsh anti-avoidance statutory penalty (W.S.I. 2026/43 / LGF(W)A 2024)
    pub fn resolve_welsh_avoidance_penalty(&self, rv: Decimal) -> (Decimal, String, String) {
        let matching = self.scheme_caps.iter()
            .find(|c| c.scheme_code == "WELSH_AVOIDANCE_PENALTY");
        if let Some(c) = matching {
            let fixed = c.cap_value;
            let rate = c.secondary_cap_value.unwrap_or(dec!(0.03));
            let total = fixed + (rv * rate).round_dp(2);
            let formula = format!("£{} fixed fee + {}% of Rateable Value (£{})", fixed, (rate * dec!(100.00)).round_dp(0), rv);
            (total, formula, c.statutory_authority.clone())
        } else {
            let total = dec!(500.00) + (rv * dec!(0.03)).round_dp(2);
            (total, format!("£500 flat fee + 3% of Rateable Value (£{})", rv), "NDR (Artificial Avoidance Arrangements) (Wales) Regs 2026 (WSI 2026/43)".to_string())
        }
    }
}

impl RepoData {
    fn new_seeded() -> Self {
        let versions = ParameterVersionIds {
            param_multiplier_version: "v2026.1".to_string(),
            param_sbrr_version: "v2026.1".to_string(),
            param_sector_relief_version: "v2026.2".to_string(),
        };

        let mut data = Self {
            multipliers: Vec::new(),
            sbrr_thresholds: Vec::new(),
            sector_reliefs: Vec::new(),
            transitional_caps: Vec::new(),
            reference_codes: Vec::new(),
            empty_property_thresholds: Vec::new(),
            letting_tests: Vec::new(),
            scheme_caps: Vec::new(),
            versions,
        };

        data.seed_multipliers();
        data.seed_sbrr_thresholds();
        data.seed_sector_reliefs();
        data.seed_transitional_caps();
        data.seed_reference_codes();
        data.seed_empty_property_thresholds();
        data.seed_letting_tests();
        data.seed_scheme_caps();

        data
    }

    // --- Private Seeders ---

    fn seed_multipliers(&mut self) {
        let mut add = |id: &str, fy: &str, jur: Jurisdiction, m_type: &str, min: Decimal, max: Option<Decimal>, rate: Decimal, start: &str, end: &str| {
            self.multipliers.push(ParamMultiplier {
                parameter_id: id.to_string(),
                financial_year: fy.to_string(),
                jurisdiction: jur,
                multiplier_type: m_type.to_string(),
                rv_threshold_min: min,
                rv_threshold_max: max,
                multiplier_rate: rate,
                effective_from: NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap(),
                effective_to: NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap(),
            });
        };

        // Historical 1989–2004 Standard Multipliers (England, Wales, Scotland)
        let hist_data = [
            ("1989/90", dec!(0.348), dec!(0.368), dec!(0.410), "1989-04-01", "1990-03-31"),
            ("1990/91", dec!(0.348), dec!(0.368), dec!(0.410), "1990-04-01", "1991-03-31"),
            ("1991/92", dec!(0.386), dec!(0.386), dec!(0.448), "1991-04-01", "1992-03-31"),
            ("1992/93", dec!(0.406), dec!(0.406), dec!(0.467), "1992-04-01", "1993-03-31"),
            ("1993/94", dec!(0.416), dec!(0.416), dec!(0.477), "1993-04-01", "1994-03-31"),
            ("1994/95", dec!(0.423), dec!(0.423), dec!(0.458), "1994-04-01", "1995-03-31"),
            ("1995/96", dec!(0.432), dec!(0.390), dec!(0.432), "1995-04-01", "1996-03-31"),
            ("1996/97", dec!(0.449), dec!(0.405), dec!(0.449), "1996-04-01", "1997-03-31"),
            ("1997/98", dec!(0.458), dec!(0.414), dec!(0.458), "1997-04-01", "1998-03-31"),
            ("1998/99", dec!(0.474), dec!(0.428), dec!(0.474), "1998-04-01", "1999-03-31"),
            ("1999/00", dec!(0.489), dec!(0.442), dec!(0.489), "1999-04-01", "2000-03-31"),
            ("2000/01", dec!(0.416), dec!(0.412), dec!(0.458), "2000-04-01", "2001-03-31"),
            ("2001/02", dec!(0.430), dec!(0.426), dec!(0.470), "2001-04-01", "2002-03-31"),
            ("2002/03", dec!(0.437), dec!(0.433), dec!(0.478), "2002-04-01", "2003-03-31"),
            ("2003/04", dec!(0.444), dec!(0.440), dec!(0.478), "2003-04-01", "2004-03-31"),
            ("2004/05", dec!(0.456), dec!(0.452), dec!(0.488), "2004-04-01", "2005-03-31"),
        ];

        for (fy, eng, wal, sco, s, e) in hist_data {
            add(&format!("m_eng_{}", fy), fy, Jurisdiction::ENGLAND, "STANDARD", dec!(0.00), None, eng, s, e);
            add(&format!("m_wal_{}", fy), fy, Jurisdiction::WALES, "STANDARD", dec!(0.00), None, wal, s, e);
            add(&format!("m_sco_{}", fy), fy, Jurisdiction::SCOTLAND, "STANDARD", dec!(0.00), None, sco, s, e);
        }

        // 2005 to 2016 lists (SBRR era)
        let sbrr_era_england = [
            ("2005/06", dec!(0.415), dec!(0.422), "2005-04-01", "2006-03-31"),
            ("2006/07", dec!(0.426), dec!(0.433), "2006-04-01", "2007-03-31"),
            ("2007/08", dec!(0.441), dec!(0.444), "2007-04-01", "2008-03-31"),
            ("2008/09", dec!(0.458), dec!(0.462), "2008-04-01", "2009-03-31"),
            ("2009/10", dec!(0.481), dec!(0.485), "2009-04-01", "2010-03-31"),
            ("2010/11", dec!(0.407), dec!(0.414), "2010-04-01", "2011-03-31"),
            ("2011/12", dec!(0.426), dec!(0.433), "2011-04-01", "2012-03-31"),
            ("2012/13", dec!(0.450), dec!(0.458), "2012-04-01", "2013-03-31"),
            ("2013/14", dec!(0.462), dec!(0.471), "2013-04-01", "2014-03-31"),
            ("2014/15", dec!(0.471), dec!(0.482), "2014-04-01", "2015-03-31"),
            ("2015/16", dec!(0.480), dec!(0.493), "2015-04-01", "2016-03-31"),
            ("2016/17", dec!(0.484), dec!(0.497), "2016-04-01", "2017-03-31"),
        ];

        for (fy, sm, st, s, e) in sbrr_era_england {
            add(&format!("m_eng_sm_{}", fy), fy, Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(51000.00)), sm, s, e);
            add(&format!("m_eng_st_{}", fy), fy, Jurisdiction::ENGLAND, "STANDARD", dec!(51000.01), None, st, s, e);
            add(&format!("m_sco_{}", fy), fy, Jurisdiction::SCOTLAND, "STANDARD", dec!(0.00), None, sm, s, e);
        }

        // Wales Multipliers 2005/06 – 2016/17 (Official StatsWales single uniform multiplier)
        let sbrr_era_wales = [
            ("2005/06", dec!(0.421), "2005-04-01", "2006-03-31"),
            ("2006/07", dec!(0.432), "2006-04-01", "2007-03-31"),
            ("2007/08", dec!(0.448), "2007-04-01", "2008-03-31"),
            ("2008/09", dec!(0.466), "2008-04-01", "2009-03-31"),
            ("2009/10", dec!(0.489), "2009-04-01", "2010-03-31"),
            ("2010/11", dec!(0.409), "2010-04-01", "2011-03-31"),
            ("2011/12", dec!(0.428), "2011-04-01", "2012-03-31"),
            ("2012/13", dec!(0.452), "2012-04-01", "2013-03-31"),
            ("2013/14", dec!(0.464), "2013-04-01", "2014-03-31"),
            ("2014/15", dec!(0.473), "2014-04-01", "2015-03-31"),
            ("2015/16", dec!(0.482), "2015-04-01", "2016-03-31"),
            ("2016/17", dec!(0.486), "2016-04-01", "2017-03-31"),
        ];
        for (fy, rate, s, e) in sbrr_era_wales {
            add(&format!("m_wal_{}", fy), fy, Jurisdiction::WALES, "STANDARD", dec!(0.00), None, rate, s, e);
        }

        // 2017–2022 lists
        let list_2017 = [
            ("2017/18", dec!(0.466), dec!(0.479), dec!(0.499), dec!(0.466), "2017-04-01", "2018-03-31"),
            ("2018/19", dec!(0.480), dec!(0.493), dec!(0.514), dec!(0.480), "2018-04-01", "2019-03-31"),
            ("2019/20", dec!(0.491), dec!(0.504), dec!(0.526), dec!(0.490), "2019-04-01", "2020-03-31"),
            ("2020/21", dec!(0.499), dec!(0.512), dec!(0.535), dec!(0.498), "2020-04-01", "2021-03-31"),
            ("2021/22", dec!(0.499), dec!(0.512), dec!(0.535), dec!(0.490), "2021-04-01", "2022-03-31"),
            ("2022/23", dec!(0.499), dec!(0.512), dec!(0.535), dec!(0.498), "2022-04-01", "2023-03-31"),
        ];

        for (fy, eng_sm, eng_st, wal, sco, s, e) in list_2017 {
            add(&format!("m_eng_sm_{}", fy), fy, Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(50999.00)), eng_sm, s, e);
            add(&format!("m_eng_st_{}", fy), fy, Jurisdiction::ENGLAND, "STANDARD", dec!(51000.00), None, eng_st, s, e);
            add(&format!("m_wal_{}", fy), fy, Jurisdiction::WALES, "STANDARD", dec!(0.00), None, wal, s, e);
            add(&format!("m_sco_{}", fy), fy, Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), sco, s, e);
        }

        // 2023/24
        add("m_eng_sm_2023", "2023/24", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(50999.00)), dec!(0.499), "2023-04-01", "2024-03-31");
        add("m_eng_st_2023", "2023/24", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.00), None, dec!(0.512), "2023-04-01", "2024-03-31");
        add("m_wal_2023", "2023/24", Jurisdiction::WALES, "STANDARD", dec!(0.00), None, dec!(0.535), "2023-04-01", "2024-03-31");
        add("m_sco_b_2023", "2023/24", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), dec!(0.498), "2023-04-01", "2024-03-31");
        add("m_sco_i_2023", "2023/24", Jurisdiction::SCOTLAND, "INTERMEDIATE", dec!(51000.01), Some(dec!(100000.00)), dec!(0.511), "2023-04-01", "2024-03-31");
        add("m_sco_h_2023", "2023/24", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.524), "2023-04-01", "2024-03-31");

        // 2024/25
        add("m_eng_sm_2024", "2024/25", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(50999.00)), dec!(0.499), "2024-04-01", "2025-03-31");
        add("m_eng_st_2024", "2024/25", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.00), None, dec!(0.546), "2024-04-01", "2025-03-31");
        add("m_wal_2024", "2024/25", Jurisdiction::WALES, "STANDARD", dec!(0.00), None, dec!(0.562), "2024-04-01", "2025-03-31");
        add("m_sco_b_2024", "2024/25", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), dec!(0.498), "2024-04-01", "2025-03-31");
        add("m_sco_i_2024", "2024/25", Jurisdiction::SCOTLAND, "INTERMEDIATE", dec!(51000.01), Some(dec!(100000.00)), dec!(0.545), "2024-04-01", "2025-03-31");
        add("m_sco_h_2024", "2024/25", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.559), "2024-04-01", "2025-03-31");

        // 2025/26
        add("m_eng_sm_2025", "2025/26", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(50999.00)), dec!(0.499), "2025-04-01", "2026-03-31");
        add("m_eng_st_2025", "2025/26", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.00), None, dec!(0.555), "2025-04-01", "2026-03-31");
        add("m_wal_2025", "2025/26", Jurisdiction::WALES, "STANDARD", dec!(0.00), None, dec!(0.568), "2025-04-01", "2026-03-31");
        add("m_sco_b_2025", "2025/26", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), dec!(0.498), "2025-04-01", "2026-03-31");
        add("m_sco_i_2025", "2025/26", Jurisdiction::SCOTLAND, "INTERMEDIATE", dec!(51000.01), Some(dec!(100000.00)), dec!(0.554), "2025-04-01", "2026-03-31");
        add("m_sco_h_2025", "2025/26", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.568), "2025-04-01", "2026-03-31");

        // 2026/27 (Revaluation Year — NDRA 2023 5-multiplier system for England)
        add("m_eng_hv_2026", "2026/27", Jurisdiction::ENGLAND, "HIGH_VALUE", dec!(500000.00), None, dec!(0.508), "2026-04-01", "2027-03-31");
        add("m_eng_sm_rhl_2026", "2026/27", Jurisdiction::ENGLAND, "SMALL_BUSINESS_RHL", dec!(0.00), Some(dec!(50999.00)), dec!(0.382), "2026-04-01", "2027-03-31");
        add("m_eng_sm_2026", "2026/27", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(50999.00)), dec!(0.432), "2026-04-01", "2027-03-31");
        add("m_eng_st_rhl_2026", "2026/27", Jurisdiction::ENGLAND, "STANDARD_RHL", dec!(51000.00), Some(dec!(499999.99)), dec!(0.430), "2026-04-01", "2027-03-31");
        add("m_eng_st_2026", "2026/27", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.00), Some(dec!(499999.99)), dec!(0.480), "2026-04-01", "2027-03-31");

        // 2026/27 (Revaluation Year — Local Government Finance (Wales) Act 2024 3-multiplier system for Wales)
        add("m_wal_ret_2026", "2026/27", Jurisdiction::WALES, "RETAIL", dec!(0.00), Some(dec!(51000.00)), dec!(0.350), "2026-04-01", "2027-03-31");
        add("m_wal_std_2026", "2026/27", Jurisdiction::WALES, "STANDARD", dec!(0.00), Some(dec!(100000.00)), dec!(0.502), "2026-04-01", "2027-03-31");
        add("m_wal_hgh_2026", "2026/27", Jurisdiction::WALES, "HIGHER", dec!(100000.01), None, dec!(0.515), "2026-04-01", "2027-03-31");

        add("m_sco_b_2026", "2026/27", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), dec!(0.481), "2026-04-01", "2027-03-31");
        add("m_sco_i_2026", "2026/27", Jurisdiction::SCOTLAND, "INTERMEDIATE", dec!(51000.01), Some(dec!(100000.00)), dec!(0.535), "2026-04-01", "2027-03-31");
        add("m_sco_h_2026", "2026/27", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.548), "2026-04-01", "2027-03-31");

        // 2027/28 (Revaluation List Year 2)
        add("m_eng_hv_2027", "2027/28", Jurisdiction::ENGLAND, "HIGH_VALUE", dec!(500000.00), None, dec!(0.508), "2027-04-01", "2028-03-31");
        add("m_eng_sm_rhl_2027", "2027/28", Jurisdiction::ENGLAND, "SMALL_BUSINESS_RHL", dec!(0.00), Some(dec!(50999.00)), dec!(0.382), "2027-04-01", "2028-03-31");
        add("m_eng_sm_2027", "2027/28", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(50999.00)), dec!(0.432), "2027-04-01", "2028-03-31");
        add("m_eng_st_rhl_2027", "2027/28", Jurisdiction::ENGLAND, "STANDARD_RHL", dec!(51000.00), Some(dec!(499999.99)), dec!(0.430), "2027-04-01", "2028-03-31");
        add("m_eng_st_2027", "2027/28", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.00), Some(dec!(499999.99)), dec!(0.480), "2027-04-01", "2028-03-31");

        add("m_wal_ret_2027", "2027/28", Jurisdiction::WALES, "RETAIL_PROVISIONAL", dec!(0.00), Some(dec!(51000.00)), dec!(0.350), "2027-04-01", "2028-03-31");
        add("m_wal_std_2027", "2027/28", Jurisdiction::WALES, "STANDARD_PROVISIONAL", dec!(0.00), Some(dec!(100000.00)), dec!(0.502), "2027-04-01", "2028-03-31");
        add("m_wal_hgh_2027", "2027/28", Jurisdiction::WALES, "HIGHER_PROVISIONAL", dec!(100000.01), None, dec!(0.515), "2027-04-01", "2028-03-31");

        add("m_sco_b_2027", "2027/28", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), dec!(0.481), "2027-04-01", "2028-03-31");
        add("m_sco_i_2027", "2027/28", Jurisdiction::SCOTLAND, "INTERMEDIATE", dec!(51000.01), Some(dec!(100000.00)), dec!(0.535), "2027-04-01", "2028-03-31");
        add("m_sco_h_2027", "2027/28", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.548), "2027-04-01", "2028-03-31");
    }

    fn seed_sbrr_thresholds(&mut self) {
        let mut add = |id: &str, fy: &str, jur: Jurisdiction, min: Decimal, max: Decimal, pct: Decimal, tap: bool, start: &str, end: &str| {
            self.sbrr_thresholds.push(ParamSbrrSbbsThreshold {
                parameter_id: id.to_string(),
                financial_year: fy.to_string(),
                jurisdiction: jur,
                min_rv: min,
                max_rv: max,
                relief_percentage: pct,
                is_tapered: tap,
                effective_from: NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap(),
                effective_to: NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap(),
            });
        };

        // Note: Pre-2005 (1990/91 – 2004/05): SBRR did not exist in primary legislation
        // (enacted by Local Government Act 2003 s.61, effective 1 April 2005).
        // Pre-2005 calculations correctly resolve to None (0% discount).

        // 2005 Revaluation List (2005/06 – 2009/10)
        for fy in ["2005/06", "2006/07", "2007/08", "2008/09", "2009/10"] {
            let start = format!("{}-04-01", &fy[0..4]);
            let next_y: i32 = fy[0..4].parse::<i32>().unwrap() + 1;
            let end = format!("{}-03-31", next_y);

            // England (SI 2004/3315: 50% max relief up to £5,000; linear taper 50% to 0% between £5,001 and £10,000)
            add(&format!("sbrr_eng_full_{}", fy), fy, Jurisdiction::ENGLAND, dec!(0.00), dec!(5000.00), dec!(50.00), false, &start, &end);
            add(&format!("sbrr_eng_tap_{}", fy), fy, Jurisdiction::ENGLAND, dec!(5000.01), dec!(10000.00), dec!(50.00), true, &start, &end);

            // Wales (WSI 2005/309 & WSI 2008/707: 50% up to £2,000; 25% for £2,001 to £5,000)
            add(&format!("sbrr_wal_tier1_{}", fy), fy, Jurisdiction::WALES, dec!(0.00), dec!(2000.00), dec!(50.00), false, &start, &end);
            add(&format!("sbrr_wal_tier2_{}", fy), fy, Jurisdiction::WALES, dec!(2000.01), dec!(5000.00), dec!(25.00), false, &start, &end);
        }

        // Scotland SBBS 2008/09 (SSI 2008/106: 80% <= £8k, 40% <= £10k, 20% <= £15k)
        add("sbbs_sco_t1_2008/09", "2008/09", Jurisdiction::SCOTLAND, dec!(0.00), dec!(8000.00), dec!(80.00), false, "2008-04-01", "2009-03-31");
        add("sbbs_sco_t2_2008/09", "2008/09", Jurisdiction::SCOTLAND, dec!(8000.01), dec!(10000.00), dec!(40.00), false, "2008-04-01", "2009-03-31");
        add("sbbs_sco_t3_2008/09", "2008/09", Jurisdiction::SCOTLAND, dec!(10000.01), dec!(15000.00), dec!(20.00), false, "2008-04-01", "2009-03-31");

        // Scotland SBBS 2009/10 (100% <= £8k, 50% <= £10k, 25% <= £15k)
        add("sbbs_sco_t1_2009/10", "2009/10", Jurisdiction::SCOTLAND, dec!(0.00), dec!(8000.00), dec!(100.00), false, "2009-04-01", "2010-03-31");
        add("sbbs_sco_t2_2009/10", "2009/10", Jurisdiction::SCOTLAND, dec!(8000.01), dec!(10000.00), dec!(50.00), false, "2009-04-01", "2010-03-31");
        add("sbbs_sco_t3_2009/10", "2009/10", Jurisdiction::SCOTLAND, dec!(10000.01), dec!(15000.00), dec!(25.00), false, "2009-04-01", "2010-03-31");

        // 2010 Revaluation List (2010/11 – 2016/17)
        for fy in ["2010/11", "2011/12", "2012/13", "2013/14", "2014/15", "2015/16", "2016/17"] {
            let start = format!("{}-04-01", &fy[0..4]);
            let next_y: i32 = fy[0..4].parse::<i32>().unwrap() + 1;
            let end = format!("{}-03-31", next_y);

            // England (SI 2010/1959: statutory doubling to 100% up to £6,000; tapered £6,001 to £12,000)
            add(&format!("sbrr_eng_full_{}", fy), fy, Jurisdiction::ENGLAND, dec!(0.00), dec!(6000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbrr_eng_tap_{}", fy), fy, Jurisdiction::ENGLAND, dec!(6000.01), dec!(12000.00), dec!(100.00), true, &start, &end);

            // Wales (WSI 2010/248: 100% up to £6,000; tapered £6,001 to £12,000)
            add(&format!("sbrr_wal_full_{}", fy), fy, Jurisdiction::WALES, dec!(0.00), dec!(6000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbrr_wal_tap_{}", fy), fy, Jurisdiction::WALES, dec!(6000.01), dec!(12000.00), dec!(100.00), true, &start, &end);

            // Scotland SBBS (SSI 2010/37: 100% <= £10,000; 50% <= £12,000; 25% <= £18,000)
            add(&format!("sbbs_sco_t1_{}", fy), fy, Jurisdiction::SCOTLAND, dec!(0.00), dec!(10000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbbs_sco_t2_{}", fy), fy, Jurisdiction::SCOTLAND, dec!(10000.01), dec!(12000.00), dec!(50.00), false, &start, &end);
            add(&format!("sbbs_sco_t3_{}", fy), fy, Jurisdiction::SCOTLAND, dec!(12000.01), dec!(18000.00), dec!(25.00), false, &start, &end);
        }

        // 2017 Revaluation List (2017/18 – 2022/23)
        for fy in ["2017/18", "2018/19", "2019/20", "2020/21", "2021/22", "2022/23"] {
            let start = format!("{}-04-01", &fy[0..4]);
            let next_y: i32 = fy[0..4].parse::<i32>().unwrap() + 1;
            let end = format!("{}-03-31", next_y);

            // England (SI 2017/411: 100% up to £12,000; tapered £12,001 to £15,000)
            add(&format!("sbrr_eng_full_{}", fy), fy, Jurisdiction::ENGLAND, dec!(0.00), dec!(12000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbrr_eng_tap_{}", fy), fy, Jurisdiction::ENGLAND, dec!(12000.01), dec!(15000.00), dec!(100.00), true, &start, &end);

            // Wales (WSI 2017/1229: 100% up to £6,000; tapered £6,001 to £12,000)
            add(&format!("sbrr_wal_full_{}", fy), fy, Jurisdiction::WALES, dec!(0.00), dec!(6000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbrr_wal_tap_{}", fy), fy, Jurisdiction::WALES, dec!(6000.01), dec!(12000.00), dec!(100.00), true, &start, &end);

            // Scotland SBBS (SSI 2017/36: 100% up to £15,000; 25% for £15,001 to £18,000)
            add(&format!("sbbs_sco_full_{}", fy), fy, Jurisdiction::SCOTLAND, dec!(0.00), dec!(15000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbbs_sco_tap_{}", fy), fy, Jurisdiction::SCOTLAND, dec!(15000.01), dec!(18000.00), dec!(25.00), false, &start, &end);
        }

        // 2023 Revaluation List (2023/24 through 2027/28)
        for fy in ["2023/24", "2024/25", "2025/26", "2026/27", "2027/28"] {
            let start = format!("{}-04-01", &fy[0..4]);
            let next_y: i32 = fy[0..4].parse::<i32>().unwrap() + 1;
            let end = format!("{}-03-31", next_y);
            add(&format!("sbrr_eng_full_{}", fy), fy, Jurisdiction::ENGLAND, dec!(0.00), dec!(12000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbrr_eng_tap_{}", fy), fy, Jurisdiction::ENGLAND, dec!(12000.01), dec!(15000.00), dec!(100.00), true, &start, &end);
            add(&format!("sbrr_wal_full_{}", fy), fy, Jurisdiction::WALES, dec!(0.00), dec!(6000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbrr_wal_tap_{}", fy), fy, Jurisdiction::WALES, dec!(6000.01), dec!(12000.00), dec!(100.00), true, &start, &end);
            add(&format!("sbbs_sco_full_{}", fy), fy, Jurisdiction::SCOTLAND, dec!(0.00), dec!(12000.00), dec!(100.00), false, &start, &end);
            add(&format!("sbbs_sco_tap_{}", fy), fy, Jurisdiction::SCOTLAND, dec!(12000.01), dec!(15000.00), dec!(25.00), false, &start, &end);
            add(&format!("sbbs_sco_tap2_{}", fy), fy, Jurisdiction::SCOTLAND, dec!(15000.01), dec!(20000.00), dec!(25.00), true, &start, &end);
        }
    }

    fn seed_sector_reliefs(&mut self) {
        let mut add = |id: &str, fy: &str, jur: Jurisdiction, code: &str, pct: Decimal, cap: Option<Decimal>, start: &str, end: &str| {
            self.sector_reliefs.push(ParamSectorRelief {
                parameter_id: id.to_string(),
                financial_year: fy.to_string(),
                jurisdiction: jur,
                relief_code: code.to_string(),
                discount_percentage: pct,
                cash_cap_gbp: cap,
                effective_from: NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap(),
                effective_to: NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap(),
            });
        };

        // England Retail, Hospitality & Leisure Relief (RHL)
        // Ceased on 31 March 2026; no percentage relief in 2026/27 (replaced by lower high-street multipliers)
        add("sec_eng_rhl_2023", "2023/24", Jurisdiction::ENGLAND, "RHL_2023", dec!(75.00), Some(dec!(110000.00)), "2023-04-01", "2024-03-31");
        add("sec_eng_rhl_2024", "2024/25", Jurisdiction::ENGLAND, "RHL_2024", dec!(75.00), Some(dec!(110000.00)), "2024-04-01", "2025-03-31");
        add("sec_eng_rhl_2025", "2025/26", Jurisdiction::ENGLAND, "RHL_2025", dec!(40.00), Some(dec!(110000.00)), "2025-04-01", "2026-03-31");

        // Wales Retail, Leisure & Hospitality
        // 2026/27 transitions to 15% Food and Drink Hospitality scheme (capped at £110k)
        add("sec_wal_rhl_2023", "2023/24", Jurisdiction::WALES, "RHL_WALES_2023", dec!(75.00), Some(dec!(110000.00)), "2023-04-01", "2024-03-31");
        add("sec_wal_rhl_2024", "2024/25", Jurisdiction::WALES, "RHL_WALES_2024", dec!(40.00), Some(dec!(110000.00)), "2024-04-01", "2025-03-31");
        add("sec_wal_rhl_2025", "2025/26", Jurisdiction::WALES, "RHL_WALES_2025", dec!(40.00), Some(dec!(110000.00)), "2025-04-01", "2026-03-31");
        add("sec_wal_rhl_2026", "2026/27", Jurisdiction::WALES, "RHL_WALES_2026", dec!(15.00), Some(dec!(110000.00)), "2026-04-01", "2027-03-31");

        // Scotland Retail, Hospitality & Leisure Relief
        // 2026/27: 15% mainland retail/leisure (RV <= £100k), 40% hospitality (RV <= £100k), or 100% on islands/remote mainland, £110k cap
        add("sec_sco_rhl_mainland_2026", "2026/27", Jurisdiction::SCOTLAND, "RHL_SCOTLAND_MAINLAND_2026", dec!(15.00), Some(dec!(110000.00)), "2026-04-01", "2027-03-31");
        add("sec_sco_rhl_hospitality_2026", "2026/27", Jurisdiction::SCOTLAND, "RHL_SCOTLAND_HOSPITALITY_2026", dec!(40.00), Some(dec!(110000.00)), "2026-04-01", "2027-03-31");
        add("sec_sco_rhl_islands_2026", "2026/27", Jurisdiction::SCOTLAND, "RHL_SCOTLAND_ISLANDS_2026", dec!(100.00), Some(dec!(110000.00)), "2026-04-01", "2027-03-31");
    }

    fn seed_transitional_caps(&mut self) {
        let mut add = |id: &str, list_year: i32, fy: &str, jur: Jurisdiction, tier: &str, min: Decimal, max: Option<Decimal>, cap: Decimal, is_london: Option<bool>| {
            self.transitional_caps.push(ParamTransitionalCap {
                parameter_id: id.to_string(),
                revaluation_list_year: list_year,
                financial_year: fy.to_string(),
                jurisdiction: jur,
                property_tier: tier.to_string(),
                rv_threshold_min: min,
                rv_threshold_max: max,
                upward_cap_percentage: cap,
                is_greater_london: is_london,
            });
        };

        // 1990 Revaluation List (1990/91 – 1994/95 under LGFA 1988)
        // SI 1989/1358: Small <= £10,000 RV (£15,000 in London), Large > £10,000 RV (£15,000 in London)
        let list_1990 = [
            ("1990/91", dec!(15.00), dec!(20.00)),
            ("1991/92", dec!(15.00), dec!(20.00)),
            ("1992/93", dec!(15.00), dec!(20.00)),
            ("1993/94", dec!(15.00), dec!(20.00)),
            ("1994/95", dec!(10.00), dec!(15.00)),
        ];
        for (fy, sm, lg) in list_1990 {
            for jur in [Jurisdiction::ENGLAND, Jurisdiction::WALES] {
                let pfx = if jur == Jurisdiction::ENGLAND { "eng" } else { "wal" };
                if jur == Jurisdiction::ENGLAND {
                    add(&format!("tc_eng_sm_{}", fy), 1990, fy, jur, "SMALL", dec!(0.00), Some(dec!(10000.00)), sm, Some(false));
                    add(&format!("tc_eng_lg_{}", fy), 1990, fy, jur, "LARGE", dec!(10000.01), None, lg, Some(false));
                    add(&format!("tc_eng_sm_lon_{}", fy), 1990, fy, jur, "SMALL", dec!(0.00), Some(dec!(15000.00)), sm, Some(true));
                    add(&format!("tc_eng_lg_lon_{}", fy), 1990, fy, jur, "LARGE", dec!(15000.01), None, lg, Some(true));
                } else {
                    add(&format!("tc_{}_sm_{}", pfx, fy), 1990, fy, jur, "SMALL", dec!(0.00), Some(dec!(10000.00)), sm, None);
                    add(&format!("tc_{}_lg_{}", pfx, fy), 1990, fy, jur, "LARGE", dec!(10000.01), None, lg, None);
                }
            }
        }

        // 1995 Revaluation List (1995/96 – 1999/00)
        // SI 1994/3279: Small <= £10,000 RV (£15,000 in London), Large > £10,000 RV (£15,000 in London)
        let list_1995 = [
            ("1995/96", dec!(10.00), dec!(20.00)),
            ("1996/97", dec!(10.00), dec!(20.00)),
            ("1997/98", dec!(15.00), dec!(20.00)),
            ("1998/99", dec!(17.50), dec!(20.00)),
            ("1999/00", dec!(20.00), dec!(20.00)),
        ];
        for (fy, sm, lg) in list_1995 {
            for jur in [Jurisdiction::ENGLAND, Jurisdiction::WALES] {
                let pfx = if jur == Jurisdiction::ENGLAND { "eng" } else { "wal" };
                if jur == Jurisdiction::ENGLAND {
                    add(&format!("tc_eng_sm_{}", fy), 1995, fy, jur, "SMALL", dec!(0.00), Some(dec!(10000.00)), sm, Some(false));
                    add(&format!("tc_eng_lg_{}", fy), 1995, fy, jur, "LARGE", dec!(10000.01), None, lg, Some(false));
                    add(&format!("tc_eng_sm_lon_{}", fy), 1995, fy, jur, "SMALL", dec!(0.00), Some(dec!(15000.00)), sm, Some(true));
                    add(&format!("tc_eng_lg_lon_{}", fy), 1995, fy, jur, "LARGE", dec!(15000.01), None, lg, Some(true));
                } else {
                    add(&format!("tc_{}_sm_{}", pfx, fy), 1995, fy, jur, "SMALL", dec!(0.00), Some(dec!(10000.00)), sm, None);
                    add(&format!("tc_{}_lg_{}", pfx, fy), 1995, fy, jur, "LARGE", dec!(10000.01), None, lg, None);
                }
            }
        }

        // 2000 Revaluation List (2000/01 – 2004/05)
        // SI 1999/3379: Small <= £12,000 RV (£18,000 in London), Large > £12,000 RV (£18,000 in London)
        let list_2000 = [
            ("2000/01", dec!(5.00), dec!(12.50)),
            ("2001/02", dec!(7.50), dec!(15.00)),
            ("2002/03", dec!(10.00), dec!(17.50)),
            ("2003/04", dec!(12.50), dec!(20.00)),
            ("2004/05", dec!(15.00), dec!(20.00)),
        ];
        for (fy, sm, lg) in list_2000 {
            for jur in [Jurisdiction::ENGLAND, Jurisdiction::WALES] {
                let pfx = if jur == Jurisdiction::ENGLAND { "eng" } else { "wal" };
                if jur == Jurisdiction::ENGLAND {
                    add(&format!("tc_eng_sm_{}", fy), 2000, fy, jur, "SMALL", dec!(0.00), Some(dec!(12000.00)), sm, Some(false));
                    add(&format!("tc_eng_lg_{}", fy), 2000, fy, jur, "LARGE", dec!(12000.01), None, lg, Some(false));
                    add(&format!("tc_eng_sm_lon_{}", fy), 2000, fy, jur, "SMALL", dec!(0.00), Some(dec!(18000.00)), sm, Some(true));
                    add(&format!("tc_eng_lg_lon_{}", fy), 2000, fy, jur, "LARGE", dec!(18000.01), None, lg, Some(true));
                } else {
                    add(&format!("tc_{}_sm_{}", pfx, fy), 2000, fy, jur, "SMALL", dec!(0.00), Some(dec!(12000.00)), sm, None);
                    add(&format!("tc_{}_lg_{}", pfx, fy), 2000, fy, jur, "LARGE", dec!(12000.01), None, lg, None);
                }
            }
        }

        // 2005 Revaluation List (2005/06 – 2009/10) — England only (SI 2004/3312)
        let list_2005 = [
            ("2005/06", dec!(5.00), dec!(12.50)),
            ("2006/07", dec!(7.50), dec!(17.50)),
            ("2007/08", dec!(10.00), dec!(20.00)),
            ("2008/09", dec!(15.00), dec!(25.00)),
            ("2009/10", dec!(20.00), dec!(30.00)),
        ];
        for (fy, sm, lg) in list_2005 {
            add(&format!("tc_eng_sm_{}", fy), 2005, fy, Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(15000.00)), sm, Some(false));
            add(&format!("tc_eng_lg_{}", fy), 2005, fy, Jurisdiction::ENGLAND, "LARGE", dec!(15000.01), None, lg, Some(false));
            add(&format!("tc_eng_sm_lon_{}", fy), 2005, fy, Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(21500.00)), sm, Some(true));
            add(&format!("tc_eng_lg_lon_{}", fy), 2005, fy, Jurisdiction::ENGLAND, "LARGE", dec!(21500.01), None, lg, Some(true));
        }

        // 2010 Revaluation List (2010/11 – 2016/17) — England only (SI 2009/3343)
        let list_2010 = [
            ("2010/11", dec!(5.00), dec!(12.50), dec!(12.50)),
            ("2011/12", dec!(7.50), dec!(17.50), dec!(17.50)),
            ("2012/13", dec!(10.00), dec!(20.00), dec!(20.00)),
            ("2013/14", dec!(15.00), dec!(25.00), dec!(25.00)),
            ("2014/15", dec!(15.00), dec!(25.00), dec!(25.00)),
            ("2015/16", dec!(15.00), dec!(25.00), dec!(25.00)),
            ("2016/17", dec!(15.00), dec!(25.00), dec!(25.00)),
        ];
        for (fy, sm, med, lg) in list_2010 {
            add(&format!("tc_eng_sm_{}", fy), 2010, fy, Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(18000.00)), sm, Some(false));
            add(&format!("tc_eng_med_{}", fy), 2010, fy, Jurisdiction::ENGLAND, "MEDIUM", dec!(18000.01), Some(dec!(100000.00)), med, Some(false));
            add(&format!("tc_eng_sm_lon_{}", fy), 2010, fy, Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(25500.00)), sm, Some(true));
            add(&format!("tc_eng_med_lon_{}", fy), 2010, fy, Jurisdiction::ENGLAND, "MEDIUM", dec!(25500.01), Some(dec!(100000.00)), med, Some(true));
            add(&format!("tc_eng_lg_{}", fy), 2010, fy, Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, lg, None);
        }

        // 2017 Revaluation List (2017/18 – 2022/23)
        // SI 2016/1265: Small <= £20k (£28k London), Medium <= £100k, Large > £100k
        let list_2017 = [
            ("2017/18", dec!(5.00), dec!(12.50), dec!(42.00)),
            ("2018/19", dec!(7.50), dec!(17.50), dec!(32.00)),
            ("2019/20", dec!(10.00), dec!(20.00), dec!(49.00)),
            ("2020/21", dec!(15.00), dec!(25.00), dec!(16.00)),
            ("2021/22", dec!(15.00), dec!(25.00), dec!(16.00)),
            ("2022/23", dec!(15.00), dec!(25.00), dec!(16.00)),
        ];
        for (fy, sm, med, lg) in list_2017 {
            add(&format!("tc_eng_sm_{}", fy), 2017, fy, Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), sm, Some(false));
            add(&format!("tc_eng_med_{}", fy), 2017, fy, Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), med, Some(false));
            add(&format!("tc_eng_sm_lon_{}", fy), 2017, fy, Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(28000.00)), sm, Some(true));
            add(&format!("tc_eng_med_lon_{}", fy), 2017, fy, Jurisdiction::ENGLAND, "MEDIUM", dec!(28000.01), Some(dec!(100000.00)), med, Some(true));
            add(&format!("tc_eng_lg_{}", fy), 2017, fy, Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, lg, None);
        }

        // 2023 Revaluation List — England (Non-Domestic Rating Chargeable Amounts Regs 2022 / S.I. 2022/1403)
        // 2023/24: Small (+5%), Medium (+15%), Large (+30%)
        add("tc_eng_sm_2023", 2023, "2023/24", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(5.00), Some(false));
        add("tc_eng_med_2023", 2023, "2023/24", Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(15.00), Some(false));
        add("tc_eng_sm_lon_2023", 2023, "2023/24", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(28000.00)), dec!(5.00), Some(true));
        add("tc_eng_med_lon_2023", 2023, "2023/24", Jurisdiction::ENGLAND, "MEDIUM", dec!(28000.01), Some(dec!(100000.00)), dec!(15.00), Some(true));
        add("tc_eng_lg_2023", 2023, "2023/24", Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, dec!(30.00), None);

        // 2024/25: Small (+10%), Medium (+25%), Large (+40%)
        add("tc_eng_sm_2024", 2023, "2024/25", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(10.00), Some(false));
        add("tc_eng_med_2024", 2023, "2024/25", Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(25.00), Some(false));
        add("tc_eng_sm_lon_2024", 2023, "2024/25", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(28000.00)), dec!(10.00), Some(true));
        add("tc_eng_med_lon_2024", 2023, "2024/25", Jurisdiction::ENGLAND, "MEDIUM", dec!(28000.01), Some(dec!(100000.00)), dec!(25.00), Some(true));
        add("tc_eng_lg_2024", 2023, "2024/25", Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, dec!(40.00), None);

        // 2025/26: Small (+25%), Medium (+40%), Large (+55%)
        add("tc_eng_sm_2025", 2023, "2025/26", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(25.00), Some(false));
        add("tc_eng_med_2025", 2023, "2025/26", Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(40.00), Some(false));
        add("tc_eng_sm_lon_2025", 2023, "2025/26", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(28000.00)), dec!(25.00), Some(true));
        add("tc_eng_med_lon_2025", 2023, "2025/26", Jurisdiction::ENGLAND, "MEDIUM", dec!(28000.01), Some(dec!(100000.00)), dec!(40.00), Some(true));
        add("tc_eng_lg_2025", 2023, "2025/26", Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, dec!(55.00), None);

        // 2026 Revaluation List (NDRA 2023 3-year cycle) — England 2026/27: Small (+5%), Medium (+15%), Large (+30%)
        add("tc_eng_sm_2026", 2026, "2026/27", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(5.00), Some(false));
        add("tc_eng_med_2026", 2026, "2026/27", Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(15.00), Some(false));
        add("tc_eng_sm_lon_2026", 2026, "2026/27", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(28000.00)), dec!(5.00), Some(true));
        add("tc_eng_med_lon_2026", 2026, "2026/27", Jurisdiction::ENGLAND, "MEDIUM", dec!(28000.01), Some(dec!(100000.00)), dec!(15.00), Some(true));
        add("tc_eng_lg_2026", 2026, "2026/27", Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, dec!(30.00), None);

        // Wales Transitional Relief (Local Government Finance (Wales) Act 2024)
        add("tc_wal_sm_2026", 2026, "2026/27", Jurisdiction::WALES, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(10.00), None);
        add("tc_wal_med_2026", 2026, "2026/27", Jurisdiction::WALES, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(25.00), None);
        add("tc_wal_lg_2026", 2026, "2026/27", Jurisdiction::WALES, "LARGE", dec!(100000.01), None, dec!(40.00), None);

        // 2023 Revaluation List — Scotland (The Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2023 / SSI 2023/31)
        // 2023/24: Small (+12.5%), Medium (+25%), Large (+37.5%)
        add("tc_sco_sm_2023", 2023, "2023/24", Jurisdiction::SCOTLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(12.50), None);
        add("tc_sco_med_2023", 2023, "2023/24", Jurisdiction::SCOTLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(25.00), None);
        add("tc_sco_lg_2023", 2023, "2023/24", Jurisdiction::SCOTLAND, "LARGE", dec!(100000.01), None, dec!(37.50), None);

        // 2024/25: Small (+25%), Medium (+50%), Large (+75%)
        add("tc_sco_sm_2024", 2023, "2024/25", Jurisdiction::SCOTLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(25.00), None);
        add("tc_sco_med_2024", 2023, "2024/25", Jurisdiction::SCOTLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(50.00), None);
        add("tc_sco_lg_2024", 2023, "2024/25", Jurisdiction::SCOTLAND, "LARGE", dec!(100000.01), None, dec!(75.00), None);

        // 2025/26: Small (+37.5%), Medium (+75%), Large (+112.5%)
        add("tc_sco_sm_2025", 2023, "2025/26", Jurisdiction::SCOTLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(37.50), None);
        add("tc_sco_med_2025", 2023, "2025/26", Jurisdiction::SCOTLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(75.00), None);
        add("tc_sco_lg_2025", 2023, "2025/26", Jurisdiction::SCOTLAND, "LARGE", dec!(100000.01), None, dec!(112.50), None);

        // 2026 Revaluation List — Scotland (The Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2026 (SSI 2026/70) reg. 9)
        // 2026/27: Small (+15.00%, X=1.150), Medium (+30.00%, X=1.300), Large (+50.00%, X=1.500)
        add("tc_sco_sm_2026", 2026, "2026/27", Jurisdiction::SCOTLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(15.00), None);
        add("tc_sco_med_2026", 2026, "2026/27", Jurisdiction::SCOTLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(30.00), None);
        add("tc_sco_lg_2026", 2026, "2026/27", Jurisdiction::SCOTLAND, "LARGE", dec!(100000.01), None, dec!(50.00), None);
    }

    fn seed_reference_codes(&mut self) {
        let mut add = |category: &str, code: &str, label: &str, description: &str, jur: Option<&str>, order: i32| {
            self.reference_codes.push(ParamReferenceCode {
                category: category.to_string(),
                code: code.to_string(),
                label: label.to_string(),
                description: description.to_string(),
                jurisdiction: jur.map(|s| s.to_string()),
                display_order: order,
            });
        };

        // JURISDICTION
        add("JURISDICTION", "ENGLAND", "England", "Statutory non-domestic rating under LGFA 1988 & NDRA 2023. Standard multiplier applies for RV > £51,000.", Some("ENGLAND"), 1);
        add("JURISDICTION", "WALES", "Wales", "Devolved rating under Local Government Finance (Wales) Act 2024 & Welsh Statutory Instruments (WSI).", Some("WALES"), 2);
        add("JURISDICTION", "SCOTLAND", "Scotland", "Devolved rating under Non-Domestic Rates (Scotland) Act 2020. Three-tier multiplier structure and SBBS.", Some("SCOTLAND"), 3);

        // PARTY_TYPE (Common across GB)
        add("PARTY_TYPE", "CORPORATE", "Corporate Entity / Company", "A limited company (Ltd), PLC, LLP, or other incorporated commercial entity liable for non-domestic rates.", None, 1);
        add("PARTY_TYPE", "INDIVIDUAL", "Individual / Sole Proprietor", "A natural person, sole trader, or unincorporated commercial partnership.", None, 2);
        add("PARTY_TYPE", "CHARITY", "Registered Charity", "Registered charity or trustees holding property for charitable purposes, eligible for 80% mandatory relief under LGFA 1988 s.43(5).", None, 3);
        add("PARTY_TYPE", "CASC", "Community Amateur Sports Club", "HMRC-registered CASC eligible for 80% mandatory non-domestic rate relief under LGFA 1988 s.43(6).", None, 4);
        add("PARTY_TYPE", "LOCAL_AUTHORITY", "Local Authority / Public Body", "Billing authority, municipal council, or other statutory public body occupier.", None, 5);

        // OCCUPANCY_TYPE (Common across GB)
        add("OCCUPANCY_TYPE", "OCCUPIED", "Occupied Premises", "Hereditament is actively occupied for business/commercial operations (s.43 liability).", None, 1);
        add("OCCUPANCY_TYPE", "UNOCCUPIED", "Unoccupied / Empty", "Hereditament is vacant/empty (s.45 liability applies after statutory 3-month or 6-month exemption).", None, 2);
        add("OCCUPANCY_TYPE", "PART_OCCUPIED", "Partially Occupied", "Hereditament is temporarily partially vacant, subject to s.44A apportionment certificates.", None, 3);

        // SCAT_CODE (Special Category Codes)
        add("SCAT_CODE", "003", "003 - Airport / Major Aviation Infrastructure", "Airport / high-value complex requiring standard national multiplier. Explicitly excluded from small business relief.", None, 1);
        add("SCAT_CODE", "091", "091 - General Offices / Commercial Premises", "Standard commercial office accommodation. Entitled to 3 months 100% empty property relief.", None, 2);
        add("SCAT_CODE", "096", "096 - Industrial Hereditament / Warehouse / Factory", "Industrial property qualifying for 6 months 100% empty property exemption under S.I. 2008/386.", None, 3);
        add("SCAT_CODE", "209", "209 - Public House / Bar / Licensed Premises", "Licensed premises eligible for Retail, Hospitality & Leisure (RHL) relief schemes.", None, 4);
        add("SCAT_CODE", "211", "211 - Petrol Filling Station & Forecourt", "Automotive fuel sales and forecourt shop. Explicitly excluded from retail discount in Wales under 2026/27 rules.", None, 5);
        add("SCAT_CODE", "225", "225 - Retail Shop / High Street Unit", "Standard retail shop or store eligible for SBRR and RHL discount schemes.", None, 6);
        add("SCAT_CODE", "249", "249 - Self-Catering Holiday Accommodation", "Holiday let subject to statutory minimum letting criteria (minimum 70 actual days let / 140 days available in Scotland).", None, 7);
        add("SCAT_CODE", "084", "084 - Day Nursery / Crèche / Registered Childcare", "Premises wholly used for childcare eligible for 100% SBRR up to £100,000 RV in Wales under W.S.I. 2017/1229 art. 8.", None, 8);
        add("SCAT_CODE", "227", "227 - Post Office", "Post office premises eligible for 100% SBRR up to £9,000 RV and 50% relief up to £12,000 RV in Wales under W.S.I. 2017/1229 art. 9.", None, 9);
        add("SCAT_CODE", "024", "024 - Beach Hut", "Beach hut. Explicitly excepted from Welsh small business rates relief under W.S.I. 2017/1229 art. 2.", None, 10);
        add("SCAT_CODE", "050", "050 - Car Park (Surface / Multi-Storey)", "Vehicle parking premises. Explicitly excepted from Welsh small business rates relief under W.S.I. 2017/1229 art. 2.", None, 11);
        add("SCAT_CODE", "027", "027 - Betting Office / Shop", "Betting premises. Explicitly excluded from Scottish Small Business Bonus Scheme under S.S.I. 2023/30 reg. 3(5).", None, 12);

        // RELIEF_CODE
        add("RELIEF_CODE", "SBRR", "Small Business Rate Relief", "100% discount up to £12,000 RV, with linear taper up to £15,000 RV for single property occupiers (England).", Some("ENGLAND"), 1);
        add("RELIEF_CODE", "WALES_SBRR", "Small Business Rate Relief (Wales)", "100% relief up to £6,000 RV, with linear taper up to £12,000 RV (childcare 100% to £100,000 RV; post office 100%/50%) under LGFA 1988 Sched 4ZA para 4 / W.S.I. 2017/1229.", Some("WALES"), 2);
        add("RELIEF_CODE", "SBBS", "Small Business Bonus Scheme", "Scottish rating relief for small businesses: 100% up to £12,000 RV, tapering to 25% at £15,000 RV and 0% at £20,000 RV (£35,000 cumulative cap) under S.S.I. 2023/30 / S.S.I. 2026/68.", Some("SCOTLAND"), 3);
        add("RELIEF_CODE", "RHL", "Retail, Hospitality & Leisure Relief", "Statutory sector discount (40% in England 2025/26, expired 2026/27; 15% in Wales 2026/27; 15% mainland / 100% islands in Scotland 2026/27 up to £110,000 cash cap).", None, 4);
        add("RELIEF_CODE", "CHARITY", "Mandatory Charitable Relief", "80% mandatory relief under LGFA 1988 s.43(5) where property is used wholly or mainly for charitable purposes.", None, 5);
        add("RELIEF_CODE", "IMPROV", "Improvement Relief (G-Factor)", "100% relief for 12 months on rateable value increases resulting from physical property improvements under NDRA 2023 (England).", Some("ENGLAND"), 6);
        add("RELIEF_CODE", "EMPTY", "Empty Property Exemption", "100% relief for initial 3 months (commercial) or 6 months (industrial) unoccupied periods under LGFA 1988 s.45.", None, 7);
        add("RELIEF_CODE", "RURAL", "Rural Settlement Relief", "100% mandatory relief for sole general stores, food shops, or post offices in designated rural settlements.", None, 8);
        add("RELIEF_CODE", "LAVATORY", "Public Lavatories Relief", "100% statutory rating exemption under Rating (Coronavirus and Public Lavatories) Act 2021.", Some("ENGLAND"), 9);
        add("RELIEF_CODE", "HEAT_NET", "Heat Network Relief", "100% relief for low-carbon district heating distribution hereditaments under NDRA 2023 (England).", Some("ENGLAND"), 10);
        add("RELIEF_CODE", "WELSH_AVOIDANCE", "Anti-Avoidance Statutory Penalty", "Statutory penalty under Welsh Statutory Instrument 2026/43 (£500 fixed + 3% rateable value) for artificial avoidance.", Some("WALES"), 11);
        add("RELIEF_CODE", "SBBS_TR", "Small Business Bonus Scheme Transitional Relief", "Scottish statutory protection capping net bill increases for properties losing or seeing reduced SBBS (£600 in 2023/24 to £1,800 in 2025/26; 25% bill increase limit in 2026/27).", Some("SCOTLAND"), 12);
        add("RELIEF_CODE", "SSBR", "Supporting Small Business Relief", "English statutory protection capping net bill increases for properties losing or seeing reduced SBRR, rural rate relief, or RHL relief (£600 annual increase cap in 2023–2025; £800 annual increase cap in 2026/27).", Some("ENGLAND"), 13);
        add("RELIEF_CODE", "WALES_TR", "Welsh Transitional Relief", "Statutory two-year transitional phasing scheme under W.S.I. 2025/1371 / W.S.I. 2022/1350 (67% relief on bill increases > £300 in Year 1, 34% in Year 2).", Some("WALES"), 14);
        add("RELIEF_CODE", "TELECOM", "Telecommunications Infrastructure Relief", "100% statutory relief for new fibre telecommunications infrastructure under Telecommunications Infrastructure Act 2018 / SSI 2019/111.", None, 15);
        add("RELIEF_CODE", "CHARITY_TOPUP", "Discretionary Charitable Relief Top-Up", "Up to 20% discretionary top-up relief by billing authorities under LGFA 1988 s.47(2)(a) / 1962 Act s.4(5).", None, 16);
        add("RELIEF_CODE", "RENEWABLE", "Renewable Energy Generation Relief", "Tiered relief for renewable energy generation hereditaments under SSI 2010/336 (100% up to £145k RV, scaling to 10% up to £1m RV).", Some("SCOTLAND"), 17);
        add("RELIEF_CODE", "COUNCIL_TAX_TRANSFER", "Council Tax Transfer (Holiday Let Disqualification)", "Disqualification from non-domestic rating list where self-catering accommodation fails the statutory letting days test (Scotland from 2022/23: 70 days let, 140 available; Wales from 2023/24: 182 let, 252 available; England: 70 let, 140 available).", None, 18);

        // MULTIPLIER_TYPE
        add("MULTIPLIER_TYPE", "STANDARD", "Standard Multiplier", "National baseline non-domestic rating multiplier.", None, 1);
        add("MULTIPLIER_TYPE", "SMALL_BUSINESS", "Small Business Multiplier", "Reduced multiplier for qualifying small businesses.", None, 2);
        add("MULTIPLIER_TYPE", "HIGH_VALUE", "High-Value Multiplier", "Higher rating multiplier for high-value properties (e.g. RV >= £500,000 in England).", None, 3);
        add("MULTIPLIER_TYPE", "SMALL_BUSINESS_RHL", "Small Business RHL Multiplier", "Permanent reduced multiplier for qualifying retail, hospitality, leisure under £51k RV in England.", Some("ENGLAND"), 4);
        add("MULTIPLIER_TYPE", "STANDARD_RHL", "Standard RHL Multiplier", "Permanent reduced multiplier for qualifying retail, hospitality, leisure £51k-£500k RV in England.", Some("ENGLAND"), 5);
        add("MULTIPLIER_TYPE", "RETAIL", "Welsh Retail Multiplier", "Lower statutory multiplier for small retail premises under £51k RV in Wales under LGF(W)A 2024 (35.0p in 2026/27).", Some("WALES"), 6);
        add("MULTIPLIER_TYPE", "HIGHER", "Higher Multiplier (Wales)", "Higher statutory multiplier for properties exceeding £100k RV in Wales under LGF(W)A 2024 (51.5p in 2026/27).", Some("WALES"), 7);
    }

    fn seed_empty_property_thresholds(&mut self) {
        let mut add = |id: &str, jur: Jurisdiction, threshold: Decimal, auth: &str, start: &str, end: &str| {
            self.empty_property_thresholds.push(ParamEmptyPropertyThreshold {
                parameter_id: id.to_string(),
                jurisdiction: jur,
                threshold_rv: threshold,
                statutory_authority: auth.to_string(),
                effective_from: NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap(),
                effective_to: NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap(),
            });
        };

        // Scotland: £1,700 (Local Government (Scotland) Act 1975 s.7A / S.S.I. 2018/77)
        add("ept_sco_default", Jurisdiction::SCOTLAND, dec!(1700.00), "Local Government (Scotland) Act 1975 s.7A / S.S.I. 2018/77", "1990-04-01", "2099-12-31");

        // Wales: £2,200 (LGFA 1988 s.45 / W.S.I. 2008/2499 reg. 4(g))
        add("ept_wal_default", Jurisdiction::WALES, dec!(2200.00), "LGFA 1988 s.45 / W.S.I. 2008/2499 reg. 4(g)", "1990-04-01", "2099-12-31");

        // England: Historical thresholds
        // Pre-2009/10: £2,200 (LGFA 1988 s.45 / S.I. 2008/386 reg. 4(g))
        add("ept_eng_pre2009", Jurisdiction::ENGLAND, dec!(2200.00), "LGFA 1988 s.45 / S.I. 2008/386 reg. 4(g)", "1990-04-01", "2009-03-31");
        // 2009/10: £15,000 (LGFA 1988 s.45 / S.I. 2008/3283 reg. 2)
        add("ept_eng_2009", Jurisdiction::ENGLAND, dec!(15000.00), "LGFA 1988 s.45 / S.I. 2008/3283 reg. 2", "2009-04-01", "2010-03-31");
        // 2010/11: £18,000 (LGFA 1988 s.45 / S.I. 2010/408 reg. 2(2)(b))
        add("ept_eng_2010", Jurisdiction::ENGLAND, dec!(18000.00), "LGFA 1988 s.45 / S.I. 2010/408 reg. 2(2)(b)", "2010-04-01", "2011-03-31");
        // 2011/12 - 2016/17: £2,600 (LGFA 1988 s.45 / S.I. 2010/408 reg. 2(2)(a))
        add("ept_eng_2011", Jurisdiction::ENGLAND, dec!(2600.00), "LGFA 1988 s.45 / S.I. 2010/408 reg. 2(2)(a)", "2011-04-01", "2017-03-31");
        // 2017/18 onwards: £2,900 (LGFA 1988 s.45 / S.I. 2017/102 art. 4)
        add("ept_eng_2017", Jurisdiction::ENGLAND, dec!(2900.00), "LGFA 1988 s.45 / S.I. 2017/102 art. 4", "2017-04-01", "2099-12-31");
    }

    fn seed_letting_tests(&mut self) {
        let mut add = |id: &str, jur: Jurisdiction, let_days: i64, avail_days: i64, auth: &str, start: &str, end: &str| {
            self.letting_tests.push(ParamLettingTest {
                parameter_id: id.to_string(),
                jurisdiction: jur,
                min_actual_let_days: let_days,
                min_available_days: avail_days,
                statutory_authority: auth.to_string(),
                effective_from: NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap(),
                effective_to: NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap(),
            });
        };

        // Wales
        add("let_wal_pre2010", Jurisdiction::WALES, 0, 140, "LGFA 1988 s.66(2B)", "1990-04-01", "2010-03-31");
        add("let_wal_2010", Jurisdiction::WALES, 70, 140, "LGFA 1988 s.66(2B) / W.S.I. 2010/682", "2010-04-01", "2023-03-31");
        add("let_wal_2023", Jurisdiction::WALES, 182, 252, "LGFA 1988 s.66(2BB) / W.S.I. 2022/563", "2023-04-01", "2099-12-31");

        // Scotland
        add("let_sco_pre2022", Jurisdiction::SCOTLAND, 0, 140, "Council Tax (Dwellings and Part Residential Subjects) (Scotland) Regs 1992 sched 2 para 2(b)", "1990-04-01", "2022-03-31");
        add("let_sco_2022", Jurisdiction::SCOTLAND, 70, 140, "Council Tax (Dwellings and Part Residential Subjects) (Scotland) Regs 1992 sched 2 para 2(b) / S.S.I. 2021/489 reg 2(4)", "2022-04-01", "2099-12-31");

        // England
        add("let_eng_pre2023", Jurisdiction::ENGLAND, 0, 140, "LGFA 1988 s.66(2B)", "1990-04-01", "2023-03-31");
        add("let_eng_2023", Jurisdiction::ENGLAND, 70, 140, "LGFA 1988 s.66(2B) / S.I. 2022/217", "2023-04-01", "2099-12-31");
    }

    fn seed_scheme_caps(&mut self) {
        let mut add = |id: &str, scheme: &str, fy: Option<&str>, jur: Option<Jurisdiction>, cap: Decimal, sec_cap: Option<Decimal>, unit: &str, auth: &str, start: Option<&str>, end: Option<&str>| {
            self.scheme_caps.push(ParamSchemeCap {
                parameter_id: id.to_string(),
                scheme_code: scheme.to_string(),
                financial_year: fy.map(|s| s.to_string()),
                jurisdiction: jur,
                cap_value: cap,
                secondary_cap_value: sec_cap,
                cap_unit: unit.to_string(),
                statutory_authority: auth.to_string(),
                effective_from: start.map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()),
                effective_to: end.map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()),
            });
        };

        // Rural Settlement Relief Caps (LGFA 1988 s.43(6B) / LGFA 1997 / S.I. 1997/1888)
        add("cap_rural_pub_petrol", "RURAL_PUB_PETROL", None, None, dec!(12500.00), None, "GBP_RV", "LGFA 1988 s.43(6B) / LGFA 1997", None, None);
        add("cap_rural_general", "RURAL_GENERAL", None, None, dec!(8500.00), None, "GBP_RV", "LGFA 1988 s.43(6B) / LGFA 1997", None, None);

        // Renewable Energy Generation Relief (Scotland) (SSI 2010/336 reg. 3)
        add("cap_renew_tier1", "RENEWABLE_ENERGY_SCOTLAND", None, Some(Jurisdiction::SCOTLAND), dec!(145000.00), Some(dec!(100.00)), "GBP_RV", "SSI 2010/336 reg. 3", None, None);
        add("cap_renew_tier2", "RENEWABLE_ENERGY_SCOTLAND", None, Some(Jurisdiction::SCOTLAND), dec!(200000.00), Some(dec!(50.00)), "GBP_RV", "SSI 2010/336 reg. 3", None, None);
        add("cap_renew_tier3", "RENEWABLE_ENERGY_SCOTLAND", None, Some(Jurisdiction::SCOTLAND), dec!(250000.00), Some(dec!(25.00)), "GBP_RV", "SSI 2010/336 reg. 3", None, None);
        add("cap_renew_tier4", "RENEWABLE_ENERGY_SCOTLAND", None, Some(Jurisdiction::SCOTLAND), dec!(1000000.00), Some(dec!(10.00)), "GBP_RV", "SSI 2010/336 reg. 3", None, None);

        // Occupancy Reset Days for Empty Property Relief
        // England post-1 April 2024: 91 days (13 weeks) under NDRA 2023 s.15 / S.I. 2024/412
        add("cap_reset_eng_2024", "EMPTY_OCCUPIED_RESET", None, Some(Jurisdiction::ENGLAND), dec!(91.00), None, "DAYS", "NDRA 2023 s.15 / S.I. 2024/412", Some("2024-04-01"), Some("2099-12-31"));
        // England pre-2024: 42 days (6 weeks) under LGFA 1988 s.45A / S.I. 2008/386 reg. 4(d)
        add("cap_reset_eng_pre2024", "EMPTY_OCCUPIED_RESET", None, Some(Jurisdiction::ENGLAND), dec!(42.00), None, "DAYS", "LGFA 1988 s.45A / S.I. 2008/386 reg. 4(d)", Some("1990-04-01"), Some("2024-03-31"));
        // Wales: 42 days (6 weeks) under LGFA 1988 s.45A / W.S.I. 2008/2499 reg. 4(d)
        add("cap_reset_wal", "EMPTY_OCCUPIED_RESET", None, Some(Jurisdiction::WALES), dec!(42.00), None, "DAYS", "LGFA 1988 s.45A / W.S.I. 2008/2499 reg. 4(d)", Some("1990-04-01"), Some("2099-12-31"));
        // Scotland: 42 days (6 weeks) under Local Government (Scotland) Act 1975 s.7A / S.S.I. 2008/77
        add("cap_reset_sco", "EMPTY_OCCUPIED_RESET", None, Some(Jurisdiction::SCOTLAND), dec!(42.00), None, "DAYS", "Local Government (Scotland) Act 1975 s.7A / S.S.I. 2008/77", Some("1990-04-01"), Some("2099-12-31"));

        // Transitional Relief Supplement (TRS) Addition in England (S.I. 2026/81 reg. 16)
        add("cap_trs_eng_2026", "TRANSITIONAL_RELIEF_SUPPLEMENT", Some("2026/27"), Some(Jurisdiction::ENGLAND), dec!(0.010), None, "MULTIPLIER_ADDITION", "Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2026 (S.I. 2026/81) reg. 16", Some("2026-04-01"), Some("2027-03-31"));

        // Welsh Anti-Avoidance Statutory Penalty (W.S.I. 2026/43 / LGF(W)A 2024)
        add("cap_welsh_avoidance", "WELSH_AVOIDANCE_PENALTY", None, Some(Jurisdiction::WALES), dec!(500.00), Some(dec!(0.03)), "GBP_AND_RATE", "NDR (Artificial Avoidance Arrangements) (Wales) Regs 2026 (WSI 2026/43)", None, None);

        // Welsh SBRR Special Scheme Caps (W.S.I. 2017/1229)
        add("cap_wal_sbrr_childcare", "WELSH_SBRR_CHILDCARE", None, Some(Jurisdiction::WALES), dec!(100000.00), Some(dec!(100.00)), "GBP_RV", "W.S.I. 2017/1229 art. 8 / W.S.I. 2018/1193", Some("2019-04-01"), Some("2099-12-31"));
        add("cap_wal_sbrr_po_full", "WELSH_SBRR_POST_OFFICE_FULL", None, Some(Jurisdiction::WALES), dec!(9000.00), Some(dec!(100.00)), "GBP_RV", "W.S.I. 2017/1229 art. 9 & 11(2)", None, None);
        add("cap_wal_sbrr_po_half", "WELSH_SBRR_POST_OFFICE_HALF", None, Some(Jurisdiction::WALES), dec!(12000.00), Some(dec!(50.00)), "GBP_RV", "W.S.I. 2017/1229 art. 9 & 11(2)", None, None);

        // Scottish SBBS Transitional Relief Caps (SSI 2023/31 / SSI 2026/70)
        add("cap_sbbs_tr_2023", "SBBS_TR", Some("2023/24"), Some(Jurisdiction::SCOTLAND), dec!(600.00), None, "FIXED_CASH", "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2023 (SSI 2023/31) Part 3", Some("2023-04-01"), Some("2024-03-31"));
        add("cap_sbbs_tr_2024", "SBBS_TR", Some("2024/25"), Some(Jurisdiction::SCOTLAND), dec!(1200.00), None, "FIXED_CASH", "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2023 (SSI 2023/31) Part 3", Some("2024-04-01"), Some("2025-03-31"));
        add("cap_sbbs_tr_2025", "SBBS_TR", Some("2025/26"), Some(Jurisdiction::SCOTLAND), dec!(1800.00), None, "FIXED_CASH", "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2023 (SSI 2023/31) Part 3", Some("2025-04-01"), Some("2026-03-31"));
        add("cap_sbbs_tr_2026", "SBBS_TR", Some("2026/27"), Some(Jurisdiction::SCOTLAND), dec!(25.00), None, "PERCENTAGE_INCREASE", "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2026 (SSI 2026/70) reg. 14", Some("2026-04-01"), Some("2027-03-31"));
        add("cap_sbbs_tr_2027", "SBBS_TR", Some("2027/28"), Some(Jurisdiction::SCOTLAND), dec!(50.00), None, "PERCENTAGE_INCREASE", "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2026 (SSI 2026/70) reg. 14", Some("2027-04-01"), Some("2028-03-31"));
        add("cap_sbbs_tr_2028", "SBBS_TR", Some("2028/29"), Some(Jurisdiction::SCOTLAND), dec!(75.00), None, "PERCENTAGE_INCREASE", "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2026 (SSI 2026/70) reg. 14", Some("2028-04-01"), Some("2029-03-31"));

        // English Supporting Small Business Relief (SSBR) Caps (LGFA 1988 s.47)
        add("cap_ssbr_2023", "SSBR", Some("2023/24"), Some(Jurisdiction::ENGLAND), dec!(600.00), None, "GBP_PER_YEAR", "LGFA 1988 s.47 / Supporting Small Business Scheme (2023 Revaluation)", Some("2023-04-01"), Some("2024-03-31"));
        add("cap_ssbr_2024", "SSBR", Some("2024/25"), Some(Jurisdiction::ENGLAND), dec!(600.00), None, "GBP_PER_YEAR", "LGFA 1988 s.47 / Supporting Small Business Scheme (2023 Revaluation)", Some("2024-04-01"), Some("2025-03-31"));
        add("cap_ssbr_2025", "SSBR", Some("2025/26"), Some(Jurisdiction::ENGLAND), dec!(600.00), None, "GBP_PER_YEAR", "LGFA 1988 s.47 / Supporting Small Business Scheme (2023 Revaluation)", Some("2025-04-01"), Some("2026-03-31"));
        add("cap_ssbr_2026", "SSBR", Some("2026/27"), Some(Jurisdiction::ENGLAND), dec!(800.00), None, "GBP_PER_YEAR", "LGFA 1988 s.47 / Supporting Small Business Scheme (2026 Revaluation)", Some("2026-04-01"), Some("2027-03-31"));

        // Welsh Transitional Relief Percentages (LGFA 1988 s.58 / W.S.I. 2025/1371 / W.S.I. 2022/1350 / W.S.I. 2016/1247)
        add("cap_wal_tr_2017", "WALES_TR", Some("2017/18"), Some(Jurisdiction::WALES), dec!(75.00), None, "PERCENTAGE", "LGFA 1988 s.58 / W.S.I. 2016/1247 (W. 295)", Some("2017-04-01"), Some("2018-03-31"));
        add("cap_wal_tr_2018", "WALES_TR", Some("2018/19"), Some(Jurisdiction::WALES), dec!(50.00), None, "PERCENTAGE", "LGFA 1988 s.58 / W.S.I. 2016/1247 (W. 295)", Some("2018-04-01"), Some("2019-03-31"));
        add("cap_wal_tr_2019", "WALES_TR", Some("2019/20"), Some(Jurisdiction::WALES), dec!(25.00), None, "PERCENTAGE", "LGFA 1988 s.58 / W.S.I. 2016/1247 (W. 295)", Some("2019-04-01"), Some("2020-03-31"));
        add("cap_wal_tr_2023", "WALES_TR", Some("2023/24"), Some(Jurisdiction::WALES), dec!(67.00), None, "PERCENTAGE", "LGFA 1988 s.58 / S.I. 2022/1350 (W. 272)", Some("2023-04-01"), Some("2024-03-31"));
        add("cap_wal_tr_2024", "WALES_TR", Some("2024/25"), Some(Jurisdiction::WALES), dec!(34.00), None, "PERCENTAGE", "LGFA 1988 s.58 / S.I. 2022/1350 (W. 272)", Some("2024-04-01"), Some("2025-03-31"));
        add("cap_wal_tr_2026", "WALES_TR", Some("2026/27"), Some(Jurisdiction::WALES), dec!(67.00), None, "PERCENTAGE", "LGFA 1988 s.58 / S.I. 2025/1371 (W. 226) / LGF(W)A 2024", Some("2026-04-01"), Some("2027-03-31"));
        add("cap_wal_tr_2027", "WALES_TR", Some("2027/28"), Some(Jurisdiction::WALES), dec!(34.00), None, "PERCENTAGE", "LGFA 1988 s.58 / S.I. 2025/1371 (W. 226) / LGF(W)A 2024", Some("2027-04-01"), Some("2028-03-31"));
    }
}
