//! Suite H: Cross-Jurisdictional Reliefs, Lifespans, Holiday Lets & Empty Property
//!
//! Part of the Business Rates Engine statutory test suite.

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
async fn test_statutory_heat_network_exemption() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(40000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_heat_network: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let heat_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "HEAT_NETWORK");
    assert!(heat_relief.is_some());
    let r = heat_relief.unwrap();
    assert_eq!(r.statutory_authority, "Non-Domestic Rating Act 2023 s.2");
    assert_eq!(r.discount_percentage, dec!(100.00));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_heat_network_relief_not_granted_before_1_april_2024() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2023-04-01",
            "end_date": "2024-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_heat_network": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code == "HEAT_NETWORK"),
        "Heat network relief must not be granted before 1 April 2024"
    );
}

#[tokio::test]
async fn test_heat_network_relief_not_granted_on_or_after_1_april_2035() {
    let repo = ParameterRepository::new_seeded().with_additional_multiplier(
        business_rates_engine::db::repository::ParamMultiplier {
            parameter_id: "m_eng_test_2035".to_string(),
            financial_year: "2035/36".to_string(),
            jurisdiction: Jurisdiction::ENGLAND,
            multiplier_type: "STANDARD".to_string(),
            rv_threshold_min: dec!(0.00),
            rv_threshold_max: None,
            multiplier_rate: dec!(0.500),
            effective_from: NaiveDate::from_ymd_opt(2035, 4, 1).unwrap(),
            effective_to: NaiveDate::from_ymd_opt(2036, 3, 31).unwrap(),
        },
    );
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2035-04-01",
            "end_date": "2036-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_heat_network": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code == "HEAT_NETWORK"),
        "Heat network relief must not be granted on or after 1 April 2035"
    );
}

#[tokio::test]
async fn test_ndra_2023_reliefs_granted_in_wales_from_2024() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // 1. Improvement Relief (G-factor) in Wales from 1 April 2024
    let json_imp = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "qualifying_improvement_rv_increment": 5000.00
        }
    });

    let request_imp: LiabilityCalculationRequest = serde_json::from_value(json_imp).unwrap();
    let response_imp = engine.calculate(request_imp);
    let seg_imp = &response_imp.annual_billing_segments[0];
    assert!(
        seg_imp.itemized_reliefs.iter().any(|r| r.relief_code == "IMPROVEMENT_G_FACTOR"),
        "Improvement relief must be granted in Wales from 1 April 2024"
    );

    // 2. Heat Network Relief in Wales from 1 April 2024
    let json_heat = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_heat_network": true
        }
    });

    let request_heat: LiabilityCalculationRequest = serde_json::from_value(json_heat).unwrap();
    let response_heat = engine.calculate(request_heat);
    let seg_heat = &response_heat.annual_billing_segments[0];
    let relief_heat = seg_heat.itemized_reliefs.iter().find(|r| r.relief_code == "HEAT_NETWORK");
    assert!(relief_heat.is_some(), "Heat network relief must be granted in Wales from 1 April 2024");
    assert_eq!(
        relief_heat.unwrap().statutory_authority,
        "Non-Domestic Rating (Heat Networks Relief) (Wales) Regulations 2024"
    );
}

#[tokio::test]
async fn test_improvement_relief_not_granted_before_1_april_2024() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2023-04-01",
            "end_date": "2024-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "qualifying_improvement_rv_increment": 5000.00
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code == "IMPROVEMENT_G_FACTOR"),
        "Improvement relief must not be granted before 1 April 2024"
    );
}

#[tokio::test]
async fn test_improvement_relief_not_granted_on_or_after_1_april_2029() {
    let repo = ParameterRepository::new_seeded().with_additional_multiplier(
        business_rates_engine::db::repository::ParamMultiplier {
            parameter_id: "m_eng_test_2029".to_string(),
            financial_year: "2029/30".to_string(),
            jurisdiction: Jurisdiction::ENGLAND,
            multiplier_type: "STANDARD".to_string(),
            rv_threshold_min: dec!(0.00),
            rv_threshold_max: None,
            multiplier_rate: dec!(0.500),
            effective_from: NaiveDate::from_ymd_opt(2029, 4, 1).unwrap(),
            effective_to: NaiveDate::from_ymd_opt(2030, 3, 31).unwrap(),
        },
    );
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2029-04-01",
            "end_date": "2030-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "qualifying_improvement_rv_increment": 5000.00
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code == "IMPROVEMENT_G_FACTOR"),
        "Improvement relief must not be granted on or after 1 April 2029"
    );
}

