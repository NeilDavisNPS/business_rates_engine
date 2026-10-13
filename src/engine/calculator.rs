use std::collections::HashMap;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use chrono::{Utc, NaiveDate, Datelike};

use crate::models::calculation::*;
use crate::models::enums::{Jurisdiction, OccupancyType, PartyType};
use crate::db::repository::ParameterRepository;
use super::slicer::slice_occupancy_periods;
use super::narrative::generate_narrative;

#[derive(Clone)]
pub struct CalculationEngine {
    repo: ParameterRepository,
}

impl CalculationEngine {
    pub fn new(repo: ParameterRepository) -> Self {
        Self { repo }
    }

    fn resolve_rv_for_date(&self, history: &[RvEntry], date: NaiveDate) -> Decimal {
        history.iter()
            .find(|entry| {
                entry.effective_from <= date && entry.effective_to.is_none_or(|end| date <= end)
            })
            .map(|e| e.rateable_value)
            .unwrap_or(dec!(0.00))
    }

    fn resolve_previous_rv(&self, history: &[RvEntry], date: NaiveDate) -> Option<Decimal> {
        history.iter()
            .filter(|entry| entry.effective_from < date)
            .max_by_key(|entry| entry.effective_from)
            .map(|entry| entry.rateable_value)
    }

    fn get_previous_financial_year(&self, fy: &str) -> Option<String> {
        let parts: Vec<&str> = fy.split('/').collect();
        if parts.len() == 2 {
            if let Ok(y) = parts[0].parse::<i32>() {
                return Some(format!("{}/{:02}", y - 1, y % 100));
            }
        }
        None
    }

    fn resolve_welsh_base_fy_and_date(&self, current_fy: &str) -> Option<(&'static str, NaiveDate)> {
        match current_fy {
            "2026/27" | "2027/28" => Some(("2025/26", NaiveDate::from_ymd_opt(2026, 3, 31).unwrap())),
            "2023/24" | "2024/25" => Some(("2022/23", NaiveDate::from_ymd_opt(2023, 3, 31).unwrap())),
            "2017/18" | "2018/19" => Some(("2016/17", NaiveDate::from_ymd_opt(2017, 3, 31).unwrap())),
            "2010/11" | "2011/12" => Some(("2009/10", NaiveDate::from_ymd_opt(2010, 3, 31).unwrap())),
            "2005/06" | "2006/07" => Some(("2004/05", NaiveDate::from_ymd_opt(2005, 3, 31).unwrap())),
            _ => None,
        }
    }

