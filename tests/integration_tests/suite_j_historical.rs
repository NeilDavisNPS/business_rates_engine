//! Suite J: Historical Statutory Parameters & Revaluations (1990–2022)
//!
//! Part of the Business Rates Calculation Engine statutory test suite.

#[allow(unused_imports)]
use super::common::*;
#[allow(unused_imports)]
use rust_decimal_macros::dec;
#[allow(unused_imports)]
use chrono::NaiveDate;
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use axum::body::Body;
#[allow(unused_imports)]
use axum::http::{header, Request, StatusCode, HeaderMap};
#[allow(unused_imports)]
use tower::ServiceExt;

#[allow(unused_imports)]
use business_rates_engine::models::calculation::*;
#[allow(unused_imports)]
use business_rates_engine::models::enums::{Jurisdiction, PartyType, OccupancyType};
#[allow(unused_imports)]
use business_rates_engine::engine::calculator::CalculationEngine;
#[allow(unused_imports)]
use business_rates_engine::db::repository::ParameterRepository;
#[allow(unused_imports)]
use business_rates_engine::models::AppState;
#[allow(unused_imports)]
use business_rates_engine::api::handlers::resolve_language;
#[allow(unused_imports)]
use business_rates_engine::i18n::Language;
#[allow(unused_imports)]
use business_rates_engine::error::AppError;

#[tokio::test]
async fn test_historical_calculation_1990_91_inaugural_nndr() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1990/91: Inaugural non-domestic rating year under LGFA 1988 (1 April 1990 to 31 March 1991)
    // England standard multiplier = 0.348
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1991, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(10000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    // 10000 * 0.348 = 3480.00. SBRR did not exist in 1990/91.
    assert_eq!(seg.financial_year, "1990/91");
    assert_eq!(seg.rateable_value, dec!(10000.00));
    assert_eq!(seg.multiplier_used.rate, dec!(0.348));
    assert_eq!(seg.gross_liability, dec!(3480.00));
    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "SBRR"));
    assert_eq!(seg.net_segment_payable, dec!(3480.00));
}

