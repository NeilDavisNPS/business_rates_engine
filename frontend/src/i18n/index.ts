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
      sbrrTitle: "Small Business Rate Relief (SBRR)",
      sbrrDesc: "100% discount up to £12k RV; linear taper to £15k RV for sole-property occupiers (plus 12-month second property grace period under SI 2017/411).",
      rhlTitle: "Retail, Hospitality & Leisure (RHL)",
      rhlDesc: "Discretionary 40% discount in 2025/26 (£110k cap). From 2026/27 replaced by 2 permanent lower high-street multipliers (38.2p for RV < £51k, 43.0p for RV £51k–£500k; RV >= £500k pays 50.8p High-Value rate).",
      charityDesc: "80% mandatory relief under LGFA 1988 s.43(5), plus billing authority discretionary top-up of up to 20% under s.47(2)(a)."
    },
    WALES: {
      name: "Wales",
      pillClass: "bg-emerald-100 text-emerald-800",
      sbrrTitle: "Welsh Small Business Rate Relief",
      sbrrDesc: "100% discount up to £6k RV; linear taper to £12k RV under devolved Welsh rules (maximum 2 properties per ratepayer).",
      rhlTitle: "Welsh Food & Drink Hospitality Relief",
      rhlDesc: "15% Welsh scheme for 2026/27 (subject to £110k cap). Note: Petrol stations excluded.",
      charityDesc: "80% mandatory relief under LGFA 1988 s.43(5), plus billing authority discretionary top-up of up to 20% under s.47(2)(a)."
    },
    SCOTLAND: {
      name: "Scotland",
      pillClass: "bg-purple-100 text-purple-900",
      sbrrTitle: "Small Business Bonus Scheme (SBBS)",
      sbrrDesc: "Scottish 100% relief up to £12,000 RV for single-property ratepayers, with tiered tapers up to £20,000 RV (£35k cumulative cap).",
      rhlTitle: "Scottish Retail, Hospitality & Leisure Relief",
      rhlDesc: "15% relief on mainland (RV <= £100k) or 100% on islands (Islands (Scotland) Act 2018), £110k cap.",
      charityDesc: "80% mandatory relief under 1962 Act s.4(2), plus discretionary top-up of up to 20% under s.4(5)."
    }
  },
  cy: {
    ENGLAND: {
      name: "Lloegr",
      pillClass: "bg-blue-100 text-blue-800",
      sbrrTitle: "Rhyddhad Ardrethi Busnesau Bach (SBRR)",
      sbrrDesc: "Disgownt o 100% hyd at £12k RV; tapr llinol i £15k RV ar gyfer meddianwyr un eiddo.",
      rhlTitle: "Manwerthu, Lletygarwch a Hamdden (RHL)",
      rhlDesc: "Disgownt o 40% yn 2025/26 (cap £110k). Daeth i ben yn 2026/27 (disodlwyd gan luosyddion stryd fawr is).",
      charityDesc: "Rhyddhad gorfodol o 80% o dan DCLL 1988 adr. 43(5)."
    },
    WALES: {
      name: "Cymru",
      pillClass: "bg-emerald-100 text-emerald-800",
      sbrrTitle: "Rhyddhad Busnesau Bach Cymru",
      sbrrDesc: "Disgownt o 100% hyd at £6k RV; tapr llinol i £12k RV o dan reolau datganoledig Cymru.",
      rhlTitle: "Cynllun Lletygarwch Bwyd a Diod Cymru",
      rhlDesc: "Cynllun Cymru o 15% ar gyfer 2026/27 (yn amodol ar gap o £110k). Nodyn: Gorsafoedd petrol wedi'u heithrio.",
      charityDesc: "Rhyddhad gorfodol o 80% o dan DCLL 1988 adr. 43(5)."
    },
    SCOTLAND: {
      name: "Yr Alban",
      pillClass: "bg-purple-100 text-purple-900",
      sbrrTitle: "Cynllun Bonws Busnesau Bach (SBBS)",
      sbrrDesc: "Rhyddhad 100% yr Alban hyd at £11,500/£12,000 RV ar gyfer trethdalwyr un eiddo.",
      rhlTitle: "Rhyddhad Manwerthu a Lletygarwch yr Alban",
      rhlDesc: "15% ar y tir mawr (RV <= £100k) neu 100% ar ynysoedd (Deddf Ynysoedd 2018), cap o £110k.",
      charityDesc: "Rhyddhad gorfodol o 80% o dan Ddeddf Llywodraeth Leol (Yr Alban) 1962 adr. 4."
    }
  },
  sco: {
    ENGLAND: {
      name: "Ingland",
      pillClass: "bg-blue-100 text-blue-800",
      sbrrTitle: "Small Business Rate Relief (SBRR)",
      sbrrDesc: "100% discoont up tae £12k RV; linear taper tae £15k RV for sole-proaperty occupiers.",
      rhlTitle: "Retail, Hospitality & Leisure (RHL)",
      rhlDesc: "40% discoont in 2025/26 (£110k cap). Expired in 2026/27 (replaced by lawer high-street multipliers).",
      charityDesc: "80% mandatar relief unner LGFA 1988 s.43(5)."
    },
    WALES: {
      name: "Wales",
      pillClass: "bg-emerald-100 text-emerald-800",
      sbrrTitle: "Welsh Small Business Rate Relief",
      sbrrDesc: "100% discoont up tae £6k RV; linear taper tae £12k RV unner devolved Welsh rules.",
      rhlTitle: "Welsh Food & Drink Hospitality Relief",
      rhlDesc: "15% Welsh scheme for 2026/27 (subject tae £110k cap). Note: Petrol stations excludit.",
      charityDesc: "80% mandatar relief unner LGFA 1988 s.43(5)."
    },
    SCOTLAND: {
      name: "Scotland",
      pillClass: "bg-purple-100 text-purple-900",
      sbrrTitle: "Small Business Bonus Scheme (SBBS)",
      sbrrDesc: "Scots 100% relief up tae £11,500/£12,000 RV for single-proaperty ratepeyers.",
      rhlTitle: "Scots Retail, Hospitality & Leisure Relief",
      rhlDesc: "15% relief on mainland (RV <= £100k) or 100% on islands (Islands (Scotland) Act 2018), £110k cap.",
      charityDesc: "80% mandatar relief unner Local Government (Financial Provisions) (Scotland) Act 1962 s.4."
    }
  },
  gd: {
    ENGLAND: {
      name: "Sasainn",
      pillClass: "bg-blue-100 text-blue-800",
      sbrrTitle: "Faochadh Ìrean Gnìomhachais Bheaga (SBRR)",
      sbrrDesc: "Lasachadh 100% suas gu £12k RV; teip loidhneach gu £15k RV airson luchd-còmhnaidh aon-sheilbh.",
      rhlTitle: "Mion-reic, Aoigheachd & Cur-seachad (RHL)",
      rhlDesc: "Lasachadh 40% ann an 2025/26 (crìoch £110k). Thàinig e gu crìch ann an 2026/27.",
      charityDesc: "Faochadh èigneachail 80% fo LGFA 1988 e.43(5)."
    },
    WALES: {
      name: "A' Chuimrigh",
      pillClass: "bg-emerald-100 text-emerald-800",
      sbrrTitle: "Faochadh Gnìomhachasan Beaga na Cuimrigh",
      sbrrDesc: "Lasachadh 100% suas gu £6k RV; teip loidhneach gu £12k RV fo riaghailtean tiomnaichte na Cuimrigh.",
      rhlTitle: "Faochadh Aoigheachd Bidhe is Dibhe na Cuimrigh",
      rhlDesc: "Sgeama Cuimreach 15% airson 2026/27 (fo smachd crìoch £110k). Nòta: Stèiseanan peatrail air an dùnadh a-mach.",
      charityDesc: "Faochadh èigneachail 80% fo LGFA 1988 e.43(5)."
    },
    SCOTLAND: {
      name: "Alba",
      pillClass: "bg-purple-100 text-purple-900",
      sbrrTitle: "Sgeama Bònas Ghnìomhachasan Beaga (SBBS)",
      sbrrDesc: "Faochadh Albannach 100% suas gu £11,500/£12,000 RV airson luchd-pàighidh aon-sheilbh.",
      rhlTitle: "Faochadh Mion-reic is Aoigheachd na h-Alba",
      rhlDesc: "15% air tìr-mòr (RV <= £100k) no 100% air eileanan (Achd nan Eilean 2018), crìoch £110k.",
      charityDesc: "Faochadh èigneachail 80% fo Local Government (Financial Provisions) (Scotland) Act 1962 e.4."
    }
  }
};
