<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  let isExpanded = $state<boolean>(false);
  let selectedCountry = $state<'ENGLAND' | 'WALES' | 'SCOTLAND'>('ENGLAND');

  interface LegislationLink {
    key: string;
    url: string;
    title: string;
    primary?: boolean;
  }

  interface CountryRatingList {
    list: string;
    years: string;
    multipliers: string;
    sbrr: string;
    transition: string;
    statute: string;
  }

  interface CountryFrameworkData {
    name: string;
    legislation: LegislationLink[];
    currentRegimeSummary: {
      frameworkTitle: string;
      frameworkSub: string;
      multiplierTitle: string;
      multipliers: { label: string; rate: string; rvThreshold: string }[];
      reliefHighlights: { title: string; desc: string }[];
    };
    preDevolution: {
      heading: string;
      era: string;
      statute: string;
      overview: string;
      rules: string[];
    };
    ratingLists: CountryRatingList[];
  }

  const countryData: Record<'ENGLAND' | 'WALES' | 'SCOTLAND', CountryFrameworkData> = {
    ENGLAND: {
      name: 'England',
      legislation: [
        {
          key: 'LGFA 1988 (Primary GB)',
          url: 'https://www.legislation.gov.uk/ukpga/1988/41/contents',
          title: 'Local Government Finance Act 1988 (c. 41) — Base Statutory Rating Scheme',
          primary: true,
        },
        {
          key: 'NDRA 2023 (England)',
          url: 'https://www.legislation.gov.uk/ukpga/2023/53/contents/enacted',
          title: 'Non-Domestic Rating Act 2023 (c. 53) — 3-Year Lists, Multipliers & Relief Reforms',
          primary: true,
        },
        {
          key: 'SI 1989/1058 (Installments)',
          url: 'https://www.legislation.gov.uk/uksi/1989/1058/contents/made',
          title: 'The Non-Domestic Rating (Collection and Enforcement) Regulations 1989 (S.I. 1989/1058)',
          primary: false,
        },
        {
          key: 'SI 2017/411 (SBRR England)',
          url: 'https://www.legislation.gov.uk/uksi/2017/411/contents/made',
          title: 'The Non-Domestic Rating (Small Business Rate Relief) (England) Order 2017',
          primary: false,
        },
        {
          key: 'SI 2008/386 (Empty Property)',
          url: 'https://www.legislation.gov.uk/uksi/2008/386/contents/made',
          title: 'The Non-Domestic Rating (Unoccupied Property) (England) Regulations 2008',
          primary: false,
        },
      ],
      currentRegimeSummary: {
        frameworkTitle: 'Current & Upcoming Regime (2023–2027)',
        frameworkSub: 'Local Government Finance Act 1988 & Non-Domestic Rating Act 2023 (NDRA 2023)',
        multiplierTitle: '2026/27 5-Multiplier System (Permanent High-Street Rates):',
        multipliers: [
          { label: 'Small Business RHL', rate: '38.2p', rvThreshold: 'RV < £51,000' },
          { label: 'Standard RHL', rate: '43.0p', rvThreshold: 'RV £51,000 to £499,999' },
          { label: 'Small Business Non-RHL', rate: '43.2p', rvThreshold: 'RV < £51,000' },
          { label: 'Standard Non-RHL', rate: '48.0p', rvThreshold: 'RV £51,000 to £499,999' },
          { label: 'High-Value Multiplier', rate: '50.8p', rvThreshold: 'RV ≥ £500,000' },
        ],
        reliefHighlights: [
          {
            title: 'Small Business Rate Relief (SBRR)',
            desc: '100% relief up to £12k RV; linear taper to £15k RV for sole-property occupiers. 12-month second property grace period under S.I. 2017/411.',
          },
          {
            title: 'Supporting Small Business Relief (SSBR)',
            desc: 'Caps net annual bill increases at £600/year for 2023–2026, and £800/year for 2026/27 under s.47.',
          },
          {
            title: 'Abolition of Downward Caps',
            desc: 'Downward transitional relief caps permanently abolished from 2023/24; upward transition caps only under s.57A.',
          },
          {
            title: 'Targeted Exemptions',
            desc: '100% Improvement Relief for 12 months (NDRA 2023 s.1 G-factor), 100% Low-Carbon Heat Networks (s.2), 100% Public Lavatories Relief.',
          },
        ],
      },
      preDevolution: {
        heading: 'UK Unified Statutory Rating Framework (Pre-Devolution: 1990–2004)',
        era: '1990/91 to 2004/05 (1990, 1995 & 2000 Rating Lists)',
        statute: 'Local Government Finance Act 1988 (LGFA 1988 s.43, s.45, s.58)',
        overview:
          'Prior to the Local Government Act 2003 reforms which took effect on 1 April 2005, England operated under the original unified Great Britain non-domestic rating framework enacted by Parliament in the LGFA 1988.',
        rules: [
          'Single Unified Multiplier: A single non-domestic multiplier (e.g. 34.8p in 1990/91) applied to all non-domestic hereditaments. No small business multiplier existed.',
          'Zero Small Business Relief: Small Business Rate Relief (SBRR) did not exist. Every property, regardless of size, paid pure gross liability (Rateable Value × Single Multiplier).',
          'Symmetrical 5-Year Transitional Phasing: Revaluations under LGFA 1988 s.58 featured upward caps (+15%/+20%) funded by downward reduction caps that delayed bill reductions for falling-value properties, indexed to annual RPI.',
          'Pre-2008 Empty Property Rules: Empty commercial premises received 3 months 100% exemption, then paid 50% rates indefinitely; industrial hereditaments enjoyed 100% permanent exemption until the Rating (Empty Properties) Act 2007 imposed 100% empty liability on 1 April 2008.',
        ],
      },
      ratingLists: [
        {
          list: '1990 List',
          years: '1990/91 – 1994/95',
          multipliers: 'Single Unified Multiplier (34.8p in 1990/91 rising to 42.3p in 1994/95)',
          sbrr: 'None (0% — Pure RV × Multiplier)',
          transition: 'Symmetrical 5-year transitional phasing with upward caps and downward reduction caps',
          statute: 'LGFA 1988 s.58 / S.I. 1989/1060',
        },
        {
          list: '1995 List',
          years: '1995/96 – 1999/00',
          multipliers: 'Single Unified Multiplier (43.2p in 1995/96 rising to 48.9p in 1999/00)',
          sbrr: 'None (0%)',
          transition: '5-year symmetrical phasing caps indexed to annual RPI; 50% indefinite empty rates',
          statute: 'LGFA 1988 s.58 / S.I. 1994/3229',
        },
        {
          list: '2000 List',
          years: '2000/01 – 2004/05',
          multipliers: 'Single Unified Multiplier (41.6p in 2000/01 rising to 45.6p in 2004/05)',
          sbrr: 'None (0%)',
          transition: '5-year symmetrical phasing caps; 50% rates after 3 months empty',
          statute: 'LGFA 1988 s.58 / S.I. 1999/3379',
        },
        {
          list: '2005 List',
          years: '2005/06 – 2009/10',
          multipliers: 'Twin Multipliers created: Standard Multiplier (42.2p) vs Small Business Multiplier (41.5p)',
          sbrr: '50% max discount for RV ≤ £5,000, linear taper to £10,000 RV (£15k London). Formal application required.',
          transition: '5-year phasing caps. Rating (Empty Properties) Act 2007 abolished 50% empty rates on 1 April 2008',
          statute: 'LGA 2003 / S.I. 2004/3315 / 2007 Act',
        },
        {
          list: '2010 List',
          years: '2010/11 – 2016/17',
          multipliers: 'Small Business (40.7p in 2010/11) vs Standard (41.4p). Standard applied on RV > £18k/£25.5k',
          sbrr: 'Doubled to 100% up to £6,000 RV, linear taper to £12,000 RV. Annual claim form removed (automatic award).',
          transition: '5-year transitional relief scheme under S.I. 2009/3371 with tiered Small/Medium/Large caps',
          statute: 'S.I. 2010/1659 / S.I. 2009/3371',
        },
        {
          list: '2017 List',
          years: '2017/18 – 2022/23',
          multipliers: 'Small Business Multiplier (46.6p rising to 49.9p) vs Standard Multiplier (47.9p rising to 51.2p for RV > £51k)',
          sbrr: '100% relief up to £12,000 RV, linear taper to £15,000 RV (S.I. 2017/411). 12-mo 2nd property grace period. COVID 100%/66%/50% discounts.',
          transition: 'Upward caps + downward phasing. S.I. 2017/411 / COVID emergency schemes',
          statute: 'S.I. 2017/411 / Rating Act 2021',
        },
        {
          list: '2023 List',
          years: '2023/24 – 2025/26',
          multipliers: 'Decoupled multipliers under NDRA 2023. Small Business Multiplier frozen at 49.9p; Standard 51.2p rising to 54.6p',
          sbrr: '100% up to £12k RV; RHL relief 75% in 2023–2025, 40% in 2025/26 (£110k cap). SSBR caps bills at £600/year.',
          transition: 'Downward transition caps permanently abolished (immediate savings for RV falls). Upward caps only.',
          statute: 'NDRA 2023 / S.I. 2022/1239',
        },
        {
          list: '2026 List',
          years: '2026/27 – 2028/29',
          multipliers: '5 Multipliers: Small Business RHL (38.2p), Standard RHL (43.0p), Small Non-RHL (43.2p), Standard Non-RHL (48.0p), High-Value (50.8p)',
          sbrr: 'Permanent lower high-street multipliers replace temporary RHL relief. 100% SBRR up to £12k RV. SSBR £800 cap in 2026/27.',
          transition: 'Upward transitional caps; 12-month Improvement Relief (100% G-factor); 100% Heat Networks exemption',
          statute: 'NDRA 2023 / Rating Lists 2026',
        },
      ],
    },
    WALES: {
      name: 'Wales',
      legislation: [
        {
          key: 'LGFA 1988 (Primary GB)',
          url: 'https://www.legislation.gov.uk/ukpga/1988/41/contents',
          title: 'Local Government Finance Act 1988 (c. 41) — Base Statutory Rating Scheme',
          primary: true,
        },
        {
          key: 'LGF(W)A 2024 (Wales)',
          url: 'https://www.legislation.gov.uk/asc/2024/6/contents/enacted',
          title: 'Local Government Finance (Wales) Act 2024 (asc 6) — Devolved Multipliers & Anti-Avoidance',
          primary: true,
        },
        {
          key: 'WSI 2025/1371 (Transitional Relief)',
          url: 'https://www.legislation.gov.uk/wsi/2025/1371/contents/made',
          title: 'The Non-Domestic Rating (Chargeable Amounts) (Wales) Regulations 2025 (W.S.I. 2025/1371)',
          primary: false,
        },
        {
          key: 'WSI 2026/43 (Anti-Avoidance & Penalties)',
          url: 'https://www.legislation.gov.uk/wsi/2026/43/contents/made',
          title: 'The Non-Domestic Rating (Avoidance and Penalties) (Wales) Regulations 2026 (W.S.I. 2026/43)',
          primary: false,
        },
        {
          key: 'WSI 2022/575 (182-Day Holiday Let)',
          url: 'https://www.legislation.gov.uk/wsi/2022/575/contents/made',
          title: 'The Non-Domestic Rating (Definition of Domestic Property) (Wales) Order 2022 (W.S.I. 2022/575)',
          primary: false,
        },
      ],
      currentRegimeSummary: {
        frameworkTitle: 'Current & Upcoming Regime (2023–2027)',
        frameworkSub: 'Local Government Finance Act 1988 & Local Government Finance (Wales) Act 2024 (LGF(W)A 2024)',
        multiplierTitle: '2026/27 3-Multiplier System (Devolved Welsh Rates):',
        multipliers: [
          { label: 'Retail Multiplier', rate: '35.0p', rvThreshold: 'RV ≤ £51,000 (Retail)' },
          { label: 'Standard Multiplier', rate: '50.2p', rvThreshold: 'RV ≤ £100,000' },
          { label: 'Higher Multiplier', rate: '51.5p', rvThreshold: 'RV > £100,000' },
        ],
        reliefHighlights: [
          {
            title: 'Welsh Small Business Rate Relief',
            desc: '100% relief up to £6,000 RV; linear taper to £12,000 RV. Restricted to max 2 properties per ratepayer per billing authority.',
          },
          {
            title: '2-Year Transitional Phasing (W.S.I. 2025/1371)',
            desc: '67% relief in Year 1 (2026/27) and 34% in Year 2 (2027/28) for net bill increases exceeding £300.',
          },
          {
            title: 'Food & Drink Hospitality Relief',
            desc: '15% discount for 2026/27 (capped at £110,000 across Wales). Petrol filling stations (SCAT 211) are explicitly excluded.',
          },
          {
            title: 'Anti-Avoidance Framework (W.S.I. 2026/43)',
            desc: 'Empty rate reset period extended from 42 days to 180 days; statutory billing authority investigation penalty notices (£500 + 3% RV).',
          },
        ],
      },
      preDevolution: {
        heading: 'UK Unified Statutory Rating Framework (Pre-Devolution: 1990–1999/2004)',
        era: '1990/91 to 2004/05 (1990, 1995 & early 2000 Rating Lists)',
        statute: 'Local Government Finance Act 1988 (Secretary of State for Wales orders)',
        overview:
          'Prior to the transfer of rating order powers to the National Assembly for Wales under the Government of Wales Act 1998 and the later introduction of devolved Welsh SBRR in 2005, Wales operated under common UK rating regulations.',
        rules: [
          'Unified National Multiplier: Wales shared a single non-domestic multiplier set nationally (e.g. 36.8p in 1990/91, 39.0p in 1995/96) with no small business differentiation.',
          'Zero Small Business Relief: Pre-devolution Welsh rating lists provided 0% small business relief; full rates were payable on all commercial properties.',
          'Symmetrical 5-Year Transitional Phasing: LGFA 1988 s.58 applied across Wales, featuring symmetrical upward protection caps funded by downward phasing reduction caps, indexed to RPI.',
          'Pre-2008 Empty Rates: Empty commercial hereditaments received 3 months 100% exemption, then paid 50% rates indefinitely until empty rate parity was introduced in 2008.',
        ],
      },
      ratingLists: [
        {
          list: '1990 List',
          years: '1990/91 – 1994/95',
          multipliers: 'Single Unified Multiplier (36.8p in 1990/91 rising to 44.0p in 1994/95)',
          sbrr: 'None (0% — Pure RV × Multiplier)',
          transition: 'Symmetrical 5-year transitional phasing with upward caps and downward reduction caps',
          statute: 'LGFA 1988 s.58 / S.I. 1989/1060',
        },
        {
          list: '1995 List',
          years: '1995/96 – 1999/00',
          multipliers: 'Single Unified Multiplier (39.0p in 1995/96 rising to 43.1p in 1999/00)',
          sbrr: 'None (0%)',
          transition: '5-year symmetrical phasing caps indexed to annual RPI; 50% indefinite empty rates',
          statute: 'LGFA 1988 s.58 / S.I. 1994/3229',
        },
        {
          list: '2000 List',
          years: '2000/01 – 2004/05',
          multipliers: 'Single Unified Multiplier (41.2p in 2000/01 rising to 45.2p in 2004/05)',
          sbrr: 'None (0%)',
          transition: '5-year transitional relief phasing under Welsh statutory instruments',
          statute: 'GWA 1998 / LGFA 1988 / W.S.I. 1999/3444',
        },
        {
          list: '2005 List',
          years: '2005/06 – 2009/10',
          multipliers: 'Single Welsh Multiplier retained (42.2p in 2005/06 rising to 48.5p in 2009/10)',
          sbrr: 'Welsh Banded SBRR introduced: 50% relief for RV ≤ £2,000; 25% relief for RV £2,001–£5,000',
          transition: 'Devolved transitional phasing scheme. Rating (Empty Properties) Act 2007 abolished 50% empty rates on 1 April 2008',
          statute: 'W.S.I. 2005/257 / 2007 Act',
        },
        {
          list: '2010 List',
          years: '2010/11 – 2016/17',
          multipliers: 'Single Welsh Multiplier (40.9p in 2010/11 rising to 48.6p in 2016/17)',
          sbrr: 'Welsh SBRR expanded: 100% relief for RV ≤ £6,000, tapering linearly to £12,000 RV',
          transition: 'Welsh 5-year transitional phasing scheme',
          statute: 'W.S.I. 2010/414 / W.S.I. 2010/2908',
        },
        {
          list: '2017 List',
          years: '2017/18 – 2022/23',
          multipliers: 'Standard Welsh Multiplier (49.9p rising to 53.5p)',
          sbrr: '100% up to £6k RV, taper to £12k RV (max 2 properties). W.S.I. 2022/575 holiday let threshold (182 days actual let, 252 days available).',
          transition: 'W.S.I. 2017/262 Welsh transitional relief scheme',
          statute: 'W.S.I. 2017/262 / W.S.I. 2022/575',
        },
        {
          list: '2023 List',
          years: '2023/24 – 2025/26',
          multipliers: 'Welsh Multiplier (53.5p in 2023/24, 56.2p in 2024/25 & 2025/26)',
          sbrr: '100% up to £6k RV, taper to £12k RV. Temporary Welsh Retail, Leisure & Hospitality: 75% in 2023/24, 40% in 2024/25 & 2025/26 (£110k cap).',
          transition: '2-year transitional relief scheme under W.S.I. 2022/1350 capping net bill increases',
          statute: 'LGFA 1988 / W.S.I. 2022/1350',
        },
        {
          list: '2026 List',
          years: '2026/27 – 2028/29',
          multipliers: '3 Multipliers under LGF(W)A 2024: Retail Multiplier (35.0p for RV ≤ £51k), Standard (50.2p ≤ £100k), Higher (51.5p > £100k)',
          sbrr: '100% up to £6k RV, taper to £12k RV. 15% Food & Drink Hospitality Relief (petrol stations excluded).',
          transition: '2-year TR scheme under W.S.I. 2025/1371 (67% Year 1, 34% Year 2 over £300). W.S.I. 2026/43 anti-avoidance (180-day reset).',
          statute: 'LGF(W)A 2024 (asc 6) / W.S.I. 2025/1371 / W.S.I. 2026/43',
        },
      ],
    },
    SCOTLAND: {
      name: 'Scotland',
      legislation: [
        {
          key: 'NDR 2020 (Scotland)',
          url: 'https://www.legislation.gov.uk/asp/2020/4/contents',
          title: 'Non-Domestic Rates (Scotland) Act 2020 (asp 4) — Barclay Review Implementation',
          primary: true,
        },
        {
          key: '1975 Act (Valuation & Poundage)',
          url: 'https://www.legislation.gov.uk/ukpga/1975/30/contents',
          title: 'Local Government (Scotland) Act 1975 — Roll, Rate Setting & Statutory Powers',
          primary: true,
        },
        {
          key: '1962 Act (Charities & Mandatory)',
          url: 'https://www.legislation.gov.uk/ukpga/1962/9/contents',
          title: 'Local Government (Financial Provisions) (Scotland) Act 1962 — Mandatory Charity 80% Relief',
          primary: false,
        },
        {
          key: 'SSI 2023/45 (Transitional Relief)',
          url: 'https://www.legislation.gov.uk/ssi/2023/45/contents/made',
          title: 'The Non-Domestic Rating (Transitional Relief) (Scotland) Regulations 2023 (S.S.I. 2023/45)',
          primary: false,
        },
        {
          key: 'SSI 2022/86 (Holiday Let Criteria)',
          url: 'https://www.legislation.gov.uk/ssi/2022/86/contents/made',
          title: 'The Council Tax (Variation for Unoccupied Dwellings) (Scotland) Regulations 2022 (S.S.I. 2022/86)',
          primary: false,
        },
        {
          key: 'SSI 2010/336 (Renewable Energy)',
          url: 'https://www.legislation.gov.uk/ssi/2010/336/contents/made',
          title: 'The Non-Domestic Rates (Renewable Energy Generation Relief) (Scotland) Regulations 2010',
          primary: false,
        },
      ],
      currentRegimeSummary: {
        frameworkTitle: 'Current & Upcoming Regime (2023–2027)',
        frameworkSub: 'Local Government (Scotland) Act 1975 & Non-Domestic Rates (Scotland) Act 2020',
        multiplierTitle: 'Three-Tier Poundage System (Barclay Review Decoupled Rates):',
        multipliers: [
          { label: 'Basic Property Rate (BPR)', rate: '49.8p', rvThreshold: 'RV ≤ £51,000' },
          { label: 'Intermediate Property Rate', rate: '54.5p', rvThreshold: 'RV £51,001 to £100,000' },
          { label: 'Higher Property Rate (HPR)', rate: '55.9p', rvThreshold: 'RV > £100,000' },
        ],
        reliefHighlights: [
          {
            title: 'Small Business Bonus Scheme (SBBS)',
            desc: '100% relief for RV ≤ £12,000; tiered tapers: 100% to 25% for RV £12,001–£15,000; 25% to 0% for RV £15,001–£20,000; cumulative cap £35,000 across multiple properties.',
          },
          {
            title: 'Transitional Relief (S.S.I. 2023/45 & 2026/50)',
            desc: 'General TR and SBBS TR capping bill increases at £600 (Year 1), £1,200 (Year 2), £1,800 (Year 3), and a 25% cap in 2026/27.',
          },
          {
            title: 'Retail, Hospitality & Leisure (RHL)',
            desc: '15% discount on mainland (RV ≤ £100k) or 100% on islands under Islands (Scotland) Act 2018 (£110k cap).',
          },
          {
            title: 'Renewable Energy Generation Relief',
            desc: 'Tiered relief up to 100% for hydro, wind, solar, biomass under S.S.I. 2010/336.',
          },
        ],
      },
      preDevolution: {
        heading: 'UK Unified Statutory Rating Framework (Pre-Devolution: 1990–1999/2008)',
        era: '1990/91 to 2007/08 (1990, 1995, 2000 & early 2005 Rating Lists)',
        statute: 'Local Government (Scotland) Act 1975 & Scotland Act 1998',
        overview:
          'Prior to devolution under the Scotland Act 1998 and the subsequent creation of the Small Business Bonus Scheme (SBBS) on 1 April 2008, non-domestic rates in Scotland were administered under the UK Scottish Office framework.',
        rules: [
          'Single Scottish Poundage: A single uniform poundage rate (e.g. 52.5p in 1990/91, 43.2p in 1995/96) was set by the Secretary of State for Scotland, applied uniformly across all commercial properties.',
          'Pre-SBBS Era: No universal small business relief existed until the pilot Small Business Rate Relief Scheme (SBRRS) in 2003, which was later replaced by SBBS in 2008.',
          'Symmetrical Transitional Phasing: Scottish revaluations featured symmetrical transitional phasing schemes capping annual poundage shifts with upward limits and downward delay caps.',
          'Pre-2008 Empty Rating: 3 months initial empty property exemption followed by 50% indefinite rating.',
        ],
      },
      ratingLists: [
        {
          list: '1990 List',
          years: '1990/91 – 1994/95',
          multipliers: 'Single Scottish Poundage (52.5p in 1990/91 falling to 48.6p in 1994/95)',
          sbrr: 'None (0% — Pure RV × Poundage)',
          transition: 'Symmetrical transitional phasing scheme under Scottish statutory instruments',
          statute: 'Local Government (Scotland) Act 1975 / S.I. 1990/602',
        },
        {
          list: '1995 List',
          years: '1995/96 – 1999/00',
          multipliers: 'Single Scottish Poundage (43.2p in 1995/96 rising to 48.9p in 1999/00)',
          sbrr: 'None (0%)',
          transition: '5-year transitional phasing caps; 50% indefinite empty rates',
          statute: 'Local Government (Scotland) Act 1975 / S.I. 1995/543',
        },
        {
          list: '2000 List',
          years: '2000/01 – 2004/05',
          multipliers: 'Single Scottish Poundage (45.8p in 2000/01 rising to 48.8p in 2004/05)',
          sbrr: 'Small Business Rate Relief Scheme (SBRRS) pilot introduced in 2003',
          transition: 'Scottish transitional relief regulations',
          statute: 'Scotland Act 1998 / 1975 Act / S.S.I. 2000/87',
        },
        {
          list: '2005 List',
          years: '2005/06 – 2009/10',
          multipliers: 'Single Poundage (41.5p in 2005/06 rising to 48.1p in 2009/10)',
          sbrr: 'Small Business Bonus Scheme (SBBS) created on 1 April 2008: 100% relief for single businesses with RV ≤ £8k/£10k',
          transition: 'Devolved transitional phasing schemes under Scottish statutory instruments',
          statute: 'S.S.I. 2008/106 / 1975 Act',
        },
        {
          list: '2010 List',
          years: '2010/11 – 2016/17',
          multipliers: 'Basic Poundage (40.7p in 2010/11 rising to 48.4p in 2016/17); Large Business Supplement on RV > £35k',
          sbrr: 'SBBS expanded: 100% relief for RV ≤ £10k; 50% for RV £10,001–£12k; 25% for RV £12,001–£18k',
          transition: 'Scottish 5-year transitional phasing scheme',
          statute: 'S.S.I. 2010/37 / S.S.I. 2010/336 (Renewables)',
        },
        {
          list: '2017 List',
          years: '2017/18 – 2022/23',
          multipliers: 'Barclay Review Decoupling: Basic (46.6p rising to 49.8p), Intermediate (51.1p), Higher (52.4p)',
          sbrr: 'SBBS 100% relief up to £15,000 RV. S.S.I. 2022/86 holiday let letting days criteria (70 days actual let, 140 days available).',
          transition: 'Scottish transitional relief regulations',
          statute: 'S.S.I. 2017/85 / NDR Act 2020 / S.S.I. 2022/86',
        },
        {
          list: '2023 List',
          years: '2023/24 – 2025/26',
          multipliers: '3-Tier Poundage: Basic (49.8p ≤ £51k), Intermediate (54.5p £51k–£100k), Higher (55.9p > £100k)',
          sbrr: 'SBBS reformed with tiered tapers (100% ≤ £12k, 100% → 25% £12k–£15k, 25% → 0% £15k–£20k; £35k cumulative cap)',
          transition: 'General TR and SBBS TR under S.S.I. 2023/45 capping bill increases at £600 (Year 1), £1,200 (Year 2), £1,800 (Year 3)',
          statute: 'NDR (Scotland) Act 2020 / S.S.I. 2023/45',
        },
        {
          list: '2026 List',
          years: '2026/27 – 2028/29',
          multipliers: '3-Tier Poundage: Basic (49.8p ≤ £51k), Intermediate (54.5p £51k–£100k), Higher (55.9p > £100k)',
          sbrr: '100% SBBS up to £12k RV with tiered tapers to £20k RV. SBBS TR 25% bill increase cap in 2026/27 under S.S.I. 2026/50. 15% mainland / 100% island RHL relief.',
          transition: 'SBBS TR 25% cap; Renewable Energy Generation relief (up to 100% under S.S.I. 2010/336)',
          statute: 'NDR Act 2020 / S.S.I. 2026/50 / Islands Act 2018',
        },
      ],
    },
  };
