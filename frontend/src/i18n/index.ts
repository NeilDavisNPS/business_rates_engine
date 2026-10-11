import { en } from './en';
import { cy } from './cy';
import { sco } from './sco';
import { gd } from './gd';
import type { SupportedLanguage, Jurisdiction } from '../types';

export const dictionaries = {
  en,
  cy,
  sco,
  gd,
} as const;

export type TranslationKey = keyof typeof en;
export type Dictionary = typeof en;

export function getDictionary(lang: SupportedLanguage): Dictionary {
  return dictionaries[lang] || dictionaries.en;
}

export interface JurisdictionDetail {
  name: string;
  pillClass: string;
  bannerBg: string;
  framework: string;
  sbrrTitle: string;
  sbrrDesc: string;
  rhlTitle: string;
  rhlDesc: string;
  charityDesc: string;
}

export const JURISDICTION_CONFIGS: Record<SupportedLanguage, Record<Jurisdiction, JurisdictionDetail>> = {
  en: {
    ENGLAND: {
      name: "England",
      pillClass: "bg-blue-100 text-blue-800",
      bannerBg: "bg-blue-50 border-blue-200 text-blue-900",
      framework: "Local Government Finance Act 1988 & Non-Domestic Rating Act 2023 (NDRA 2023). Standard Multiplier applies for RV > £51,000. 3-Year revaluation list.",
      sbrrTitle: "Small Business Rate Relief (SBRR)",
      sbrrDesc: "100% discount up to £12k RV; linear taper to £15k RV for sole-property occupiers.",
      rhlTitle: "Retail, Hospitality & Leisure (RHL)",
      rhlDesc: "40% statutory discount for 2026/27 (subject to £110k cap).",
      charityDesc: "80% mandatory relief under LGFA 1988 s.43(5)."
    },
    WALES: {
      name: "Wales",
      pillClass: "bg-emerald-100 text-emerald-800",
      bannerBg: "bg-emerald-50 border-emerald-200 text-emerald-900",
      framework: "Local Government Finance (Wales) Act 2024. WSI statutory instruments, differential multiplier system, anti-avoidance penalty framework.",
      sbrrTitle: "Welsh Small Business Rate Relief",
      sbrrDesc: "100% discount up to £6k RV; linear taper to £12k RV under devolved Welsh rules.",
      rhlTitle: "Welsh Retail, Leisure & Hospitality Relief",
      rhlDesc: "40% Welsh scheme for 2026/27 (subject to £110k cap). Note: Petrol stations excluded.",
      charityDesc: "80% mandatory relief under LGFA 1988 s.43(5)."
    },
    SCOTLAND: {
      name: "Scotland",
      pillClass: "bg-purple-100 text-purple-900",
      bannerBg: "bg-purple-50 border-purple-200 text-purple-900",
      framework: "Non-Domestic Rates (Scotland) Act 2020. Three-tier multiplier system (Basic, Intermediate, Higher). Statutory holiday let criteria in effect.",
      sbrrTitle: "Small Business Bonus Scheme (SBBS)",
      sbrrDesc: "Scottish 100% relief up to £11,500/£12,000 RV for single-property ratepayers.",
      rhlTitle: "Sector Relief",
      rhlDesc: "Not applicable under England/Wales RHL scheme.",
      charityDesc: "80% mandatory relief under Local Government (Financial Provisions) (Scotland) Act 1962 s.4."
    }
  },
  cy: {
    ENGLAND: {
      name: "Lloegr",
      pillClass: "bg-blue-100 text-blue-800",
      bannerBg: "bg-blue-50 border-blue-200 text-blue-900",
      framework: "Deddf Cyllid Llywodraeth Leol 1988 a Deddf Ardrethi Annomestig 2023 (NDRA 2023). Mae'r Lluosydd Safonol yn berthnasol ar gyfer RV > £51,000. Rhestr ailbrisio 3 blynedd.",
      sbrrTitle: "Rhyddhad Ardrethi Busnesau Bach (SBRR)",
      sbrrDesc: "Disgownt o 100% hyd at £12k RV; tapr llinol i £15k RV ar gyfer meddianwyr un eiddo.",
      rhlTitle: "Manwerthu, Lletygarwch a Hamdden (RHL)",
      rhlDesc: "Disgownt statudol o 40% ar gyfer 2026/27 (yn amodol ar gap o £110k).",
      charityDesc: "Rhyddhad gorfodol o 80% o dan DCLL 1988 adr. 43(5)."
    },
    WALES: {
      name: "Cymru",
      pillClass: "bg-emerald-100 text-emerald-800",
      bannerBg: "bg-emerald-50 border-emerald-200 text-emerald-900",
      framework: "Deddf Cyllid Llywodraeth Leol (Cymru) 2024. Offerynnau statudol OSC, system luosydd wahaniaethol, fframwaith cosbau gwrth-osgoi.",
      sbrrTitle: "Rhyddhad Busnesau Bach Cymru",
      sbrrDesc: "Disgownt o 100% hyd at £6k RV; tapr llinol i £12k RV o dan reolau datganoledig Cymru.",
      rhlTitle: "Rhyddhad Manwerthu, Hamdden a Lletygarwch Cymru",
      rhlDesc: "Cynllun Cymru o 40% ar gyfer 2026/27 (yn amodol ar gap o £110k). Nodyn: Gorsafoedd petrol wedi'u heithrio.",
      charityDesc: "Rhyddhad gorfodol o 80% o dan DCLL 1988 adr. 43(5)."
    },
    SCOTLAND: {
      name: "Yr Alban",
      pillClass: "bg-purple-100 text-purple-900",
      bannerBg: "bg-purple-50 border-purple-200 text-purple-900",
      framework: "Deddf Ardrethi Annomestig (Yr Alban) 2020. System luosydd tair haen (Sylfaenol, Canolradd, Uwch). Meini prawf statudol llety gwyliau ar waith.",
      sbrrTitle: "Cynllun Bonws Busnesau Bach (SBBS)",
      sbrrDesc: "Rhyddhad 100% yr Alban hyd at £11,500/£12,000 RV ar gyfer trethdalwyr un eiddo.",
      rhlTitle: "Rhyddhad Sector",
      rhlDesc: "Ddim yn berthnasol o dan gynllun RHL Lloegr/Cymru.",
      charityDesc: "Rhyddhad gorfodol o 80% o dan Ddeddf Llywodraeth Leol (Yr Alban) 1962 adr. 4."
    }
  },
  sco: {
    ENGLAND: {
      name: "Ingland",
      pillClass: "bg-blue-100 text-blue-800",
      bannerBg: "bg-blue-50 border-blue-200 text-blue-900",
      framework: "Local Government Finance Act 1988 & Non-Domestic Rating Act 2023 (NDRA 2023). Standard Multiplier applies for RV > £51,000. 3-Year revaluation leet.",
      sbrrTitle: "Small Business Rate Relief (SBRR)",
      sbrrDesc: "100% discoont up tae £12k RV; linear taper tae £15k RV for sole-proaperty occupiers.",
      rhlTitle: "Retail, Hospitality & Leisure (RHL)",
      rhlDesc: "40% statutary discoont for 2026/27 (subject tae £110k cap).",
      charityDesc: "80% mandatar relief unner LGFA 1988 s.43(5)."
    },
    WALES: {
      name: "Wales",
      pillClass: "bg-emerald-100 text-emerald-800",
      bannerBg: "bg-emerald-50 border-emerald-200 text-emerald-900",
      framework: "Local Government Finance (Wales) Act 2024. WSI statutary instruments, differential multiplier seestem, anti-avoidance penalty framewark.",
      sbrrTitle: "Welsh Small Business Rate Relief",
      sbrrDesc: "100% discoont up tae £6k RV; linear taper tae £12k RV unner devolved Welsh rules.",
      rhlTitle: "Welsh Retail, Leisure & Hospitality Relief",
      rhlDesc: "40% Welsh scheme for 2026/27 (subject tae £110k cap). Note: Petrol stations excludit.",
      charityDesc: "80% mandatar relief unner LGFA 1988 s.43(5)."
    },
    SCOTLAND: {
      name: "Scotland",
      pillClass: "bg-purple-100 text-purple-900",
      bannerBg: "bg-purple-50 border-purple-200 text-purple-900",
      framework: "Non-Domestic Rates (Scotland) Act 2020. Three-tier multiplier seestem (Basic, Intermediate, Heicher). Statutary halyday let creeteria in effeck.",
      sbrrTitle: "Small Business Bonus Scheme (SBBS)",
      sbrrDesc: "Scots 100% relief up tae £11,500/£12,000 RV for single-proaperty ratepeyers.",
      rhlTitle: "Sector Relief",
      rhlDesc: "Nae applicable unner Ingland/Wales RHL scheme.",
      charityDesc: "80% mandatar relief unner Local Government (Financial Provisions) (Scotland) Act 1962 s.4."
    }
  },
  gd: {
    ENGLAND: {
      name: "Sasainn",
      pillClass: "bg-blue-100 text-blue-800",
      bannerBg: "bg-blue-50 border-blue-200 text-blue-900",
      framework: "Local Government Finance Act 1988 & Non-Domestic Rating Act 2023 (NDRA 2023). Bidh am prìomh iomadair an sàs airson RV > £51,000. Liosta ath-luachaidh 3-bliadhna.",
      sbrrTitle: "Faochadh Ìrean Gnìomhachais Bheaga (SBRR)",
      sbrrDesc: "Lasachadh 100% suas gu £12k RV; teip loidhneach gu £15k RV airson luchd-còmhnaidh aon-sheilbh.",
      rhlTitle: "Mion-reic, Aoigheachd & Cur-seachad (RHL)",
      rhlDesc: "Lasachadh reachdail 40% airson 2026/27 (fo smachd crìoch £110k).",
      charityDesc: "Faochadh èigneachail 80% fo LGFA 1988 e.43(5)."
    },
    WALES: {
      name: "A' Chuimrigh",
      pillClass: "bg-emerald-100 text-emerald-800",
      bannerBg: "bg-emerald-50 border-emerald-200 text-emerald-900",
      framework: "Local Government Finance (Wales) Act 2024. Innealan reachdail WSI, siostam iomadair eadar-dhealaichte, frèam peanais an-aghaidh seachnadh.",
      sbrrTitle: "Faochadh Gnìomhachasan Beaga na Cuimrigh",
      sbrrDesc: "Lasachadh 100% suas gu £6k RV; teip loidhneach gu £12k RV fo riaghailtean tiomnaichte na Cuimrigh.",
      rhlTitle: "Faochadh Mion-reic, Cur-seachad & Aoigheachd na Cuimrigh",
      rhlDesc: "Sgeama Cuimreach 40% airson 2026/27 (fo smachd crìoch £110k). Nòta: Stèiseanan peatrail air an dùnadh a-mach.",
      charityDesc: "Faochadh èigneachail 80% fo LGFA 1988 e.43(5)."
    },
    SCOTLAND: {
      name: "Alba",
      pillClass: "bg-purple-100 text-purple-900",
      bannerBg: "bg-purple-50 border-purple-200 text-purple-900",
      framework: "Achd nan Ìrean Neo-Dachaigheil (Alba) 2020. Siostam iomadair trì-ìre (Bunaiteach, Meadhanach, Àrd). Slatan-tomhais màil saor-làithean reachdail an sàs.",
      sbrrTitle: "Sgeama Bònas Ghnìomhachasan Beaga (SBBS)",
      sbrrDesc: "Faochadh Albannach 100% suas gu £11,500/£12,000 RV airson luchd-pàighidh aon-sheilbh.",
      rhlTitle: "Faochadh Roinne",
      rhlDesc: "Chan eil e iomchaidh fo sgeama RHL Shasainn/na Cuimrigh.",
      charityDesc: "Faochadh èigneachail 80% fo Local Government (Financial Provisions) (Scotland) Act 1962 e.4."
    }
  }
};
