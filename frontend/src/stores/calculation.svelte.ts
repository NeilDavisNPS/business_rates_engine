import type {
  Jurisdiction,
  PartyType,
  OccupancyType,
  SupportedLanguage,
  RvEntry,
  OccupancyPeriodInput,
  QualifyingReliefFlags,
  LiabilityCalculationRequest,
  LiabilityCalculationResponse,
  ParamReferenceCode,
  ParamMultiplier,
  ParamSbrrSbbsThreshold,
  ParamSectorRelief,
  ParamTransitionalCap,
} from '../types';
import { getDictionary, JURISDICTION_CONFIGS } from '../i18n';

class CalculationStore {
  language = $state<SupportedLanguage>('en');
  theme = $state<'dark' | 'light'>('light');
  payloadMode = $state<'form' | 'json'>('form');
  jurisdiction = $state<Jurisdiction>('ENGLAND');
  partyType = $state<PartyType>('CORPORATE');
  scatCode = $state<string>('091');

  occupancyPeriods = $state<OccupancyPeriodInput[]>([
    {
      start_date: '2024-04-01',
      end_date: '2026-03-31',
      occupancy_type: 'OCCUPIED',
      part_occupied_apportionment: undefined,
    },
  ]);

  rateableValueHistory = $state<RvEntry[]>([
    {
      effective_from: '2023-04-01',
      effective_to: null,
      rateable_value: 45000,
    },
  ]);

  qualifyingReliefFlags = $state<QualifyingReliefFlags>({
    is_small_business_sole_property: false,
    is_charitable_purposes: false,
    is_rhl_qualifying: false,
    is_rural_settlement: false,
    is_public_lavatory: false,
    is_heat_network: false,
    qualifying_improvement_rv_increment: 0,
    is_industrial_hereditament: false,
    days_previously_unoccupied: null,
    is_artificial_avoidance_suspected: false,
    holiday_let_actual_days_let: null,
  });

  rawJsonRequest = $state<string>('');
  rawJsonResponse = $state<string>('');
  activeResultsTab = $state<'breakdown' | 'narrative' | 'json'>('breakdown');
  calcStatus = $state<'ready' | 'calculating' | 'success' | 'error'>('ready');
  errorMessage = $state<string | null>(null);
  response = $state<LiabilityCalculationResponse | null>(null);

  referenceCodes = $state<ParamReferenceCode[]>([]);
  dictFilter = $state<'ALL' | 'ENGLAND' | 'WALES' | 'SCOTLAND'>('ALL');
  isDictExpanded = $state<boolean>(false);

  multipliers = $state<ParamMultiplier[]>([]);
  isMultipliersExpanded = $state<boolean>(false);

  sbrrThresholds = $state<ParamSbrrSbbsThreshold[]>([]);
  isSbrrExpanded = $state<boolean>(false);

  sectorReliefs = $state<ParamSectorRelief[]>([]);
  isSectorReliefsExpanded = $state<boolean>(false);

  transitionalCaps = $state<ParamTransitionalCap[]>([]);
  isTransitionalCapsExpanded = $state<boolean>(false);

  t = $derived(getDictionary(this.language));
  jurisdictionInfo = $derived(JURISDICTION_CONFIGS[this.language][this.jurisdiction]);

  constructor() {
    this.syncFormToJson();
  }

  setLanguage(lang: SupportedLanguage) {
    this.language = lang;
    this.loadReferenceCodes();
  }

