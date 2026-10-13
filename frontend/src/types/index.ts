// TypeScript Domain Models for UK Business Rates Calculation Engine

export type Jurisdiction = 'ENGLAND' | 'WALES' | 'SCOTLAND';

export type PartyType =
  | 'CORPORATE'
  | 'INDIVIDUAL'
  | 'CHARITY'
  | 'CASC'
  | 'LOCAL_AUTHORITY';

export type OccupancyType = 'OCCUPIED' | 'UNOCCUPIED' | 'PART_OCCUPIED';

export type SupportedLanguage = 'en' | 'cy' | 'sco' | 'gd';

export interface RvEntry {
  effective_from: string;
  effective_to: string | null;
  rateable_value: number;
}

export interface PartOccupiedApportionment {
  occupied_rateable_value?: number;
  unoccupied_rateable_value?: number;
  occupied_percentage?: number;
}

export interface QualifyingReliefFlags {
  is_small_business_sole_property: boolean;
  is_small_business_second_property_grace?: boolean;
  is_charitable_purposes: boolean;
  is_rhl_qualifying: boolean;
  is_retail_premises?: boolean;
  is_island_property: boolean;
  is_rural_settlement: boolean;
  is_public_lavatory: boolean;
  is_heat_network: boolean;
  qualifying_improvement_rv_increment: number;
  is_industrial_hereditament: boolean;
  days_previously_unoccupied: number | null;
  is_artificial_avoidance_suspected: boolean;
  holiday_let_actual_days_let: number | null;
  holiday_let_days_available?: number | null;
  is_sbbs_tr_qualifying: boolean;
  is_ssbr_qualifying?: boolean;
  is_telecommunications_infrastructure?: boolean;
  is_charity_discretionary_top_up?: boolean;
  is_renewable_energy_generation?: boolean;
  payment_installments_count?: number;
  apply_transitional_relief?: boolean;
}

export interface OccupancyPeriodInput {
  start_date: string;
  end_date: string;
  occupancy_type: OccupancyType;
  qualifying_relief_flags?: Partial<QualifyingReliefFlags>;
  part_occupied_apportionment?: PartOccupiedApportionment;
}

export interface HereditamentInput {
  jurisdiction: Jurisdiction;
  scat_code: string;
  rateable_value_history: RvEntry[];
}

export interface DateRange {
  start_date: string;
  end_date: string;
}

export interface LiabilityCalculationRequest {
  party_type: PartyType;
  occupation_period: DateRange;
  hereditament: HereditamentInput;
  occupancy_type: OccupancyType;
  qualifying_relief_flags: QualifyingReliefFlags;
  occupancy_periods?: OccupancyPeriodInput[];
  language?: string;
}

export interface MultiplierInfo {
  type: string;
  rate: number;
}

export interface TransitionalAdjustment {
  type: string;
  amount: number;
}

export interface ItemizedRelief {
  relief_code: string;
  relief_name: string;
  statutory_authority: string;
  discount_percentage: number;
  amount_deducted: number;
}

export interface StatutoryPenalty {
  penalty_type: string;
  statutory_authority: string;
  calculation_formula: string;
  amount: number;
}

export interface StatutoryInstallment {
  installment_number: number;
  due_date: string;
  amount: number;
}

export interface SegmentPeriod {
  start_date: string;
  end_date: string;
  days: number;
}

export interface AnnualBillingSegment {
  financial_year: string;
  segment_period: SegmentPeriod;
  rateable_value: number;
  multiplier_used: MultiplierInfo;
  gross_liability: number;
  transitional_adjustment: TransitionalAdjustment;
  itemized_reliefs: ItemizedRelief[];
  net_segment_payable: number;
  statutory_penalties?: StatutoryPenalty[];
  statutory_installments?: StatutoryInstallment[];
  occupancy_type?: OccupancyType;
}

export interface SummaryFinancials {
  total_gross_liability: number;
  total_transitional_adjustments: number;
  total_reliefs_and_discounts: number;
  total_net_payable: number;
  total_statutory_penalties?: number;
  currency: string;
}

export interface ParameterVersionIds {
  param_multiplier_version: string;
  param_sbrr_version: string;
  param_sector_relief_version: string;
}

export interface LiabilityCalculationResponse {
  calculation_id: string;
  calculated_at: string;
  parameter_version_ids: ParameterVersionIds;
  jurisdiction: Jurisdiction;
  occupation_period: SegmentPeriod;
  summary_financials: SummaryFinancials;
  annual_billing_segments: AnnualBillingSegment[];
  calculation_narrative: string;
}

export interface ParamReferenceCode {
  category: string;
  code: string;
  label: string;
  description: string;
  jurisdiction?: Jurisdiction;
  display_order?: number;
}

export interface ParamMultiplier {
  parameter_id: string;
  financial_year: string;
  jurisdiction: Jurisdiction;
  multiplier_type: string;
  rv_threshold_min: number;
  rv_threshold_max: number | null;
  multiplier_rate: number;
  effective_from: string;
  effective_to: string;
}

export interface ParamSbrrSbbsThreshold {
  parameter_id: string;
  financial_year: string;
  jurisdiction: Jurisdiction;
  min_rv: number;
  max_rv: number;
  relief_percentage: number;
  is_tapered: boolean;
  effective_from: string;
  effective_to: string;
}

export interface ParamSectorRelief {
  parameter_id: string;
  financial_year: string;
  jurisdiction: Jurisdiction;
  relief_code: string;
  discount_percentage: number;
  cash_cap_gbp: number | null;
  effective_from: string;
  effective_to: string;
}

export interface ParamTransitionalCap {
  parameter_id: string;
  revaluation_list_year: number;
  financial_year: string;
  jurisdiction: Jurisdiction;
  property_tier: string;
  rv_threshold_min: number;
  rv_threshold_max: number | null;
  upward_cap_percentage: number;
}