#[tokio::test]
async fn test_improvement_and_heat_network_not_granted_to_scotland() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "qualifying_improvement_rv_increment": 5000.00,
            "is_heat_network": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code == "IMPROVEMENT_G_FACTOR"),
        "Improvement relief (LGFA 1988 Sched 4ZA) must not be granted in Scotland"
    );
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code == "HEAT_NETWORK"),
        "Heat network relief (NDRA 2023 s.2) must not be granted in Scotland"
    );
}

#[tokio::test]
async fn test_statutory_telecommunications_infrastructure_relief_england_and_scotland() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: England (Telecommunications Infrastructure (Relief from Non-Domestic Rates) Act 2018 / SI 2018/715)
    // 2024/25 Small Business Multiplier 0.499. Gross = 50,000 * 0.499 = £24,950.00. Relief = 100% (£24,950.00). Net = £0.00.
    let eng_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(50000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_telecommunications_infrastructure: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let eng_res = engine.calculate(eng_req);
    let eng_seg = &eng_res.annual_billing_segments[0];
    assert_eq!(eng_seg.gross_liability, dec!(24950.00));
    let telecom_relief = eng_seg.itemized_reliefs.iter().find(|r| r.relief_code == "TELECOM_FIBRE_RELIEF").unwrap();
    assert_eq!(telecom_relief.discount_percentage, dec!(100.00));
    assert_eq!(telecom_relief.amount_deducted, dec!(24950.00));
    assert_eq!(eng_seg.net_segment_payable, dec!(0.00));

    // Case 2: Scotland (SSI 2019/111 / SSI 2024/76)
    // 2024/25 Scotland Intermediate Multiplier 0.545. Gross = 60,000 * 0.545 = £32,700.00. Net = £0.00.
    let sco_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(60000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_telecommunications_infrastructure: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let sco_res = engine.calculate(sco_req);
    let sco_seg = &sco_res.annual_billing_segments[0];
    assert_eq!(sco_seg.gross_liability, dec!(32700.00));
    let sco_telecom = sco_seg.itemized_reliefs.iter().find(|r| r.relief_code == "TELECOM_FIBRE_RELIEF").unwrap();
    assert_eq!(sco_telecom.discount_percentage, dec!(100.00));
    assert_eq!(sco_telecom.amount_deducted, dec!(32700.00));
    assert_eq!(sco_seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_statutory_public_lavatory_exemption() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::LOCAL_AUTHORITY,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(8000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_public_lavatory: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let lav_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "PUBLIC_LAVATORY");
    assert!(lav_relief.is_some());
    let r = lav_relief.unwrap();
    assert_eq!(r.statutory_authority, "Rating (Public Lavatories) Act 2021");
    assert_eq!(r.discount_percentage, dec!(100.00));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_rural_settlement_relief_rv_cap_enforcement() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: General store (SCAT 225) with RV £7,500 (<= £8,500 statutory cap under LGFA 1988 s.43(6A)-(6E)) -> 100% relief
    let pass_req = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(7500.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rural_settlement: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let pass_res = engine.calculate(pass_req);
    let pass_seg = &pass_res.annual_billing_segments[0];
    assert_eq!(pass_seg.net_segment_payable, dec!(0.00));
    assert!(pass_seg.itemized_reliefs.iter().any(|r| r.relief_code == "RURAL_SETTLEMENT"));

    // Case 2: General store (SCAT 225) with RV £10,000 (> £8,500 statutory cap) -> 0% rural relief awarded
    let fail_req = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(10000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rural_settlement: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let fail_res = engine.calculate(fail_req);
    let fail_seg = &fail_res.annual_billing_segments[0];
    assert!(!fail_seg.itemized_reliefs.iter().any(|r| r.relief_code == "RURAL_SETTLEMENT"));

    // Case 3: Public house (SCAT 209) with RV £11,000 (<= £12,500 higher statutory cap) -> 100% relief awarded
    let pub_pass_req = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "209".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(11000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rural_settlement: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let pub_pass_res = engine.calculate(pub_pass_req);
    let pub_pass_seg = &pub_pass_res.annual_billing_segments[0];
    assert_eq!(pub_pass_seg.net_segment_payable, dec!(0.00));
    assert!(pub_pass_seg.itemized_reliefs.iter().any(|r| r.relief_code == "RURAL_SETTLEMENT"));
}

#[tokio::test]
async fn test_discretionary_charitable_relief_top_up() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // LGFA 1988 s.43(5) grants 80% mandatory charity relief, and s.47(2)(a) empowers councils to award 20% discretionary top-up
    // RV £20,000 * 0.499 = £9,980.00 gross. Mandatory: £7,984.00 (80%). Top-up: £1,996.00 (20%). Net: £0.00.
    let req = LiabilityCalculationRequest {
        party_type: PartyType::CHARITY,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_charitable_purposes: true,
            is_charity_discretionary_top_up: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res = engine.calculate(req);
    let seg = &res.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(9980.00));
    let mandatory = seg.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(mandatory.discount_percentage, dec!(80.00));
    assert_eq!(mandatory.amount_deducted, dec!(7984.00));

    let topup = seg.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_DISCRETIONARY_TOPUP").unwrap();
    assert_eq!(topup.discount_percentage, dec!(20.00));
    assert_eq!(topup.amount_deducted, dec!(1996.00));

    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[test]
fn test_part_occupied_charity_reliefs_capped_at_occupied_chargeable() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CHARITY",
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "225",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "rateable_value": 10000.00
                }
            ]
        },
        "occupancy_type": "PART_OCCUPIED",
        "qualifying_relief_flags": {
            "is_charitable_purposes": true,
            "is_charity_discretionary_top_up": true,
            "is_small_business_sole_property": true,
            "days_previously_unoccupied": 91
        },
        "part_occupied_apportionment": {
            "occupied_rateable_value": 5000.00,
            "unoccupied_rateable_value": 5000.00,
            "occupied_percentage": 50.00
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);

    let seg = &response.annual_billing_segments[0];
    let occ_chargeable = seg.gross_liability - (seg.gross_liability * dec!(0.50)).round_dp(2);
    let total_itemized_reliefs: rust_decimal::Decimal = seg.itemized_reliefs.iter().map(|r| r.amount_deducted).sum();

    // Reliefs on occupied portion must not exceed occ_chargeable (£2,495.00)
    assert!(
        total_itemized_reliefs <= occ_chargeable,
        "Total reliefs ({}) must not exceed occupied chargeable amount ({})",
        total_itemized_reliefs,
        occ_chargeable
    );
    // Unoccupied part had 91 days prior unoccupied, so it owes the full £2,495.00
    assert_eq!(
        seg.net_segment_payable,
        occ_chargeable,
        "Net payable should be the unoccupied portion since occupied is 100% relieved"
    );
    assert_eq!(
        response.summary_financials.total_gross_liability
            - response.summary_financials.total_transitional_adjustments
            - response.summary_financials.total_reliefs_and_discounts,
        response.summary_financials.total_net_payable,
        "Gross - Transitional - Reliefs must equal Net payable"
    );
}

#[tokio::test]
async fn test_statutory_empty_low_value_exemption() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // Empty property with RV £2,500 (< £2,900 statutory threshold under SI 2008/386)
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(2500.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let empty_low = seg.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_LOW_VALUE");
    assert!(empty_low.is_some());
    assert_eq!(empty_low.unwrap().discount_percentage, dec!(100.00));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_statutory_empty_property_prior_unoccupied_days() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // Commercial office (SCAT 091, 91 max exempt days)
    // 30 days already unoccupied before occupation start -> 61 remaining exempt days
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(), // 365 days
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_industrial_hereditament: false,
            days_previously_unoccupied: Some(30),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let empty_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION");
    assert!(empty_relief.is_some());
    let r = empty_relief.unwrap();
    // 61 days exempt out of 365 days: discount ratio 61 / 365 * 100% = 16.71%
    assert_eq!(r.discount_percentage, dec!(16.71));
    assert!(seg.net_segment_payable > dec!(0.00));
}

#[test]
fn test_rhl_relief_applies_in_sbrr_taper_band_england_and_wales() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1. England: RV £14,000 in 2024/25. SBRR partial (33.33%), RHL 75% applies to remainder.
    let json_england = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "rateable_value": 14000.00
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_sole_property": true,
            "is_rhl_qualifying": true,
            "apply_transitional_relief": false
        }
    });

    let request_eng: LiabilityCalculationRequest = serde_json::from_value(json_england).unwrap();
    let resp_eng = engine.calculate(request_eng);
    let seg_eng = &resp_eng.annual_billing_segments[0];

    let sbrr_eng = seg_eng.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").expect("SBRR must be present");
    let rhl_eng = seg_eng.itemized_reliefs.iter().find(|r| r.relief_code.starts_with("RHL")).expect("RHL relief must be present in taper band");
    // Issue #50: SBRR chargeable amount is 6,986.00 / E (E = 1.500) = £4,657.33, so SBRR is £2,328.67;
    // RHL 75% of £4,657.33 = £3,493.00; net £1,164.33
    assert_eq!(sbrr_eng.amount_deducted, dec!(2328.67));
    assert_eq!(rhl_eng.amount_deducted, dec!(3493.00));
    assert_eq!(seg_eng.net_segment_payable, dec!(1164.33));

    // 2. Wales: RV £9,000 in 2024/25. SBRR 50%, RHL 40% applies to remainder.
    let json_wales = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "rateable_value": 9000.00
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_sole_property": true,
            "is_rhl_qualifying": true,
            "apply_transitional_relief": false
        }
    });

    let request_wal: LiabilityCalculationRequest = serde_json::from_value(json_wales).unwrap();
    let resp_wal = engine.calculate(request_wal);
    let seg_wal = &resp_wal.annual_billing_segments[0];

    let sbrr_wal = seg_wal.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR_WALES").expect("Welsh SBRR must be present");
    let rhl_wal = seg_wal.itemized_reliefs.iter().find(|r| r.relief_code.starts_with("RHL")).expect("Welsh RHL must be present in taper band");
    assert_eq!(sbrr_wal.amount_deducted, dec!(2529.00));
    assert_eq!(rhl_wal.amount_deducted, dec!(1011.60));
    assert_eq!(seg_wal.net_segment_payable, dec!(1517.40));
}

