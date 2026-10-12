use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::calculation::{MultiplierInfo, ParameterVersionIds};
use crate::models::enums::Jurisdiction;

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

/// In-memory parameter repository seeded with GB rating parameters (1990–2027)
#[derive(Clone, Debug)]
pub struct ParameterRepository {
    multipliers: Vec<ParamMultiplier>,
    sbrr_thresholds: Vec<ParamSbrrSbbsThreshold>,
    sector_reliefs: Vec<ParamSectorRelief>,
    transitional_caps: Vec<ParamTransitionalCap>,
    reference_codes: Vec<ParamReferenceCode>,
    versions: ParameterVersionIds,
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
        let versions = ParameterVersionIds {
            param_multiplier_version: "v2026.1".to_string(),
            param_sbrr_version: "v2026.1".to_string(),
            param_sector_relief_version: "v2026.2".to_string(),
        };

        let mut repo = Self {
            multipliers: Vec::new(),
            sbrr_thresholds: Vec::new(),
            sector_reliefs: Vec::new(),
            transitional_caps: Vec::new(),
            reference_codes: Vec::new(),
            versions,
        };

        repo.seed_multipliers();
        repo.seed_sbrr_thresholds();
        repo.seed_sector_reliefs();
        repo.seed_transitional_caps();
        repo.seed_reference_codes();

