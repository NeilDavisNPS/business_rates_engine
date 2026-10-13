use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::models::calculation::{LiabilityCalculationRequest, AnnualBillingSegment, QualifyingReliefFlags};
use crate::models::enums::{Jurisdiction, OccupancyType, PartyType};
use crate::i18n::{Language, LocaleBundle};

pub fn generate_narrative(
    req: &LiabilityCalculationRequest,
    segments: &[AnnualBillingSegment],
    total_net: Decimal,
    lang: Language,
) -> String {
    let bundle = crate::i18n::get_locale(lang);
    let n = &bundle.narrative;
    let mut lines = Vec::new();

    let (start_d, end_d) = req.resolve_overall_period();

    lines.push(n.header_title.clone());
    lines.push(n.divider.clone());

    let framework = match req.hereditament.jurisdiction {
        Jurisdiction::ENGLAND => &n.frameworks.england,
        Jurisdiction::WALES => &n.frameworks.wales,
        Jurisdiction::SCOTLAND => &n.frameworks.scotland,
    };
    lines.push(crate::i18n::format_template(&n.framework_intro, &[("framework", framework)]));

    let seg_count = segments.len().to_string();
    let start_str = start_d.to_string();
    let end_str = end_d.to_string();
    lines.push(crate::i18n::format_template(
        &n.liability_period,
        &[("start_date", &start_str), ("end_date", &end_str), ("segments_count", &seg_count)],
    ));

    for (idx, seg) in segments.iter().enumerate() {
        let idx_str = (idx + 1).to_string();
        lines.push(crate::i18n::format_template(
            &n.financial_year_header,
            &[("index", &idx_str), ("financial_year", &seg.financial_year)],
        ));

        let occ_label = match seg.occupancy_type.unwrap_or(req.occupancy_type) {
            OccupancyType::OCCUPIED => &n.occupancy_status.occupied,
            OccupancyType::UNOCCUPIED => &n.occupancy_status.unoccupied,
            OccupancyType::PART_OCCUPIED => &n.occupancy_status.part_occupied,
        };
        lines.push(crate::i18n::format_template(&n.lines.occupancy_status, &[("status", occ_label)]));
        lines.push(crate::i18n::format_template(&n.lines.rateable_value, &[("rateable_value", &seg.rateable_value.to_string())]));

        let rate_str = seg.multiplier_used.rate.to_string();
        lines.push(crate::i18n::format_template(
            &n.lines.multiplier,
            &[("multiplier_type", &seg.multiplier_used.multiplier_type), ("rate", &rate_str)],
        ));

        if seg.multiplier_used.multiplier_type.ends_with("_PROVISIONAL") {
            if let Some(note) = n.disqualifications.get("multiplier_wales_2027_provisional") {
                lines.push(format!("    - {}", note));
            }
        }
        if req.qualifying_relief_flags.is_higher_multiplier_excepted
            && req.hereditament.jurisdiction == Jurisdiction::WALES
            && seg.rateable_value > dec!(100000.00)
        {
            if let Some(note) = n.disqualifications.get("multiplier_wales_higher_excepted") {
                let note_formatted = crate::i18n::format_template(note, &[("rate", &rate_str)]);
                lines.push(format!("    - {}", note_formatted));
            }
        }

        let days_str = seg.segment_period.days.to_string();
        let s_str = seg.segment_period.start_date.to_string();
        let e_str = seg.segment_period.end_date.to_string();
        lines.push(crate::i18n::format_template(
            &n.lines.segment_duration,
            &[("days", &days_str), ("start_date", &s_str), ("end_date", &e_str)],
        ));

        lines.push(crate::i18n::format_template(&n.lines.gross_liability, &[("gross_liability", &seg.gross_liability.to_string())]));

        if seg.transitional_adjustment.adjustment_type != "NONE" {
            let trans_type = if seg.transitional_adjustment.adjustment_type == "RELIEF" {
                n.lines.transitional_relief_label.as_deref().unwrap_or("RELIEF")
            } else {
                n.lines.transitional_premium_label.as_deref().unwrap_or(&seg.transitional_adjustment.adjustment_type)
            };
            let amount_str = seg.transitional_adjustment.amount.to_string();
            lines.push(crate::i18n::format_template(
                &n.lines.transitional_adjustment,
                &[("type", trans_type), ("amount", &amount_str)],
            ));
        }

        if seg.itemized_reliefs.is_empty() {
            lines.push(n.lines.applied_reliefs_none.clone());
        } else {
            for r in &seg.itemized_reliefs {
                let pct_str = r.discount_percentage.to_string();
                let amt_str = r.amount_deducted.to_string();
                lines.push(crate::i18n::format_template(
                    &n.lines.applied_relief_item,
                    &[
                        ("relief_code", &r.relief_code),
                        ("relief_name", &r.relief_name),
                        ("discount_percentage", &pct_str),
                        ("amount_deducted", &amt_str),
                        ("statutory_authority", &r.statutory_authority),
                    ],
                ));
            }
        }

        let audit_notes = get_relief_non_award_explanations(req, seg, lang);
        if !audit_notes.is_empty() {
            lines.push(n.lines.audit_header.clone());
            for note in audit_notes {
                lines.push(crate::i18n::format_template(&n.lines.audit_item, &[("note", &note)]));
            }
        }

        if !seg.statutory_penalties.is_empty() {
            for p in &seg.statutory_penalties {
                let amt_str = p.amount.to_string();
                lines.push(crate::i18n::format_template(
                    &n.lines.statutory_penalty_item,
                    &[
                        ("penalty_type", &p.penalty_type),
                        ("amount", &amt_str),
                        ("calculation_formula", &p.calculation_formula),
                        ("statutory_authority", &p.statutory_authority),
                    ],
                ));
            }
        }

        lines.push(crate::i18n::format_template(&n.lines.net_segment_payable, &[("net_segment_payable", &seg.net_segment_payable.to_string())]));
    }

    if req.hereditament.scat_code == "249" {
        lines.push(n.holiday_let.header.clone());
        let period_start = req.occupation_period.start_date;
        let is_wales_post_2023 = req.hereditament.jurisdiction == Jurisdiction::WALES
            && period_start >= NaiveDate::from_ymd_opt(2023, 4, 1).unwrap();
        let threshold = if is_wales_post_2023 || req.hereditament.jurisdiction == Jurisdiction::SCOTLAND {
            182
        } else {
            70
        };
        let avail_threshold = if is_wales_post_2023 { 252 } else { 140 };

        let actual_disqualified = req.qualifying_relief_flags.holiday_let_actual_days_let
            .map(|act| act < threshold)
            .unwrap_or(false);
        let avail_disqualified = req.qualifying_relief_flags.holiday_let_days_available
            .map(|av| av < avail_threshold)
            .unwrap_or(false);

        if req.qualifying_relief_flags.holiday_let_actual_days_let.is_some() || req.qualifying_relief_flags.holiday_let_days_available.is_some() {
            if actual_disqualified || avail_disqualified {
                let actual = req.qualifying_relief_flags.holiday_let_actual_days_let.unwrap_or(0);
                let act_str = actual.to_string();
                let thr_str = threshold.to_string();
                lines.push(crate::i18n::format_template(
                    &n.holiday_let.disqualified_actual,
                    &[("actual", &act_str), ("threshold", &thr_str)],
                ));
                lines.push(n.holiday_let.disqualified_status.clone());
            } else {
                let actual = req.qualifying_relief_flags.holiday_let_actual_days_let.unwrap_or(threshold);
                let act_str = actual.to_string();
                let thr_str = threshold.to_string();
                lines.push(crate::i18n::format_template(
                    &n.holiday_let.qualified_actual,
                    &[("actual", &act_str), ("threshold", &thr_str)],
                ));
                lines.push(n.holiday_let.qualified_status.clone());
            }
        } else {
            lines.push(n.holiday_let.default_actual.clone());
            lines.push(n.holiday_let.default_status.clone());
        }
    }

    lines.push(n.divider.clone());
    lines.push(crate::i18n::format_template(&n.total_net_payable, &[("total_net", &total_net.to_string())]));

    lines.join("\n")
}