#[test]
fn test_rhl_cash_cap_enforced_per_financial_year_across_segments() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "rateable_value": 600000.00
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_rhl_qualifying": true,
            "apply_transitional_relief": false
        },
        "occupancy_periods": [
            {
                "start_date": "2024-04-01",
                "end_date": "2024-09-30",
                "occupancy_type": "OCCUPIED",
                "qualifying_relief_flags": {
                    "is_rhl_qualifying": true,
                    "apply_transitional_relief": false
                }
            },
            {
                "start_date": "2024-10-01",
                "end_date": "2025-03-31",
                "occupancy_type": "OCCUPIED",
                "qualifying_relief_flags": {
                    "is_rhl_qualifying": true,
                    "apply_transitional_relief": false
                }
            }
        ]
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);

    assert_eq!(response.annual_billing_segments.len(), 2);
    let total_rhl_relief: rust_decimal::Decimal = response
        .annual_billing_segments
        .iter()
        .flat_map(|seg| seg.itemized_reliefs.iter())
        .filter(|r| r.relief_code.starts_with("RHL"))
        .map(|r| r.amount_deducted)
        .sum();

    // RHL cash cap in 2024/25 is £110,000 per business per year
    assert_eq!(
        total_rhl_relief,
        dec!(110000.00),
        "Total RHL relief across segments in 2024/25 must not exceed £110,000 cash cap"
    );
}