        repo
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
        self.transitional_caps.iter()
            .find(|c| {
                c.jurisdiction == jurisdiction 
                    && c.financial_year == financial_year 
                    && rv >= c.rv_threshold_min 
                    && c.rv_threshold_max.map_or(true, |max| rv <= max)
            })
    }

    /// Dynamically resolves the statutory multiplier by querying in-memory parameter tables
    pub fn resolve_multiplier(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> MultiplierInfo {
        self.resolve_multiplier_ex(jurisdiction, financial_year, rv, false)
    }

    /// Dynamically resolves the statutory multiplier, accounting for RHL sector qualifications in 2026/27 (5-multiplier system)
    pub fn resolve_multiplier_ex(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal, is_rhl: bool) -> MultiplierInfo {
        let matches: Vec<&ParamMultiplier> = self.multipliers.iter()
            .filter(|m| m.jurisdiction == jurisdiction && m.financial_year == financial_year)
            .collect();

        if matches.is_empty() {
            return MultiplierInfo {
                multiplier_type: "HISTORICAL_DEFAULT".to_string(),
                rate: dec!(0.450),
            };
        }

        // 1. High-Value Multiplier (NDRA 2023 - England 2026/27, RV >= £500,000)
        if let Some(m) = matches.iter().find(|m| m.multiplier_type == "HIGH_VALUE" && rv >= m.rv_threshold_min) {
            return MultiplierInfo {
                multiplier_type: m.multiplier_type.clone(),
                rate: m.multiplier_rate,
            };
        }

        // 1b. Wales 2026/27 Differential Multipliers (Local Government Finance (Wales) Act 2024)
        if jurisdiction == Jurisdiction::WALES && financial_year == "2026/27" {
            if rv > dec!(100000.00) {
                if let Some(m) = matches.iter().find(|m| m.multiplier_type == "HIGHER") {
                    return MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    };
                }
            } else if is_rhl && rv < dec!(51000.00) {
                if let Some(m) = matches.iter().find(|m| m.multiplier_type == "RETAIL") {
                    return MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    };
                }
            } else {
                if let Some(m) = matches.iter().find(|m| m.multiplier_type == "STANDARD") {
                    return MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    };
                }
            }
        }

        // 2. Retail, Hospitality & Leisure (RHL) permanent lower multipliers (England 2026/27)
        if is_rhl {
            if let Some(m) = matches.iter().find(|m| {
                (m.multiplier_type == "SMALL_BUSINESS_RHL" || m.multiplier_type == "STANDARD_RHL")
                    && rv >= m.rv_threshold_min
                    && m.rv_threshold_max.map_or(true, |max| rv <= max)
            }) {
                return MultiplierInfo {
                    multiplier_type: m.multiplier_type.clone(),
                    rate: m.multiplier_rate,
                };
            }
        }

        // 3. Small Business / Basic rate threshold check
        for m in &matches {
            if m.multiplier_type == "SMALL_BUSINESS" || m.multiplier_type == "BASIC" {
                let min_ok = rv >= m.rv_threshold_min;
                let max_ok = m.rv_threshold_max.map_or(true, |max| rv <= max);
                if min_ok && max_ok {
                    return MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    };
                }
            }
        }

        // 4. Intermediate / Welsh Differential check
        for m in &matches {
            if m.multiplier_type == "INTERMEDIATE" || m.multiplier_type == "WELSH_DIFFERENTIAL" {
                let min_ok = rv >= m.rv_threshold_min;
                let max_ok = m.rv_threshold_max.map_or(true, |max| rv <= max);
                if min_ok && max_ok {
                    return MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    };
                }
            }
        }

        // 4b. Higher / High-Value rate threshold check (e.g. Scotland Higher Multiplier for RV > £100,000, Wales HIGHER, England HIGH_VALUE)
        for m in &matches {
            if m.multiplier_type == "HIGHER" || m.multiplier_type == "HIGH_VALUE" {
                let min_ok = rv >= m.rv_threshold_min;
                let max_ok = m.rv_threshold_max.map_or(true, |max| rv <= max);
                if min_ok && max_ok {
                    return MultiplierInfo {
                        multiplier_type: m.multiplier_type.clone(),
                        rate: m.multiplier_rate,
                    };
                }
            }
        }

        // 5. Fallback to Standard or first available multiplier in matches
        let standard = matches.iter()
            .find(|m| m.multiplier_type == "STANDARD" || m.multiplier_type == "WELSH_DIFFERENTIAL")
            .unwrap_or(&matches[0]);

        MultiplierInfo {
            multiplier_type: standard.multiplier_type.clone(),
            rate: standard.multiplier_rate,
        }
    }

    /// Resolves SBRR / SBBS discount percentage and statutory authority
    pub fn resolve_sbrr(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> Option<SbrrResolution> {
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
                        if financial_year >= "2024/25" {
                            "Local Government Finance (Wales) Act 2024"
                        } else if financial_year >= "2018/19" {
                            "WSI 2017/1229 (W.293)"
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

    /// Resolves Sector Relief (e.g. Retail, Hospitality & Leisure)
    pub fn resolve_sector_relief(
        &self,
        jurisdiction: Jurisdiction,
        financial_year: &str,
        is_rhl: bool,
        is_island: bool,
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
                    "Local Government (Financial Provisions) (Scotland) Act 1962 s.4 / Islands (Scotland) Act 2018".to_string()
                } else {
                    "Local Government (Financial Provisions) (Scotland) Act 1962 s.4".to_string()
                }
            }
        };

        let relief_name = match jurisdiction {
            Jurisdiction::SCOTLAND => {
                if is_island {
                    "Scottish Islands Retail, Hospitality & Leisure Relief".to_string()
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
        let (cap_type, authority) = match financial_year {
            "2023/24" => (
                SbbsTrCapType::FixedCash(dec!(600.00)),
                "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2023 (SSI 2023/31) Part 3",
            ),
            "2024/25" => (
                SbbsTrCapType::FixedCash(dec!(1200.00)),
                "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2023 (SSI 2023/31) Part 3",
            ),
            "2025/26" => (
                SbbsTrCapType::FixedCash(dec!(1800.00)),
                "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2023 (SSI 2023/31) Part 3",
            ),
            "2026/27" => (
                SbbsTrCapType::PercentageOfIncrease(dec!(25.00)),
                "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2026 (SSI 2026/50) Part 3",
            ),
            "2027/28" => (
                SbbsTrCapType::PercentageOfIncrease(dec!(50.00)),
                "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2026 (SSI 2026/50) Part 3",
            ),
            "2028/29" => (
                SbbsTrCapType::PercentageOfIncrease(dec!(75.00)),
                "Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2026 (SSI 2026/50) Part 3",
            ),
            _ => return None,
        };

        Some(SbbsTrResolution {
            relief_code: "SBBS_TR".to_string(),
            relief_name: "Small Business Bonus Scheme Transitional Relief".to_string(),
            statutory_authority: authority.to_string(),
            cap_type,
        })
    }

    /// Resolves English Supporting Small Business Relief (SSBR) under LGFA 1988 s.47
    pub fn resolve_ssbr(&self, financial_year: &str) -> Option<SsbrResolution> {
        let (annual_cap, authority) = match financial_year {
            "2023/24" | "2024/25" | "2025/26" => (
                dec!(600.00),
                "LGFA 1988 s.47 / Supporting Small Business Scheme (2023 Revaluation)",
            ),
            "2026/27" => (
                dec!(800.00),
                "LGFA 1988 s.47 / Supporting Small Business Scheme (2026 Revaluation)",
            ),
            _ => return None,
        };

        Some(SsbrResolution {
            relief_code: "SSBR".to_string(),
            relief_name: "Supporting Small Business Relief".to_string(),
            statutory_authority: authority.to_string(),
            annual_cap,
        })
    }

    /// Resolves Welsh Transitional Relief scheme parameters under W.S.I. 2025/1371 / W.S.I. 2022/1350 / W.S.I. 2016/1243 / W.S.I. 2009/3364 / W.S.I. 2004/3400
    pub fn resolve_welsh_transitional_relief_rate(&self, financial_year: &str) -> Option<(Decimal, &'static str)> {
        match financial_year {
            "2026/27" => Some((dec!(67.00), "LGFA 1988 s.58 / S.I. 2025/1371 (W. 226) / LGF(W)A 2024")),
            "2027/28" => Some((dec!(34.00), "LGFA 1988 s.58 / S.I. 2025/1371 (W. 226) / LGF(W)A 2024")),
            "2023/24" => Some((dec!(67.00), "LGFA 1988 s.58 / S.I. 2022/1350 (W. 272)")),
            "2024/25" => Some((dec!(34.00), "LGFA 1988 s.58 / S.I. 2022/1350 (W. 272)")),
            "2017/18" => Some((dec!(67.00), "LGFA 1988 s.58 / W.S.I. 2016/1243 (W. 297)")),
            "2018/19" => Some((dec!(34.00), "LGFA 1988 s.58 / W.S.I. 2016/1243 (W. 297)")),
            "2010/11" => Some((dec!(67.00), "LGFA 1988 s.58 / W.S.I. 2009/3364 (W. 299)")),
            "2011/12" => Some((dec!(34.00), "LGFA 1988 s.58 / W.S.I. 2009/3364 (W. 299)")),
            "2005/06" => Some((dec!(67.00), "LGFA 1988 s.58 / W.S.I. 2004/3400 (W. 295)")),
            "2006/07" => Some((dec!(34.00), "LGFA 1988 s.58 / W.S.I. 2004/3400 (W. 295)")),
            _ => None,
        }
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
            add(&format!("m_wal_{}", fy), fy, Jurisdiction::WALES, "STANDARD", dec!(0.00), None, st, s, e);
            add(&format!("m_sco_{}", fy), fy, Jurisdiction::SCOTLAND, "STANDARD", dec!(0.00), None, sm, s, e);
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
            add(&format!("m_eng_sm_{}", fy), fy, Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(51000.00)), eng_sm, s, e);
            add(&format!("m_eng_st_{}", fy), fy, Jurisdiction::ENGLAND, "STANDARD", dec!(51000.01), None, eng_st, s, e);
            add(&format!("m_wal_{}", fy), fy, Jurisdiction::WALES, "STANDARD", dec!(0.00), None, wal, s, e);
            add(&format!("m_sco_{}", fy), fy, Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), sco, s, e);
        }

        // 2023/24
        add("m_eng_sm_2023", "2023/24", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(51000.00)), dec!(0.499), "2023-04-01", "2024-03-31");
        add("m_eng_st_2023", "2023/24", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.01), None, dec!(0.512), "2023-04-01", "2024-03-31");
        add("m_wal_2023", "2023/24", Jurisdiction::WALES, "STANDARD", dec!(0.00), None, dec!(0.535), "2023-04-01", "2024-03-31");
        add("m_sco_b_2023", "2023/24", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), dec!(0.498), "2023-04-01", "2024-03-31");
        add("m_sco_i_2023", "2023/24", Jurisdiction::SCOTLAND, "INTERMEDIATE", dec!(51000.01), Some(dec!(100000.00)), dec!(0.511), "2023-04-01", "2024-03-31");
        add("m_sco_h_2023", "2023/24", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.524), "2023-04-01", "2024-03-31");

        // 2024/25
        add("m_eng_sm_2024", "2024/25", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(51000.00)), dec!(0.499), "2024-04-01", "2025-03-31");
        add("m_eng_st_2024", "2024/25", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.01), None, dec!(0.546), "2024-04-01", "2025-03-31");
        add("m_wal_2024", "2024/25", Jurisdiction::WALES, "STANDARD", dec!(0.00), None, dec!(0.562), "2024-04-01", "2025-03-31");
        add("m_sco_b_2024", "2024/25", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), dec!(0.498), "2024-04-01", "2025-03-31");
        add("m_sco_i_2024", "2024/25", Jurisdiction::SCOTLAND, "INTERMEDIATE", dec!(51000.01), Some(dec!(100000.00)), dec!(0.545), "2024-04-01", "2025-03-31");
        add("m_sco_h_2024", "2024/25", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.559), "2024-04-01", "2025-03-31");

        // 2025/26
        add("m_eng_sm_2025", "2025/26", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(51000.00)), dec!(0.499), "2025-04-01", "2026-03-31");
        add("m_eng_st_2025", "2025/26", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.01), None, dec!(0.546), "2025-04-01", "2026-03-31");
        add("m_wal_2025", "2025/26", Jurisdiction::WALES, "STANDARD", dec!(0.00), None, dec!(0.562), "2025-04-01", "2026-03-31");
        add("m_sco_b_2025", "2025/26", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(51000.00)), dec!(0.498), "2025-04-01", "2026-03-31");
        add("m_sco_i_2025", "2025/26", Jurisdiction::SCOTLAND, "INTERMEDIATE", dec!(51000.01), Some(dec!(100000.00)), dec!(0.545), "2025-04-01", "2026-03-31");
        add("m_sco_h_2025", "2025/26", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.559), "2025-04-01", "2026-03-31");

        // 2026/27 (Revaluation Year — NDRA 2023 5-multiplier system for England)
        add("m_eng_hv_2026", "2026/27", Jurisdiction::ENGLAND, "HIGH_VALUE", dec!(500000.00), None, dec!(0.508), "2026-04-01", "2027-03-31");
        add("m_eng_sm_rhl_2026", "2026/27", Jurisdiction::ENGLAND, "SMALL_BUSINESS_RHL", dec!(0.00), Some(dec!(51000.00)), dec!(0.382), "2026-04-01", "2027-03-31");
        add("m_eng_sm_2026", "2026/27", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(51000.00)), dec!(0.432), "2026-04-01", "2027-03-31");
        add("m_eng_st_rhl_2026", "2026/27", Jurisdiction::ENGLAND, "STANDARD_RHL", dec!(51000.01), Some(dec!(499999.99)), dec!(0.430), "2026-04-01", "2027-03-31");
        add("m_eng_st_2026", "2026/27", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.01), Some(dec!(499999.99)), dec!(0.480), "2026-04-01", "2027-03-31");

        // 2026/27 (Revaluation Year — Local Government Finance (Wales) Act 2024 3-multiplier system for Wales)
        add("m_wal_ret_2026", "2026/27", Jurisdiction::WALES, "RETAIL", dec!(0.00), Some(dec!(51000.00)), dec!(0.350), "2026-04-01", "2027-03-31");
        add("m_wal_std_2026", "2026/27", Jurisdiction::WALES, "STANDARD", dec!(0.00), Some(dec!(100000.00)), dec!(0.502), "2026-04-01", "2027-03-31");
        add("m_wal_hgh_2026", "2026/27", Jurisdiction::WALES, "HIGHER", dec!(100000.01), None, dec!(0.515), "2026-04-01", "2027-03-31");

        add("m_sco_b_2026", "2026/27", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(12000.00)), dec!(0.498), "2026-04-01", "2027-03-31");
        add("m_sco_st_2026", "2026/27", Jurisdiction::SCOTLAND, "STANDARD", dec!(12000.01), Some(dec!(100000.00)), dec!(0.545), "2026-04-01", "2027-03-31");
        add("m_sco_h_2026", "2026/27", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.559), "2026-04-01", "2027-03-31");

        // 2027/28 (Revaluation List Year 2)
        add("m_eng_hv_2027", "2027/28", Jurisdiction::ENGLAND, "HIGH_VALUE", dec!(500000.00), None, dec!(0.508), "2027-04-01", "2028-03-31");
        add("m_eng_sm_rhl_2027", "2027/28", Jurisdiction::ENGLAND, "SMALL_BUSINESS_RHL", dec!(0.00), Some(dec!(51000.00)), dec!(0.382), "2027-04-01", "2028-03-31");
        add("m_eng_sm_2027", "2027/28", Jurisdiction::ENGLAND, "SMALL_BUSINESS", dec!(0.00), Some(dec!(51000.00)), dec!(0.432), "2027-04-01", "2028-03-31");
        add("m_eng_st_rhl_2027", "2027/28", Jurisdiction::ENGLAND, "STANDARD_RHL", dec!(51000.01), Some(dec!(499999.99)), dec!(0.430), "2027-04-01", "2028-03-31");
        add("m_eng_st_2027", "2027/28", Jurisdiction::ENGLAND, "STANDARD", dec!(51000.01), Some(dec!(499999.99)), dec!(0.480), "2027-04-01", "2028-03-31");

        add("m_wal_ret_2027", "2027/28", Jurisdiction::WALES, "RETAIL", dec!(0.00), Some(dec!(51000.00)), dec!(0.350), "2027-04-01", "2028-03-31");
        add("m_wal_std_2027", "2027/28", Jurisdiction::WALES, "STANDARD", dec!(0.00), Some(dec!(100000.00)), dec!(0.502), "2027-04-01", "2028-03-31");
        add("m_wal_hgh_2027", "2027/28", Jurisdiction::WALES, "HIGHER", dec!(100000.01), None, dec!(0.515), "2027-04-01", "2028-03-31");

        add("m_sco_b_2027", "2027/28", Jurisdiction::SCOTLAND, "BASIC", dec!(0.00), Some(dec!(12000.00)), dec!(0.498), "2027-04-01", "2028-03-31");
        add("m_sco_st_2027", "2027/28", Jurisdiction::SCOTLAND, "STANDARD", dec!(12000.01), Some(dec!(100000.00)), dec!(0.545), "2027-04-01", "2028-03-31");
        add("m_sco_h_2027", "2027/28", Jurisdiction::SCOTLAND, "HIGHER", dec!(100000.01), None, dec!(0.559), "2027-04-01", "2028-03-31");
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
        // 2026/27: 15% mainland (RV <= £100,000) or 100% on islands (Islands (Scotland) Act 2018), £110k cap
        add("sec_sco_rhl_mainland_2026", "2026/27", Jurisdiction::SCOTLAND, "RHL_SCOTLAND_MAINLAND_2026", dec!(15.00), Some(dec!(110000.00)), "2026-04-01", "2027-03-31");
        add("sec_sco_rhl_islands_2026", "2026/27", Jurisdiction::SCOTLAND, "RHL_SCOTLAND_ISLANDS_2026", dec!(100.00), Some(dec!(110000.00)), "2026-04-01", "2027-03-31");
    }

    fn seed_transitional_caps(&mut self) {
        let mut add = |id: &str, list_year: i32, fy: &str, jur: Jurisdiction, tier: &str, min: Decimal, max: Option<Decimal>, cap: Decimal| {
            self.transitional_caps.push(ParamTransitionalCap {
                parameter_id: id.to_string(),
                revaluation_list_year: list_year,
                financial_year: fy.to_string(),
                jurisdiction: jur,
                property_tier: tier.to_string(),
                rv_threshold_min: min,
                rv_threshold_max: max,
                upward_cap_percentage: cap,
            });
        };

        // 1990 Revaluation List (1990/91 – 1994/95 under LGFA 1988)
        // SI 1989/1358: Small <= £10,000 RV, Large > £10,000 RV
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
                add(&format!("tc_{}_sm_{}", pfx, fy), 1990, fy, jur, "SMALL", dec!(0.00), Some(dec!(10000.00)), sm);
                add(&format!("tc_{}_lg_{}", pfx, fy), 1990, fy, jur, "LARGE", dec!(10000.01), None, lg);
            }
        }

        // 1995 Revaluation List (1995/96 – 1999/00)
        // SI 1994/3279: Small <= £10,000 RV, Large > £10,000 RV
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
                add(&format!("tc_{}_sm_{}", pfx, fy), 1995, fy, jur, "SMALL", dec!(0.00), Some(dec!(10000.00)), sm);
                add(&format!("tc_{}_lg_{}", pfx, fy), 1995, fy, jur, "LARGE", dec!(10000.01), None, lg);
            }
        }

        // 2000 Revaluation List (2000/01 – 2004/05)
        // SI 1999/3379: Small <= £12,000 RV, Large > £12,000 RV
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
                add(&format!("tc_{}_sm_{}", pfx, fy), 2000, fy, jur, "SMALL", dec!(0.00), Some(dec!(12000.00)), sm);
                add(&format!("tc_{}_lg_{}", pfx, fy), 2000, fy, jur, "LARGE", dec!(12000.01), None, lg);
            }
        }

        // 2005 Revaluation List (2005/06 – 2009/10)
        // SI 2004/3312 & WSI 2005/309: Small <= £15,000 RV, Large > £15,000 RV
        let list_2005 = [
            ("2005/06", dec!(5.00), dec!(12.50)),
            ("2006/07", dec!(7.50), dec!(17.50)),
            ("2007/08", dec!(10.00), dec!(20.00)),
            ("2008/09", dec!(15.00), dec!(25.00)),
            ("2009/10", dec!(20.00), dec!(30.00)),
        ];
        for (fy, sm, lg) in list_2005 {
            for jur in [Jurisdiction::ENGLAND, Jurisdiction::WALES] {
                let pfx = if jur == Jurisdiction::ENGLAND { "eng" } else { "wal" };
                add(&format!("tc_{}_sm_{}", pfx, fy), 2005, fy, jur, "SMALL", dec!(0.00), Some(dec!(15000.00)), sm);
                add(&format!("tc_{}_lg_{}", pfx, fy), 2005, fy, jur, "LARGE", dec!(15000.01), None, lg);
            }
        }

        // 2010 Revaluation List (2010/11 – 2016/17)
        // SI 2009/3343 & WSI 2010/248: Small <= £18k, Medium <= £50k, Large > £50k
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
            for jur in [Jurisdiction::ENGLAND, Jurisdiction::WALES] {
                let pfx = if jur == Jurisdiction::ENGLAND { "eng" } else { "wal" };
                add(&format!("tc_{}_sm_{}", pfx, fy), 2010, fy, jur, "SMALL", dec!(0.00), Some(dec!(18000.00)), sm);
                add(&format!("tc_{}_med_{}", pfx, fy), 2010, fy, jur, "MEDIUM", dec!(18000.01), Some(dec!(50000.00)), med);
                add(&format!("tc_{}_lg_{}", pfx, fy), 2010, fy, jur, "LARGE", dec!(50000.01), None, lg);
            }
        }

        // 2017 Revaluation List (2017/18 – 2022/23)
        // SI 2016/1265: Small <= £20k, Medium <= £100k, Large > £100k
        let list_2017 = [
            ("2017/18", dec!(5.00), dec!(12.50), dec!(42.00)),
            ("2018/19", dec!(7.50), dec!(17.50), dec!(32.00)),
            ("2019/20", dec!(10.00), dec!(20.00), dec!(49.00)),
            ("2020/21", dec!(15.00), dec!(25.00), dec!(16.00)),
            ("2021/22", dec!(15.00), dec!(25.00), dec!(16.00)),
            ("2022/23", dec!(15.00), dec!(25.00), dec!(16.00)),
        ];
        for (fy, sm, med, lg) in list_2017 {
            add(&format!("tc_eng_sm_{}", fy), 2017, fy, Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), sm);
            add(&format!("tc_eng_med_{}", fy), 2017, fy, Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), med);
            add(&format!("tc_eng_lg_{}", fy), 2017, fy, Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, lg);
        }

        // 2023 Revaluation List — England (Non-Domestic Rating Chargeable Amounts Regs 2022)
        // 2023/24: Small (+5%), Medium (+15%), Large (+30%)
        add("tc_eng_sm_2023", 2023, "2023/24", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(5.00));
        add("tc_eng_med_2023", 2023, "2023/24", Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(15.00));
        add("tc_eng_lg_2023", 2023, "2023/24", Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, dec!(30.00));

        // 2024/25: Small (+10%), Medium (+25%), Large (+40%)
        add("tc_eng_sm_2024", 2023, "2024/25", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(10.00));
        add("tc_eng_med_2024", 2023, "2024/25", Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(25.00));
        add("tc_eng_lg_2024", 2023, "2024/25", Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, dec!(40.00));

        // 2025/26: Small (+25%), Medium (+40%), Large (+55%)
        add("tc_eng_sm_2025", 2023, "2025/26", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(25.00));
        add("tc_eng_med_2025", 2023, "2025/26", Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(40.00));
        add("tc_eng_lg_2025", 2023, "2025/26", Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, dec!(55.00));

        // 2026 Revaluation List (NDRA 2023 3-year cycle) — England 2026/27: Small (+5%), Medium (+15%), Large (+30%)
        add("tc_eng_sm_2026", 2026, "2026/27", Jurisdiction::ENGLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(5.00));
        add("tc_eng_med_2026", 2026, "2026/27", Jurisdiction::ENGLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(15.00));
        add("tc_eng_lg_2026", 2026, "2026/27", Jurisdiction::ENGLAND, "LARGE", dec!(100000.01), None, dec!(30.00));

        // Wales Transitional Relief (Local Government Finance (Wales) Act 2024)
        add("tc_wal_sm_2026", 2026, "2026/27", Jurisdiction::WALES, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(10.00));
        add("tc_wal_med_2026", 2026, "2026/27", Jurisdiction::WALES, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(25.00));
        add("tc_wal_lg_2026", 2026, "2026/27", Jurisdiction::WALES, "LARGE", dec!(100000.01), None, dec!(40.00));

        // 2023 Revaluation List — Scotland (The Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2023 / SSI 2023/31)
        // 2023/24: Small (+12.5%), Medium (+25%), Large (+37.5%)
        add("tc_sco_sm_2023", 2023, "2023/24", Jurisdiction::SCOTLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(12.50));
        add("tc_sco_med_2023", 2023, "2023/24", Jurisdiction::SCOTLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(25.00));
        add("tc_sco_lg_2023", 2023, "2023/24", Jurisdiction::SCOTLAND, "LARGE", dec!(100000.01), None, dec!(37.50));

        // 2024/25: Small (+25%), Medium (+50%), Large (+75%)
        add("tc_sco_sm_2024", 2023, "2024/25", Jurisdiction::SCOTLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(25.00));
        add("tc_sco_med_2024", 2023, "2024/25", Jurisdiction::SCOTLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(50.00));
        add("tc_sco_lg_2024", 2023, "2024/25", Jurisdiction::SCOTLAND, "LARGE", dec!(100000.01), None, dec!(75.00));

        // 2025/26: Small (+37.5%), Medium (+75%), Large (+112.5%)
        add("tc_sco_sm_2025", 2023, "2025/26", Jurisdiction::SCOTLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(37.50));
        add("tc_sco_med_2025", 2023, "2025/26", Jurisdiction::SCOTLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(75.00));
        add("tc_sco_lg_2025", 2023, "2025/26", Jurisdiction::SCOTLAND, "LARGE", dec!(100000.01), None, dec!(112.50));

        // 2026 Revaluation List — Scotland (The Non-Domestic Rates (Transitional Relief) (Scotland) Regs 2026)
        // 2026/27: Small (+12.5%), Medium (+25%), Large (+37.5%)
        add("tc_sco_sm_2026", 2026, "2026/27", Jurisdiction::SCOTLAND, "SMALL", dec!(0.00), Some(dec!(20000.00)), dec!(12.50));
        add("tc_sco_med_2026", 2026, "2026/27", Jurisdiction::SCOTLAND, "MEDIUM", dec!(20000.01), Some(dec!(100000.00)), dec!(25.00));
        add("tc_sco_lg_2026", 2026, "2026/27", Jurisdiction::SCOTLAND, "LARGE", dec!(100000.01), None, dec!(37.50));
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

        // RELIEF_CODE
        add("RELIEF_CODE", "SBRR", "Small Business Rate Relief", "100% discount up to £12,000 RV, with linear taper up to £15,000 RV for single property occupiers (England).", Some("ENGLAND"), 1);
        add("RELIEF_CODE", "WALES_SBRR", "Small Business Rate Relief (Wales)", "100% relief up to £6,000 RV, with linear taper up to £12,000 RV under LGF(W)A 2024.", Some("WALES"), 2);
        add("RELIEF_CODE", "SBBS", "Small Business Bonus Scheme", "Scottish rating relief for small businesses up to £12,000 RV (2023-2026) or £11,500 RV (2026+).", Some("SCOTLAND"), 3);
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
        add("RELIEF_CODE", "COUNCIL_TAX_TRANSFER", "Council Tax Transfer (Holiday Let Disqualification)", "Disqualification from non-domestic rating list where self-catering accommodation fails statutory actual letting days threshold (182 days Scotland, 70 days England/Wales).", None, 18);

        // MULTIPLIER_TYPE
        add("MULTIPLIER_TYPE", "STANDARD", "Standard Multiplier", "National baseline non-domestic rating multiplier.", None, 1);
        add("MULTIPLIER_TYPE", "SMALL_BUSINESS", "Small Business Multiplier", "Reduced multiplier for qualifying small businesses.", None, 2);
        add("MULTIPLIER_TYPE", "HIGH_VALUE", "High-Value Multiplier", "Higher rating multiplier for high-value properties (e.g. RV >= £500,000 in England).", None, 3);
        add("MULTIPLIER_TYPE", "SMALL_BUSINESS_RHL", "Small Business RHL Multiplier", "Permanent reduced multiplier for qualifying retail, hospitality, leisure under £51k RV in England.", Some("ENGLAND"), 4);
        add("MULTIPLIER_TYPE", "STANDARD_RHL", "Standard RHL Multiplier", "Permanent reduced multiplier for qualifying retail, hospitality, leisure £51k-£500k RV in England.", Some("ENGLAND"), 5);
        add("MULTIPLIER_TYPE", "RETAIL", "Welsh Retail Multiplier", "Lower statutory multiplier for small retail premises under £51k RV in Wales under LGF(W)A 2024 (35.0p in 2026/27).", Some("WALES"), 6);
        add("MULTIPLIER_TYPE", "HIGHER", "Higher Multiplier (Wales)", "Higher statutory multiplier for properties exceeding £100k RV in Wales under LGF(W)A 2024 (51.5p in 2026/27).", Some("WALES"), 7);
    }
}
