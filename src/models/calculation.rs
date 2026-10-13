use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::NaiveDate;
use uuid::Uuid;
use super::enums::{Jurisdiction, PartyType, OccupancyType};

fn default_rating_end_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2027, 3, 31).unwrap()
}

fn deserialize_flexible_end_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if !s.trim().is_empty() => {
            NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").map_err(serde::de::Error::custom)
        }
        _ => Ok(default_rating_end_date()),
    }
}

fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if !s.trim().is_empty() => {
            NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        _ => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: NaiveDate,
    #[serde(default = "default_rating_end_date", deserialize_with = "deserialize_flexible_end_date")]
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RvEntry {
    pub effective_from: NaiveDate,
    #[serde(default, deserialize_with = "deserialize_optional_date")]
    pub effective_to: Option<NaiveDate>,
    pub rateable_value: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HereditamentInput {
    pub jurisdiction: Jurisdiction,
    pub scat_code: String,
    pub rateable_value_history: Vec<RvEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QualifyingReliefFlags {
    #[serde(default)]
    pub is_small_business_sole_property: bool,
    #[serde(default)]
    pub is_small_business_second_property_grace: bool,
    #[serde(default)]
    pub is_charitable_purposes: bool,
    #[serde(default)]
    pub is_rhl_qualifying: bool,
    #[serde(default)]
    pub is_retail_premises: bool,
    #[serde(default)]
    pub is_island_property: bool,
    #[serde(default)]
    pub is_rural_settlement: bool,
    #[serde(default)]
    pub is_telecommunications_infrastructure: bool,
    #[serde(default)]
    pub is_charity_discretionary_top_up: bool,
    #[serde(default)]
    pub is_renewable_energy_generation: bool,
    #[serde(default)]
    pub is_higher_multiplier_excepted: bool,
    #[serde(default)]
    pub is_food_and_drink: bool,
    #[serde(default)]
    pub is_public_lavatory: bool,
    #[serde(default)]
    pub is_heat_network: bool,
    #[serde(default)]
    pub qualifying_improvement_rv_increment: Decimal,
    #[serde(default)]
    pub is_industrial_hereditament: bool,
    #[serde(default)]
    pub days_previously_unoccupied: Option<i64>,
    #[serde(default)]
    pub is_artificial_avoidance_suspected: bool,
    #[serde(default)]
    pub holiday_let_actual_days_let: Option<i64>,
    #[serde(default)]
    pub holiday_let_days_available: Option<i64>,
    #[serde(default)]
    pub is_sbbs_tr_qualifying: bool,
    #[serde(default)]
    pub is_ssbr_qualifying: bool,
    #[serde(default)]
    pub payment_installments_count: Option<u32>,
    #[serde(default = "default_true_opt")]
    pub apply_transitional_relief: Option<bool>,
}

fn default_true_opt() -> Option<bool> {
    Some(true)
}

fn default_occupancy_type() -> OccupancyType {
    OccupancyType::OCCUPIED
}

fn default_date_range() -> DateRange {
    DateRange {
        start_date: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartOccupiedApportionment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occupied_rateable_value: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unoccupied_rateable_value: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occupied_percentage: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupancyPeriodInput {
    pub start_date: NaiveDate,
    #[serde(default = "default_rating_end_date", deserialize_with = "deserialize_flexible_end_date")]
    pub end_date: NaiveDate,
    pub occupancy_type: OccupancyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifying_relief_flags: Option<QualifyingReliefFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub part_occupied_apportionment: Option<PartOccupiedApportionment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilityCalculationRequest {
    pub party_type: PartyType,
    #[serde(default = "default_date_range")]
    pub occupation_period: DateRange,
    pub hereditament: HereditamentInput,
    #[serde(default = "default_occupancy_type")]
    pub occupancy_type: OccupancyType,
    #[serde(default)]
    pub qualifying_relief_flags: QualifyingReliefFlags,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occupancy_periods: Option<Vec<OccupancyPeriodInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl LiabilityCalculationRequest {
    pub fn resolve_occupancy_periods(&self) -> Vec<OccupancyPeriodInput> {
        if let Some(ref periods) = self.occupancy_periods {
            if !periods.is_empty() {
                return periods.clone();
            }
        }
        vec![OccupancyPeriodInput {
            start_date: self.occupation_period.start_date,
            end_date: self.occupation_period.end_date,
            occupancy_type: self.occupancy_type,
            qualifying_relief_flags: Some(self.qualifying_relief_flags.clone()),
            part_occupied_apportionment: None,
        }]
    }

    pub fn resolve_overall_period(&self) -> (NaiveDate, NaiveDate) {
        let periods = self.resolve_occupancy_periods();
        if periods.is_empty() {
            return (self.occupation_period.start_date, self.occupation_period.end_date);
        }
        let min_start = periods.iter().map(|p| p.start_date).min().unwrap();
        let max_end = periods.iter().map(|p| p.end_date).max().unwrap();
        (min_start, max_end)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentPeriod {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterVersionIds {
    pub param_multiplier_version: String,
    pub param_sbrr_version: String,
    pub param_sector_relief_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplierInfo {
    #[serde(rename = "type")]
    pub multiplier_type: String,
    pub rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemizedRelief {
    pub relief_code: String,
    pub relief_name: String,
    pub statutory_authority: String,
    pub discount_percentage: Decimal,
    pub amount_deducted: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionalAdjustment {
    #[serde(rename = "type")]
    pub adjustment_type: String,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatutoryPenalty {
    pub penalty_type: String,
    pub statutory_authority: String,
    pub calculation_formula: String,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatutoryInstallment {
    pub installment_number: u32,
    pub due_date: NaiveDate,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualBillingSegment {
    pub financial_year: String,
    pub segment_period: SegmentPeriod,
    pub rateable_value: Decimal,
    pub multiplier_used: MultiplierInfo,
    pub gross_liability: Decimal,
    pub transitional_adjustment: TransitionalAdjustment,
    pub itemized_reliefs: Vec<ItemizedRelief>,
    pub net_segment_payable: Decimal,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statutory_penalties: Vec<StatutoryPenalty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statutory_installments: Option<Vec<StatutoryInstallment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occupancy_type: Option<OccupancyType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryFinancials {
    pub total_gross_liability: Decimal,
    pub total_transitional_adjustments: Decimal,
    pub total_reliefs_and_discounts: Decimal,
    pub total_net_payable: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_statutory_penalties: Option<Decimal>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilityCalculationResponse {
    pub calculation_id: Uuid,
    pub calculated_at: String,
    pub parameter_version_ids: ParameterVersionIds,
    pub jurisdiction: Jurisdiction,
    pub occupation_period: SegmentPeriod,
    pub summary_financials: SummaryFinancials,
    pub annual_billing_segments: Vec<AnnualBillingSegment>,
    pub calculation_narrative: String,
}