#[tokio::test]
async fn test_self_catering_holiday_let_letting_days_test() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: Scotland (S.I. 1992/2955 sched 2 para 2(b) as substituted by S.S.I. 2021/489) - 70-day actual letting threshold
    // Sub-case A: 200 actual let days (>= 70 days) -> Qualifies as commercial holiday let on NDR list
    let sco_pass_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            holiday_let_actual_days_let: Some(200),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let sco_pass_res = engine.calculate(sco_pass_req);
    let sco_pass_seg = &sco_pass_res.annual_billing_segments[0];
    assert_eq!(sco_pass_seg.net_segment_payable, dec!(9960.00)); // 20,000 * 0.498 = £9,960.00
    assert!(!sco_pass_seg.itemized_reliefs.iter().any(|r| r.relief_code == "COUNCIL_TAX_TRANSFER"));

    // Sub-case B: 60 actual let days (< 70 days) -> Disqualified from NDR list and transferred to Council Tax
    let sco_fail_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            holiday_let_actual_days_let: Some(60),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let sco_fail_res = engine.calculate(sco_fail_req);
    let sco_fail_seg = &sco_fail_res.annual_billing_segments[0];
    assert_eq!(sco_fail_seg.net_segment_payable, dec!(0.00));
    assert!(sco_fail_seg.itemized_reliefs.iter().any(|r| r.relief_code == "COUNCIL_TAX_TRANSFER"));
    assert!(sco_fail_res.calculation_narrative.contains("Disqualified from Non-Domestic Rating List"));

    // Case 2: England (70-day actual letting threshold)
    // 50 actual let days (< 70 days) -> Disqualified and transferred to Council Tax
    let eng_fail_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            holiday_let_actual_days_let: Some(50),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let eng_fail_res = engine.calculate(eng_fail_req);
    let eng_fail_seg = &eng_fail_res.annual_billing_segments[0];
    assert_eq!(eng_fail_seg.net_segment_payable, dec!(0.00));
    assert!(eng_fail_seg.itemized_reliefs.iter().any(|r| r.relief_code == "COUNCIL_TAX_TRANSFER"));
}