    fn compute_statutory_installments(
        &self,
        net_payable: Decimal,
        fy: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        requested_count: Option<u32>,
    ) -> Option<Vec<StatutoryInstallment>> {
        if net_payable <= dec!(0.00) {
            return Some(Vec::new());
        }

        let start_year = fy.split('/').next()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or_else(|| start_date.year());

        let target_count = requested_count.unwrap_or(10);
        let potential_dates: Vec<NaiveDate> = if target_count == 12 {
            vec![
                NaiveDate::from_ymd_opt(start_year, 4, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 5, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 6, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 7, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 8, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 9, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 10, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 11, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 12, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year + 1, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year + 1, 2, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year + 1, 3, 1).unwrap(),
            ]
        } else {
            vec![
                NaiveDate::from_ymd_opt(start_year, 5, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 6, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 7, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 8, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 9, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 10, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 11, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year, 12, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year + 1, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(start_year + 1, 2, 1).unwrap(),
            ]
        };

        let applicable_dates: Vec<NaiveDate> = potential_dates
            .into_iter()
            .filter(|&d| d >= start_date && d <= end_date)
            .collect();

        let dates = if applicable_dates.is_empty() {
            vec![start_date]
        } else {
            applicable_dates
        };

        let count = dates.len();
        let count_dec = Decimal::from(count);
        let base_amount = ((net_payable / count_dec) * dec!(100.00)).floor() / dec!(100.00);
        let remainder = net_payable - (base_amount * count_dec);

        let mut installments = Vec::with_capacity(count);
        for (i, date) in dates.into_iter().enumerate() {
            let amount = if i == 0 {
                base_amount + remainder
            } else {
                base_amount
            };
            installments.push(StatutoryInstallment {
                installment_number: (i + 1) as u32,
                due_date: date,
                amount,
            });
        }

        Some(installments)
    }

    pub fn calculate(&self, req: LiabilityCalculationRequest) -> LiabilityCalculationResponse {
        let lang = req.language.as_deref().map(crate::i18n::Language::from_code).unwrap_or_default();
        self.calculate_with_language(req, lang)
    }

    pub fn calculate_with_language(&self, req: LiabilityCalculationRequest, lang: crate::i18n::Language) -> LiabilityCalculationResponse {
        let mut periods = req.resolve_occupancy_periods();
        periods.sort_by_key(|p| (p.start_date, p.end_date));
        let windows = slice_occupancy_periods(&periods, &req.qualifying_relief_flags);
        let mut annual_segments = Vec::new();

        let mut total_gross = dec!(0.00);
        let mut total_trans = dec!(0.00);
        let mut total_reliefs = dec!(0.00);
        let mut total_net = dec!(0.00);
        let mut total_penalties = dec!(0.00);

        // Track empty rate-free days and statutory occupation reset state across episodes
        let initial_prior_unoccupied = periods.first()
            .and_then(|p| p.qualifying_relief_flags.as_ref())
            .and_then(|f| f.days_previously_unoccupied)
            .or(req.qualifying_relief_flags.days_previously_unoccupied)
            .unwrap_or(0);
        let mut cumulative_unoccupied_days = initial_prior_unoccupied;
        let mut consecutive_occupied_days: i64 = 0;
        let mut rhl_awarded_by_financial_year: HashMap<String, Decimal> = HashMap::new();
        let mut welsh_avoidance_penalty_charged = false;

        for win in &windows {
            let rv = self.resolve_rv_for_date(&req.hereditament.rateable_value_history, win.period.start_date);
            let is_retail_or_rhl = win.period_relief_flags.is_retail_premises
                || (win.period_relief_flags.is_rhl_qualifying && req.hereditament.scat_code != "209");
            let mult = self.repo.resolve_multiplier_ex(
                req.hereditament.jurisdiction,
                &win.financial_year,
                rv,
                is_retail_or_rhl,
            );

            let days_dec = Decimal::from(win.period.days);
            let year_days_dec = Decimal::from(win.year_days);

            let gross_liability = ((rv * mult.rate) / year_days_dec * days_dec).round_dp(2);
            total_gross += gross_liability;

            // Transitional Phasing Calculation (LGFA 1988 s.57A / s.58 / NDR Chargeable Amounts Regs)
            let mut transitional_adj = TransitionalAdjustment {
                adjustment_type: "NONE".to_string(),
                amount: dec!(0.00),
            };
            let mut segment_trans_relief = dec!(0.00);

            if win.period_relief_flags.apply_transitional_relief.unwrap_or(true) {
                if let Some(prev_fy) = self.get_previous_financial_year(&win.financial_year) {
                    if let Some(prev_rv) = self.resolve_previous_rv(&req.hereditament.rateable_value_history, win.period.start_date) {
                        if req.hereditament.jurisdiction == Jurisdiction::WALES {
                            // Welsh Transitional Relief (W.S.I. 2025/1371 / W.S.I. 2022/1350)
                            // 2-Year Phasing Scheme: 67% relief in Year 1, 34% relief in Year 2 on bill increases > £300
                            if let Some((relief_pct, _authority)) = self.repo.resolve_welsh_transitional_relief_rate(&win.financial_year) {
                                let (base_fy, base_date) = self.resolve_welsh_base_fy_and_date(&win.financial_year)
                                    .unwrap_or((prev_fy.as_str(), win.period.start_date));
                                let base_rv = {
                                    let rv_at_date = self.resolve_rv_for_date(&req.hereditament.rateable_value_history, base_date);
                                    if rv_at_date > dec!(0.00) {
                                        rv_at_date
                                    } else {
                                        prev_rv
                                    }
                                };

                                let prev_is_retail = win.period_relief_flags.is_retail_premises
                                    || (win.period_relief_flags.is_rhl_qualifying && req.hereditament.scat_code != "209");
                                let prev_mult = self.repo.resolve_multiplier_ex(
                                    Jurisdiction::WALES,
                                    base_fy,
                                    base_rv,
                                    prev_is_retail,
                                );
                                let prev_gross_annual = base_rv * prev_mult.rate;
                                let prev_sbrr_pct = if win.period_relief_flags.is_small_business_sole_property {
                                    self.repo.resolve_sbrr(Jurisdiction::WALES, base_fy, base_rv)
                                        .map(|s| s.discount_percentage)
                                        .unwrap_or(dec!(0.00))
                                } else {
                                    dec!(0.00)
                                };
                                let prev_net_annual = (prev_gross_annual * (dec!(1.00) - (prev_sbrr_pct / dec!(100.00)))).round_dp(2);
                                let base_segment_liability = ((prev_net_annual / year_days_dec) * days_dec).round_dp(2);

                                if gross_liability > base_segment_liability {
                                    let segment_increase = gross_liability - base_segment_liability;
                                    let annual_increase = ((segment_increase / days_dec) * year_days_dec).round_dp(2);

                                    // Statutory threshold: increase must exceed £300 to trigger transitional relief
                                    if annual_increase > dec!(300.00) {
                                        segment_trans_relief = (segment_increase * (relief_pct / dec!(100.00))).round_dp(2);
                                        transitional_adj = TransitionalAdjustment {
                                            adjustment_type: "RELIEF".to_string(),
                                            amount: segment_trans_relief,
                                        };
                                    }
                                }
                            }
                        } else if let Some(cap) = self.repo.resolve_transitional_cap(req.hereditament.jurisdiction, &win.financial_year, rv) {
                            let prev_mult = self.repo.resolve_multiplier_ex(
                                req.hereditament.jurisdiction,
                                &prev_fy,
                                prev_rv,
                                win.period_relief_flags.is_rhl_qualifying,
                            );
                            let base_liability_annual = prev_rv * prev_mult.rate;
                            let cap_factor = dec!(1.00) + (cap.upward_cap_percentage / dec!(100.00));
                            let capped_annual_liability = base_liability_annual * cap_factor;
                            let capped_segment_liability = ((capped_annual_liability / year_days_dec) * days_dec).round_dp(2);

                            if gross_liability > capped_segment_liability {
                                segment_trans_relief = (gross_liability - capped_segment_liability).round_dp(2);
                                transitional_adj = TransitionalAdjustment {
                                    adjustment_type: "RELIEF".to_string(),
                                    amount: segment_trans_relief,
                                };
                            }
                        }
                    }
                }
            }
            total_trans += segment_trans_relief;

            let chargeable_before_reliefs = gross_liability - segment_trans_relief;
            let mut reliefs = Vec::new();
            let mut segment_relief_total = dec!(0.00);
            let flags = &win.period_relief_flags;

            match win.occupancy_type {
                OccupancyType::OCCUPIED => {
                    consecutive_occupied_days += win.period.days;
                    // Reset threshold under NDRA 2023 s.15: 91 days (13 weeks) in England post-1 April 2024; 42 days (6 weeks) in Wales/Scotland or pre-2024
                    let reset_threshold: i64 = if req.hereditament.jurisdiction == Jurisdiction::ENGLAND
                        && win.period.start_date >= NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()
                    {
                        91
                    } else {
                        42
                    };
                    if consecutive_occupied_days >= reset_threshold {
                        cumulative_unoccupied_days = 0;
                    }

                    // Priority 5: Self-Catering Holiday Accommodation Letting Days Test
                    let holiday_let_disqualified = if req.hereditament.scat_code == "249" {
                        if let Some(actual_days) = flags.holiday_let_actual_days_let {
                            let min_required = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                182
                            } else {
                                70
                            };
                            actual_days < min_required
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if holiday_let_disqualified {
                        let amount = chargeable_before_reliefs;
                        segment_relief_total += amount;
                        let auth = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                            "Local Government (Scotland) Act 1975 / SSI 2022/86 reg. 2".to_string()
                        } else {
                            "LGFA 1988 s.66 / SI 2022/1098 / NDR (Alteration of Lists) Regs".to_string()
                        };
                        reliefs.push(ItemizedRelief {
                            relief_code: "COUNCIL_TAX_TRANSFER".to_string(),
                            relief_name: crate::i18n::localize_relief_name("Council Tax Transfer (Holiday Let Disqualification)", lang),
                            statutory_authority: auth,
                            discount_percentage: dec!(100.00),
                            amount_deducted: amount,
                        });
                    } else {
                        // 1. Mandatory Charitable Relief (80%) - LGFA 1988 s.43(5)
                        if flags.is_charitable_purposes 
                            || req.party_type == PartyType::CHARITY 
                            || req.party_type == PartyType::CASC 
                        {
                            let char_pct = dec!(80.00);
                            let amount = (chargeable_before_reliefs * (char_pct / dec!(100.00))).round_dp(2);
                            segment_relief_total += amount;
                            let auth = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                "Local Government (Financial Provisions) (Scotland) Act 1962 s.4(2)".to_string()
                            } else {
                                "LGFA 1988 s.43(5) / s.43(6)".to_string()
                            };
                            reliefs.push(ItemizedRelief {
                                relief_code: "CHARITY_MANDATORY".to_string(),
                                relief_name: crate::i18n::localize_relief_name("Mandatory Charitable Rate Relief", lang),
                                statutory_authority: auth,
                                discount_percentage: char_pct,
                                amount_deducted: amount,
                            });

                            // Priority 2: Discretionary Charitable Relief Top-Up (up to 20%) - LGFA 1988 s.47(2)(a) / 1962 Act s.4(5)
                            if flags.is_charity_discretionary_top_up {
                                let topup_pct = dec!(20.00);
                                let topup_amount = (chargeable_before_reliefs * (topup_pct / dec!(100.00))).round_dp(2).min(chargeable_before_reliefs - segment_relief_total);
                                if topup_amount > dec!(0.00) {
                                    segment_relief_total += topup_amount;
                                    let topup_auth = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                        "Local Government (Financial Provisions) (Scotland) Act 1962 s.4(5)".to_string()
                                    } else {
                                        "LGFA 1988 s.47(2)(a)".to_string()
                                    };
                                    reliefs.push(ItemizedRelief {
                                        relief_code: "CHARITY_DISCRETIONARY_TOPUP".to_string(),
                                        relief_name: crate::i18n::localize_relief_name("Discretionary Charitable Rate Relief Top-Up", lang),
                                        statutory_authority: topup_auth,
                                        discount_percentage: topup_pct,
                                        amount_deducted: topup_amount,
                                    });
                                }
                            }
                        }

                        // Priority 1: Telecommunications Infrastructure Relief (100%) - Telecommunications Infrastructure (Relief from Non-Domestic Rates) Act 2018 / SSI 2019/111
                        if flags.is_telecommunications_infrastructure && segment_relief_total < chargeable_before_reliefs {
                            let amount = chargeable_before_reliefs - segment_relief_total;
                            if amount > dec!(0.00) {
                                segment_relief_total += amount;
                                let auth = match req.hereditament.jurisdiction {
                                    Jurisdiction::ENGLAND => "Telecommunications Infrastructure (Relief from Non-Domestic Rates) Act 2018 / SI 2018/715".to_string(),
                                    Jurisdiction::SCOTLAND => "Non-Domestic Rates (Telecommunication New Fibre Infrastructure Relief) (Scotland) Regs (SSI 2019/111 / SSI 2024/76)".to_string(),
                                    Jurisdiction::WALES => "Telecommunications Infrastructure (Relief from Non-Domestic Rates) Act 2018 / LGFA 1988 s.47".to_string(),
                                };
                                reliefs.push(ItemizedRelief {
                                    relief_code: "TELECOM_FIBRE_RELIEF".to_string(),
                                    relief_name: crate::i18n::localize_relief_name("Telecommunications Infrastructure Relief", lang),
                                    statutory_authority: auth,
                                    discount_percentage: dec!(100.00),
                                    amount_deducted: amount,
                                });
                            }
                        }

                        // 2. NDRA 2023 Improvement Relief (G-factor) - 100% relief on RV increment
                        if flags.qualifying_improvement_rv_increment > dec!(0.00) && segment_relief_total < chargeable_before_reliefs {
                            let inc = flags.qualifying_improvement_rv_increment;
                            let g_amount = ((inc * mult.rate) / year_days_dec * days_dec).round_dp(2).min(chargeable_before_reliefs - segment_relief_total);
                            segment_relief_total += g_amount;
                            reliefs.push(ItemizedRelief {
                                relief_code: "IMPROVEMENT_G_FACTOR".to_string(),
                                relief_name: crate::i18n::localize_relief_name("Improvement Relief (G-Factor)", lang),
                                statutory_authority: "Non-Domestic Rating Act 2023 s.1 / LGFA 1988 Sched 4ZA".to_string(),
                                discount_percentage: dec!(100.00),
                                amount_deducted: g_amount,
                            });
                        }

                        // 3. Heat Networks Exemption (100%) - NDRA 2023 s.2
                        if flags.is_heat_network && segment_relief_total < chargeable_before_reliefs {
                            let amount = chargeable_before_reliefs - segment_relief_total;
                            if amount > dec!(0.00) {
                                segment_relief_total += amount;
                                reliefs.push(ItemizedRelief {
                                    relief_code: "HEAT_NETWORK".to_string(),
                                    relief_name: crate::i18n::localize_relief_name("Heat Network Exemption", lang),
                                    statutory_authority: "Non-Domestic Rating Act 2023 s.2".to_string(),
                                    discount_percentage: dec!(100.00),
                                    amount_deducted: amount,
                                });
                            }
                        }

                        // 4. Public Lavatories Exemption (100%) - Rating (Public Lavatories) Act 2021
                        if flags.is_public_lavatory && segment_relief_total < chargeable_before_reliefs {
                            let amount = chargeable_before_reliefs - segment_relief_total;
                            if amount > dec!(0.00) {
                                segment_relief_total += amount;
                                reliefs.push(ItemizedRelief {
                                    relief_code: "PUBLIC_LAVATORY".to_string(),
                                    relief_name: crate::i18n::localize_relief_name("Public Lavatory Relief", lang),
                                    statutory_authority: "Rating (Public Lavatories) Act 2021".to_string(),
                                    discount_percentage: dec!(100.00),
                                    amount_deducted: amount,
                                });
                            }
                        }

                        // Priority 4: Rural Settlement Relief (100%) - LGFA 1997 / LGFA 1988 s.43(6B)
                        // Mandatory relief applies only if RV <= £8,500 (general store/post office) or <= £12,500 (pub/petrol station)
                        if flags.is_rural_settlement && segment_relief_total < chargeable_before_reliefs {
                            let rv_cap = if req.hereditament.scat_code == "209" || req.hereditament.scat_code == "211" {
                                dec!(12500.00)
                            } else {
                                dec!(8500.00)
                            };
                            if rv <= rv_cap {
                                let amount = chargeable_before_reliefs - segment_relief_total;
                                if amount > dec!(0.00) {
                                    segment_relief_total += amount;
                                    let auth = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                        "Local Government (Scotland) Act 1975 Sched 1A / LGFA 1997".to_string()
                                    } else {
                                        "LGFA 1988 s.43(6B) / LGFA 1997".to_string()
                                    };
                                    reliefs.push(ItemizedRelief {
                                        relief_code: "RURAL_SETTLEMENT".to_string(),
                                        relief_name: crate::i18n::localize_relief_name("Rural Settlement Relief", lang),
                                        statutory_authority: auth,
                                        discount_percentage: dec!(100.00),
                                        amount_deducted: amount,
                                    });
                                }
                            }
                        }

                        // Priority 3: Scottish Renewable Energy Generation Relief (SSI 2010/336)
                        if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND
                            && flags.is_renewable_energy_generation
                            && segment_relief_total < chargeable_before_reliefs
                        {
                            let renew_pct = if rv <= dec!(145000.00) {
                                dec!(100.00)
                            } else if rv <= dec!(200000.00) {
                                dec!(50.00)
                            } else if rv <= dec!(250000.00) {
                                dec!(25.00)
                            } else if rv <= dec!(1000000.00) {
                                dec!(10.00)
                            } else {
                                dec!(0.00)
                            };

                            if renew_pct > dec!(0.00) {
                                let remaining = chargeable_before_reliefs - segment_relief_total;
                                let amount = (remaining * (renew_pct / dec!(100.00))).round_dp(2);
                                if amount > dec!(0.00) {
                                    segment_relief_total += amount;
                                    reliefs.push(ItemizedRelief {
                                        relief_code: "RENEWABLE_ENERGY_RELIEF".to_string(),
                                        relief_name: crate::i18n::localize_relief_name("Renewable Energy Generation Relief", lang),
                                        statutory_authority: "Non-Domestic Rates (Renewable Energy Generation Relief) (Scotland) Regs 2010 (SSI 2010/336)".to_string(),
                                        discount_percentage: renew_pct,
                                        amount_deducted: amount,
                                    });
                                }
                            }
                        }

                        // 6. Dynamic SBRR / SBBS logic resolved from parameter repository tables
                        if (flags.is_small_business_sole_property || flags.is_small_business_second_property_grace)
                            && segment_relief_total < chargeable_before_reliefs
                        {
                            if let Some(sbrr) = self.repo.resolve_sbrr(req.hereditament.jurisdiction, &win.financial_year, rv) {
                                let remaining = chargeable_before_reliefs - segment_relief_total;
                                let amount = (remaining * (sbrr.discount_percentage / dec!(100.00))).round_dp(2);
                                if amount > dec!(0.00) {
                                    segment_relief_total += amount;
                                    let authority = if flags.is_small_business_second_property_grace && req.hereditament.jurisdiction == Jurisdiction::ENGLAND {
                                        format!("{} / S.I. 2017/411 art. 3(4) (12-Month Grace Period)", sbrr.statutory_authority)
                                    } else {
                                        sbrr.statutory_authority
                                    };
                                    reliefs.push(ItemizedRelief {
                                        relief_code: sbrr.relief_code,
                                        relief_name: crate::i18n::localize_relief_name(&sbrr.relief_name, lang),
                                        statutory_authority: authority,
                                        discount_percentage: sbrr.discount_percentage,
                                        amount_deducted: amount,
                                    });
                                }
                            }
                        }

                        // 6b. Scottish Small Business Bonus Scheme Transitional Relief (SBBS TR)
                        if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND
                            && flags.is_sbbs_tr_qualifying
                            && segment_relief_total < chargeable_before_reliefs
                        {
                            if let Some(sbbs_tr) = self.repo.resolve_sbbs_tr(&win.financial_year) {
                                let prev_net_segment = if let Some(prev_fy) = self.get_previous_financial_year(&win.financial_year) {
                                    if let Some(prev_rv) = self.resolve_previous_rv(&req.hereditament.rateable_value_history, win.period.start_date) {
                                        let prev_mult = self.repo.resolve_multiplier(Jurisdiction::SCOTLAND, &prev_fy, prev_rv);
                                        let prev_gross_annual = prev_rv * prev_mult.rate;
                                        let prev_sbbs_pct = self.repo.resolve_sbrr(Jurisdiction::SCOTLAND, &prev_fy, prev_rv)
                                            .map(|r| r.discount_percentage)
                                            .unwrap_or(dec!(0.00));
                                        let prev_net_annual = (prev_gross_annual * (dec!(1.00) - (prev_sbbs_pct / dec!(100.00)))).round_dp(2);
                                        ((prev_net_annual / year_days_dec) * days_dec).round_dp(2)
                                    } else {
                                        dec!(0.00)
                                    }
                                } else {
                                    dec!(0.00)
                                };

                                let current_net_before_tr = chargeable_before_reliefs - segment_relief_total;
                                if current_net_before_tr > prev_net_segment {
                                    let unmitigated_increase = current_net_before_tr - prev_net_segment;
                                    let tr_deduction = match sbbs_tr.cap_type {
                                        crate::db::repository::SbbsTrCapType::FixedCash(annual_cap) => {
                                            let segment_cap = ((annual_cap / year_days_dec) * days_dec).round_dp(2);
                                            (unmitigated_increase - segment_cap).max(dec!(0.00))
                                        }
                                        crate::db::repository::SbbsTrCapType::PercentageOfIncrease(payable_pct) => {
                                            let relief_pct = (dec!(100.00) - payable_pct) / dec!(100.00);
                                            (unmitigated_increase * relief_pct).round_dp(2)
                                        }
                                    };

                                    if tr_deduction > dec!(0.00) {
                                        segment_relief_total += tr_deduction;
                                        let effective_pct = ((tr_deduction / current_net_before_tr) * dec!(100.00)).round_dp(2);
                                        reliefs.push(ItemizedRelief {
                                            relief_code: sbbs_tr.relief_code,
                                            relief_name: crate::i18n::localize_relief_name(&sbbs_tr.relief_name, lang),
                                            statutory_authority: sbbs_tr.statutory_authority,
                                            discount_percentage: effective_pct,
                                            amount_deducted: tr_deduction,
                                        });
                                    }
                                }
                            }
                        }

                        // 6c. Supporting Small Business Relief (SSBR) in England (LGFA 1988 s.47)
                        if req.hereditament.jurisdiction == Jurisdiction::ENGLAND
                            && flags.is_ssbr_qualifying
                            && segment_relief_total < chargeable_before_reliefs
                        {
                            if let Some(ssbr) = self.repo.resolve_ssbr(&win.financial_year) {
                                let prev_net_segment = if let Some(prev_fy) = self.get_previous_financial_year(&win.financial_year) {
                                    if let Some(prev_rv) = self.resolve_previous_rv(&req.hereditament.rateable_value_history, win.period.start_date) {
                                        let prev_mult = self.repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, &prev_fy, prev_rv, flags.is_rhl_qualifying);
                                        let prev_gross_annual = prev_rv * prev_mult.rate;
                                        let prev_sbrr_pct = self.repo.resolve_sbrr(Jurisdiction::ENGLAND, &prev_fy, prev_rv)
                                            .map(|r| r.discount_percentage)
                                            .unwrap_or(dec!(0.00));
                                        let prev_net_annual = (prev_gross_annual * (dec!(1.00) - (prev_sbrr_pct / dec!(100.00)))).round_dp(2);
                                        ((prev_net_annual / year_days_dec) * days_dec).round_dp(2)
                                    } else {
                                        dec!(0.00)
                                    }
                                } else {
                                    dec!(0.00)
                                };

                                let current_net_before_ssbr = chargeable_before_reliefs - segment_relief_total;
                                if current_net_before_ssbr > prev_net_segment {
                                    let unmitigated_increase = current_net_before_ssbr - prev_net_segment;
                                    let segment_cap = ((ssbr.annual_cap / year_days_dec) * days_dec).round_dp(2);
                                    let ssbr_deduction = (unmitigated_increase - segment_cap).max(dec!(0.00));

                                    if ssbr_deduction > dec!(0.00) {
                                        segment_relief_total += ssbr_deduction;
                                        let effective_pct = ((ssbr_deduction / current_net_before_ssbr) * dec!(100.00)).round_dp(2);
                                        reliefs.push(ItemizedRelief {
                                            relief_code: ssbr.relief_code,
                                            relief_name: crate::i18n::localize_relief_name(&ssbr.relief_name, lang),
                                            statutory_authority: ssbr.statutory_authority,
                                            discount_percentage: effective_pct,
                                            amount_deducted: ssbr_deduction,
                                        });
                                    }
                                }
                            }
                        }

                        // 7. Dynamic Sector Relief (RHL) resolved from parameter repository tables
                        if flags.is_rhl_qualifying && segment_relief_total < gross_liability {
                            let eligible_rv = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND && !flags.is_island_property {
                                rv <= dec!(100000.00)
                            } else {
                                true
                            };

                            if eligible_rv {
                                if let Some(sec) = self.repo.resolve_sector_relief(
                                    req.hereditament.jurisdiction,
                                    &win.financial_year,
                                    true,
                                    flags.is_island_property,
                                ) {
                                    let qualifies = match req.hereditament.jurisdiction {
                                        Jurisdiction::SCOTLAND => true,
                                        Jurisdiction::WALES => req.hereditament.scat_code != "211",
                                        Jurisdiction::ENGLAND => true,
                                    };

                                    if qualifies {
                                        let remaining = chargeable_before_reliefs - segment_relief_total;
                                        let mut amount = (remaining * (sec.discount_percentage / dec!(100.00))).round_dp(2);
                                        if let Some(cap) = sec.cash_cap_gbp {
                                            let already_awarded = rhl_awarded_by_financial_year
                                                .get(&win.financial_year)
                                                .copied()
                                                .unwrap_or(dec!(0.00));
                                            let cap_remaining = (cap - already_awarded).max(dec!(0.00));
                                            amount = amount.min(cap_remaining);
                                        }
                                        if amount > dec!(0.00) {
                                            *rhl_awarded_by_financial_year
                                                .entry(win.financial_year.clone())
                                                .or_insert(dec!(0.00)) += amount;
                                            segment_relief_total += amount;
                                            reliefs.push(ItemizedRelief {
                                                relief_code: sec.relief_code,
                                                relief_name: crate::i18n::localize_relief_name(&sec.relief_name, lang),
                                                statutory_authority: sec.statutory_authority,
                                                discount_percentage: sec.discount_percentage,
                                                amount_deducted: amount,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                OccupancyType::UNOCCUPIED => {
                    consecutive_occupied_days = 0;
                    let is_industrial = req.hereditament.scat_code == "096" || flags.is_industrial_hereditament;
                    let max_exempt_days: i64 = if is_industrial { 182 } else { 91 };

                    let low_value_threshold = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                        dec!(1700.00)
                    } else {
                        dec!(2900.00)
                    };

                    if rv < low_value_threshold {
                        let amount = chargeable_before_reliefs - segment_relief_total;
                        segment_relief_total += amount;
                        let auth = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                            "Local Government (Scotland) Act 1975 s.7A / SSI 2018/77".to_string()
                        } else {
                            "LGFA 1988 s.45 / SI 2008/386".to_string()
                        };
                        reliefs.push(ItemizedRelief {
                            relief_code: "EMPTY_LOW_VALUE".to_string(),
                            relief_name: crate::i18n::localize_relief_name("Empty Property Low RV Exemption", lang),
                            statutory_authority: auth,
                            discount_percentage: dec!(100.00),
                            amount_deducted: amount,
                        });
                    } else if cumulative_unoccupied_days < max_exempt_days {
                        let remaining_exempt_days = (max_exempt_days - cumulative_unoccupied_days).min(win.period.days);
                        let exempt_ratio = Decimal::from(remaining_exempt_days) / days_dec;
                        let remaining = chargeable_before_reliefs - segment_relief_total;

                        let relief_fraction = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                            dec!(0.50)
                        } else {
                            dec!(1.00)
                        };
                        let amount = (remaining * exempt_ratio * relief_fraction).round_dp(2);
                        cumulative_unoccupied_days += win.period.days;
                        if amount > dec!(0.00) {
                            segment_relief_total += amount;
                            let empty_name = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                if is_industrial { "Empty Industrial Rate Relief (50% for 6 Months)" } else { "Empty Property Rate Relief (50% for 3 Months)" }
                            } else {
                                if is_industrial { "Empty Industrial Rate-Free Period (6 Months)" } else { "Empty Property Rate-Free Period (3 Months)" }
                            };
                            let auth = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                "Local Government (Scotland) Act 1975 s.7A / SSI 2018/77".to_string()
                            } else {
                                "LGFA 1988 s.45 / Non-Domestic Rating (Unoccupied Property) Regs".to_string()
                            };
                            reliefs.push(ItemizedRelief {
                                relief_code: "EMPTY_PROPERTY_EXEMPTION".to_string(),
                                relief_name: crate::i18n::localize_relief_name(empty_name, lang),
                                statutory_authority: auth,
                                discount_percentage: (exempt_ratio * relief_fraction * dec!(100.00)).round_dp(2),
                                amount_deducted: amount,
                            });
                        }
                    } else {
                        cumulative_unoccupied_days += win.period.days;
                    }
                }
                OccupancyType::PART_OCCUPIED => {
                    consecutive_occupied_days = 0;
                    let (_occ_rv, unocc_rv) = if let Some(ref app) = win.part_apportionment {
                        if let Some(o_rv) = app.occupied_rateable_value {
                            let u_rv = app.unoccupied_rateable_value.unwrap_or_else(|| (rv - o_rv).max(dec!(0.00)));
                            (o_rv, u_rv)
                        } else if let Some(pct) = app.occupied_percentage {
                            let o_rv = (rv * (pct / dec!(100.00))).round_dp(2);
                            let u_rv = (rv - o_rv).max(dec!(0.00));
                            (o_rv, u_rv)
                        } else if let Some(u_rv) = app.unoccupied_rateable_value {
                            let o_rv = (rv - u_rv).max(dec!(0.00));
                            (o_rv, u_rv)
                        } else {
                            let o_rv = (rv * dec!(0.50)).round_dp(2);
                            (o_rv, rv - o_rv)
                        }
                    } else {
                        let o_rv = (rv * dec!(0.50)).round_dp(2);
                        (o_rv, rv - o_rv)
                    };

                    let unocc_ratio = if rv > dec!(0.00) { (unocc_rv / rv).min(dec!(1.00)) } else { dec!(0.00) };
                    let unocc_chargeable = (chargeable_before_reliefs * unocc_ratio).round_dp(2);
                    let occ_chargeable = chargeable_before_reliefs - unocc_chargeable;

                    // Unoccupied part: Section 44A empty property relief
                    let is_industrial = req.hereditament.scat_code == "096" || flags.is_industrial_hereditament;
                    let max_exempt_days: i64 = if is_industrial { 182 } else { 91 };
                    let low_value_threshold = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                        dec!(1700.00)
                    } else {
                        dec!(2900.00)
                    };

                    if unocc_rv < low_value_threshold {
                        if unocc_chargeable > dec!(0.00) {
                            segment_relief_total += unocc_chargeable;
                            reliefs.push(ItemizedRelief {
                                relief_code: "SECTION_44A_PART_OCCUPIED".to_string(),
                                relief_name: crate::i18n::localize_relief_name("Section 44A Part-Occupied Relief", lang),
                                statutory_authority: if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                    "Local Government (Scotland) Act 1975 s.7A / s.44A".to_string()
                                } else {
                                    "LGFA 1988 s.44A / s.45".to_string()
                                },
                                discount_percentage: dec!(100.00),
                                amount_deducted: unocc_chargeable,
                            });
                        }
                    } else if cumulative_unoccupied_days < max_exempt_days {
                        let remaining_exempt_days = (max_exempt_days - cumulative_unoccupied_days).min(win.period.days);
                        let exempt_ratio = Decimal::from(remaining_exempt_days) / days_dec;
                        let relief_fraction = if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                            dec!(0.50)
                        } else {
                            dec!(1.00)
                        };
                        let s44a_amount = (unocc_chargeable * exempt_ratio * relief_fraction).round_dp(2);
                        cumulative_unoccupied_days += win.period.days;
                        if s44a_amount > dec!(0.00) {
                            segment_relief_total += s44a_amount;
                            reliefs.push(ItemizedRelief {
                                relief_code: "SECTION_44A_PART_OCCUPIED".to_string(),
                                relief_name: crate::i18n::localize_relief_name("Section 44A Part-Occupied Relief", lang),
                                statutory_authority: if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                    "Local Government (Scotland) Act 1975 s.7A / s.44A / SSI 2018/77".to_string()
                                } else {
                                    "LGFA 1988 s.44A".to_string()
                                },
                                discount_percentage: (exempt_ratio * relief_fraction * dec!(100.00)).round_dp(2),
                                amount_deducted: s44a_amount,
                            });
                        }
                    } else {
                        cumulative_unoccupied_days += win.period.days;
                    }

                    // Occupied part reliefs (e.g. Charitable Relief or SBRR applied to occupied portion)
                    let mut occ_relief_total = Decimal::ZERO;

                    if flags.is_charitable_purposes 
                        || req.party_type == PartyType::CHARITY 
                        || req.party_type == PartyType::CASC 
                    {
                        let char_pct = dec!(80.00);
                        let amount = (occ_chargeable * (char_pct / dec!(100.00))).round_dp(2).min(occ_chargeable - occ_relief_total);
                        if amount > dec!(0.00) {
                            occ_relief_total += amount;
                            segment_relief_total += amount;
                            reliefs.push(ItemizedRelief {
                                relief_code: "CHARITY_MANDATORY".to_string(),
                                relief_name: crate::i18n::localize_relief_name("Mandatory Charitable Rate Relief", lang),
                                statutory_authority: if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                    "Local Government (Financial Provisions) (Scotland) Act 1962 s.4(2) / s.44A".to_string()
                                } else {
                                    "LGFA 1988 s.43(5) / s.44A".to_string()
                                },
                                discount_percentage: char_pct,
                                amount_deducted: amount,
                            });
                        }

                        if flags.is_charity_discretionary_top_up && occ_relief_total < occ_chargeable {
                            let topup_pct = dec!(20.00);
                            let topup_amount = (occ_chargeable * (topup_pct / dec!(100.00))).round_dp(2).min(occ_chargeable - occ_relief_total);
                            if topup_amount > dec!(0.00) {
                                occ_relief_total += topup_amount;
                                segment_relief_total += topup_amount;
                                reliefs.push(ItemizedRelief {
                                    relief_code: "CHARITY_DISCRETIONARY_TOPUP".to_string(),
                                    relief_name: crate::i18n::localize_relief_name("Discretionary Charitable Rate Relief Top-Up", lang),
                                    statutory_authority: if req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
                                        "Local Government (Financial Provisions) (Scotland) Act 1962 s.4(5) / s.44A".to_string()
                                    } else {
                                        "LGFA 1988 s.47(2)(a) / s.44A".to_string()
                                    },
                                    discount_percentage: topup_pct,
                                    amount_deducted: topup_amount,
                                });
                            }
                        }
                    }

                    if (flags.is_small_business_sole_property || flags.is_small_business_second_property_grace)
                        && occ_relief_total < occ_chargeable
                    {
                        if let Some(sbrr) = self.repo.resolve_sbrr(req.hereditament.jurisdiction, &win.financial_year, rv) {
                            let remaining = occ_chargeable - occ_relief_total;
                            let amount = (remaining * (sbrr.discount_percentage / dec!(100.00))).round_dp(2).min(remaining);
                            if amount > dec!(0.00) {
                                occ_relief_total += amount;
                                segment_relief_total += amount;
                                let authority = if flags.is_small_business_second_property_grace && req.hereditament.jurisdiction == Jurisdiction::ENGLAND {
                                    format!("{} / S.I. 2017/411 art. 3(4) / LGFA 1988 s.44A", sbrr.statutory_authority)
                                } else {
                                    format!("{} / LGFA 1988 s.44A", sbrr.statutory_authority)
                                };
                                reliefs.push(ItemizedRelief {
                                    relief_code: sbrr.relief_code,
                                    relief_name: crate::i18n::localize_relief_name(&sbrr.relief_name, lang),
                                    statutory_authority: authority,
                                    discount_percentage: sbrr.discount_percentage,
                                    amount_deducted: amount,
                                });
                            }
                        }
                    }
                }
            }

            // Statutory Anti-Avoidance Penalties (Welsh WSI 2026/43 / LGF(W)A 2024)
            let mut segment_penalties = Vec::new();
            if req.hereditament.jurisdiction == Jurisdiction::WALES
                && flags.is_artificial_avoidance_suspected
                && !welsh_avoidance_penalty_charged
            {
                welsh_avoidance_penalty_charged = true;
                let penalty_amount = dec!(500.00) + (rv * dec!(0.03)).round_dp(2);
                segment_penalties.push(StatutoryPenalty {
                    penalty_type: "WELSH_ARTIFICIAL_AVOIDANCE".to_string(),
                    statutory_authority: "NDR (Artificial Avoidance Arrangements) (Wales) Regs 2026 (WSI 2026/43)".to_string(),
                    calculation_formula: format!("£500 flat fee + 3% of Rateable Value (£{})", rv),
                    amount: penalty_amount,
                });
                total_penalties += penalty_amount;
            }

            total_reliefs += segment_relief_total;

            let net_payable = (chargeable_before_reliefs - segment_relief_total).max(dec!(0.00));
            total_net += net_payable;

            let installments = self.compute_statutory_installments(
                net_payable,
                &win.financial_year,
                win.period.start_date,
                win.period.end_date,
                flags.payment_installments_count,
            );

            annual_segments.push(AnnualBillingSegment {
                financial_year: win.financial_year.clone(),
                segment_period: win.period.clone(),
                rateable_value: rv,
                multiplier_used: mult,
                gross_liability,
                transitional_adjustment: transitional_adj,
                itemized_reliefs: reliefs,
                net_segment_payable: net_payable,
                statutory_penalties: segment_penalties,
                statutory_installments: installments,
                occupancy_type: Some(win.occupancy_type),
            });
        }

        let total_days = windows.iter().map(|w| w.period.days).sum();
        let (overall_start, overall_end) = req.resolve_overall_period();
        let total_period = SegmentPeriod {
            start_date: overall_start,
            end_date: overall_end,
            days: total_days,
        };

        let summary = SummaryFinancials {
            total_gross_liability: total_gross,
            total_transitional_adjustments: total_trans,
            total_reliefs_and_discounts: total_reliefs,
            total_net_payable: total_net,
            total_statutory_penalties: if total_penalties > dec!(0.00) { Some(total_penalties) } else { None },
            currency: "GBP".to_string(),
        };

        let narrative = generate_narrative(&req, &annual_segments, total_net, lang);

        LiabilityCalculationResponse {
            calculation_id: Uuid::new_v4(),
            calculated_at: Utc::now().to_rfc3339(),
            parameter_version_ids: self.repo.get_versions().clone(),
            jurisdiction: req.hereditament.jurisdiction,
            occupation_period: total_period,
            summary_financials: summary,
            annual_billing_segments: annual_segments,
            calculation_narrative: narrative,
        }
    }
}