</script>

<!-- Statutory Framework Section -->
<section class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-sm overflow-hidden transition-all">
  <!-- Expandable Header Bar -->
  <div class="px-5 py-4 bg-white dark:bg-slate-900">
    <button
      type="button"
      onclick={() => (isExpanded = !isExpanded)}
      aria-expanded={isExpanded}
      class="w-full flex items-center justify-between text-left group p-1 -m-1 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800/60 transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-blue-500"
    >
      <div class="flex items-center gap-3">
        <span class="inline-flex items-center justify-center w-7 h-7 rounded-lg bg-blue-100 dark:bg-blue-950 text-blue-700 dark:text-blue-300 text-sm font-bold shadow-xs">
          §
        </span>
        <div>
          <div class="flex items-center gap-2">
            <h2 class="text-base font-bold text-slate-900 dark:text-slate-50">
              {calcStore.t.statutory_framework_title}
            </h2>
            <span class="text-xs font-semibold px-2 py-0.5 rounded bg-blue-50 dark:bg-blue-950 text-blue-700 dark:text-blue-300 border border-blue-200 dark:border-blue-800">
              {calcStore.t.statutory_coverage_sub}
            </span>
          </div>
          <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
            {calcStore.t.statutory_framework_sub}
          </p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <span class="text-xs font-semibold text-blue-600 dark:text-blue-400 group-hover:underline">
          {isExpanded ? calcStore.t.statutory_framework_collapse : calcStore.t.statutory_framework_expand}
        </span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-4 h-4 text-slate-400 dark:text-slate-500 transition-transform duration-200 {isExpanded ? 'rotate-180 text-blue-600' : ''}"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </div>
    </button>
  </div>

  <!-- Expandable Content Panel -->
  {#if isExpanded}
    <div class="border-t border-slate-200 dark:border-slate-800 p-6 bg-slate-50/40 dark:bg-slate-950/40 space-y-6">
      <!-- Country Selection Buttons (England, Wales, Scotland) with No Icons -->
      <div class="flex flex-wrap gap-2.5 pb-2 border-b border-slate-200 dark:border-slate-800" role="tablist">
        <button
          type="button"
          role="tab"
          aria-selected={selectedCountry === 'ENGLAND'}
          onclick={() => (selectedCountry = 'ENGLAND')}
          class="px-5 py-2 rounded-lg text-xs font-bold transition border {selectedCountry === 'ENGLAND'
            ? 'bg-blue-700 text-white border-blue-700 shadow-sm ring-2 ring-blue-400/40'
            : 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-950 dark:hover:text-white border-slate-300 dark:border-slate-700'}"
        >
          England
        </button>
        <button
          type="button"
          role="tab"
          aria-selected={selectedCountry === 'WALES'}
          onclick={() => (selectedCountry = 'WALES')}
          class="px-5 py-2 rounded-lg text-xs font-bold transition border {selectedCountry === 'WALES'
            ? 'bg-emerald-700 text-white border-emerald-700 shadow-sm ring-2 ring-emerald-400/40'
            : 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-950 dark:hover:text-white border-slate-300 dark:border-slate-700'}"
        >
          Wales
        </button>
        <button
          type="button"
          role="tab"
          aria-selected={selectedCountry === 'SCOTLAND'}
          onclick={() => (selectedCountry = 'SCOTLAND')}
          class="px-5 py-2 rounded-lg text-xs font-bold transition border {selectedCountry === 'SCOTLAND'
            ? 'bg-purple-700 text-white border-purple-700 shadow-sm ring-2 ring-purple-400/40'
            : 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-950 dark:hover:text-white border-slate-300 dark:border-slate-700'}"
        >
          Scotland
        </button>
      </div>

      <!-- Country Region Details -->
      {#key selectedCountry}
        <div class="space-y-6 animate-fadeIn">
          <!-- Top of Country Region: Appropriate Legislation Links -->
          <div class="bg-white dark:bg-slate-900 rounded-xl p-4 border border-slate-200 dark:border-slate-800 shadow-xs flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
            <div>
              <span class="text-[11px] font-bold text-slate-600 dark:text-slate-300 uppercase tracking-wider block">
                {countryData[selectedCountry].name} Statutory Authorities
              </span>
              <span class="text-xs text-slate-800 dark:text-slate-100 font-semibold">
                Official Primary Legislation & Statutory Instruments (legislation.gov.uk)
              </span>
            </div>
            <div class="flex flex-wrap gap-2 text-xs">
              {#each countryData[selectedCountry].legislation as b}
                <a
                  href={b.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  title={b.title + " (opens on legislation.gov.uk)"}
                  class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded text-xs transition border group {b.primary
                    ? 'bg-blue-50 dark:bg-blue-950/60 text-blue-900 dark:text-blue-200 hover:bg-blue-100 dark:hover:bg-blue-900/60 hover:text-blue-950 dark:hover:text-white border-blue-300 dark:border-blue-700 font-bold'
                    : 'bg-slate-100 dark:bg-slate-800 text-slate-800 dark:text-slate-200 hover:bg-slate-200 dark:hover:bg-slate-700 hover:text-slate-950 dark:hover:text-white border-slate-300 dark:border-slate-700 font-semibold'}"
                >
                  <span>{b.key}</span>
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 opacity-50 group-hover:opacity-100 transition-opacity" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z" />
                    <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z" />
                  </svg>
                </a>
              {/each}
            </div>
          </div>

          <!-- Section 1: Pre-Devolution UK Regulations Block -->
          <div class="bg-amber-50/70 dark:bg-amber-950/30 rounded-xl p-5 border border-amber-200 dark:border-amber-800/60 shadow-xs space-y-3">
            <div class="flex items-center justify-between border-b border-amber-200/80 dark:border-amber-800/50 pb-2">
              <div class="flex items-center gap-2">
                <span class="w-2.5 h-2.5 rounded-full bg-amber-500"></span>
                <h3 class="font-bold text-xs sm:text-sm text-amber-950 dark:text-amber-200">
                  {countryData[selectedCountry].preDevolution.heading}
                </h3>
              </div>
              <span class="text-[11px] font-mono px-2 py-0.5 rounded bg-amber-100 dark:bg-amber-900/60 text-amber-800 dark:text-amber-300 border border-amber-300 dark:border-amber-700">
                {countryData[selectedCountry].preDevolution.era}
              </span>
            </div>
            <p class="text-xs text-amber-900/90 dark:text-amber-200/90 leading-relaxed">
              {countryData[selectedCountry].preDevolution.overview}
            </p>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2 text-xs text-amber-950 dark:text-amber-100">
              {#each countryData[selectedCountry].preDevolution.rules as rule}
                <div class="flex items-start gap-2 bg-white/70 dark:bg-slate-900/60 p-2.5 rounded-lg border border-amber-200/60 dark:border-amber-800/40">
                  <span class="text-amber-600 dark:text-amber-400 font-bold">•</span>
                  <span class="leading-relaxed">{rule}</span>
                </div>
              {/each}
            </div>
          </div>

          <!-- Section 2: Summary Regulations & Current Regime (2023–2027) -->
          <div class="bg-white dark:bg-slate-900 rounded-xl p-5 border border-slate-200 dark:border-slate-800 shadow-xs space-y-4">
            <div class="flex items-center justify-between border-b border-slate-100 dark:border-slate-800 pb-2">
              <div class="flex items-center gap-2">
                <span class="w-2.5 h-2.5 rounded-full bg-blue-600"></span>
                <h3 class="font-bold text-xs sm:text-sm text-slate-900 dark:text-slate-100">
                  {countryData[selectedCountry].currentRegimeSummary.frameworkTitle}
                </h3>
              </div>
              <span class="text-xs font-medium text-slate-500 dark:text-slate-400">
                {countryData[selectedCountry].currentRegimeSummary.frameworkSub}
              </span>
            </div>

            <!-- Multiplier Architecture Grid -->
            <div>
              <h4 class="text-xs font-bold text-slate-800 dark:text-slate-200 mb-2">
                {countryData[selectedCountry].currentRegimeSummary.multiplierTitle}
              </h4>
              <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2 text-xs font-mono">
                {#each countryData[selectedCountry].currentRegimeSummary.multipliers as m}
                  <div class="p-2.5 rounded-lg bg-slate-50 dark:bg-slate-800/60 border border-slate-200 dark:border-slate-700 flex flex-col justify-between">
                    <span class="text-[11px] font-sans font-semibold text-slate-600 dark:text-slate-400">{m.label}</span>
                    <div class="flex items-baseline justify-between mt-1">
                      <span class="text-sm font-bold text-blue-600 dark:text-blue-400">{m.rate}</span>
                      <span class="text-[10px] text-slate-500 dark:text-slate-400 font-sans">{m.rvThreshold}</span>
                    </div>
                  </div>
                {/each}
              </div>
            </div>

            <!-- Reliefs and Protections -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs pt-1">
              {#each countryData[selectedCountry].currentRegimeSummary.reliefHighlights as rh}
                <div class="p-3 rounded-lg bg-slate-50/60 dark:bg-slate-800/40 border border-slate-100 dark:border-slate-800">
                  <span class="font-bold text-slate-900 dark:text-slate-100 block mb-0.5">{rh.title}:</span>
                  <span class="text-slate-600 dark:text-slate-300 leading-relaxed block">{rh.desc}</span>
                </div>
              {/each}
            </div>
          </div>

          <!-- Section 3: Country-Specific 8 Rating Lists Matrix (1990–2027) -->
          <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs overflow-hidden space-y-0">
            <div class="p-4 bg-slate-50/80 dark:bg-slate-800/60 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between">
              <div>
                <h4 class="font-bold text-xs sm:text-sm text-slate-900 dark:text-slate-100">
                  {countryData[selectedCountry].name} — 8 Statutory Rating Lists Timeline (1990–2027)
                </h4>
                <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-0.5">
                  Complete rating lists history from inaugural 1990 list to modern 2026/27 list
                </p>
              </div>
              <span class="text-xs font-bold px-2.5 py-1 rounded-md bg-blue-50 dark:bg-blue-950 text-blue-700 dark:text-blue-300 border border-blue-200 dark:border-blue-800">
                8 Rating Lists
              </span>
            </div>

            <div class="overflow-x-auto">
              <table class="w-full text-left border-collapse text-xs">
                <thead>
                  <tr class="bg-slate-100 dark:bg-slate-800/80 text-slate-700 dark:text-slate-300 border-b border-slate-200 dark:border-slate-700 font-semibold">
                    <th class="p-3 whitespace-nowrap">Rating List</th>
                    <th class="p-3 whitespace-nowrap">Active Period</th>
                    <th class="p-3">{countryData[selectedCountry].name} Multipliers</th>
                    <th class="p-3">Small Business Relief Scheme</th>
                    <th class="p-3">Transitional Phasing & Empty Rates</th>
                    <th class="p-3 whitespace-nowrap">Governing Statutory Instruments</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-slate-100 dark:divide-slate-800 text-slate-600 dark:text-slate-300">
                  {#each countryData[selectedCountry].ratingLists as item}
                    <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition">
                      <td class="p-3 font-bold text-slate-900 dark:text-slate-100 whitespace-nowrap">
                        {item.list}
                      </td>
                      <td class="p-3 font-mono text-[11px] whitespace-nowrap text-blue-600 dark:text-blue-400 font-medium">
                        {item.years}
                      </td>
                      <td class="p-3 min-w-[200px]">
                        {item.multipliers}
                      </td>
                      <td class="p-3 min-w-[220px]">
                        {item.sbrr}
                      </td>
                      <td class="p-3 min-w-[220px]">
                        {item.transition}
                      </td>
                      <td class="p-3 font-mono text-[10px] text-slate-500 dark:text-slate-400 whitespace-nowrap">
                        {item.statute}
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      {/key}
    </div>
  {/if}
</section>