#[tokio::test]
async fn test_welsh_holiday_let_182_days_qualifies_post_2023() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Wales 2024/25 with 182 days actually let and 252 days available qualifies
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 20000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "holiday_let_actual_days_let": 182,
            "holiday_let_days_available": 252
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "COUNCIL_TAX_TRANSFER"), "182 days qualifies in Wales");
    assert!(seg.net_segment_payable > dec!(0.00));
}

#[tokio::test]
async fn test_welsh_holiday_let_70_days_disqualified_post_2023() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Wales 2023/24 onwards requires >= 182 days actually let (LGFA 1988 s.66(2BB) / W.S.I. 2022/563)
    // 100 days let is >= 70 (English threshold) but < 182 (Welsh threshold), so MUST be disqualified
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 20000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "holiday_let_actual_days_let": 100
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.net_segment_payable, dec!(0.00), "Disqualified holiday let must pay £0 NDR");
    let transfer = seg.itemized_reliefs.iter().find(|r| r.relief_code == "COUNCIL_TAX_TRANSFER");
    assert!(transfer.is_some(), "Must transfer to Council Tax under W.S.I. 2022/563");
    assert_eq!(transfer.unwrap().statutory_authority, "LGFA 1988 s.66(2BB) / W.S.I. 2022/563");
}

#[tokio::test]
async fn test_welsh_holiday_let_70_days_qualifies_pre_2023() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Prior to 1 April 2023, Welsh threshold was 70 days actually let
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2017-04-01",
                    "effective_to": null,
                    "rateable_value": 20000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2022-04-01",
            "end_date": "2023-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "holiday_let_actual_days_let": 70
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "COUNCIL_TAX_TRANSFER"), "70 days qualified before 1 April 2023");
}

#[tokio::test]
async fn test_welsh_holiday_let_availability_threshold_disqualification() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // In Wales post-2023, let for 200 days (>= 182) but only available for 200 days (< 252) -> Disqualified
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 20000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "holiday_let_actual_days_let": 200,
            "holiday_let_days_available": 200
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.net_segment_payable, dec!(0.00));
    assert!(seg.itemized_reliefs.iter().any(|r| r.relief_code == "COUNCIL_TAX_TRANSFER"));
}

