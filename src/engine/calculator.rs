use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};

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
                entry.effective_from <= date && entry.effective_to.map_or(true, |end| date <= end)
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

    pub fn calculate(&self, req: LiabilityCalculationRequest) -> LiabilityCalculationResponse {
        let lang = req.language.as_deref().map(crate::i18n::Language::from_code).unwrap_or_default();
        self.calculate_with_language(req, lang)
    }

    pub fn calculate_with_language(&self, req: LiabilityCalculationRequest, lang: crate::i18n::Language) -> LiabilityCalculationResponse {
        let periods = req.resolve_occupancy_periods();
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

        for win in &windows {
            let rv = self.resolve_rv_for_date(&req.hereditament.rateable_value_history, win.period.start_date);
            let mult = self.repo.resolve_multiplier(req.hereditament.jurisdiction, &win.financial_year, rv);

            let days_dec = Decimal::from(win.period.days);
            let year_days_dec = Decimal::from(win.year_days);

            let gross_liability = ((rv * mult.rate) / year_days_dec * days_dec).round_dp(2);
            total_gross += gross_liability;

            // Transitional Phasing Calculation (LGFA 1988 s.57A / NDR Chargeable Amounts Regs)
            let mut transitional_adj = TransitionalAdjustment {
                adjustment_type: "NONE".to_string(),
                amount: dec!(0.00),
            };
            let mut segment_trans_relief = dec!(0.00);

            if let Some(prev_fy) = self.get_previous_financial_year(&win.financial_year) {
                if let Some(prev_rv) = self.resolve_previous_rv(&req.hereditament.rateable_value_history, win.period.start_date) {
                    if let Some(cap) = self.repo.resolve_transitional_cap(req.hereditament.jurisdiction, &win.financial_year, rv) {
                        let prev_mult = self.repo.resolve_multiplier(req.hereditament.jurisdiction, &prev_fy, prev_rv);
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

                    // 1. Mandatory Charitable Relief (80%) - LGFA 1988 s.43(5)
                    if flags.is_charitable_purposes 
                        || req.party_type == PartyType::CHARITY 
                        || req.party_type == PartyType::CASC 
                    {
                        let char_pct = dec!(80.00);
                        let amount = (gross_liability * (char_pct / dec!(100.00))).round_dp(2);
                        segment_relief_total += amount;
                        reliefs.push(ItemizedRelief {
                            relief_code: "CHARITY_MANDATORY".to_string(),
                            relief_name: crate::i18n::localize_relief_name("Mandatory Charitable Rate Relief", lang),
                            statutory_authority: "LGFA 1988 s.43(5) / s.43(6)".to_string(),
                            discount_percentage: char_pct,
                            amount_deducted: amount,
                        });
                    }

                    // 2. NDRA 2023 Improvement Relief (G-factor) - 100% relief on RV increment
                    if flags.qualifying_improvement_rv_increment > dec!(0.00) {
                        let inc = flags.qualifying_improvement_rv_increment;
                        let g_amount = ((inc * mult.rate) / year_days_dec * days_dec).round_dp(2);
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
                    if flags.is_heat_network {
                        let amount = gross_liability - segment_relief_total;
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
                    if flags.is_public_lavatory {
                        let amount = gross_liability - segment_relief_total;
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

                    // 5. Rural Settlement Relief (100%) - LGFA 1997 / LGFA 1988 s.43(6B)
                    if flags.is_rural_settlement {
                        let amount = gross_liability - segment_relief_total;
                        if amount > dec!(0.00) {
                            segment_relief_total += amount;
                            reliefs.push(ItemizedRelief {
                                relief_code: "RURAL_SETTLEMENT".to_string(),
                                relief_name: crate::i18n::localize_relief_name("Rural Settlement Relief", lang),
                                statutory_authority: "LGFA 1988 s.43(6B) / LGFA 1997".to_string(),
                                discount_percentage: dec!(100.00),
                                amount_deducted: amount,
                            });
                        }
                    }

                    // 6. Dynamic SBRR / SBBS logic resolved from parameter repository tables
                    if flags.is_small_business_sole_property && segment_relief_total < gross_liability {
                        if let Some(sbrr) = self.repo.resolve_sbrr(req.hereditament.jurisdiction, &win.financial_year, rv) {
                            let remaining = gross_liability - segment_relief_total;
                            let amount = (remaining * (sbrr.discount_percentage / dec!(100.00))).round_dp(2);
                            if amount > dec!(0.00) {
                                segment_relief_total += amount;
                                reliefs.push(ItemizedRelief {
                                    relief_code: sbrr.relief_code,
                                    relief_name: crate::i18n::localize_relief_name(&sbrr.relief_name, lang),
                                    statutory_authority: sbrr.statutory_authority,
                                    discount_percentage: sbrr.discount_percentage,
                                    amount_deducted: amount,
                                });
                            }
                        }
                    }

                    // 7. Dynamic Sector Relief (RHL) resolved from parameter repository tables
                    if flags.is_rhl_qualifying && segment_relief_total < gross_liability {
                        if let Some(sec) = self.repo.resolve_sector_relief(req.hereditament.jurisdiction, &win.financial_year, true) {
                            if rv > dec!(15000.00) || !flags.is_small_business_sole_property {
                                let remaining = chargeable_before_reliefs - segment_relief_total;
                                let mut amount = (remaining * (sec.discount_percentage / dec!(100.00))).round_dp(2);
                                if let Some(cap) = sec.cash_cap_gbp {
                                    amount = amount.min(cap);
                                }
                                if amount > dec!(0.00) {
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
                OccupancyType::UNOCCUPIED => {
                    consecutive_occupied_days = 0;
                    let is_industrial = req.hereditament.scat_code == "096" || flags.is_industrial_hereditament;
                    let max_exempt_days: i64 = if is_industrial { 182 } else { 91 };

                    if rv < dec!(2900.00) {
                        let amount = chargeable_before_reliefs - segment_relief_total;
                        segment_relief_total += amount;
                        reliefs.push(ItemizedRelief {
                            relief_code: "EMPTY_LOW_VALUE".to_string(),
                            relief_name: crate::i18n::localize_relief_name("Empty Property Low RV Exemption", lang),
                            statutory_authority: "LGFA 1988 s.45 / SI 2008/386".to_string(),
                            discount_percentage: dec!(100.00),
                            amount_deducted: amount,
                        });
                    } else if cumulative_unoccupied_days < max_exempt_days {
                        let remaining_exempt_days = (max_exempt_days - cumulative_unoccupied_days).min(win.period.days);
                        let exempt_ratio = Decimal::from(remaining_exempt_days) / days_dec;
                        let remaining = chargeable_before_reliefs - segment_relief_total;
                        let amount = (remaining * exempt_ratio).round_dp(2);
                        cumulative_unoccupied_days += win.period.days;
                        if amount > dec!(0.00) {
                            segment_relief_total += amount;
                            let empty_name = if is_industrial { "Empty Industrial Rate-Free Period (6 Months)" } else { "Empty Property Rate-Free Period (3 Months)" };
                            reliefs.push(ItemizedRelief {
                                relief_code: "EMPTY_PROPERTY_EXEMPTION".to_string(),
                                relief_name: crate::i18n::localize_relief_name(empty_name, lang),
                                statutory_authority: "LGFA 1988 s.45 / Non-Domestic Rating (Unoccupied Property) Regs".to_string(),
                                discount_percentage: (exempt_ratio * dec!(100.00)).round_dp(2),
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

                    // Unoccupied part: Section 44A(5) empty property relief
                    let is_industrial = req.hereditament.scat_code == "096" || flags.is_industrial_hereditament;
                    let max_exempt_days: i64 = if is_industrial { 182 } else { 91 };

                    if unocc_rv < dec!(2900.00) {
                        if unocc_chargeable > dec!(0.00) {
                            segment_relief_total += unocc_chargeable;
                            reliefs.push(ItemizedRelief {
                                relief_code: "SECTION_44A_PART_OCCUPIED".to_string(),
                                relief_name: crate::i18n::localize_relief_name("Section 44A Part-Occupied Relief", lang),
                                statutory_authority: "LGFA 1988 s.44A / s.45".to_string(),
                                discount_percentage: dec!(100.00),
                                amount_deducted: unocc_chargeable,
                            });
                        }
                    } else if cumulative_unoccupied_days < max_exempt_days {
                        let remaining_exempt_days = (max_exempt_days - cumulative_unoccupied_days).min(win.period.days);
                        let exempt_ratio = Decimal::from(remaining_exempt_days) / days_dec;
                        let s44a_amount = (unocc_chargeable * exempt_ratio).round_dp(2);
                        cumulative_unoccupied_days += win.period.days;
                        if s44a_amount > dec!(0.00) {
                            segment_relief_total += s44a_amount;
                            reliefs.push(ItemizedRelief {
                                relief_code: "SECTION_44A_PART_OCCUPIED".to_string(),
                                relief_name: crate::i18n::localize_relief_name("Section 44A Part-Occupied Relief", lang),
                                statutory_authority: "LGFA 1988 s.44A".to_string(),
                                discount_percentage: (exempt_ratio * dec!(100.00)).round_dp(2),
                                amount_deducted: s44a_amount,
                            });
                        }
                    } else {
                        cumulative_unoccupied_days += win.period.days;
                    }

                    // Occupied part reliefs (e.g. Charitable Relief or SBRR applied to occupied portion)
                    if flags.is_charitable_purposes 
                        || req.party_type == PartyType::CHARITY 
                        || req.party_type == PartyType::CASC 
                    {
                        let char_pct = dec!(80.00);
                        let amount = (occ_chargeable * (char_pct / dec!(100.00))).round_dp(2);
                        segment_relief_total += amount;
                        reliefs.push(ItemizedRelief {
                            relief_code: "CHARITY_MANDATORY".to_string(),
                            relief_name: crate::i18n::localize_relief_name("Mandatory Charitable Rate Relief", lang),
                            statutory_authority: "LGFA 1988 s.43(5) / s.44A".to_string(),
                            discount_percentage: char_pct,
                            amount_deducted: amount,
                        });
                    }

                    if flags.is_small_business_sole_property {
                        if let Some(sbrr) = self.repo.resolve_sbrr(req.hereditament.jurisdiction, &win.financial_year, rv) {
                            let amount = (occ_chargeable * (sbrr.discount_percentage / dec!(100.00))).round_dp(2);
                            if amount > dec!(0.00) {
                                segment_relief_total += amount;
                                reliefs.push(ItemizedRelief {
                                    relief_code: sbrr.relief_code,
                                    relief_name: crate::i18n::localize_relief_name(&sbrr.relief_name, lang),
                                    statutory_authority: format!("{} / LGFA 1988 s.44A", sbrr.statutory_authority),
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
            {
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