  setTheme(newTheme: 'light' | 'dark') {
    this.theme = newTheme;
    if (typeof document !== 'undefined') {
      if (newTheme === 'dark') {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
    }
  }

  setPayloadMode(mode: 'form' | 'json') {
    if (mode === 'json' && this.payloadMode === 'form') {
      this.syncFormToJson();
    } else if (mode === 'form' && this.payloadMode === 'json') {
      this.syncJsonToForm();
    }
    this.payloadMode = mode;
  }

  setJurisdiction(jur: Jurisdiction) {
    this.jurisdiction = jur;
    // Set appropriate SCAT code if changing jurisdiction
    if (jur === 'SCOTLAND' && this.scatCode === '211') {
      this.scatCode = '249';
    } else if (jur === 'WALES' && this.scatCode === '249') {
      this.scatCode = '211';
    }
    this.syncFormToJson();
  }

  addOccupancyPeriod(period?: Partial<OccupancyPeriodInput>) {
    const defaultStart = this.occupancyPeriods.length > 0
      ? this.occupancyPeriods[this.occupancyPeriods.length - 1].end_date || '2025-04-01'
      : '2024-04-01';

    this.occupancyPeriods.push({
      start_date: period?.start_date || defaultStart,
      end_date: period?.end_date || '2027-03-31',
      occupancy_type: period?.occupancy_type || 'OCCUPIED',
      part_occupied_apportionment: period?.part_occupied_apportionment,
    });
    this.syncFormToJson();
  }

  removeOccupancyPeriod(index: number) {
    if (this.occupancyPeriods.length > 1) {
      this.occupancyPeriods.splice(index, 1);
      this.syncFormToJson();
    }
  }

  addValuationEra(entry?: Partial<RvEntry>) {
    const defaultFrom = this.rateableValueHistory.length > 0
      ? this.rateableValueHistory[this.rateableValueHistory.length - 1].effective_to || '2026-04-01'
      : '2023-04-01';

    this.rateableValueHistory.push({
      effective_from: entry?.effective_from || defaultFrom,
      effective_to: entry?.effective_to || null,
      rateable_value: entry?.rateable_value ?? 45000,
    });
    this.syncFormToJson();
  }

  removeValuationEra(index: number) {
    if (this.rateableValueHistory.length > 1) {
      this.rateableValueHistory.splice(index, 1);
      this.syncFormToJson();
    }
  }

  buildPayload(): LiabilityCalculationRequest {
    const firstOcc = this.occupancyPeriods[0] || {
      start_date: '2024-04-01',
      end_date: '2026-03-31',
      occupancy_type: 'OCCUPIED' as OccupancyType,
    };
    const lastOcc = this.occupancyPeriods[this.occupancyPeriods.length - 1] || firstOcc;

    const jur = this.jurisdiction;
    const flags: QualifyingReliefFlags = {
      is_small_business_sole_property: this.qualifyingReliefFlags.is_small_business_sole_property,
      is_charitable_purposes: this.qualifyingReliefFlags.is_charitable_purposes,
      is_rhl_qualifying: jur !== 'SCOTLAND' ? this.qualifyingReliefFlags.is_rhl_qualifying : false,
      is_rural_settlement: this.qualifyingReliefFlags.is_rural_settlement,
      is_public_lavatory: jur !== 'SCOTLAND' ? this.qualifyingReliefFlags.is_public_lavatory : false,
      is_heat_network: jur === 'ENGLAND' ? this.qualifyingReliefFlags.is_heat_network : false,
      qualifying_improvement_rv_increment: jur === 'ENGLAND' ? (this.qualifyingReliefFlags.qualifying_improvement_rv_increment || 0) : 0,
      is_industrial_hereditament: this.qualifyingReliefFlags.is_industrial_hereditament,
      days_previously_unoccupied: this.qualifyingReliefFlags.days_previously_unoccupied,
      is_artificial_avoidance_suspected: jur === 'WALES' ? this.qualifyingReliefFlags.is_artificial_avoidance_suspected : false,
      holiday_let_actual_days_let: jur === 'SCOTLAND' ? this.qualifyingReliefFlags.holiday_let_actual_days_let : null,
    };

    return {
      party_type: this.partyType,
      occupation_period: {
        start_date: firstOcc.start_date,
        end_date: lastOcc.end_date,
      },
      hereditament: {
        jurisdiction: jur,
        scat_code: this.scatCode,
        rateable_value_history: $state.snapshot(this.rateableValueHistory),
      },
      occupancy_type: firstOcc.occupancy_type,
      occupancy_periods: $state.snapshot(this.occupancyPeriods),
      qualifying_relief_flags: flags,
      language: this.language,
    };
  }

  syncFormToJson() {
    try {
      const payload = this.buildPayload();
      this.rawJsonRequest = JSON.stringify(payload, null, 2);
    } catch (e) {
      console.error('Error generating payload JSON:', e);
    }
  }

  syncJsonToForm() {
    try {
      const p: LiabilityCalculationRequest = JSON.parse(this.rawJsonRequest);
      if (p.hereditament?.jurisdiction) {
        this.jurisdiction = p.hereditament.jurisdiction;
      }
      if (p.party_type) {
        this.partyType = p.party_type;
      }
      if (p.hereditament?.scat_code) {
        this.scatCode = p.hereditament.scat_code;
      }
      if (p.hereditament?.rateable_value_history && p.hereditament.rateable_value_history.length > 0) {
        this.rateableValueHistory = p.hereditament.rateable_value_history.map(rv => ({
          effective_from: rv.effective_from,
          effective_to: rv.effective_to,
          rateable_value: Number(rv.rateable_value) || 0,
        }));
      }
      if (p.occupancy_periods && p.occupancy_periods.length > 0) {
        this.occupancyPeriods = p.occupancy_periods.map(o => ({
          start_date: o.start_date,
          end_date: o.end_date,
          occupancy_type: o.occupancy_type || 'OCCUPIED',
          part_occupied_apportionment: o.part_occupied_apportionment ? {
            occupied_percentage: o.part_occupied_apportionment.occupied_percentage ?? 50,
            occupied_rateable_value: o.part_occupied_apportionment.occupied_rateable_value,
            unoccupied_rateable_value: o.part_occupied_apportionment.unoccupied_rateable_value,
          } : undefined,
        }));
      } else if (p.occupation_period) {
        this.occupancyPeriods = [{
          start_date: p.occupation_period.start_date,
          end_date: p.occupation_period.end_date,
          occupancy_type: p.occupancy_type || 'OCCUPIED',
        }];
      }
      if (p.qualifying_relief_flags) {
        this.qualifyingReliefFlags = {
          is_small_business_sole_property: !!p.qualifying_relief_flags.is_small_business_sole_property,
          is_charitable_purposes: !!p.qualifying_relief_flags.is_charitable_purposes,
          is_rhl_qualifying: !!p.qualifying_relief_flags.is_rhl_qualifying,
          is_rural_settlement: !!p.qualifying_relief_flags.is_rural_settlement,
          is_public_lavatory: !!p.qualifying_relief_flags.is_public_lavatory,
          is_heat_network: !!p.qualifying_relief_flags.is_heat_network,
          qualifying_improvement_rv_increment: Number(p.qualifying_relief_flags.qualifying_improvement_rv_increment) || 0,
          is_industrial_hereditament: !!p.qualifying_relief_flags.is_industrial_hereditament,
          days_previously_unoccupied: p.qualifying_relief_flags.days_previously_unoccupied != null ? Number(p.qualifying_relief_flags.days_previously_unoccupied) : null,
          is_artificial_avoidance_suspected: !!p.qualifying_relief_flags.is_artificial_avoidance_suspected,
          holiday_let_actual_days_let: p.qualifying_relief_flags.holiday_let_actual_days_let != null ? Number(p.qualifying_relief_flags.holiday_let_actual_days_let) : null,
        };
      }
      this.errorMessage = null;
    } catch (err: any) {
      this.errorMessage = `Invalid JSON: ${err.message}`;
    }
  }

  async executeCalculation() {
    this.calcStatus = 'calculating';
    this.errorMessage = null;

    let payload: LiabilityCalculationRequest;
    try {
      if (this.payloadMode === 'form') {
        payload = this.buildPayload();
        this.syncFormToJson();
      } else {
        payload = JSON.parse(this.rawJsonRequest);
      }
      payload.language = this.language;
    } catch (err: any) {
      this.calcStatus = 'error';
      this.errorMessage = `Malformed Request: ${err.message}`;
      return;
    }

    try {
      const res = await fetch(`/api/v1/calculate-liability?lang=${this.language}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept-Language': this.language,
        },
        body: JSON.stringify(payload),
      });

      if (!res.ok) {
        let errDesc = `HTTP ${res.status} (${res.statusText})`;
        try {
          const errData = await res.json();
          if (errData?.message) {
            errDesc = `${errData.error || this.t.calc_error}: ${errData.message}`;
          } else if (errData?.error) {
            errDesc = `${this.t.calc_error}: ${errData.error}`;
          }
        } catch (_) {
          const rawText = await res.text();
          if (rawText) errDesc = `${this.t.calc_error} (${res.status}): ${rawText}`;
        }
        throw new Error(errDesc);
      }

      const data: LiabilityCalculationResponse = await res.json();
      this.response = data;
      this.rawJsonResponse = JSON.stringify(data, null, 2);
      this.calcStatus = 'success';
    } catch (err: any) {
      this.calcStatus = 'error';
      this.errorMessage = err.message || 'Unknown network error';
    }
  }

  async loadReferenceCodes() {
    try {
      const res = await fetch(`/api/v1/reference-codes?lang=${this.language}`);
      if (res.ok) {
        this.referenceCodes = await res.json();
      }
    } catch (e) {
      console.warn('Failed to load reference codes from API:', e);
    }
  }

  async loadParameters() {
    try {
      const res = await fetch('/api/v1/parameters');
      if (res.ok) {
        const data = await res.json();
        this.multipliers = data.multipliers || [];
        this.sbrrThresholds = data.sbrr_thresholds || [];
        this.sectorReliefs = data.sector_reliefs || [];
        this.transitionalCaps = data.transitional_caps || [];
      }
    } catch (e) {
      console.warn('Failed to load parameters from API:', e);
    }
  }
}

export const calcStore = new CalculationStore();