fn resolve_segment_flags<'a>(
    req: &'a LiabilityCalculationRequest,
    seg: &AnnualBillingSegment,
) -> &'a QualifyingReliefFlags {
    if let Some(ref periods) = req.occupancy_periods {
        for p in periods {
            if seg.segment_period.start_date >= p.start_date && seg.segment_period.start_date <= p.end_date {
                if let Some(ref flags) = p.qualifying_relief_flags {
                    return flags;
                }
            }
        }
    }
    &req.qualifying_relief_flags
}

fn get_disqualification(bundle: &LocaleBundle, key: &str, vars: &[(&str, &str)]) -> String {
    if let Some(template) = bundle.narrative.disqualifications.get(key) {
        crate::i18n::format_template(template, vars)
    } else {
        let en = crate::i18n::get_locale(Language::En);
        if let Some(template) = en.narrative.disqualifications.get(key) {
            crate::i18n::format_template(template, vars)
        } else {
            key.to_string()
        }
    }
}

fn get_relief_non_award_explanations(
    req: &LiabilityCalculationRequest,
    seg: &AnnualBillingSegment,
    lang: Language,
) -> Vec<String> {
    let mut explanations = Vec::new();
    let bundle = crate::i18n::get_locale(lang);
    let flags = resolve_segment_flags(req, seg);
    let rv = seg.rateable_value;
    let occ = seg.occupancy_type.unwrap_or(req.occupancy_type);
    let is_unoccupied = occ == OccupancyType::UNOCCUPIED;
    let scat = &req.hereditament.scat_code;
    let jurisdiction = req.hereditament.jurisdiction;
    let zero_net = seg.net_segment_payable == Decimal::ZERO;

    // 1. Rural Settlement Relief
    if flags.is_rural_settlement && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "RURAL_SETTLEMENT") {
        let is_pub_or_petrol = scat == "209" || scat == "211";
        let rv_cap = if is_pub_or_petrol {
            Decimal::from(12500)
        } else {
            Decimal::from(8500)
        };
        let auth_key = if jurisdiction == Jurisdiction::SCOTLAND {
            "rural_authority_sco"
        } else {
            "rural_authority_eng_wal"
        };
        let authority = get_disqualification(bundle, auth_key, &[]);

        let note = if is_unoccupied {
            get_disqualification(bundle, "rural_unoccupied", &[])
        } else if rv > rv_cap {
            let cap_str = if is_pub_or_petrol { "£12,500" } else { "£8,500" };
            let cat_key = if is_pub_or_petrol { "rural_cat_pub_petrol" } else { "rural_cat_general" };
            let cat = get_disqualification(bundle, cat_key, &[]);
            let rv_str = rv.to_string();
            get_disqualification(bundle, "rural_rv_cap", &[
                ("rv", &rv_str),
                ("cap_str", cap_str),
                ("cat", &cat),
                ("authority", &authority),
            ])
        } else if zero_net {
            get_disqualification(bundle, "rural_zero_net", &[])
        } else {
            get_disqualification(bundle, "rural_default", &[("authority", &authority)])
        };
        explanations.push(note);
    }

    // 2. Small Business Rate Relief / SBBS
    if (flags.is_small_business_sole_property || flags.is_small_business_second_property_grace)
        && !seg.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("SB"))
    {
        let note = if is_unoccupied {
            get_disqualification(bundle, "sbrr_unoccupied", &[])
        } else {
            let rv_str = rv.to_string();
            match jurisdiction {
                Jurisdiction::ENGLAND => {
                    if rv > Decimal::from(15000) {
                        get_disqualification(bundle, "sbrr_england_rv_cap", &[("rv", &rv_str)])
                    } else if zero_net {
                        get_disqualification(bundle, "sbrr_england_zero_net", &[])
                    } else {
                        get_disqualification(bundle, "sbrr_england_default", &[])
                    }
                }
                Jurisdiction::WALES => {
                    if rv > Decimal::from(12000) {
                        get_disqualification(bundle, "sbrr_wales_rv_cap", &[("rv", &rv_str)])
                    } else if zero_net {
                        get_disqualification(bundle, "sbrr_wales_zero_net", &[])
                    } else {
                        get_disqualification(bundle, "sbrr_wales_default", &[])
                    }
                }
                Jurisdiction::SCOTLAND => {
                    if rv > Decimal::from(20000) {
                        get_disqualification(bundle, "sbrr_scotland_rv_cap", &[("rv", &rv_str)])
                    } else if zero_net {
                        get_disqualification(bundle, "sbrr_scotland_zero_net", &[])
                    } else {
                        get_disqualification(bundle, "sbrr_scotland_default", &[])
                    }
                }
            }
        };
        explanations.push(note);
    }

    // 3. Retail, Hospitality & Leisure (RHL)
    if (flags.is_rhl_qualifying || flags.is_food_and_drink) && !seg.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")) {
        let note = if is_unoccupied {
            get_disqualification(bundle, "rhl_unoccupied", &[])
        } else {
            match jurisdiction {
                Jurisdiction::ENGLAND => {
                    let is_2026_plus = seg.financial_year.starts_with("2026") || seg.financial_year.starts_with("2027");
                    if is_2026_plus {
                        let rate_str = seg.multiplier_used.rate.to_string();
                        get_disqualification(bundle, "rhl_england_2026_plus", &[("rate", &rate_str)])
                    } else if zero_net {
                        get_disqualification(bundle, "rhl_england_zero_net", &[])
                    } else {
                        get_disqualification(bundle, "rhl_england_default", &[])
                    }
                }
                Jurisdiction::WALES => {
                    let is_2026_plus = seg.financial_year.starts_with("2026") || seg.financial_year.starts_with("2027");
                    if is_2026_plus {
                        if seg.multiplier_used.multiplier_type.starts_with("RETAIL") {
                            get_disqualification(bundle, "rhl_wales_2026_retail_multiplier_excluded", &[])
                        } else if scat == "249" || flags.holiday_let_actual_days_let.is_some() || flags.holiday_let_days_available.is_some() {
                            get_disqualification(bundle, "rhl_wales_2026_accommodation_excluded", &[])
                        } else if scat == "211" {
                            get_disqualification(bundle, "rhl_wales_petrol_excluded", &[])
                        } else if flags.is_retail_premises || scat == "225" {
                            get_disqualification(bundle, "rhl_wales_2026_retail_goods_excluded", &[])
                        } else if !flags.is_food_and_drink && scat != "209" {
                            get_disqualification(bundle, "rhl_wales_2026_not_food_drink", &[])
                        } else if zero_net {
                            get_disqualification(bundle, "rhl_wales_zero_net", &[])
                        } else {
                            get_disqualification(bundle, "rhl_wales_default", &[])
                        }
                    } else if zero_net {
                        get_disqualification(bundle, "rhl_wales_zero_net", &[])
                    } else {
                        get_disqualification(bundle, "rhl_wales_default", &[])
                    }
                }
                Jurisdiction::SCOTLAND => {
                    if !flags.is_island_property && rv > Decimal::from(100000) {
                        let rv_str = rv.to_string();
                        get_disqualification(bundle, "rhl_scotland_rv_cap", &[("rv", &rv_str)])
                    } else if !flags.is_island_property && (seg.financial_year.starts_with("2024") || seg.financial_year.starts_with("2025") || seg.financial_year.starts_with("2026") || seg.financial_year.starts_with("2027")) {
                        get_disqualification(bundle, "rhl_scotland_not_island", &[])
                    } else if zero_net {
                        get_disqualification(bundle, "rhl_scotland_zero_net", &[])
                    } else {
                        get_disqualification(bundle, "rhl_scotland_default", &[])
                    }
                }
            }
        };
        explanations.push(note);
    }

    // 4. Mandatory Charitable Relief (80%)
    let charity_claimed = flags.is_charitable_purposes || req.party_type == PartyType::CHARITY || req.party_type == PartyType::CASC;
    if charity_claimed && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "CHARITY_MANDATORY") {
        let note = if is_unoccupied {
            get_disqualification(bundle, "charity_unoccupied", &[])
        } else if zero_net {
            get_disqualification(bundle, "charity_zero_net", &[])
        } else {
            get_disqualification(bundle, "charity_default", &[])
        };
        explanations.push(note);
    }

    // 5. Charity Discretionary Top-Up (20%)
    if flags.is_charity_discretionary_top_up && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "CHARITY_DISCRETIONARY_TOPUP") {
        let note = if !charity_claimed {
            get_disqualification(bundle, "charity_topup_not_qualifying", &[])
        } else if zero_net {
            get_disqualification(bundle, "charity_topup_zero_net", &[])
        } else {
            get_disqualification(bundle, "charity_topup_default", &[])
        };
        explanations.push(note);
    }

    // 6. Improvement Relief (G-Factor)
    if flags.qualifying_improvement_rv_increment > Decimal::ZERO && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "IMPROVEMENT_G_FACTOR") {
        let fy_start = seg.financial_year.split('/').next().and_then(|y| y.parse::<i32>().ok()).unwrap_or(0);
        let note = if jurisdiction != Jurisdiction::ENGLAND && jurisdiction != Jurisdiction::WALES {
            get_disqualification(bundle, "improvement_not_england", &[])
        } else if fy_start < 2024 {
            get_disqualification(bundle, "improvement_before_2024", &[])
        } else if fy_start >= 2029 {
            get_disqualification(bundle, "improvement_after_2029", &[])
        } else if is_unoccupied {
            get_disqualification(bundle, "improvement_unoccupied", &[])
        } else if zero_net {
            get_disqualification(bundle, "improvement_zero_net", &[])
        } else {
            get_disqualification(bundle, "improvement_default", &[])
        };
        explanations.push(note);
    }

    // 7. Heat Network Exemption
    if flags.is_heat_network && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "HEAT_NETWORK") {
        let fy_start = seg.financial_year.split('/').next().and_then(|y| y.parse::<i32>().ok()).unwrap_or(0);
        let note = if jurisdiction != Jurisdiction::ENGLAND && jurisdiction != Jurisdiction::WALES {
            get_disqualification(bundle, "heat_network_not_england", &[])
        } else if fy_start < 2024 {
            get_disqualification(bundle, "heat_network_before_2024", &[])
        } else if fy_start >= 2035 {
            get_disqualification(bundle, "heat_network_after_2035", &[])
        } else if is_unoccupied {
            get_disqualification(bundle, "heat_network_unoccupied", &[])
        } else if zero_net {
            get_disqualification(bundle, "heat_network_zero_net", &[])
        } else {
            get_disqualification(bundle, "heat_network_default", &[])
        };
        explanations.push(note);
    }

    // 8. Public Lavatory Relief
    if flags.is_public_lavatory && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "PUBLIC_LAVATORY") {
        let note = if jurisdiction == Jurisdiction::SCOTLAND {
            get_disqualification(bundle, "public_lavatory_scotland", &[])
        } else if is_unoccupied {
            get_disqualification(bundle, "public_lavatory_unoccupied", &[])
        } else if zero_net {
            get_disqualification(bundle, "public_lavatory_zero_net", &[])
        } else {
            get_disqualification(bundle, "public_lavatory_default", &[])
        };
        explanations.push(note);
    }

    // 9. Scottish Renewable Energy Generation Relief
    if flags.is_renewable_energy_generation && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "RENEWABLE_ENERGY_RELIEF") {
        let note = if jurisdiction != Jurisdiction::SCOTLAND {
            get_disqualification(bundle, "renewable_energy_not_scotland", &[])
        } else if rv > Decimal::from(1000000) {
            let rv_str = rv.to_string();
            get_disqualification(bundle, "renewable_energy_rv_cap", &[("rv", &rv_str)])
        } else if is_unoccupied {
            get_disqualification(bundle, "renewable_energy_unoccupied", &[])
        } else if zero_net {
            get_disqualification(bundle, "renewable_energy_zero_net", &[])
        } else {
            get_disqualification(bundle, "renewable_energy_default", &[])
        };
        explanations.push(note);
    }

    // 10. Telecommunications Infrastructure Relief
    if flags.is_telecommunications_infrastructure && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "TELECOM_FIBRE_RELIEF") {
        let note = if jurisdiction == Jurisdiction::WALES {
            get_disqualification(bundle, "telecom_wales_no_scheme", &[])
        } else if is_unoccupied {
            get_disqualification(bundle, "telecom_unoccupied", &[])
        } else if zero_net {
            get_disqualification(bundle, "telecom_zero_net", &[])
        } else {
            get_disqualification(bundle, "telecom_default", &[])
        };
        explanations.push(note);
    }

    // 11. Scottish SBBS Transitional Relief (SBBS TR)
    if flags.is_sbbs_tr_qualifying && !seg.itemized_reliefs.iter().any(|r| r.relief_code.contains("SBBS_TR")) {
        let note = if jurisdiction != Jurisdiction::SCOTLAND {
            get_disqualification(bundle, "sbbs_tr_not_scotland", &[])
        } else {
            get_disqualification(bundle, "sbbs_tr_default", &[])
        };
        explanations.push(note);
    }

    // 12. Supporting Small Business Relief (SSBR) in England
    if flags.is_ssbr_qualifying && !seg.itemized_reliefs.iter().any(|r| r.relief_code.contains("SSBR")) {
        let note = if jurisdiction != Jurisdiction::ENGLAND {
            get_disqualification(bundle, "ssbr_not_england", &[])
        } else {
            get_disqualification(bundle, "ssbr_default", &[])
        };
        explanations.push(note);
    }

    // 13. Empty Property Exemption (when unoccupied)
    if is_unoccupied && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION" || r.relief_code == "EMPTY_LOW_VALUE") {
        let note = get_disqualification(bundle, "empty_property_expired", &[]);
        explanations.push(note);
    }

    explanations
}