#[tokio::test]
async fn test_scotland_self_catering_70_day_letting_test() {
    let engine = CalculationEngine::new(ParameterRepository::new_seeded());

    // From 1 April 2022, Scottish self-catering accommodation is non-domestic where it is available to let for
    // 140 days or more in the financial year and actually let for 70 days or more of those 140
    // (Council Tax (Dwellings and Part Residential Subjects) (Scotland) Regulations 1992 sched 2 para 2(b),
    // substituted by S.S.I. 2021/489 reg 2(4); unchanged by S.S.I. 2024/10, 2025/249 and 2026/72).
    // Before 1 April 2022 only the 140-day availability test applied.
    let run = |start: &str, end: &str, let_days: Option<i64>, available: Option<i64>| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "hereditament": {
                "jurisdiction": "SCOTLAND",
                "scat_code": "249",
                "rateable_value_history": [
                    { "effective_from": "2017-04-01", "effective_to": null, "rateable_value": 20000.00 }
                ]
            },
            "occupation_period": { "start_date": start, "end_date": end },
            "occupancy_type": "OCCUPIED",
            "qualifying_relief_flags": {
                "apply_transitional_relief": false,
                "holiday_let_actual_days_let": let_days,
                "holiday_let_days_available": available
            }
        });
        let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
        engine.calculate(request)
    };
    let transferred = |res: &LiabilityCalculationResponse| {
        res.annual_billing_segments[0].itemized_reliefs.iter().any(|r| r.relief_code == "COUNCIL_TAX_TRANSFER")
    };

    // 2024/25: let 100 days, available 200 -> meets 70/140, stays on the valuation roll (20,000 x 0.498)
    let res = run("2024-04-01", "2025-03-31", Some(100), Some(200));
    assert!(!transferred(&res), "100 days let meets the Scottish 70-day test");
    assert_eq!(res.annual_billing_segments[0].net_segment_payable, dec!(9960.00));
    assert!(res.calculation_narrative.contains("meets minimum 70 days"));

    // 2024/25: exactly 70 days let qualifies
    let res = run("2024-04-01", "2025-03-31", Some(70), Some(140));
    assert!(!transferred(&res));

    // 2024/25: 69 days let -> fails the 70-day test, transferred to council tax
    let res = run("2024-04-01", "2025-03-31", Some(69), Some(200));
    assert!(transferred(&res));
    assert_eq!(res.annual_billing_segments[0].net_segment_payable, dec!(0.00));

    // 2024/25: let 100 days but available only 120 -> fails the 140-day availability test
    let res = run("2024-04-01", "2025-03-31", Some(100), Some(120));
    assert!(transferred(&res));

    // 2021/22: no actual-letting requirement before 1 April 2022; 30 days let with 150 available stays on the roll
    let res = run("2021-04-01", "2022-03-31", Some(30), Some(150));
    assert!(!transferred(&res), "no actual-letting test in Scotland before 1 April 2022");

    // 2021/22: available only 100 days -> fails the 140-day availability test
    let res = run("2021-04-01", "2022-03-31", Some(30), Some(100));
    assert!(transferred(&res));
}

#[tokio::test]
async fn test_holiday_let_actual_letting_test_start_dates_england_wales() {
    let engine = CalculationEngine::new(ParameterRepository::new_seeded());
    // The 70-day actual letting requirement starts on 1 April 2023 in England (LGFA 1988 s.66(2B) as amended by
    // S.I. 2022/217) and on 1 April 2010 in Wales (W.S.I. 2010/682). Before then only the 140-day availability
    // test applied.
    let run = |jur: &str, start: &str, end: &str, let_days: i64| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "hereditament": {
                "jurisdiction": jur,
                "scat_code": "249",
                "rateable_value_history": [
                    { "effective_from": "2005-04-01", "effective_to": null, "rateable_value": 20000.00 }
                ]
            },
            "occupation_period": { "start_date": start, "end_date": end },
            "occupancy_type": "OCCUPIED",
            "qualifying_relief_flags": {
                "apply_transitional_relief": false,
                "holiday_let_actual_days_let": let_days,
                "holiday_let_days_available": 150
            }
        });
        let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
        let res = engine.calculate(request);
        res.annual_billing_segments[0].itemized_reliefs.iter().any(|r| r.relief_code == "COUNCIL_TAX_TRANSFER")
    };
    assert!(!run("ENGLAND", "2022-04-01", "2023-03-31", 30), "England 2022/23: no actual-letting test");
    assert!(run("ENGLAND", "2023-04-01", "2024-03-31", 30), "England 2023/24: 70-day test applies");
    assert!(!run("WALES", "2009-04-01", "2010-03-31", 30), "Wales 2009/10: no actual-letting test");
    assert!(run("WALES", "2015-04-01", "2016-03-31", 30), "Wales 2015/16: 70-day test applies");
    assert!(!run("WALES", "2015-04-01", "2016-03-31", 70), "Wales 2015/16: 70 days qualifies");
}