#[tokio::test]
async fn test_historical_transitional_phasing_1991_92() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1991/92: Small property (RV £8,000 <= £10,000 under SI 1989/1358)
    // Previous year (1990/91) RV = £5,000, multiplier = 0.348. Base = 5000 * 0.348 = 1740.00
    // 1991/92 Small upward cap = +15% (factor 1.15). Capped = 1740.00 * 1.15 = 2001.00
    // 1991/92 England multiplier = 0.386. Gross = 8000 * 0.386 = 3088.00
    // Transitional relief = 3088.00 - 2001.00 = 1087.00
    // Net payable = 2001.00
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(1991, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1992, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(1991, 3, 31).unwrap()),
                    rateable_value: dec!(5000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1991, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(8000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.financial_year, "1991/92");
    assert_eq!(seg.gross_liability, dec!(3088.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(1087.00));
    assert_eq!(seg.net_segment_payable, dec!(2001.00));
}

#[tokio::test]
async fn test_historical_transitional_phasing_1993_94() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1993/94: Small property (RV £9,000 <= £10,000)
    // Previous year (1992/93) RV = £6,000, 1992/93 multiplier = 0.406
    // Base annual liability = 6000 * 0.406 = 2436.00
    // 1993/94 Small upward cap = +15% (factor 1.15)
    // Capped liability = 2436.00 * 1.15 = 2801.40
    // 1993/94 multiplier = 0.416. Gross = 9000 * 0.416 = 3744.00
    // Transitional relief = 3744.00 - 2801.40 = 942.60
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(1993, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1994, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(1993, 3, 31).unwrap()),
                    rateable_value: dec!(6000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1993, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(9000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.gross_liability, dec!(3744.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(942.60));
    assert_eq!(seg.net_segment_payable, dec!(2801.40));
}

#[tokio::test]
async fn test_historical_transitional_phasing_1995_list_revaluation() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1995/96 Revaluation (SI 1994/3279): Large property (> £10,000 RV)
    // 1994/95 RV = £8,000, 1994/95 multiplier = 0.423. Base = 8000 * 0.423 = 3384.00
    // 1995/96 Large upward cap = +20% (factor 1.20). Capped = 3384.00 * 1.20 = 4060.80
    // 1995/96 RV = £15,000, multiplier = 0.432. Gross = 15000 * 0.432 = 6480.00
    // Transitional relief = 6480.00 - 4060.80 = 2419.20
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(1995, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1996, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(1995, 3, 31).unwrap()),
                    rateable_value: dec!(8000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1995, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(15000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.gross_liability, dec!(6480.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(2419.20));
    assert_eq!(seg.net_segment_payable, dec!(4060.80));
}

#[tokio::test]
async fn test_historical_sbrr_pre_2005_returns_no_relief() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // In 2003/04 (pre-2005), SBRR did not exist in primary legislation (enacted by LGA 2003 s.61).
    let request = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2003, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2004, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2000, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(4000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    // Multiplier for England 2003/04 is 0.444. Gross = 4000 * 0.444 = 1776.00
    assert_eq!(seg.rateable_value, dec!(4000.00));
    assert_eq!(seg.gross_liability, dec!(1776.00));
    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "SBRR"));
    assert_eq!(seg.net_segment_payable, dec!(1776.00));
}

#[tokio::test]
async fn test_historical_sbrr_2005_list_england_full_and_taper() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 2007/08 England: Small Business multiplier = 0.441
    // Full relief test: RV £4,000 (max relief in 2005 list was 50% up to £5,000 under SI 2004/3315)
    let req_full = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2007, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2008, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2005, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(4000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_full = engine.calculate(req_full);
    let seg_full = &res_full.annual_billing_segments[0];
    assert_eq!(seg_full.gross_liability, dec!(1764.00));
    let r_full = seg_full.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(r_full.discount_percentage, dec!(50.00));
    assert_eq!(r_full.amount_deducted, dec!(882.00));
    assert_eq!(seg_full.net_segment_payable, dec!(882.00));
    assert_eq!(r_full.statutory_authority, "LGFA 1988 s.43(4B) / SI 2004/3315");

    // Taper test: RV £7,500 (taper from 50% to 0% between £5,001 and £10,000)
    // Formula: (10,000 - 7,500) / 4999.99 * 50 = 25.00%
    let req_tap = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2007, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2008, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2005, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(7500.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_tap = engine.calculate(req_tap);
    let seg_tap = &res_tap.annual_billing_segments[0];
    assert_eq!(seg_tap.gross_liability, dec!(3307.50));
    let r_tap = seg_tap.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(r_tap.discount_percentage, dec!(25.00));
    assert_eq!(r_tap.amount_deducted, dec!(826.88));
    assert_eq!(seg_tap.net_segment_payable, dec!(2480.62));
}

#[tokio::test]
async fn test_historical_sbrr_2010_list_england_and_scotland() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // England 2012/13: Small Business multiplier = 0.450
    // Doubled relief under SI 2010/1959: 100% up to £6,000; taper £6,001 to £12,000
    let req_eng = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2012, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2013, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(9000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_eng = engine.calculate(req_eng);
    let seg_eng = &res_eng.annual_billing_segments[0];
    // Taper at £9,000: (12,000 - 9,000) / 5999.99 * 100% = 50.00%
    let r_eng = seg_eng.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(r_eng.discount_percentage, dec!(50.00));
    assert_eq!(r_eng.amount_deducted, dec!(2025.00));
    assert_eq!(seg_eng.net_segment_payable, dec!(2025.00));

    // Scotland 2012/13: Multiplier = 0.450
    // SSI 2010/37: 100% <= £10,000; 50% <= £12,000; 25% <= £18,000
    let req_sco_mid = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2012, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2013, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(11000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_sco = engine.calculate(req_sco_mid);
    let seg_sco = &res_sco.annual_billing_segments[0];
    let r_sco = seg_sco.itemized_reliefs.iter().find(|r| r.relief_code == "SBBS").unwrap();
    assert_eq!(r_sco.discount_percentage, dec!(50.00));
    assert_eq!(r_sco.amount_deducted, dec!(2475.00));
    assert_eq!(seg_sco.net_segment_payable, dec!(2475.00));
}

#[tokio::test]
async fn test_historical_transitional_phasing_2000_list() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // 2000 List Revaluation (SI 1999/3379)
    // England 1999/00: RV £4,000 * 0.489 = £1,956.00
    // 2000/01: RV £8,000 (Small <= £12,000). Multiplier 0.416. Gross = 8,000 * 0.416 = £3,328.00.
    // Small upward cap = 5.00%. Capped base = 1,956.00 * 1.05 = £2,053.80.
    // Transitional relief = £3,328.00 - £2,053.80 = £1,274.20.
    // Net = £2,053.80.
    let req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2000, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2001, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1995, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2000, 3, 31).unwrap()),
                    rateable_value: dec!(4000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2000, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(8000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(true),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res = engine.calculate(req);
    let seg = &res.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(3328.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(1274.20));
    assert_eq!(seg.net_segment_payable, dec!(2053.80));
}

#[tokio::test]
async fn test_historical_welsh_transitional_relief_2017_list() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Welsh 2017 List Revaluation (W.S.I. 2016/1247)
    // Base 2016/17: RV £4,000 qualifies for 100% SBRR -> BL = £0.00
    // 2017/18: RV £10,000 * 0.499 = £4,990.00 gross; SBRR taper (33.33% relief) = £1,663.17
    // NCA = £4,990.00 - £1,663.17 = £3,326.83 (> £100 threshold under reg 9(b))
    // Year 1 (2017/18) Welsh TR = 75% of £3,326.83 = £2,495.12
    // Net = £4,990.00 - £1,663.17 - £2,495.12 = £831.71
    let req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2017, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2018, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2017, 3, 31).unwrap()),
                    rateable_value: dec!(4000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2017, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(10000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(true),
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res = engine.calculate(req);
    let seg = &res.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(4990.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(2495.12));
    assert_eq!(seg.net_segment_payable, dec!(831.71));
}

#[tokio::test]
async fn test_historical_transitional_phasing_2017_list_revaluation() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 2017/18 Revaluation (SI 2016/1265): Large property (> £100,000 RV)
    // 2016/17 RV = £80,000, 2016/17 standard multiplier = 0.497. Base = 80000 * 0.497 = 39760.00
    // 2017/18 Large upward cap = +42% (factor 1.42). Capped = 39760.00 * 1.42 = 56459.20
    // 2017/18 RV = £150,000, standard multiplier = 0.479. Gross = 150000 * 0.479 = 71850.00
    // Transitional relief = 71850.00 - 56459.20 = 15390.80
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2017, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2018, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2017, 3, 31).unwrap()),
                    rateable_value: dec!(80000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2017, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(150000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.gross_liability, dec!(71850.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(15390.80));
    assert_eq!(seg.net_segment_payable, dec!(56459.20));
}

#[tokio::test]
async fn test_england_2017_list_transitional_relief_large_cap_2021_22_is_6_percent() {
    // Reproduction for Issue #63:
    // Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2016 (S.I. 2016/1265),
    // Schedule Table 1: Upward limit for large hereditaments (> £100,000 RV) in 2021/22 is 6.0%, not 16.0%.
    let repo = ParameterRepository::new_seeded();
    let cap_2021 = repo
        .all_transitional_caps()
        .iter()
        .find(|c| {
            c.jurisdiction == Jurisdiction::ENGLAND
                && c.revaluation_list_year == 2017
                && c.financial_year == "2021/22"
                && c.property_tier == "LARGE"
        })
        .expect("Expected 2021/22 LARGE transitional cap for England 2017 list");

    assert_eq!(
        cap_2021.upward_cap_percentage,
        dec!(6.00),
        "S.I. 2016/1265 large property cap for 2021/22 must be 6.00%, got {}",
        cap_2021.upward_cap_percentage
    );
}

#[tokio::test]
async fn test_historical_sbrr_2010_11_england_pre_and_post_october() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1. ENG-SBRR-2010-PRE-OCT: 1 April 2010 – 30 September 2010 (50% max under SI 2004/3315 / SI 2010/145)
    let req_pre = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2010, 9, 30).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2005, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(5000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(false),
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let res_pre = engine.calculate(req_pre);
    let seg_pre = &res_pre.annual_billing_segments[0];
    let r_pre = seg_pre.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(r_pre.discount_percentage, dec!(50.00), "Pre-October 2010 SBRR in England must be 50.00%");

    // 2. ENG-SBRR-2010-POST-OCT: 1 October 2010 – 31 March 2011 (100% max under SI 2010/1659 temporary doubling)
    let req_post = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2010, 10, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2011, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2005, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(5000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(false),
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let res_post = engine.calculate(req_post);
    let seg_post = &res_post.annual_billing_segments[0];
    let r_post = seg_post.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(r_post.discount_percentage, dec!(100.00), "Post-October 2010 SBRR in England must be 100.00%");

    // 3. Full rating year 2010/11: slices into 2 segments across 1 October 2010
    let req_full = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2011, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2005, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(5000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(false),
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let res_full = engine.calculate(req_full);
    assert_eq!(res_full.annual_billing_segments.len(), 2, "Full year 2010/11 must slice into 2 segments across 1 October 2010");
    assert_eq!(res_full.annual_billing_segments[0].itemized_reliefs[0].discount_percentage, dec!(50.00));
    assert_eq!(res_full.annual_billing_segments[1].itemized_reliefs[0].discount_percentage, dec!(100.00));
}


