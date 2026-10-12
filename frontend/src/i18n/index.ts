import en from '../locales/en.json';
import cy from '../locales/cy.json';
import sco from '../locales/sco.json';
import gd from '../locales/gd.json';
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
  return (dictionaries[lang] || dictionaries.en) as Dictionary;
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

export const PILL_CLASSES: Record<Jurisdiction, string> = {
  ENGLAND: 'bg-blue-100 text-blue-800',
  WALES: 'bg-emerald-100 text-emerald-800',
  SCOTLAND: 'bg-purple-100 text-purple-900',
};

export function getJurisdictionDetail(lang: SupportedLanguage, jur: Jurisdiction): JurisdictionDetail {
  const dict = getDictionary(lang);
  const j = dict.jurisdictions[jur] || dict.jurisdictions.ENGLAND;
  return {
    ...j,
    pillClass: PILL_CLASSES[jur] || PILL_CLASSES.ENGLAND,
  };
}

export const JURISDICTION_CONFIGS: Record<SupportedLanguage, Record<Jurisdiction, JurisdictionDetail>> = {
  en: {
    ENGLAND: getJurisdictionDetail('en', 'ENGLAND'),
    WALES: getJurisdictionDetail('en', 'WALES'),
    SCOTLAND: getJurisdictionDetail('en', 'SCOTLAND'),
  },
  cy: {
    ENGLAND: getJurisdictionDetail('cy', 'ENGLAND'),
    WALES: getJurisdictionDetail('cy', 'WALES'),
    SCOTLAND: getJurisdictionDetail('cy', 'SCOTLAND'),
  },
  sco: {
    ENGLAND: getJurisdictionDetail('sco', 'ENGLAND'),
    WALES: getJurisdictionDetail('sco', 'WALES'),
    SCOTLAND: getJurisdictionDetail('sco', 'SCOTLAND'),
  },
  gd: {
    ENGLAND: getJurisdictionDetail('gd', 'ENGLAND'),
    WALES: getJurisdictionDetail('gd', 'WALES'),
    SCOTLAND: getJurisdictionDetail('gd', 'SCOTLAND'),
  },
};
