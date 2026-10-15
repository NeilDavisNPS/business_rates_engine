# Business Rates Calculation Engine — Specification, Test Cases & Source Index

> **Target Environment:** Full-Stack Rust Workspace & Svelte 5 Component Framework  
> **Backend Architecture:** Axum 0.7 + Tokio 1.38 + Built-In Parameter Repository (Seeded 1990–2027)  
> **Frontend Architecture:** Svelte 5 (Runes `$state`, `$derived`) + Vite 6 + TypeScript 5.7 + Standalone Tailwind CSS (`frontend/`)  
> **API Access Mode:** Public REST API  
> **API Documentation:** Interactive Scalar API Reference (`/docs`) & OpenAPI 3.1.0 (`/api/v1/openapi.json`)  
> **Internationalization (i18n):** Four-language support: English (`en`), Welsh (`cy`), Scots (`sco`), and Scottish Gaelic (`gd`) across UI, statutory narratives, relief names, error payloads, and reference codes  
> **Security Baseline:** OWASP-compliant HTTP security headers with strict Content Security Policy  
> **Coverage Period:** 1 April 1990 – 31 March 2027 (England, Wales, and Scotland)  

---

## 1. Statutory Source Index & Legislative References

The calculation engine's business logic, parameter tables, and statutory audit narrative rules are grounded in primary legislation, secondary statutory instruments (SIs, SSIs, WSIs), and guidance manuals across England, Wales, and Scotland:

### 1.1 Primary Legislation & Core Rating Acts
1. [**Local Government Finance Act 1988 (c. 41)**](https://www.legislation.gov.uk/ukpga/1988/41/contents) — Foundation of national non-domestic rating across England and Wales; sections 43 (occupied liability), 44A (partly occupied apportionments), 45 (unoccupied liability), 47 (discretionary relief), 57A (transitional relief power), and 64 (hereditament definition).
2. [**Non-Domestic Rating Act 2023 (c. 53)**](https://www.legislation.gov.uk/ukpga/2023/53/contents/enacted) — English 3-year revaluations, SBRR statutory amendments, Improvement Relief (s. 1 / Schedule 1 G-factor certificates), Heat Network exemptions (s. 2), high-value rating multipliers ($\ge £500,000$ RV), and empty rate reset rules (s. 15: 13-week occupation requirement).
3. [**Local Government Finance (Wales) Act 2024 (asc 6)**](https://www.legislation.gov.uk/asc/2024/6/contents/enacted) — Welsh 3-year revaluation synchronization, Welsh differential multipliers ($M \times N$), bilingual statutory audit trails, and anti-avoidance reforms.
4. [**Non-Domestic Rates (Scotland) Act 2020 (asp 4)**](https://www.legislation.gov.uk/asp/2020/4/contents) — Scottish 3-year revaluations, Valuation Roll transfer rules, and statutory three-tier multiplier system (Basic, Intermediate, Higher Property Rates).
5. [**Local Government (Scotland) Act 1975 (c. 30)**](https://www.legislation.gov.uk/ukpga/1975/30/contents) — Foundational Scottish rating statute; sections 7, 7A & 7B (rate-setting and differential multipliers), section 24 (unoccupied property rating), and section 24A (partly occupied lands and heritages apportionment certificates).
6. [**Local Government (Financial Provisions) (Scotland) Act 1962 (c. 9, s. 4)**](https://www.legislation.gov.uk/ukpga/1962/9/section/4) — Mandatory 80% charitable relief and Scottish local authority discretionary rating relief powers.
7. [**Local Government Act 2003 (c. 26)**](https://www.legislation.gov.uk/ukpga/2003/26/contents) — Section 61 inserted Section 43(4A) and Schedule 4ZA into LGFA 1988 creating statutory Small Business Rate Relief (SBRR); sections 63 (rural settlement relief) and 64 (discretionary rate relief).
8. [**Rating (Empty Properties) Act 2007 (c. 9)**](https://www.legislation.gov.uk/ukpga/2007/9/contents) — Reformed unoccupied property rating in England and Wales by amending LGFA 1988 s.45, establishing 100% empty liability after initial rate-free periods.
9. [**Localism Act 2011 (c. 20, s. 69)**](https://www.legislation.gov.uk/ukpga/2011/20/section/69) — Granted local authorities general power under LGFA 1988 s.47 to award local discretionary business rate discounts for any purpose (statutory mechanism for RHL and SSBR schemes).
10. [**Islands (Scotland) Act 2018 (asp 12)**](https://www.legislation.gov.uk/asp/2018/12/contents) — Statutory island definition conferring 100% Retail, Hospitality & Leisure relief for island commercial properties up to £110,000 statutory cash cap.
11. [**Rating (Coronavirus and Public Lavatories) Act 2021 (c. 12)**](https://www.legislation.gov.uk/ukpga/2021/12/contents) — 100% statutory rating exemption for standalone public conveniences in England and Wales.
12. [**Local Government and Rating Act 1997 (c. 29)**](https://www.legislation.gov.uk/ukpga/1997/29/contents) — Rural commercial settlement relief framework under LGFA 1988 s.43(6A)–(6E).
13. [**Telecommunications Infrastructure (Relief from Non-Domestic Rates) Act 2018 (c. 1)**](https://www.legislation.gov.uk/ukpga/2018/1/contents) — 100% mandatory non-domestic rate relief for new fibre telecommunications infrastructure in England under LGFA 1988 s.43(4E) and Scotland under S.S.I. 2019/111; Wales is explicitly excluded under LGFA 1988 s.43(4E).
14. [**Civic Government (Scotland) Act 1982 (Licensing of Short-term Lets) Order 2022 (S.S.I. 2022/86)**](https://www.legislation.gov.uk/ssi/2022/86/contents/made) — Statutory licensing scheme for short-term lets under Civic Government (Scotland) Act 1982 s.44; short-term let licence is mandatory for self-catering holiday accommodation to qualify for Scottish RHL relief (S.S.I. 2026/69 reg 4(2)(b)) and the 15% transitional relief cap (S.S.I. 2026/135 reg 4).
### 1.2 Statutory Instruments — Transitional Relief & Phasing Schemes
- **Scotland**:
  - [**The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2026 (S.S.I. 2026/70)**](https://www.legislation.gov.uk/ssi/2026/70/contents/made):
    - [**Regulation 9**](https://www.legislation.gov.uk/ssi/2026/70/regulation/9/made): 2026 Revaluation List General Transitional Relief capping gross liability increases for Small ($\text{RV} \le £20\text{k}$: +15%), Medium ($£20\text{k} < \text{RV} \le £100\text{k}$: +30%), and Large ($\text{RV} > £100\text{k}$: +50%) properties.
    - [**Regulation 14**](https://www.legislation.gov.uk/ssi/2026/70/regulation/14/made): Part 3 Small Business Bonus Scheme Transitional Relief (SBBS TR) limiting ratepayer liability on net bill escalation to 25% in 2026/27 (relief deducts 75% of increase), stepping up to 50% in 2027/28 and 75% in 2028/29.
  - [**The Non-Domestic Rates (Transitional Relief) (Scotland) Amendment Regulations 2026 (S.S.I. 2026/135)**](https://www.legislation.gov.uk/ssi/2026/135/contents/made):
    - [**Regulation 4**](https://www.legislation.gov.uk/ssi/2026/135/regulation/4/made): Inserts Regulation 9A establishing a 15% transitional cap for licensed self-catering holiday accommodation properties regardless of rateable value.
  - [**The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2023 (S.S.I. 2023/31)**](https://www.legislation.gov.uk/ssi/2023/31/contents/made):
    - [**Part 2 (Regs 4–9)**](https://www.legislation.gov.uk/ssi/2023/31/part/2/made): Scottish General Revaluation Transitional Relief capping gross liability for Small ($\le £20\text{k}$), Medium ($\le £100\text{k}$), and Large ($> £100\text{k}$) properties across 2023/24 (+12.5%/+25%/+37.5%), 2024/25 (+25%/+50%/+75%), and 2025/26 (+37.5%/+75%/+112.5%).
    - [**Part 3 (Regs 10–14)**](https://www.legislation.gov.uk/ssi/2023/31/part/3/made): Small Business Bonus Scheme Transitional Relief (SBBS TR) capping net bill increases over the 31 March 2023 baseline bill to £600 in 2023/24, £1,200 in 2024/25, and £1,800 in 2025/26.
    - [**Part 4 (Regs 15–18)**](https://www.legislation.gov.uk/ssi/2023/31/part/4/made): Parks Transitional Relief.
  - [**The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2017 (S.S.I. 2017/85)**](https://www.legislation.gov.uk/ssi/2017/85/contents/made) — 2017 Revaluation List transitional relief capping hospitality and renewable energy hereditaments in Scotland.
- **England**:
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2026**](https://www.legislation.gov.uk/uksi) — 2026 Revaluation List upward transitional phasing caps: Small (+5%), Medium (+15%), Large (+30%) under LGFA 1988 s.57A / NDRA 2023.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2022 (S.I. 2022/1403)**](https://www.legislation.gov.uk/uksi/2022/1403/contents/made) (revoking S.I. 2022/1263) — 2023 Revaluation List upward phasing caps: Small (5%/15%/25%), Medium (15%/25%/40%), and Large (30%/40%/55%); regulation 10(5) defines Small hereditaments as $\text{RV} \le £28,000$ in Greater London and $\le £20,000$ outside Greater London.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2016 (S.I. 2016/1265)**](https://www.legislation.gov.uk/uksi/2016/1265/contents/made) — 2017 Revaluation List upward and downward transitional phasing percentages.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2009 (S.I. 2009/3178)**](https://www.legislation.gov.uk/uksi/2009/3178/contents/made) — 2010 Revaluation List transitional phasing caps.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2004 (S.I. 2004/3312)**](https://www.legislation.gov.uk/uksi/2004/3312/contents/made) — 2005 Revaluation List transitional phasing caps.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 1999 (S.I. 1999/3379)**](https://www.legislation.gov.uk/uksi/1999/3379/contents/made) — 2000 Revaluation List transitional phasing caps.
  - [**The Non-Domestic Rating (Chargeable Amounts) Regulations 1994 (S.I. 1994/3279)**](https://www.legislation.gov.uk/uksi/1994/3279/contents/made) — 1995 Revaluation List transitional phasing caps.
  - [**The Non-Domestic Rating (Transitional Period) Regulations 1990 (S.I. 1990/608)**](https://www.legislation.gov.uk/uksi/1990/608/contents/made) & [**The Non-Domestic Rating (Transitional Period) (Amendment) Regulations 1993 (S.I. 1993/616)**](https://www.legislation.gov.uk/uksi/1993/616/contents/made) — Inaugural national non-domestic rating transitional phasing caps.
- **Wales**:
  - [**The Non-Domestic Rating (Chargeable Amounts) (Wales) Regulations 2025 (W.S.I. 2025/1371 / W. 226)**](https://www.legislation.gov.uk/wsi/2025/1371/contents/made) — 2026 Revaluation List 2-Year Transitional Relief Scheme: applies to net bill increases exceeding £300 over the 31 March 2026 baseline bill; grants 67% relief on the excess in Year 1 (2026/27, ratepayer pays 33%), and 34% relief on the excess in Year 2 (2027/28, ratepayer pays 66%). Reg 6(b) restricts relief strictly to hereditaments continuously occupied by the same ratepayer since list date; partly occupied properties (s.44A) are excluded under Reg 6(c).
  - [**The Non-Domestic Rating (Chargeable Amounts) (Wales) Regulations 2022 (W.S.I. 2022/1350 / W. 272)**](https://www.legislation.gov.uk/wsi/2022/1350/contents/made) — 2023 Revaluation List 2-Year Transitional Relief Scheme: 67% relief in 2023/24 and 34% relief in 2024/25 on net bill increases exceeding £300, subject to continuous occupation by the same ratepayer (Reg 9(b)).
  - [**The Non-Domestic Rating (Chargeable Amounts) (Wales) Regulations 2016 (W.S.I. 2016/1247 / W. 295)**](https://www.legislation.gov.uk/wsi/2016/1247/contents/made) — 2017 Revaluation List Welsh Small Business Transitional Relief Scheme (75% in 2017/18, 50% in 2018/19, 25% in 2019/20 on increases over £100 for ratepayers in receipt of SBRR on 31 March 2017).
  - *Historical Revaluations (2005 & 2010)*: Wales did not enact transitional relief schemes for the 2005 or 2010 Revaluation Lists.
### 1.3 Statutory Instruments — Small Business Rate Relief (SBRR & SBBS) & Multiplier Frameworks
- **Scotland (SBBS & Property Rates)**:
  - [**The Non-Domestic Rates (Levying and Setting of Rates) (Scotland) Regulations 2026 (S.S.I. 2026/39)**](https://www.legislation.gov.uk/ssi/2026/39/contents/made) & [**The Non-Domestic Rates (Levying and Setting of Rates and Relief) (Scotland) Amendment Regulations 2026 (S.S.I. 2026/68)**](https://www.legislation.gov.uk/ssi/2026/68/contents/made):
    - Regulations 5–6: Setting 2026/27 Scottish property rates: Basic Property Rate `0.481` ($\text{RV} \le £51,000$), Intermediate Property Rate `0.535` ($£51,000 < \text{RV} \le £100,000$), and Higher Property Rate `0.548` ($\text{RV} > £100,000$).
    - Regulation 3(3): SBBS relief rates: 100% relief for $\text{RV} \le £12,000$; linear taper from 100% to 25% for $£12,000 < \text{RV} \le £15,000$; linear taper from 25% to 0% for $£15,000 < \text{RV} \le £20,000$.
    - Regulation 4: SBBS Multiple-Property Rules: Where a ratepayer occupies more than one non-domestic property in Scotland, 100% relief on an individual property $\le £12,000$ is granted only if the cumulative rateable value across all occupied properties in Scotland does not exceed £12,000. If cumulative RV is between £12,000 and £35,000, 25% relief is granted for any property with $\text{RV} \le £15,000$. If cumulative RV exceeds £35,000, 0% relief is granted across all properties (Reg 4(3)).
    - Regulation 3(5): Statutory exclusions from SBBS (advertising rights, betting shops, etc.).
  - [**The Non-Domestic Rates (Small Business Bonus Scheme) (Scotland) Regulations 2023 (S.S.I. 2023/34)**](https://www.legislation.gov.uk/ssi/2023/34/contents/made) — 2023 List: 100% relief for $\text{RV} \le £12,000$; tapered relief for $£12,001 \le \text{RV} \le £20,000$.
  - [**The Non-Domestic Rates (Small Business Bonus Scheme) (Scotland) Regulations 2017 (S.S.I. 2017/36)**](https://www.legislation.gov.uk/ssi/2017/36/contents/made) — 2017 List: 100% relief for $\text{RV} \le £15,000$; 25% for $£15,001 \le \text{RV} \le £18,000$.
  - [**The Non-Domestic Rates (Small Business Bonus Scheme) (Scotland) Regulations 2010 (S.S.I. 2010/37)**](https://www.legislation.gov.uk/ssi/2010/37/contents/made) — 2010 List 3-tier SBBS framework.
  - [**The Non-Domestic Rates (Small Business Bonus Scheme) (Scotland) Regulations 2008 (S.S.I. 2008/106)**](https://www.legislation.gov.uk/ssi/2008/106/contents/made) — Inaugural SBBS regulations.
  - [**The Non-Domestic Rates (Levying and Setting of Rates) (Scotland) Regulations 2025 (S.S.I. 2025/37)**](https://www.legislation.gov.uk/ssi/2025/37/contents/made) & [**S.S.I. 2025/38 regs 5(4) & 6(4)**](https://www.legislation.gov.uk/ssi/2025/38/contents/made) — Statutory setting of 2025/26 Basic (49.8p), Intermediate (55.4p), and Higher (56.8p) property rates.
- **England (SBRR, 2026/27 5-Multiplier Regime & SSBR)**:
  - [**The Non-Domestic Rating (Small Business Rate Relief) (England) Order 2017 (S.I. 2017/411)**](https://www.legislation.gov.uk/uksi/2017/411/contents/made), [**S.I. 2017/102 art. 3**](https://www.legislation.gov.uk/uksi/2017/102/article/3/made), and [**S.I. 2023/1250 reg. 3**](https://www.legislation.gov.uk/uksi/2023/1250/regulation/3/made) — 100% relief for $\text{RV} \le £12,000$; for $£12,001 \le \text{RV} \le £15,000$, chargeable amount is $(A \times M) / E$ where $E = 3000 / (A - 12000)$ rounded to 3 decimal places (adding 0.001 only where remainder > 0.0005).
  - [**S.I. 2017/411 art. 3(4)**](https://www.legislation.gov.uk/uksi/2017/411/article/3/made) & [**S.I. 2014/43**](https://www.legislation.gov.uk/uksi/2014/43/contents/made) under LGFA 1988 s.43(4B) — Statutory 12-month grace period allowing a ratepayer who takes on an additional hereditament to retain full SBRR on their primary hereditament for 12 months.
  - [**Autumn Budget 2024 / Non-Domestic Rating Act 2023 Multiplier Framework**](https://www.gov.uk/government/publications/autumn-budget-2024) — Permanent 5-multiplier rating system effective 1 April 2026:
    - Small Business RHL: `38.2p` ($\text{RV} < £51,000$ for qualifying retail, hospitality, leisure)
    - Small Business non-RHL: `43.2p` ($\text{RV} < £51,000$ standard small business rate)
    - Standard RHL: `43.0p` ($£51,000 \le \text{RV} < £500,000$ qualifying retail, hospitality, leisure)
    - Standard non-RHL: `48.0p` ($£51,000 \le \text{RV} < £500,000$ standard national rate)
    - High-Value: `50.8p` ($\text{RV} \ge £500,000$ premium rate)
  - [**The Non-Domestic Rating (Multipliers and Rateable Value Thresholds) (England) Regulations 2026 (S.I. 2026/152)**](https://www.legislation.gov.uk/uksi/2026/152/contents/made) regs 3–5: Small business multipliers apply where RV is less than £51,000; standard multipliers apply where RV is £51,000 or more.
  - [**The Non-Domestic Rating (Small Business Rate Relief) (England) Order 2017 (S.I. 2017/102)**](https://www.legislation.gov.uk/uksi/2017/102/contents/made) art. 2: condition for small business multiplier is that RV is not more than £50,999.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations / S.I. 2024/1321**](https://www.legislation.gov.uk/uksi/2024/1321/contents/made) — Sets item C for 2025-26 under LGFA 1988 Sched 7 para A6 freezing the Small Business Multiplier at 49.9p; Standard Multiplier is uprated by CPI (1.7%) to 55.5p (0.555).
  - [**Supporting Small Business Relief Scheme (SSBR) under LGFA 1988 s.47**](https://www.gov.uk/government/publications/business-rates-supporting-small-business-relief-guidance) — Statutory discretionary relief capping annual bill increases to £600/year (2023/24–2025/26) and £800/year (2026/27) for ratepayers losing or experiencing reduced SBRR, Rural Rate Relief, or 2022/23 RHL relief due to revaluation. Compounding into subsequent years; requires valid prior-year baseline; charities are excluded.
  - [**The Non-Domestic Rating (Small Business Rate Relief) (England) Order 2012 (S.I. 2012/148)**](https://www.legislation.gov.uk/uksi/2012/148/contents/made) & [**S.I. 2010/1959**](https://www.legislation.gov.uk/uksi/2010/1959/contents/made) — 2010 list SBRR doubled taper; article 3(1)(b) establishes small business multiplier qualification thresholds: $\text{RV} \le £17,999$ outside Greater London and $\text{RV} \le £25,499$ in Greater London.
  - [**The Non-Domestic Rating (Small Business Rate Relief) (England) Order 2004 (S.I. 2004/3315)**](https://www.legislation.gov.uk/uksi/2004/3315/contents/made) — Inaugural SBRR order establishing 50% max relief and linear taper; article 3(1)(b) establishes small business multiplier qualification thresholds: $\text{RV} \le £14,999$ outside Greater London and $\text{RV} \le £21,499$ in Greater London.
- **Wales (SBRR, Multipliers & Devolved Schemes)**:
  - [**The Non-Domestic Rating (Multiplier) (Wales) Order 2025 (W.S.I. 2025/214 / W. 42)**](https://www.legislation.gov.uk/wsi/2025/214/contents/made) — Sets Welsh national rating multiplier for 2025/26 to **0.568**.
  - [**Local Government Finance (Wales) Act 2024 (asc 6)**](https://www.legislation.gov.uk/asc/2024/6/contents/enacted) — 2026/27 3-Tier Multiplier Framework:
    - **Retail Multiplier**: `35.0p` (`0.350`) for retail premises with $\text{RV} < £51,000$ (strictly for retail shops; holiday lets and pubs excluded).
    - **Standard Multiplier**: `50.2p` (`0.502`) for commercial premises with $\text{RV} \le £100,000$ (non-retail or general commercial).
    - **Higher Multiplier**: `51.5p` (`0.515`) for high-value properties with $\text{RV} > £100,000$.
    - **Higher Multiplier Statutory Exception**: Qualifying high-value properties receive statutory exception paying the standard multiplier (`0.502`).
  - [**The Non-Domestic Rating (Small Business Relief) (Wales) Order 2017 (W.S.I. 2017/1229 / W. 293)**](https://www.legislation.gov.uk/wsi/2017/1229/contents/made):
    - General taper: 100% relief for $\text{RV} \le £6,000$; for $£6,001 \le \text{RV} \le £12,000$, chargeable amount is $(A \times M) / E$ where $E = 6000 / (\text{RV} - 6000)$ rounded to 3 decimal places under Article 11(3).
    - [**Article 4**](https://www.legislation.gov.uk/wsi/2017/1229/article/4/made): Post offices receive 100% relief for $\text{RV} \le £9,000$ and 50% relief for $£9,001 \le \text{RV} \le £12,000$.
    - [**Article 5**](https://www.legislation.gov.uk/wsi/2017/1229/article/5/made): Registered childcare premises (Care Inspectorate Wales) receive 100% relief up to £100,000 RV.
    - [**Article 12**](https://www.legislation.gov.uk/wsi/2017/1229/article/12/made): Multiple properties limit — ratepayer can claim SBRR on at most **two** properties per billing authority.
    - [**Article 3**](https://www.legislation.gov.uk/wsi/2017/1229/article/3/made): Excepted hereditaments (betting shops, payday lenders, partially occupied hereditaments) disqualified from SBRR.
  - [**Welsh Food and Drink Hospitality Rates Relief Scheme (2026/27)**](https://www.gov.wales/non-domestic-rates-relief) under LGFA 1988 s.47 — Successor to Welsh RHL relief providing a **15% discount** capped at £110,000 per business; targeted specifically at qualifying food and drink hospitality premises (pubs, restaurants, cafés, bars, live music venues); excludes petrol filling stations (SCAT 211), holiday accommodation, and properties on the retail multiplier.
  - [**The Non-Domestic Rating (Multiplier) (Wales) Order 2024 (W.S.I. 2024/110 / W. 26)**](https://www.legislation.gov.uk/wsi/2024/110/contents/made) — Frozen Welsh uniform rating multiplier at 0.562 for 2024/25.
  - [**The Non-Domestic Rating (Small Business Relief) (Wales) Order 2015 (W.S.I. 2015/976 / W. 69)**](https://www.legislation.gov.uk/wsi/2015/976/contents/made), [**W.S.I. 2010/248 (W. 34)**](https://www.legislation.gov.uk/wsi/2010/248/contents/made), and [**W.S.I. 2008/707 (W. 75)**](https://www.legislation.gov.uk/wsi/2008/707/contents/made).
  - [**The Non-Domestic Rating (Small Business Relief) (Wales) Order 2005 (W.S.I. 2005/309 / W. 29)**](https://www.legislation.gov.uk/wsi/2005/309/contents/made) — Inaugural Welsh SBRR order establishing 50% relief up to £2,000 RV and 25% for £2,001–£5,000 RV.
### 1.4 Statutory Instruments — Relief Exemptions, Unoccupied Rates, Rural Settlements & Compliance
- **Empty Property & Unoccupied Rating**:
  - [**The Non-Domestic Rating (Unoccupied Property) (England) Regulations 2008 (S.I. 2008/386)**](https://www.legislation.gov.uk/uksi/2008/386/contents/made) — 3-month commercial / 6-month industrial rate-free periods; low value exemption below £2,200 RV (2008/09), amended by [**S.I. 2008/3283**](https://www.legislation.gov.uk/uksi/2008/3283/contents/made) (£15,000 for 2009/10), [**S.I. 2010/408**](https://www.legislation.gov.uk/uksi/2010/408/contents/made) (£18,000 for 2010/11; £2,600 for 2011/12–2016/17), and [**S.I. 2017/102 art. 4**](https://www.legislation.gov.uk/uksi/2017/102/contents/made) (£2,900 from 2017/18 onwards).
  - [**The Non-Domestic Rating (Unoccupied Property) (Wales) Regulations 2008 (W.S.I. 2008/2499 / W. 217)**](https://www.legislation.gov.uk/wsi/2008/2499/contents/made) — Welsh unoccupied property exemptions and reset rules; low value exemption threshold is **£2,200** RV (Reg 4(g)).
  - [**The Non-Domestic Rates (Unoccupied Property) (Scotland) Regulations 2018 (S.S.I. 2018/77)**](https://www.legislation.gov.uk/ssi/2018/77/contents/made) — Scottish unoccupied property relief rules (2018/19–2022/23): other unoccupied property pays 50% for first 3 months, then 90% (10% relief); industrial property is 100% exempt for first 6 months, then 90% (10% relief) under reg 2; £1,700 low RV threshold.
  - [**The Non-Domestic Rates (Transitional and Consequential Provisions) (Scotland) Regulations 2023 (S.S.I. 2023/44)**](https://www.legislation.gov.uk/ssi/2023/44/contents/made) & [**Non-Domestic Rates (Coronavirus) (Scotland) Act 2022 (asp 10)**](https://www.legislation.gov.uk/asp/2022/10/contents) — Devolving empty property rate relief discretion to Scottish local authorities from 1 April 2023.
- **Retail, Hospitality & Leisure (Scotland 2026/27)**:
  - [**The Non-Domestic Rates (Relief for Retail, Hospitality and Leisure) (Scotland) Regulations 2026 (S.S.I. 2026/69)**](https://www.legislation.gov.uk/ssi/2026/69/contents/made) as amended by [**S.S.I. 2026/135**](https://www.legislation.gov.uk/ssi/2026/135/contents/made):
    - Regulation 4: 15% Retail & Leisure relief on Scottish mainland for properties with $\text{RV} \le £100,000$.
    - Regulation 4A: 40% Hospitality relief tier (Schedule 1A: hotels, pubs, restaurants, cafés, live music venues) up to £110k cap per ratepayer.
    - Regulation 5: 100% relief for island properties and designated remote mainland locations (Cape Wrath, Knoydart, Scoraig).
    - Regulations 4(2)(b) & 5(1)(c): Short-term let licence required for self-catering holiday accommodation to qualify.
    - Regulations 4(3) & 4A(3): RHL relief applied against gross liability before other reliefs.
- **Improvement Relief (G-Factor Certificates) & Heat Network Relief**:
  - [**The Non-Domestic Rating (Improvement Relief) (England) Regulations 2024 (S.I. 2024/22)**](https://www.legislation.gov.uk/uksi/2024/22/contents/made) & [**W.S.I. 2024/302**](https://www.legislation.gov.uk/wsi/2024/302/contents/made) (Wales) — 12-month qualifying improvement relief under NDRA 2023 s. 1; applies from 1 April 2024 to 31 March 2029 (5-year scheme; not enacted in Scotland).
  - [**Non-Domestic Rating Act 2023 s. 2**](https://www.legislation.gov.uk/ukpga/2023/53/section/2/enacted) — 100% Heat Network Relief in England and Wales from 1 April 2024 to 31 March 2035; not enacted in Scotland.
- **Rural Settlement Relief Orders**:
  - [**The Non-Domestic Rating (Rural Settlements) (England) Order 2020 (S.I. 2020/1401)**](https://www.legislation.gov.uk/uksi/2020/1401/contents/made) — Statutory designations of rural settlements under LGFA 1988 s.42A.
  - [**The Non-Domestic Rating (Rural Settlements) (Wales) Order 2019 (W.S.I. 2019/1487 / W. 268)**](https://www.legislation.gov.uk/wsi/2019/1487/contents/made) — Statutory designation of rural settlements in Wales.
  - [**The Non-Domestic Rates (Rural Areas) (Scotland) Regulations 2009 (S.S.I. 2009/64)**](https://www.legislation.gov.uk/ssi/2009/64/contents/made) — 100% mandatory relief for qualifying rural shops, post offices, food stores, and petrol stations under £8,500 / £12,500 NAV.
- **Part-Occupied Hereditaments (Apportionment)**:
  - [**Local Government Finance Act 1988 s. 44A**](https://www.legislation.gov.uk/ukpga/1988/41/section/44A) & [**Local Government (Scotland) Act 1975 s. 24A**](https://www.legislation.gov.uk/ukpga/1975/30/section/24A) — Statutory apportionment certificates for partly occupied commercial premises in England/Wales and Scotland.
- **Renewable Energy & Telecommunications Infrastructure**:
  - [**The Non-Domestic Rates (Renewable Energy Generation Relief) (Scotland) Regulations 2010 (S.S.I. 2010/336)**](https://www.legislation.gov.uk/ssi/2010/336/contents/made) — Commercial rates relief for renewable electricity and community energy installations.
  - [**The Non-Domestic Rating (Telecommunications Infrastructure Relief) (England) Regulations 2018 (S.I. 2018/1199)**](https://www.legislation.gov.uk/uksi/2018/1199/contents/made) & [**S.S.I. 2019/111**](https://www.legislation.gov.uk/ssi/2019/111/contents/made) (Scotland) — 100% rates relief for new fibre telecommunications networks.
- **Billing, Appeals, Holiday Lets & Anti-Avoidance**:
  - [**The Non-Domestic Rating (Collection and Enforcement) (Local Lists) Regulations 1989 (S.I. 1989/1058)**](https://www.legislation.gov.uk/uksi/1989/1058/contents/made) & [**Schedule 1**](https://www.legislation.gov.uk/uksi/1989/1058/schedule/1/made) — Statutory 10-installment billing payment schedules, adjustment rules, and mid-year cessation refunds.
  - [**The Council Tax and Non-Domestic Rating (Demand Notices) (England) Regulations 2003 (S.I. 2003/2613)**](https://www.legislation.gov.uk/uksi/2003/2613/contents/made), [**The Non-Domestic Rating (Demand Notes) (Wales) Regulations 2017 (W.S.I. 2017/113 / W. 38)**](https://www.legislation.gov.uk/wsi/2017/113/contents/made), and [**The Non-Domestic Rating (Demand Notes) (Scotland) Regulations 2008 (S.S.I. 2008/28)**](https://www.legislation.gov.uk/ssi/2008/28/contents/made) — Statutory content of non-domestic rates demand notices across the three jurisdictions.
  - [**The Non-Domestic Rating (Alteration of Lists and Appeals) (Wales) Regulations 2023 (W.S.I. 2023/1162)**](https://www.legislation.gov.uk/wsi/2023/1162/contents/made) — Welsh Check, Challenge, Appeal (CCA) procedures and Valuation Tribunal for Wales appeals.
  - [**The Non-Domestic Rating (Artificial Avoidance Arrangements) (Local Lists) (Wales) Regulations 2026 (W.S.I. 2026/43)**](https://www.legislation.gov.uk/wsi/2026/43/contents/made) — Welsh artificial avoidance countermeasures; WSI 2026/43 civil financial penalties (£500 flat + 3% RV).
  - [**The Council Tax (Dwellings and Part Residential Subjects) (Scotland) Amendment Regulations 2021 (S.S.I. 2021/489)**](https://www.legislation.gov.uk/ssi/2021/489/contents/made) — Scottish self-catering statutory criteria under Schedule 2 paragraph 2 of S.I. 1992/2955 (as amended from 1 April 2022): commercially available $\ge 140$ days and actually let $\ge 70$ days in the financial year.
  - [**The Non-Domestic Rating (Definition of Domestic Property) (England) Order 2022 (S.I. 2022/217)**](https://www.legislation.gov.uk/uksi/2022/217/contents/made) — Amends LGFA 1988 s.66(2B) from 1 April 2023 to introduce the 70-day actual letting test in England (prior to which only 140 days available to let applied).
  - [**The Non-Domestic Rating (Definition of Domestic Property) (Wales) Order 2010 (W.S.I. 2010/682 / W. 67)**](https://www.legislation.gov.uk/wsi/2010/682/contents/made) — Introduces the 70-day actual letting test in Wales from 1 April 2010.
  - [**The Non-Domestic Rating (Amendment of Definition of Hereditament) (Wales) Order 2022 (W.S.I. 2022/563 / W. 130)**](https://www.legislation.gov.uk/wsi/2022/563/contents/made) — Welsh self-catering letting test post-1 April 2023: commercially available $\ge 252$ days and actually let $\ge 182$ days.

### 1.5 Valuation Office Agency (VOA) Manuals & Policy Guidance
1. [**Common Legislative Solutions: A Guide to Tackling Recurring Policy Issues**](https://www.gov.uk/government/publications/common-legislative-solutions) (Guidance) — Statutory drafting guidelines, licensing frameworks, and administrative powers.
2. [**Rating Manual Section 5a: Airports Guidance**](https://www.gov.uk/guidance/rating-manual-section-5a-valuation-of-all-property-classes/airports) (VOA Manual) — SCAT 003 Airport valuation rules; Receipts & Expenditure for major hubs vs Contractor's Basis for airfields.
3. [**Rating Manual Section 2: Domestic & Non-Domestic Borderline**](https://www.gov.uk/guidance/rating-manual-section-2-valuation-principles/domestic-and-non-domestic-borderline) (VOA Manual) — Composite hereditament rules, private garages, and holiday accommodation classification criteria.
4. [**Rating Manual Section 5a: Petrol Filling Stations Guidance**](https://www.gov.uk/guidance/rating-manual-section-5a-valuation-of-all-property-classes/petrol-filling-stations) (VOA Manual) — SCAT 211 valuation methods; fuel throughput, forecourt shop turnover, valeting, and Fair Maintainable Trade.
5. [**Rating Manual Section 4: 2023 Rental Adjustment Practice Note**](https://www.gov.uk/guidance/rating-manual-section-4-valuation-principles/2023-rental-adjustment-practice-note) (VOA Manual) — Virtual rent conversions, lease incentives, VAT adjustments, rent-free period capitalisation, and yields.

---

## 2. Software Requirements Specification (SRS)

### 2.1 Architecture Overview

The system comprises three primary components:
- **Component UI (`frontend/`)**: A Svelte 5 Single-Page Application (SPA) compiled with Vite into static assets served by the backend at `/`.
- **API Documentation (`/docs`, `/api/v1/openapi.json`)**: Self-hosted Scalar API reference rendered via `templates/scalar.html` with query parameter templating and OpenAPI 3.1.0 specifications.
- **Calculation API (REST)**: Axum endpoints executing rating calculations, cataloging reference data, and serving parameter repository tables.

```
┌────────────────────────────────────────────────────────────────────────┐
│                   Svelte 5 Component Framework UI                      │
│                                                                        │
│   TopNavbar (Brand, Language [EN|CY|SCO|GD], API Reference Link)       │
│   StatutoryCoverageBanner (Legislation Badges: NDRA, LGFWA, NDR, WSI)  │
│   JurisdictionSelector (England, Wales, Scotland Statutory Rules)      │
│   OccupancyEpisodesTable (Multi-Occupancy & Section 44A Apportionment) │
│   ValuationErasTable (Rateable Value History Eras)                     │
│   ReliefEligibilityMatrix (Devolved Relief Toggles & Inputs)           │
│   AnnualBillingSegmentsTable & StatutoryAuditNarrative                 │
│   RawJsonEditor (Bi-directional Form ↔ JSON Live Sync)                 │
│   ParameterCatalogs (Multipliers, Thresholds, Reliefs, Caps)           │
│   ReferenceCodeCatalog (Statutory Dictionary & Category Browser)       │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ HTTP / JSON
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     Axum 0.7 REST API Server                           │
│                                                                        │
│   GET  /                                -> Svelte 5 SPA index.html     │
│   GET  /docs                            -> Scalar API Reference (HTML) │
│   GET  /licenses                        -> Third-Party Licenses (HTML) │
│   GET  /healthz                         -> Service Health Check (JSON) │
│   GET  /assets/:filename                -> Bundled Assets / scalar.js  │
│   POST /api/v1/calculate-liability      -> Statutory Calculation API   │
│   GET  /api/v1/reference-codes          -> Statutory Reference Codes   │
│   GET  /api/v1/parameters               -> Aggregated Parameter Repo   │
│   GET  /api/v1/parameters/multipliers   -> Rating Multipliers (90-27)  │
│   GET  /api/v1/parameters/sbrr-thresholds                              │
│   GET  /api/v1/parameters/sector-reliefs                               │
│   GET  /api/v1/parameters/transitional-caps                            │
│   GET  /api/v1/parameters/empty-property                               │
│   GET  /api/v1/parameters/letting-tests                                │
│   GET  /api/v1/parameters/scheme-caps                                  │
│   GET  /api/v1/licenses                 -> Licenses Markdown / JSON    │
│   GET  /api/v1/healthz                  -> Service Health Check (JSON) │
│   GET  /api/v1/openapi.json             -> OpenAPI 3.1.0 Spec (JSON)   │
│   GET  /api/v1/openapi.yaml             -> OpenAPI 3.1.0 Spec (YAML)   │
│                                                                        │
│   OWASP Security Headers Middleware (Content Security Policy)          │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│              Statutory Calculation Engine & Built-In Repo              │
│                                                                        │
│   - CalculationEngine (Statutory Slicer, Multi-Occupancy, Narratives)  │
│   - ParameterRepository (Built-in In-Memory Multipliers 1990–2027,     │
│     SBRR/SBBS Tapers, Devolved Sector Reliefs, Caps & Reference Codes) │
└────────────────────────────────────────────────────────────────────────┘
```

#### Key Technical Decisions & Choices:
1. **Frontend Architecture**:
   - Built with Svelte 5 runes (`$state`, `$derived`) and TypeScript.
   - Compiled via `./build-frontend.sh` into `frontend/dist/`.
2. **REST API Access**:
   - Endpoints are accessible as a public service without authentication requirements.
3. **Four-Language Internationalization (i18n)**:
   - Supports **English (`en`)**, **Welsh (`cy`)**, **Scots (`sco`)**, and **Scottish Gaelic (`gd`)**.
   - Language is negotiated dynamically: Request body `"language"` > Query parameter `?lang=` > `Accept-Language` header > Default `en`.
4. **Decoupled Calculation Contract**:
   - Informational metadata fields (`liable_party_id`, `uarn`, street addresses) are decoupled from the statutory calculation core.
5. **Multi-Occupancy Section 44A Support**:
   - Supports multiple occupancy periods (`OCCUPIED`, `UNOCCUPIED`, `PART_OCCUPIED`) in a single calculation request.
   - Handles statutory occupation reset rules ($\ge 91$ days in England post-April 2024; $\ge 42$ days in Wales/Scotland) and Section 44A apportionments.
6. **OpenAPI 3.1.0 & Self-Hosted Scalar Reference**:
   - OpenAPI 3.1.0 specification stored in [`docs/openapi.yaml`](docs/openapi.yaml) and [`docs/openapi.json`](docs/openapi.json), served at `/api/v1/openapi.json` and `/api/v1/openapi.yaml`.
   - Interactive documentation served at `/docs` rendered via [`templates/scalar.html`](templates/scalar.html) using self-hosted `/assets/scalar.js`, supporting query parameter templating (`title`, `spec_url`, `theme`, `layout`).
7. **Unified REST API Routes**:
   - REST endpoints are grouped under `/api/v1/` (`/calculate-liability`, `/reference-codes`, `/parameters/*`, `/openapi.*`).

---

### 2.2 OWASP Secure Headers Specification

All HTTP responses (HTML web pages, API JSON responses, static assets, and error handlers) are protected by centralized security middleware enforcing OWASP recommended headers:

| Header | Production Value | Purpose |
| :--- | :--- | :--- |
| `Content-Security-Policy` | `default-src 'self'; script-src 'self'; style-src 'self'; font-src 'self' data:; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; form-action 'self'; base-uri 'self';` | Restricts script and style origins and prevents framing. |
| `X-Content-Type-Options` | `nosniff` | Prevents MIME-type sniffing. |
| `X-Frame-Options` | `DENY` | Eliminates clickjacking by disallowing framing. |
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Enforces secure transport. |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Protects URI query data on navigation. |
| `Permissions-Policy` | `camera=(), microphone=(), geolocation=(), payment=()` | Restricts privileged browser hardware APIs. |
| `Cross-Origin-Opener-Policy` | `same-origin` | Isolates browsing contexts against Spectre attacks. |
| `Cross-Origin-Resource-Policy` | `same-origin` | Restricts cross-origin resource sharing. |
| `X-Permitted-Cross-Domain-Policies` | `none` | Blocks Flash/PDF policy leaks. |

> **Route-Scoped Style CSP Policy:** The `/docs` route allows `style-src 'self' 'unsafe-inline'` for Scalar theme styling, with `script-src 'self'`. All other routes enforce `style-src 'self'`.

---

### 2.3 Internationalization (i18n) & Language Resolution

The engine implements multilingual support via [`src/i18n.rs`](src/i18n.rs) and the frontend catalog store [`frontend/src/locales/`](frontend/src/locales/):

**Precedence Order for Language Resolution:**
1. Explicit request payload field: `"language": "cy"` / `"sco"` / `"gd"`
2. URL Query parameter: `?lang=cy`
3. HTTP Header: `Accept-Language: cy` / `sco` / `gd`
4. Default fallback: English (`en`)

**Localized Outputs:**
- **Statutory Audit Narrative**: Generated in English, Welsh, Scots, or Gaelic (e.g. Gaelic: *"AITHRIS REACHDAIL AIR ÀIREAMHACHADH ÌREAN & SLIGHE SGRÙDAIDH"*, Scots: *"STATUTARY CALCULATION NARRATIVE & AUDIT TRAIL"*, Welsh: *"DEDDF CYLLID LLYWODRAETH LEOL (CYMRU) 2024"*).
- **Itemized Relief Names**: Translated relief labels (e.g. Scots: *"Mandatar Chairitable Rate Relief"*, Gaelic: *"Faochadh Ìrean Carthannais Èigneachail"*, Welsh: *"Rhyddhad Elusennol Gorfodol"*).
- **Reference Codes**: Descriptions and labels localized on demand via repository queries.
- **API Error Messages**: Localized error payloads returning domain error codes and translated messages.

---

### 2.4 Core Calculation Engine Contract (`POST /api/v1/calculate-liability`)

> **Calculation Route:** All liability calculations are handled by `POST /api/v1/calculate-liability`.

#### Multi-Occupancy Request Payload Schema
```json
{
  "party_type": "INDIVIDUAL",
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
        "effective_to": null,
        "rateable_value": 12000.00
      }
    ]
  },
  "occupancy_periods": [
    {
      "start_date": "2024-04-01",
      "end_date": "2024-06-30",
      "occupancy_type": "UNOCCUPIED",
      "qualifying_relief_flags": {
        "days_previously_unoccupied": 0
      }
    },
    {
      "start_date": "2024-07-01",
      "end_date": "2024-12-31",
      "occupancy_type": "OCCUPIED",
      "qualifying_relief_flags": {
        "is_small_business_sole_property": true
      }
    },
    {
      "start_date": "2025-01-01",
      "end_date": "2025-03-31",
      "occupancy_type": "PART_OCCUPIED",
      "qualifying_relief_flags": {
        "is_small_business_sole_property": true
      },
      "part_occupied_apportionment": {
        "occupied_percentage": 50.0
      }
    }
  ],
  "language": "en"
}
```

#### Qualifying Relief Flags Specification (`QualifyingReliefFlags`)

All relief eligibility determinations are driven by strongly typed flags within `qualifying_relief_flags`:

| Field Name | Type | Jurisdiction & Statutory Scope | Description & Policy Rule |
| :--- | :--- | :--- | :--- |
| `is_small_business_sole_property` | `bool` | England, Wales, Scotland | Indicates ratepayer occupies only one non-domestic property in the jurisdiction. Triggers 100% SBRR/SBBS or linear taper. |
| `is_small_business_second_property` | `bool` | Wales | Ratepayer's second qualifying small business property in Wales under W.S.I. 2017/1229 art. 12 (max 2 properties per billing authority). |
| `is_small_business_grace_period` | `bool` | England | 12-month grace period on primary property when acquiring a secondary hereditament under S.I. 2017/411 art. 3(4). |
| `is_small_business_multiple_properties` | `bool` | Scotland | Ratepayer occupies multiple non-domestic properties in Scotland under SBBS reg. 4 multiple-property rules (S.S.I. 2023/34 / S.S.I. 2026/68). |
| `small_business_cumulative_rv` | `f64?` | Scotland | Cumulative rateable value across all occupied hereditaments in Scotland, subject to the £35,000 statutory cap under SBBS reg. 4. |
| `is_registered_childcare` | `bool` | Wales | Registered day childcare premises with CIW qualifying for 100% SBRR up to £100k RV under W.S.I. 2017/1229 art. 5. |
| `is_post_office` | `bool` | Wales | Post office premises qualifying for 100% SBRR up to £9k RV and 50% between £9,001 and £12k RV under W.S.I. 2017/1229 art. 4. |
| `is_charitable_purposes` | `bool` | All Jurisdictions | Mandatory 80% charitable relief under LGFA 1988 s.43(5) (England/Wales) or 1962 Act s.4(2) (Scotland). |
| `is_discretionary_charity_top_up`| `bool` | All Jurisdictions | Optional 20% local authority discretionary top-up under LGFA 1988 s.47(2)(a) / 1962 Act s.4(5). |
| `is_rhl_qualifying` | `bool` | England, Wales, Scotland | Qualifying premises under Retail, Hospitality and Leisure scheme criteria. |
| `is_retail_premises` | `bool` | Scotland, Wales | Qualifying retail shop (qualifies for Scottish 15% RHL or Welsh 35.0p Retail Multiplier under LGF(W)A 2024). |
| `is_hospitality_premises` | `bool` | Scotland | Qualifying hospitality (hotel, pub, restaurant, café, live music venue) under S.S.I. 2026/69 Sched 1A for 40% relief in Scotland. |
| `is_food_and_drink` | `bool` | Wales | Qualifying food & drink hospitality premises under Welsh 2026/27 15% rate relief scheme (LGFA 1988 s.47). |
| `is_higher_multiplier_excepted` | `bool` | Wales | High-value hereditament (>£100k RV) in Wales qualifying for statutory exception paying standard multiplier (0.502). |
| `is_scottish_island` | `bool` | Scotland | Hereditament on a Scottish island designated under Islands (Scotland) Act 2018 (qualifying for 100% island relief). |
| `is_scottish_remote_mainland` | `bool` | Scotland | Situated in Cape Wrath, Knoydart, or Scoraig qualifying for 100% relief under S.S.I. 2026/69 reg. 5. |
| `has_short_term_let_licence` | `bool?` | Scotland | Short-term let licence held under Civic Government (Scotland) Act 1982 / S.S.I. 2022/86; required for Scottish self-catering RHL and 15% transitional cap. |
| `holiday_let_days_let` | `i32?` | All Jurisdictions | Days commercially let in preceding 12 months (minimum 70 days in Scotland from 2022/23 under S.S.I. 2021/489; minimum 70 days in England from 2023/24 under S.I. 2022/217; minimum 70 days in Wales from 2010 under W.S.I. 2010/682 rising to 182 days from 2023/24 under W.S.I. 2022/563). |
| `holiday_let_days_available` | `i32?` | All Jurisdictions | Days commercially available to let in preceding 12 months (minimum 252 days in Wales post-2023 under W.S.I. 2022/563, 140 days elsewhere). |
| `is_rural_settlement` | `bool` | All Jurisdictions | Mandatory 100% rural settlement relief under LGFA 1988 s.43(6A) / S.S.I. 2009/64. |
| `is_public_lavatory` | `bool` | England, Wales | 100% exemption for standalone public conveniences under Rating (Public Lavatories) Act 2021. |
| `is_heat_network` | `bool` | England, Wales | 100% relief under NDRA 2023 s. 2 (effective 1 April 2024 – 31 March 2035; not enacted in Scotland). |
| `is_improvement_qualifying` | `bool` | England, Wales | 12-month G-factor improvement relief under NDRA 2023 s. 1 (effective 1 April 2024 – 31 March 2029; not enacted in Scotland). |
| `improvement_rateable_value_increase` | `f64?`| England, Wales | Rateable value attributable to qualifying improvements certified by the VOA under NDRA 2023 Schedule 1. |
| `is_telecommunications_infrastructure`| `bool` | England, Scotland | 100% relief under Telecommunications Infrastructure Act 2018 (England & Scotland only; excluded in Wales). |
| `is_renewable_energy_project` | `bool` | Scotland | Qualifying renewable electricity installation under S.S.I. 2010/336 tiered relief scheme. |
| `is_artificial_avoidance_suspected`| `bool`| Wales | Triggers Welsh civil financial penalty (£500 + 3% RV) under W.S.I. 2026/43; assessed once per calculation request. |
| `same_occupier_since_revaluation`| `bool?`| Wales | Hereditament occupied continuously by the same ratepayer since revaluation list date (required for Welsh TR under W.S.I. 2025/1371 reg. 6(b)). |
| `apply_transitional_relief` | `bool?` | All Jurisdictions | Opt-in/opt-out flag for statutory revaluation transitional phasing relief calculations. Defaults to true. |
| `is_ssbr_qualifying` | `bool` | England | Supporting Small Business Relief under LGFA 1988 s.47; caps net bill increase to £600/£800; compounds across years. |
| `is_sbbs_tr_qualifying` | `bool` | Scotland | Small Business Bonus Scheme Transitional Relief under S.S.I. 2023/31 / S.S.I. 2026/70 reg. 14. |
| `days_previously_unoccupied` | `i32` | All Jurisdictions | Number of unoccupied days elapsed prior to period start, deducted from initial empty rate-free period. |

> **Statutory Slicing, Validation & Execution Mechanics:**
> - **Billing Segment Slicing**: Calculation periods are partitioned into discrete billing segments when crossing:
>   1. **Financial Year Boundaries** (1 April – 31 March).
>   2. **Occupancy Transitions** (`OCCUPIED` ↔ `UNOCCUPIED` ↔ `PART_OCCUPIED`).
>   3. **Rateable Value Alteration Dates** (`rateable_value_history` effective date boundaries).
> - **Input Validation Rules**:
>   - Occupancy periods must not overlap (`AppError::OverlappingOccupancyPeriods`).
>   - Occupation period start date cannot precede the earliest rateable value effective date (`AppError::CalculationPeriodBeforeFirstRv`).
>   - Periods must have valid date ordering (`start_date <= end_date`).
> - **Section 43 (Occupied)**: Standard liability with SBRR, RHL, charitable relief, and G-factor improvement relief.
> - **Section 45 (Unoccupied)**: Initial 3-month (commercial) or 6-month (industrial) empty property rate-free exemption; low value exemption (£2,900 England, £2,200 Wales, £1,700 Scotland).
> - **Statutory Occupation Reset Rule**: Consecutive occupation of $\ge 13$ weeks (91 days) in England post-1 April 2024 (or $\ge 6$ weeks / 42 days in Wales/Scotland or pre-2024) resets empty rate-free entitlement.
> - **Section 44A (Part-Occupied)**: Apportions rateable value between occupied and unoccupied portions; unoccupied portion receives Section 45 empty relief; occupied portion receives occupied reliefs.
> - **Single Penalty Assessment**: Anti-avoidance financial penalties (W.S.I. 2026/43) are levied exactly once per calculation, regardless of how many segments are sliced.
> - **Annual Cash Cap Tracking**: Statutory cash caps (e.g. £110,000 RHL / Food & Drink Hospitality) are aggregated and enforced across all billing segments within the same financial year.

#### Response Payload Schema (`LiabilityCalculationResponse`)
```json
{
  "calculation_id": "8f7e2a91-4c3d-4e2a-9f12-0b3c4d5e6f7a",
  "calculated_at": "2026-10-11T09:30:42Z",
  "parameter_version_ids": {
    "param_multiplier_version": "v2026.1",
    "param_sbrr_version": "v2026.1",
    "param_sector_relief_version": "v2026.2"
  },
  "jurisdiction": "ENGLAND",
  "occupation_period": {
    "start_date": "2025-10-01",
    "end_date": "2026-09-30",
    "days": 365
  },
  "summary_financials": {
    "total_gross_liability": 7795.28,
    "total_transitional_adjustments": 197.01,
    "total_reliefs_and_discounts": 1998.08,
    "total_net_payable": 5600.19,
    "currency": "GBP"
  },
  "annual_billing_segments": [
    {
      "financial_year": "2025/26",
      "segment_period": { "start_date": "2025-10-01", "end_date": "2026-03-31", "days": 182 },
      "rateable_value": 14500.00,
      "multiplier_used": { "type": "SMALL_BUSINESS", "rate": 0.499 },
      "gross_liability": 3607.84,
      "transitional_adjustment": { "type": "NONE", "amount": 0.00 },
      "itemized_reliefs": [
        {
          "relief_code": "SBRR",
          "relief_name": "Small Business Rate Relief",
          "statutory_authority": "LGFA 1988 s.43(4B) / NDRA 2023",
          "discount_percentage": 16.67,
          "amount_deducted": 601.43
        }
      ],
      "net_segment_payable": 3006.41,
      "occupancy_type": "OCCUPIED"
    },
    {
      "financial_year": "2026/27",
      "segment_period": { "start_date": "2026-04-01", "end_date": "2026-09-30", "days": 183 },
      "rateable_value": 16000.00,
      "multiplier_used": { "type": "SMALL_BUSINESS", "rate": 0.522 },
      "gross_liability": 4187.44,
      "transitional_adjustment": { "type": "RELIEF", "amount": 197.01 },
      "itemized_reliefs": [],
      "net_segment_payable": 3990.43,
      "occupancy_type": "OCCUPIED"
    }
  ],
  "calculation_narrative": "STATUTORY CALCULATION NARRATIVE & AUDIT TRAIL\n=============================================\n1. Statutory Framework: Calculated under Local Government Finance Act 1988 (as amended by Non-Domestic Rating Act 2023) for England.\n2. Liability Period: 2025-10-01 to 2026-09-30 (Sliced into 2 billing segments).\n3.1 Financial Year 2025/26:\n    - Occupancy Status: Occupied (Full Occupation)\n    - Rateable Value: £14500\n    - Multiplier (SMALL_BUSINESS): 0.499\n    - Segment Duration: 182 days\n    - Gross Liability: £3607.84\n    - Relief [SBRR] Small Business Rate Relief (16.67%): -£601.43 [LGFA 1988 s.43(4B) / NDRA 2023]\n    - Net Segment Payable: £3006.41\n3.2 Financial Year 2026/27:\n    - Occupancy Status: Occupied (Full Occupation)\n    - Rateable Value: £16000\n    - Multiplier (SMALL_BUSINESS): 0.522\n    - Segment Duration: 183 days\n    - Gross Liability: £4187.44\n    - Transitional Adjustment [RELIEF]: -£197.01 [LGFA 1988 s.57A / NDR Chargeable Amounts Regs]\n    - Net Segment Payable: £3990.43\n=============================================\nTOTAL NET AMOUNT PAYABLE: £6996.84"
}
```

---

## 3. Built-In Statutory Parameter Repository (`src/db/repository.rs`)

The calculation engine uses an in-memory repository (`ParameterRepository` in `src/db/repository.rs`) pre-seeded with statutory parameters from 1990 to 2027 for England, Wales, and Scotland, requiring no external database service.

### 3.1 Repository Structure & Seeded Models

The built-in repository encapsulates 8 core statutory parameter domains:

```rust
pub struct ParameterRepository {
    multipliers: Vec<ParamMultiplier>,
    sbrr_sbbs_thresholds: Vec<ParamSbrrSbbsThreshold>,
    sector_reliefs: Vec<ParamSectorRelief>,
    transitional_caps: Vec<ParamTransitionalCap>,
    empty_property_thresholds: Vec<ParamEmptyPropertyThreshold>,
    letting_tests: Vec<ParamLettingTest>,
    scheme_caps: Vec<ParamSchemeCap>,
    reference_codes: Vec<ParamReferenceCode>,
}
```

#### 1. Multipliers (`ParamMultiplier`)
- **Coverage**: Temporal coverage from **1 April 1990 to 31 March 2027**.
- **Jurisdiction Breakdown**:
  - **England**: Standard Multiplier, Small Business Multiplier, City of London Standard, City of London Small Business.
    - 2023/24: Small Business `0.499`, Standard `0.512`.
    - 2024/25: Small Business `0.499`, Standard `0.546`.
    - 2025/26: Small Business `0.499` (frozen under S.I. 2024/1321), Standard `0.555` (uprated by 1.7% CPI under LGFA 1988 Sched 7 para A4).
    - Effective 1 April 2026, permanent 5-tier system under Autumn Budget 2024 / NDRA 2023:
      - Small Business RHL: `0.382` ($\text{RV} < £51,000$ for qualifying retail, hospitality, leisure)
      - Small Business Non-RHL: `0.432` ($\text{RV} < £51,000$ standard small business rate)
      - Standard RHL: `0.430` ($£51,000 \le \text{RV} < £500,000$ qualifying retail, hospitality, leisure)
      - Standard Non-RHL: `0.480` ($£51,000 \le \text{RV} < £500,000$ standard national rate)
      - High-Value: `0.508` ($\text{RV} \ge £500,000$ premium rate)
  - **Wales**: National Welsh Non-Domestic Multiplier (1990–2025: uniform single multiplier; 2024/25: `0.562`; 2025/26: `0.568` under W.S.I. 2025/214). Effective 1 April 2026, 3-Tier Multiplier Framework under *Local Government Finance (Wales) Act 2024*:
    - Retail Multiplier: `0.350` ($\text{RV} < £51,000$ strictly for retail shops; holiday lets and pubs excluded)
    - Standard Multiplier: `0.502` ($\text{RV} \le £100,000$ non-retail or general commercial)
    - Higher Multiplier: `0.515` ($\text{RV} > £100,000$; ratepayers with statutory exception pay Standard `0.502`)
  - **Scotland**: Three-tier property rate system under Non-Domestic Rates (Scotland) Act 2020:
    - 2023/24 & 2024/25: Basic `0.498` ($\text{RV} \le £51\text{k}$), Intermediate `0.545` ($£51\text{k} < \text{RV} \le £100\text{k}$), Higher `0.559` ($\text{RV} > £100\text{k}$).
    - 2025/26 (S.S.I. 2025/37 & S.S.I. 2025/38 regs 5(4) & 6(4)): Basic Property Rate `0.498` ($\text{RV} \le £51,000$), Intermediate Property Rate `0.554` ($£51,000 < \text{RV} \le £100,000$), Higher Property Rate `0.568` ($\text{RV} > £100,000$).
    - 2026/27 (S.S.I. 2026/39 & S.S.I. 2026/68 regs 5–6): Basic Property Rate `0.481` ($\text{RV} \le £51,000$), Intermediate Property Rate `0.535` ($£51,000 < \text{RV} \le £100,000$), Higher Property Rate `0.548` ($\text{RV} > £100,000$).

#### 2. Small Business Relief Thresholds (`ParamSbrrSbbsThreshold`)
- **England (SBRR)**: 100% relief for $\text{RV} \le £12,000$; linear taper from 100% to 0% for $£12,001 \le \text{RV} \le £15,000$. S.I. 2017/411 art. 3(4) statutory 12-month grace period on primary hereditament when acquiring a secondary property.
- **Wales (Devolved SBRR — W.S.I. 2017/1229)**:
  - Standard Taper: 100% relief for $\text{RV} \le £6,000$; linear taper from 100% to 0% for $£6,001 \le \text{RV} \le £12,000$.
  - Post Offices (Article 4): 100% relief for $\text{RV} \le £9,000$; 50% relief for $£9,001 \le \text{RV} \le £12,000$.
  - Registered Childcare (Article 5): 100% relief for qualifying day care premises up to £100,000 RV.
  - Multiple Properties Limit (Article 12): Maximum of two qualifying hereditaments per billing authority.
  - Exclusions (Article 3): Betting shops, pawnbrokers, payday lenders, and partially occupied properties disqualified.
- **Scotland (SBBS — S.S.I. 2023/34 & S.S.I. 2026/68)**:
  - Standard Taper (Reg 3(3)): 100% relief for $\text{RV} \le £12,000$; linear taper from 100% to 25% for $£12,001 \le \text{RV} \le £15,000$; linear taper from 25% to 0% for $£15,001 \le \text{RV} \le £20,000$.
  - Multiple Property Rules (Reg 4): Where a ratepayer occupies multiple properties in Scotland:
    - If cumulative RV $\le £12,000$: 100% relief on individual property $\le £12,000$.
    - If cumulative RV is $£12,001–£35,000$: 25% relief for any property with $\text{RV} \le £15,000$.
    - If cumulative RV $> £35,000$: 0% relief across all properties (Reg 4(3) statutory cap).

#### 3. Sector Relief Schemes (`ParamSectorRelief`)
- **Retail, Hospitality & Leisure (RHL) / Food & Drink Hospitality**:
  - **England**:
    - 2023/24 & 2024/25: 75% discount (statutory cash cap of £110,000 per business).
    - 2025/26: 40% discount (statutory cash cap of £110,000).
    - 2026/27: Discretionary percentage relief ceased on 31 March 2026 (0% discount); replaced by permanent lower rating multipliers (38.2p / 43.0p).
  - **Wales**:
    - 2023/24: 75% discount (statutory cash cap of £110,000; petrol filling stations SCAT 211 qualify).
    - 2024/25 & 2025/26: 40% discount (statutory cash cap of £110,000; petrol filling stations SCAT 211 excluded).
    - 2026/27: 15% Food & Drink Hospitality Rates Relief scheme (pubs, restaurants, cafés, music venues; £110,000 cash cap; petrol stations, holiday accommodation, and retail multiplier properties excluded).
  - **Scotland (S.S.I. 2026/69 as amended by S.S.I. 2026/135)**:
    - 2026/27 Retail & Leisure (Reg 4): 15% relief on mainland for properties with $\text{RV} \le £100,000$ (£110,000 cap). Mainland properties with $\text{RV} > £100,000$ are ineligible.
    - 2026/27 Hospitality Tier (Reg 4A): 40% relief for qualifying hospitality (hotels, pubs, restaurants, cafés, live music venues in Schedule 1A; £110,000 cap). Applied against gross liability before other reliefs (Reg 4A(3)).
    - 2026/27 Islands & Remote Mainland (Reg 5): 100% relief for island properties and designated remote mainland locations (Cape Wrath, Knoydart, Scoraig; £110,000 cap).
    - Short-Term Let Licensing Requirement: Self-catering holiday accommodation requires a valid STL licence under Civic Government (Scotland) Act 1982 to qualify for RHL relief (Regs 4(2)(b) & 5(1)(c)).
- **Supporting Small Business Relief Scheme (SSBR) — England**:
  - Discretionary relief under LGFA 1988 s.47 capping annual bill increases to £600/year (2023/24–2025/26) and £800/year (2026/27).
  - Compounding: The annual cap applies on top of the preceding year's capped bill.
  - Requires valid prior-year baseline; charities receiving mandatory charitable relief are excluded.
- **Improvement Relief & Heat Network Relief**:
  - Improvement Relief (NDRA 2023 s.1 / S.I. 2024/22 / W.S.I. 2024/302): 12-month G-factor relief in England and Wales effective 1 April 2024 to 31 March 2029 (5-year scheme; not enacted in Scotland).
  - Heat Network Relief (NDRA 2023 s.2): 100% relief in England and Wales effective 1 April 2024 to 31 March 2035 (11-year scheme; not enacted in Scotland).
- **Telecommunications Infrastructure Relief**:
  - 100% mandatory relief under Telecommunications Infrastructure Act 2018 in England (LGFA 1988 s.43(4E)) and Scotland (S.S.I. 2019/111); Wales is excluded.

#### 4. Transitional Phasing Caps (`ParamTransitionalCap` & `SbbsTrResolution`)
- **England 2023 Revaluation List (2023/24, 2024/25, 2025/26)**:
  - Small Hereditament ($\text{RV} \le £20,000$ / London $£28,000$): Upward caps of 5%, 15%, 25%.
  - Medium Hereditament ($\text{RV} \le £100,000$): Upward caps of 15%, 25%, 40%.
  - Large Hereditament ($\text{RV} > £100,000$): Upward caps of 30%, 40%, 55%.
- **England 2026 Revaluation List**:
  - Upward phasing caps: Small (+5%), Medium (+15%), Large (+30%).
- **Scotland General Revaluation Transitional Relief**:
  - **2023 List (S.S.I. 2023/31 Regs 4–9)**: Upward caps of 12.5%/25%/37.5% (Small $\le £20\text{k}$), 25%/50%/75% (Medium $\le £100\text{k}$), and 37.5%/75%/112.5% (Large $> £100\text{k}$).
  - **2026 List (S.S.I. 2026/70 Reg 9)**: Upward caps on gross liability escalation: Small (+15%, $\text{RV} \le £20\text{k}$), Medium (+30%, $£20\text{k} < \text{RV} \le £100\text{k}$), and Large (+50%, $\text{RV} > £100\text{k}$).
  - **2026 Licensed Self-Catering Cap (S.S.I. 2026/135 Reg 4)**: Special 15% transitional cap for licensed self-catering holiday accommodation regardless of RV.
- **Scotland Small Business Bonus Scheme Transitional Relief (SBBS TR)**:
  - **2023 List (S.S.I. 2023/31 Part 3 Regs 10–14)**: Net bill increase caps over 31 March 2023 baseline bill (£600 in 2023/24, £1,200 in 2024/25, £1,800 in 2025/26).
  - **2026 List (S.S.I. 2026/70 Reg 14)**: Ratepayer liability limited to 25% of net bill escalation in 2026/27 (relief deducts 75% of increase), 50% in 2027/28, and 75% in 2028/29.
- **Wales Transitional Relief Schemes**:
  - **2026 List (W.S.I. 2025/1371)**: Applies to net bill increases exceeding £300 over 31 March 2026 baseline bill; 67% relief in Year 1 (2026/27), 34% relief in Year 2 (2027/28). Requires continuous occupation by the same ratepayer since list date (Reg 6(b)); partly occupied properties excluded (Reg 6(c)).
  - **2023 List (W.S.I. 2022/1350)**: 67% relief in 2023/24 and 34% relief in 2024/25 on net bill increases exceeding £300; requires continuous occupation by same ratepayer (Reg 9(b)).
  - **2017 List (W.S.I. 2016/1247)**: Small Business Transitional Relief for ratepayers receiving SBRR on 31 March 2017 (75% in 2017/18, 50% in 2018/19, 25% in 2019/20 on increases over £100).
  - **2005 & 2010 Lists**: Wales did not operate general transitional phasing schemes.
- **2017 Revaluation List (England)**: Historical upward statutory phasing percentages under S.I. 2016/1265 and S.I. 2022/339 (Small 5%/7.5%/10%/15%/15%/15%, Medium 12.5%/17.5%/20%/25%/25%/25%, Large 42%/32%/49%/16%/6%/6%).

#### 5. Empty Property Low-Value Thresholds (`ParamEmptyPropertyThreshold`)
- Pre-seeded statutory low-value rateable value exemption thresholds across all lists (1990–2027) for England, Wales, and Scotland:
  - **England**: £2,200 (2005 list under SI 2005/211), £15,000 (2008/09 temporary), £18,000 (2009/10 temporary), £2,600 (2010 list under SI 2010/407), £2,900 (2017 & 2023 lists under SI 2017/101 & SI 2022/1263).
  - **Wales**: £2,200 (1990–2027 under WSI 2008/2672).
  - **Scotland**: £1,700 (2018–2027 under S.S.I. 2018/77).

#### 6. Holiday Let Letting Test Requirements (`ParamLettingTest`)
- Pre-seeded actual let and commercial availability statutory thresholds:
  - **England**: 140 days available and 70 days actual let (effective 1 April 2023 under SI 2022/217).
  - **Wales**: 140 days available and 70 days actual let (1 April 2010–31 March 2023 under WSI 2010/682); 252 days available and 182 days actual let (from 1 April 2023 under WSI 2022/563).
  - **Scotland**: 140 days available and 70 days actual let (from 1 April 2022 under S.S.I. 2021/489).

#### 7. Devolved Scheme Caps & Statutory Additions (`ParamSchemeCap`)
- Dynamic parameter storage for scheme-specific financial limits, statutory additions, and penalty schedules:
  - **Rural Settlement Relief Caps**: £8,500 RV for sole general store/post office; £12,500 RV for sole pub/petrol filling station (LGFA 1988 s.43(6A)–(6E)).
  - **Scottish Renewable Energy Generation Relief**: Tiered rateable value relief percentages (100% $\le £145\text{k}$, 50% $£145\text{k}–£430\text{k}$, 25% $£430\text{k}–£860\text{k}$, 10% $£860\text{k}–£4.3\text{m}$, 2.5% $> £4.3\text{m}$ under S.S.I. 2010/336).
  - **Empty Property Occupancy Reset Period**: 91 days (13 weeks) for England post-1 April 2024 (NDRA 2023 s.15 / SI 2024/400); 42 days (6 weeks) historically and for Wales and Scotland.
  - **Transitional Relief Supplement (TRS)**: 0.010 addition to transitional multiplier in England for small/medium properties (SI 2016/1265 reg 10(7)).
  - **Welsh Artificial Avoidance Penalty**: £500 fixed sum + 3% rateable value variable addition under Local Government Finance (Wales) Act 2024 s.19.

#### 8. Statutory Reference Codes (`ParamReferenceCode`)
- Pre-seeded dictionaries for `JURISDICTION`, `OCCUPATION_STATUS`, `SCAT_CODE`, `BILLING_FREQUENCY`, `RELIEF_TYPE`, and `LANGUAGE` with localized titles and descriptions across English (`en`), Welsh (`cy`), Scots (`sco`), and Scottish Gaelic (`gd`).

### 3.2 Lookup Methods

The repository exposes query methods:

```rust
impl ParameterRepository {
    pub fn resolve_multiplier(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> MultiplierInfo;
    pub fn calculate_sbrr_sbbs_discount(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> Decimal;
    pub fn get_sector_relief(&self, jurisdiction: Jurisdiction, financial_year: &str, relief_code: &str) -> Option<&ParamSectorRelief>;
    pub fn resolve_transitional_cap(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> Option<&ParamTransitionalCap>;
    pub fn resolve_empty_property_threshold(&self, jurisdiction: Jurisdiction, financial_year: &str) -> (Decimal, &'static str);
    pub fn resolve_holiday_let_thresholds(&self, jurisdiction: Jurisdiction, financial_year: &str) -> (u32, u32);
    pub fn resolve_rural_settlement_cap(&self, category: &str) -> Decimal;
    pub fn resolve_renewable_energy_relief_pct(&self, rv: Decimal) -> Decimal;
    pub fn resolve_occupancy_reset_days(&self, jurisdiction: Jurisdiction, financial_year: &str) -> i64;
    pub fn resolve_transitional_supplement(&self, jurisdiction: Jurisdiction, financial_year: &str, property_size: &str) -> Decimal;
    pub fn resolve_welsh_avoidance_penalty(&self) -> (Decimal, Decimal);
    pub fn get_reference_codes(&self, category: &str) -> Vec<&ParamReferenceCode>;
}
```

### 3.3 Statutory Parameter REST Endpoints & OpenAPI Documentation

The in-memory parameter repository and documentation are exposed via REST endpoints:
- `GET /api/v1/parameters`: Aggregated JSON payload containing all 8 parameter datasets (multipliers, SBRR/SBBS thresholds, sector reliefs, transitional caps, empty property thresholds, holiday let letting tests, devolved scheme caps, and reference codes).
- `GET /api/v1/parameters/multipliers`: Temporal rating multipliers array (1990–2027) disaggregated by jurisdiction.
- `GET /api/v1/parameters/sbrr-thresholds`: SBRR / SBBS rateable value thresholds and statutory linear relief rules.
- `GET /api/v1/parameters/sector-reliefs`: Devolved retail, hospitality, leisure, and special relief schemes.
- `GET /api/v1/parameters/transitional-caps`: Statutory revaluation transitional phasing percentage caps.
- `GET /api/v1/parameters/empty-property`: Low-value empty property rateable value exemption thresholds across all lists (1990–2027) for England, Wales, and Scotland.
- `GET /api/v1/parameters/letting-tests`: Statutory holiday let actual let and commercial availability days thresholds.
- `GET /api/v1/parameters/scheme-caps`: Scheme-specific financial limits, statutory additions, reset days, and penalty schedules.
- `GET /api/v1/openapi.json` & `GET /api/v1/openapi.yaml`: OpenAPI 3.1.0 specifications documenting models, calculation endpoints, schemas, and parameter filters.
- `GET /docs`: Scalar API reference UI rendered from `templates/scalar.html` with query parameter templating (`?title=...&spec_url=...&theme=...&layout=...`).

---

## 4. Test Cases Specification & Scenario Fixtures

### Test Suite Overview (161 Passing Integration Tests)

The test suite is organized under [`tests/integration_tests/`](tests/integration_tests/) across 10 test suites (Suites A through J) plus shared helpers in [`tests/integration_tests/common.rs`](tests/integration_tests/common.rs), coordinated by the main test runner in [`tests/integration_tests.rs`](tests/integration_tests.rs). It validates statutory calculations, multi-occupancy slicing, i18n, parameter endpoints, documentation, third-party licenses, and web security across **161 integration tests**:

- **Suite A: Multi-Jurisdiction Scenario Fixture Tests (Scenarios 1–11)** ([`tests/integration_tests/suite_a_scenarios.rs`](tests/integration_tests/suite_a_scenarios.rs) — 11 tests):
  - `test_scenario_1_england_sbrr_and_rhl_multi_year` (`scenario_1_sbrr_rhl_england.json`): Multi-year slicing (2025/26 & 2026/27), SBRR linear taper, England 2026/27 RHL expiration verification, and Transitional Relief caps.
  - `test_scenario_2_wales_petrol_station` (`scenario_2_wales_petrol_station.json`): SCAT 211 forecourt valuation under Welsh Local Government Finance Act 2024.
  - `test_scenario_3_scotland_self_catering` (`scenario_3_scotland_self_catering.json`): Scottish SBBS £12k threshold and self-catering actual letting compliance.
  - `test_scenario_4_welsh_artificial_avoidance` (`scenario_4_welsh_artificial_avoidance.json`): Statutory avoidance penalty triggered by `is_artificial_avoidance_suspected` flag (£500 + 3% RV penalty under WSI 2026/43).
  - `test_scenario_5_airport_high_value` (`scenario_5_airport_high_value.json`): High-value hereditament (RV £1.25m) using English standard national multiplier.
  - `test_mandatory_charitable_relief_80_percent` (`scenario_6_charity_mandatory_80.json`): LGFA 1988 s.43(5) & LG(R)S 1962 s.4(2) mandatory 80% charitable reduction.
  - `test_multi_occupancy_periods_single_liable_party` (`scenario_7_multi_occupancy_episodes.json`): 3 episodes across 2024/25 (`UNOCCUPIED` → `OCCUPIED` → `PART_OCCUPIED`), Section 44A dual-portion apportionments, and statutory reset triggers.
  - `test_suite_8_empty_property_exemption` (`scenario_8_empty_property_exemption.json`): 100% exemption for initial statutory period and reset rules under SI 2008/386.
  - `test_ndra_2023_improvement_relief_g_factor` (`scenario_9_improvement_relief_g_factor.json`): Non-Domestic Rating Act 2023 s.1 G-factor certificate reduction on qualifying property improvements.
  - `test_statutory_10_and_12_installments_schedule` (`scenario_10_statutory_12_installments.json`): Statutory payment schedules under SI 1989/1058 (10 monthly installments) and ratepayer election of 12 monthly installments (LGA 2003 / SI 2014/466).
  - `test_statutory_rural_settlement_relief` (`scenario_11_rural_settlement_relief.json`): Mandatory 100% rural settlement relief under LGFA 1988 s.43(6A) for sole village shops, food stores, post offices, and pubs.

- **Suite B: Multi-Occupancy, Mid-Year RV Slicing & Period Validation** ([`tests/integration_tests/suite_b_multi_occupancy.rs`](tests/integration_tests/suite_b_multi_occupancy.rs) — 7 tests):
  - `test_multi_occupancy_api_endpoint_json_serialization`: JSON payload serialization and billing segment output for multi-occupancy arrays.
  - `test_mid_year_rateable_value_change_splits_billing_segments`: Validates segment slicing across mid-year rateable value alterations (`rateable_value_history` effective date boundaries).
  - `test_calculate_liability_period_before_first_rv_returns_bad_request`: Returns HTTP 400 Bad Request if calculation period begins prior to the earliest RV record.
  - `test_overlapping_occupancy_periods_rejected`: Validates rejection of overlapping temporal episodes (`AppError::OverlappingOccupancyPeriods`).
  - `test_open_ended_occupation_empty_string_and_null`: Open-ended occupation handling (`""`, `null`, omitted `end_date`) defaulting to 2027-03-31.
  - `test_empty_property_prior_unoccupied_days_out_of_order_periods`: Ensures ordering and prior empty days tracking when episodes are submitted out of chronological order.
  - `test_unseeded_year_raises_error_not_045_default`: Confirms requesting an unseeded rating year outside 1990–2027 returns a structured domain error rather than an arbitrary fallback multiplier.

- **Suite C: 4-Language Internationalization (i18n) & Content Negotiation** ([`tests/integration_tests/suite_c_i18n.rs`](tests/integration_tests/suite_c_i18n.rs) — 13 tests):
  - `test_i18n_welsh_narrative_and_relief_names`: Welsh audit narrative headers and localized relief naming (*"Rhyddhad Elusennol Gorfodol"*).
  - `test_i18n_scots_and_gaelic_narrative_and_relief_names`: Scots (*"Mandatar Chairitable Rate Relief"*) and Scottish Gaelic (*"Faochadh Ìrean Carthannais Èigneachail"*) narratives and reliefs.
  - `test_i18n_reference_codes_localized_welsh_and_english`: Bilingual reference code resolution for jurisdictions and reliefs.
  - `test_i18n_reference_codes_localized_scots_and_gaelic`: Scots and Scottish Gaelic reference codes.
  - `test_i18n_localized_error_messages`: Localized Welsh error payloads.
  - `test_i18n_localized_error_messages_scots_and_gaelic`: Localized Scots and Gaelic domain error payloads.
  - `test_language_resolution_precedence`: Verifies precedence: Body field > Query parameter > Accept-Language header > Default English.
  - `test_http_calculate_liability_multilingual_content_negotiation`: HTTP endpoint bilingual responses via headers and body flags.
  - `test_http_calculate_liability_invalid_dates_localized`: Localized error messages for invalid occupation periods.
  - `test_http_calculate_liability_bad_payload_error`: Validates HTTP 422 `PAYLOAD_DESERIALIZATION_ERROR` response on malformed JSON payload.
  - `test_domain_error_status_codes_and_codes`: Verifies strongly-typed `AppError` status codes and error identifiers.
  - `test_narrative_relief_assessment_audit_unawarded_reliefs`: Verifies localized statutory audit justification for disqualified/unawarded reliefs across English, Welsh, Scots, and Scottish Gaelic.
  - `test_narrative_decision_traceability_across_languages`: Verifies decision justification lines (multiplier determination, transitional relief phasing, SBRR taper divisor $E$, empty property rate-free calendar months, and statutory installment schedule) with statutory citations across English, Welsh, Scots, and Scottish Gaelic maintaining 100% key parity.

- **Suite D: Reference Codes, Parameter Resolution, Public API Access & OpenAPI** ([`tests/integration_tests/suite_d_api_and_parameters.rs`](tests/integration_tests/suite_d_api_and_parameters.rs) — 12 tests):
  - `test_app_state_repository_access`: Validates thread-safe parameter repository access on `AppState`.
  - `test_zero_hardcoding_historical_multipliers_1990_to_2026`: Validates English, Welsh, and Scottish rating multipliers from 1990 to 2026.
  - `test_preseeded_reference_codes_and_allowable_values`: Pre-seeded values for jurisdictions, party types, and reliefs.
  - `test_http_reference_codes_endpoints`: Validates public `GET /api/v1/reference-codes` and verifies unmapped prefix path returns `404 NOT FOUND`.
  - `test_http_calculate_liability_unauthenticated_access`: Verifies calculation succeeds on `POST /api/v1/calculate-liability` without requiring authorization headers.
  - `test_http_sso_endpoints`: Verifies unmapped SSO endpoints return `404 NOT FOUND`.
  - `test_http_parameters_endpoints`: Validates all 8 parameter catalog endpoints return valid HTTP 200 JSON arrays.
  - `test_openapi_and_swagger_endpoints`: Validates OpenAPI 3.1.0 specifications (`/api/v1/openapi.json`, `/api/v1/openapi.yaml`), `/docs` Scalar HTML rendering with query parameter templating, static `scalar.js`, and confirms unsupported aliases return 404.
  - `test_licenses_endpoints`: Validates `/licenses` (HTML interface styled with unified `/assets/app.css` under CSP), `/licenses.md` (raw Markdown attribution document), and `/api/v1/licenses?format=md` query routing.
  - `test_health_check_endpoints`: Validates both root `/health` and `/api/v1/health` return HTTP 200 with OWASP security headers, subsystem health checks (`engine_api: pass`, `parameters_repository: pass`, `frontend_bundle: pass`), and deployment build metadata (`version`, `commit_hash`, `build_number`, `build_timestamp`).
  - `test_openapi_app_error_response_enum_matches_engine_codes`: Validates AppErrorResponse.error enum in OpenAPI 3.1.0 spec includes all strongly-typed domain error codes returned by the engine (including MISSING_MULTIPLIER_FOR_PERIOD and RATEABLE_VALUE_MISSING_FOR_PERIOD).
  - `test_openapi_contract_alignment_and_strict_deserialization`: Validates OpenAPI `/licenses` defines both `text/markdown` and `text/html`, calculation requests strictly require `occupation_period` yielding HTTP 422 `PAYLOAD_DESERIALIZATION_ERROR` when missing, `payment_installments_count` is strictly validated to 10 or 12 under S.I. 1989/1058, and `MULTIPLIER_TYPE` reference codes include `BASIC` and `INTERMEDIATE` with Scotland included in `HIGHER`.
  - `test_issue_70_every_seeded_multiplier_type_has_reference_code`: Asserts that every `multiplier_type` seeded in parameters (including Welsh 2027/28 provisional multipliers `RETAIL_PROVISIONAL`, `STANDARD_PROVISIONAL`, and `HIGHER_PROVISIONAL`) has an active corresponding entry in `MULTIPLIER_TYPE` reference codes across all 4 locales.

- **Suite E: English Multipliers, Transitional Phasing, Grace Periods & SSBR** ([`tests/integration_tests/suite_e_england_schemes.rs`](tests/integration_tests/suite_e_england_schemes.rs) — 23 tests):
  - `test_england_2026_five_multipliers_rhl_vs_non_rhl`: Verifies Autumn Budget 2024 / NDRA 2023 five-multiplier regime in England for 2026/27 (38.2p, 43.2p, 43.0p, 48.0p, 50.8p).
  - `test_england_sbrr_second_property_grace_period`: Verifies statutory 12-month grace period on primary hereditament when taking on an additional hereditament (S.I. 2017/411 art. 3(4) under LGFA 1988 s.43(4B)).
  - `test_statutory_sbrr_england_linear_taper`: Validates English £12k–£15k SBRR linear taper.
  - `test_suite_7_transitional_phasing_relief_cap`: Chargeable amounts ceiling capping year-on-year bill escalation under NDRA 2023.
  - `test_suite_7_transitional_relief_none_when_under_cap`: Zero transitional adjustment when calculated liability remains under cap.
  - `test_england_transitional_relief_opt_out`: Verifies setting `apply_transitional_relief: Some(false)` calculates unphased gross liability without transitional relief deduction.
  - `test_english_transitional_relief_compounding_across_multiple_years`: Validates transitional relief compounding across consecutive rating years.
  - `test_transitional_relief_preserved_when_occupation_starts_partway_through_year`: Ensures transitional relief cap is preserved when occupation commences mid-year.
  - `test_england_supporting_small_business_relief_2023_cap_600`: Verifies SSBR under LGFA 1988 s.47 capping net bill increase to £600/year in 2023/24 for ratepayers losing SBRR.
  - `test_england_supporting_small_business_relief_2026_cap_800`: Verifies SSBR under LGFA 1988 s.47 capping net bill increase to £800/year in 2026/27.
  - `test_ssbr_compounds_into_year_2`: Confirms SSBR cap compounds into year 2 on top of the preceding year's capped bill.
  - `test_ssbr_not_awarded_for_new_property_without_baseline`: Validates SSBR is not awarded to new properties lacking a prior-year baseline.
  - `test_ssbr_not_awarded_to_charities`: Validates SSBR cannot be awarded to charities receiving mandatory charitable relief.
  - `test_sector_relief_england_2025_vs_2026`: Verifies English RHL 40% relief in 2025/26 and statutory expiration (0%) in 2026/27 under NDRA 2023.
  - `test_england_multiplier_2025_26_standard_is_0555`: Validates England 2025/26 standard multiplier is 0.555 (55.5p) under LGFA 1988 Sched 7 para A4 and S.I. 2024/1321.
  - `test_england_small_business_multiplier_boundary_at_51000`: Validates small business multiplier applies up to £50,999 RV and standard multiplier applies at £51,000 RV under S.I. 2017/102 art. 2 and S.I. 2026/152 regs 3–5.
  - `test_sbrr_taper_uses_statutory_e_divisor_england`: Validates England SBRR taper chargeable amount $(A \times M) / E$ where $E = 3000 / (A - 12000)$ to 3 decimal places under S.I. 2017/102 art. 3 and S.I. 2023/1250 reg. 3.
  - `test_empty_rate_free_period_is_calendar_months`: Verifies empty property rate-free periods are three / six calendar months under S.I. 2008/386 reg 4(a)-(b) & W.S.I. 2008/2499 reg 4(a)-(b) (90 days from 1 Dec, 91 from 1 Apr, 183 industrial from 1 Apr; Wales 92 days from 1 Nov).
  - `test_england_empty_property_historical_low_value_thresholds`: Validates historical English empty property low-value rateable value thresholds under S.I. 2008/386 reg 4(g) (£2,200 in 2008/09), S.I. 2008/3283 reg 2 (£15,000 in 2009/10), S.I. 2010/408 reg 2(2)(b) (£18,000 in 2010/11), S.I. 2010/408 reg 2(2)(a) (£2,600 in 2011/12–2016/17), and S.I. 2017/102 art 4 (£2,900 from 2017/18 onwards).
  - `test_statutory_installments_whole_pounds_si_1989_1058`: Verifies statutory instalment calculation under S.I. 1989/1058 Schedule 1 Part I para 1(4)-(5) where all instalments except the first are whole pounds ($A = C / D$ rounded to nearest pound) and the first instalment takes the balancing difference ($B = C - (D - 1) \times A$).
  - `test_england_transitional_relief_greater_london_small_band_threshold_28000`: Validates Greater London small-band threshold (£28,000 RV vs £20,000 outside London) under S.I. 2022/1403 reg 10(5), awarding small-band cap (+5%) rather than medium-band (+15%) to an RV £25,000 London hereditament.
  - `test_england_2026_transitional_supplement`: Validates England 2026/27 Transitional Relief Supplement (TRS) of 1p ($RS = 0.010$) replacing multiplier $M$ with $(M + RS)$ under S.I. 2026/81 reg 16 for defined hereditaments not receiving transitional relief under reg 12, and statutory exemptions for charities (reg 16(1)(c)) and newly created hereditaments (reg 5(2)).
  - `test_england_small_business_multiplier_threshold_pre_2017`: Validates historical English small business multiplier qualification thresholds prior to 2017/18 under LGFA 1988 s.43(4A), S.I. 2004/3315 art 3(1)(b) (£14,999 outside London / £21,499 in London), and S.I. 2012/148 art 3(1)(b) (£17,999 outside London / £25,499 in London), confirming that properties with RV exceeding these thresholds pay the standard multiplier, and verifying Greater London threshold differentiation.

- **Suite F: Welsh Multipliers, SBRR, Exemptions, Anti-Avoidance & Transitional Schemes** ([`tests/integration_tests/suite_f_wales_schemes.rs`](tests/integration_tests/suite_f_wales_schemes.rs) — 31 tests):
  - `test_wales_multiplier_2025_26_is_0568`: Validates Welsh uniform multiplier for 2025/26 is 0.568 under W.S.I. 2025/214.
  - `test_wales_2026_three_multipliers`: Verifies Welsh 2026/27 three-tier multiplier regime under *Local Government Finance (Wales) Act 2024* (Retail 35.0p, Standard 50.2p, Higher 51.5p).
  - `test_wales_retail_multiplier_given_to_retail_shop`: Validates 35.0p retail multiplier awarded to qualifying retail shop.
  - `test_wales_retail_multiplier_not_given_to_holiday_let_or_pub_with_rhl_flag`: Validates holiday lets and pubs are excluded from the retail multiplier.
  - `test_wales_higher_multiplier_exception_pays_standard_rate`: Validates qualifying excepted property pays Standard 0.502 instead of Higher 0.515.
  - `test_wales_sbrr_childcare_100_percent_up_to_100k`: Verifies 100% SBRR for CIW-registered childcare premises up to £100k RV (W.S.I. 2017/1229 art. 5).
  - `test_wales_sbrr_post_office_tiered_relief`: Verifies post office tiered relief (100% $\le £9k$, 50% $£9k–£12k$ under W.S.I. 2017/1229 art. 4).
  - `test_wales_sbrr_two_properties_in_billing_authority`: Enforces two-property limit per billing authority under W.S.I. 2017/1229 art. 12.
  - `test_wales_sbrr_excepted_hereditaments_no_relief`: Disqualifies excepted hereditaments (betting shops, payday lenders) under W.S.I. 2017/1229 art. 3.
  - `test_wales_sbrr_part_occupied_excluded`: Confirms partly occupied apportionments do not receive SBRR in Wales.
  - `test_wales_empty_property_low_value_threshold_boundaries`: Evaluates Welsh unoccupied property low-value threshold boundaries under W.S.I. 2008/2499 reg 4(g), confirming RV £2,000 is 100% exempt and RV £2,500 (between Welsh £2,200 and English £2,900) is non-exempt.
  - `test_wales_telecoms_infrastructure_relief_not_granted`: Confirms telecoms infrastructure relief is not granted in Wales under LGFA 1988 s.43(4E).
  - `test_welsh_artificial_avoidance_penalty_charged_once_across_segments`: Verifies W.S.I. 2026/43 penalty (£500 + 3% RV) is assessed only once across multiple segments.
  - `test_wales_petrol_station_qualifies_for_rhl_2023_24_75_percent`: Petrol filling stations qualify for Welsh RHL 75% in 2023/24.
  - `test_wales_petrol_station_qualifies_for_rhl_2024_25_40_percent`: Petrol stations are excluded from Welsh RHL 40% in 2024/25.
  - `test_sector_relief_wales_2026_food_drink_hospitality`: Verifies Welsh 15% Food & Drink Hospitality relief for pubs, restaurants, and cafés in 2026/27.
  - `test_wales_holiday_let_excluded_from_2026_27_food_drink_relief`: Validates holiday lets are excluded from Welsh food and drink relief.
  - `test_wales_retail_shop_on_retail_multiplier_excluded_from_2026_27_food_drink_relief`: Validates shops receiving the retail multiplier do not receive food & drink relief.
  - `test_wales_transitional_relief_2026_year_1_67_percent`: Verifies Welsh 2-year TR scheme (W.S.I. 2025/1371) granting 67% relief on increases > £300 in 2026/27.
  - `test_wales_transitional_relief_2027_year_2_34_percent`: Verifies Welsh 2-year TR scheme granting 34% relief on increases > £300 in 2027/28.
  - `test_wales_transitional_relief_under_300_no_relief`: Zero relief when net bill increase is $\le £300$.
  - `test_wales_transitional_relief_excluded_when_occupier_changed`: Disqualifies property from Welsh TR if occupier changed since list date (W.S.I. 2025/1371 reg 6(b)).
  - `test_wales_transitional_relief_excluded_for_part_occupied_hereditament`: Disqualifies partly occupied apportionments from Welsh TR (reg 6(c)).
  - `test_wales_transitional_relief_with_sbrr_on_both_sides`: Validates transitional calculation with SBRR on baseline and revalued bills.
  - `test_wales_2023_list_transitional_relief_2024_25_uses_2023_multiplier`: Verifies Year 2 of 2023 scheme uses 2023 multiplier for baseline calculation.
  - `test_wales_2017_list_small_business_relief_75_50_25_phasing`: Validates 2017 Welsh SBRR TR (75% Year 1) on increases > £100 under W.S.I. 2016/1247.
  - `test_wales_2017_list_no_small_business_relief_gets_zero_transitional_relief`: Confirms non-SBRR properties receive 0% TR in 2017.
  - `test_wales_2005_list_no_transitional_relief`: Confirms Wales operated no general transitional relief in 2005.
  - `test_wales_2010_list_no_transitional_relief`: Confirms Wales operated no general transitional relief in 2010.
  - `test_sbrr_taper_uses_statutory_e_divisor_wales`: Validates Welsh SBRR taper chargeable amount $(A \times M) / E$ where $E = 6000 / (\text{RV} - 6000)$ to 3 decimal places under W.S.I. 2017/1229 art. 11(3).
  - `test_wales_2026_has_no_fictitious_upward_caps_in_transitional_caps`: Validates absence of upward caps for Wales 2026 in transitional caps repository, as W.S.I. 2025/1371 operates as a share-of-increase scheme (67%/34%) published under scheme caps.

- **Suite G: Scottish Property Rates, SBBS Tapers, 40% Hospitality & Transitional Relief** ([`tests/integration_tests/suite_g_scotland_schemes.rs`](tests/integration_tests/suite_g_scotland_schemes.rs) — 21 tests):
  - `test_scotland_2026_multipliers_basic_intermediate_higher`: Validates 2026/27 rates: Basic 0.481, Intermediate 0.535, Higher 0.548 (S.S.I. 2026/39).
  - `test_scotland_sbbs_taper_band_12k_to_15k`: Verifies linear taper between 100% and 25% for RV £12k–£15k under S.S.I. 2026/68 reg 3(3).
  - `test_scotland_sbbs_taper_band_15k_to_20k`: Verifies linear taper between 25% and 0% for RV £15k–£20k under S.S.I. 2026/68 reg 3(3).
  - `test_scotland_sbbs_multiple_properties_rule_reg_4`: Verifies multiple property rule granting 25% relief for properties $\le £15k$ when cumulative RV is $£12k–£35k$.
  - `test_scotland_sbbs_multiple_properties_exceeding_35k_cap`: Validates 0% SBBS relief across all properties when cumulative RV exceeds £35,000 (reg 4(3)).
  - `test_scotland_2026_rhl_retail_15_percent_shop`: Verifies 15% Retail & Leisure relief for mainland shop $\le £100k$ RV under S.S.I. 2026/69 reg 4.
  - `test_scotland_2026_rhl_hospitality_40_percent_pub`: Verifies 40% Hospitality relief tier for pubs, restaurants, and hotels under S.S.I. 2026/69 reg 4A.
  - `test_scotland_2026_remote_mainland_rhl_100_percent`: Verifies 100% relief for Cape Wrath, Knoydart, and Scoraig under S.S.I. 2026/69 reg 5.
  - `test_sector_relief_scotland_mainland_vs_island_2026`: Verifies 15% mainland vs 100% island relief under *Islands (Scotland) Act 2018*.
  - `test_scotland_2026_self_catering_licensing_rhl_qualification`: Enforces short-term let licensing requirement under Civic Government (Scotland) Act 1982 / S.S.I. 2022/86 for self-catering RHL eligibility.
  - `test_scotland_2026_transitional_caps`: Validates 2026 transitional caps: Small +15%, Medium +30%, Large +50% under S.S.I. 2026/70 reg 9.
  - `test_scotland_2026_self_catering_licensed_transitional_cap_15_percent`: Validates 15% transitional cap for licensed self-catering under S.S.I. 2026/135 reg 4.
  - `test_scottish_general_transitional_relief_2023_list`: Verifies 2023 general transitional relief (+12.5% upward cap under S.S.I. 2023/31).
  - `test_scottish_sbbs_tr_2023_list_600_cap`: Verifies 2023 SBBS TR capping net bill increase to £600 under S.S.I. 2023/31 Part 3.
  - `test_scottish_sbbs_tr_2026_list_25_percent_cap`: Verifies 2026 SBBS TR limiting ratepayer liability to 25% of net bill escalation under S.S.I. 2026/70 reg 14.
  - `test_scottish_unoccupied_property_relief_50_percent`: Verifies 50% empty relief for 3 months and £1,700 low RV exemption under S.S.I. 2018/77.
  - `test_scottish_renewable_energy_generation_relief`: Verifies tiered renewable energy generation relief under S.S.I. 2010/336.
  - `test_scotland_2025_26_intermediate_and_higher_multipliers`: Validates 2025/26 Intermediate (0.554) and Higher (0.568) property rates under S.S.I. 2025/37 and S.S.I. 2025/38 regs 5(4) & 6(4).
  - `test_scotland_empty_property_2018_rules_90_percent_after_initial_period`: Validates S.S.I. 2018/77 reg 2 empty property rules for 2018/19–2022/23 (50% for 3 months then 90% payable; industrial 100% exempt for 6 months then 90% payable).
  - `test_scotland_sbbs_2023_onward_published_thresholds_have_both_tapers`: Validates published SBBS threshold parameters for Scotland 2023/24 onward include 100% full relief band (£0–£12k), 100%->25% linear taper band (£12k–£15k, is_tapered: true), and 25%->0% linear taper band (£15k–£20k, is_tapered: true) under S.S.I. 2023/31 and S.S.I. 2026/68 reg 3(3).
  - `test_scotland_large_business_supplement_and_property_rates_historical`: Validates Scottish Large Business Supplement (LBS) for properties exceeding statutory thresholds (> £35k from 2010/11 to 2016/17; > £51k from 2017/18 to 2019/20) and Intermediate/Higher Property Rates for 2020/21–2022/23 under Local Government (Scotland) Act 1975 s.7B, S.S.I. 2010/38, S.S.I. 2014/40, S.S.I. 2018/76, and S.S.I. 2021/63.

- **Suite H: Cross-Jurisdictional Reliefs, Lifespans, Holiday Lets & Empty Property** ([`tests/integration_tests/suite_h_cross_jurisdictional.rs`](tests/integration_tests/suite_h_cross_jurisdictional.rs) — 24 tests):
  - `test_statutory_heat_network_exemption`: NDRA 2023 s.2 100% heat network discount.
  - `test_heat_network_relief_not_granted_before_1_april_2024`: Validates heat network relief is not awarded before 1 April 2024.
  - `test_heat_network_relief_not_granted_on_or_after_1_april_2035`: Validates heat network relief sunsets on 31 March 2035.
  - `test_ndra_2023_reliefs_granted_in_wales_from_2024`: Confirms both Improvement Relief (G-factor) and Heat Network Relief are active in Wales from 1 April 2024 under W.S.I. 2024/249.
  - `test_improvement_relief_not_granted_before_1_april_2024`: Validates improvement relief is not awarded before 1 April 2024.
  - `test_improvement_relief_not_granted_on_or_after_1_april_2029`: Validates improvement relief sunsets on 31 March 2029.
  - `test_improvement_and_heat_network_not_granted_to_scotland`: Confirms neither improvement nor heat network relief applies in Scotland.
  - `test_statutory_telecommunications_infrastructure_relief_england_and_scotland`: 100% fibre relief in England and Scotland under Telecommunications Infrastructure Act 2018.
  - `test_statutory_public_lavatory_exemption`: Rating (Public Lavatories) Act 2021 100% discount.
  - `test_rural_settlement_relief_rv_cap_enforcement`: Statutory rateable value caps for mandatory rural relief (£8,500 / £12,500).
  - `test_discretionary_charitable_relief_top_up`: Billing authority 20% discretionary top-up bringing total charitable relief to 100%.
  - `test_part_occupied_charity_reliefs_capped_at_occupied_chargeable`: Caps charitable relief at the occupied portion's chargeable amount under s.44A.
  - `test_statutory_empty_low_value_exemption`: Low value unoccupied exemptions (£2,900 England, £2,200 Wales, £1,700 Scotland).
  - `test_statutory_empty_property_prior_unoccupied_days`: Calculation accounting for prior unoccupied days deducted from statutory allowance (62 exempt days — three calendar months from 2 March).
  - `test_rhl_relief_applies_in_sbrr_taper_band_england_and_wales`: Validates RHL relief applies to net liability after SBRR taper in England and Wales.
  - `test_rhl_cash_cap_enforced_per_financial_year_across_segments`: Validates £110,000 cash cap enforced across all billing segments within the same financial year.
  - `test_historical_rhl_relief_england_wales_scotland`: Validates historical retail, hospitality and leisure reliefs across England (Expanded Retail Discount 100% in 2020/21; 100% Q1 & 66% Q2-Q4 in 2021/22 under LGFA 1988 s.47; 50% in 2022/23), Wales (100% in 2020/21 & 2021/22 for RV <= £500,000; 50% in 2022/23), and Scotland (100% island hospitality in 2024/25; 40% mainland hospitality in 2025/26 for RV <= £51,000).
  - `test_self_catering_holiday_let_letting_days_test`: Minimum letting days thresholds (70 days Scotland from 2022/23, 70 days England from 2023/24, 70 days pre-2023 / 182 days post-2023 Wales).
  - `test_scotland_self_catering_70_day_letting_test`: Validates Scottish self-catering statutory criteria under S.S.I. 2021/489 (70 days actual let, 140 days available; 60 days let fails and transfers to Council Tax).
  - `test_holiday_let_actual_letting_test_start_dates_england_wales`: Validates letting test start dates (England: 70-day actual letting starts 1 April 2023 under S.I. 2022/217; Wales: 70-day test starts 1 April 2010 under W.S.I. 2010/682).
  - `test_welsh_holiday_let_182_days_qualifies_post_2023`: Validates 182-day actual letting test in Wales post-1 April 2023 under W.S.I. 2022/563.
  - `test_welsh_holiday_let_70_days_disqualified_post_2023`: Confirms 70 days actually let is disqualified in Wales post-1 April 2023.
  - `test_welsh_holiday_let_70_days_qualifies_pre_2023`: Confirms 70 days qualified in Wales prior to 1 April 2023.
  - `test_welsh_holiday_let_availability_threshold_disqualification`: Validates Welsh 252-day commercial availability threshold under W.S.I. 2022/563.

- **Suite I: Component UI Serving & OWASP Security** ([`tests/integration_tests/suite_i_ui_and_security.rs`](tests/integration_tests/suite_i_ui_and_security.rs) — 7 tests):
  - `test_owasp_secure_headers_on_static_routes`: Verifies `/` serves the Svelte 5 single-page application mount `<div id="app">`, bundled assets `/assets/*` serve with correct MIME types, and OWASP security headers with strict Content Security Policy are enforced.
  - `test_ui_html_rendering_english_default`: Verifies `/` serves the Svelte 5 single-page application mount `<div id="app">`.
  - `test_ui_decommissioned_alias_routes_return_404`: Verifies unsupported UI alias routes (`/admin`, `/component-ui`, `/app`) return `404 NOT FOUND`.
  - `test_owasp_secure_headers_on_api_json_response`: Verifies OWASP security headers on API JSON responses.
  - `test_owasp_secure_headers_on_error_and_unauthorized_responses`: Verifies OWASP security headers on 422/404 error responses.
  - `test_static_assets_served_with_correct_content_types`: CSS and Vite assets served with proper MIME types; unmapped static asset paths return 404.
  - `test_static_assets_etag_cache_control_and_version_busting`: Validates runtime ETag generation, 304 Not Modified conditional responses on matching If-None-Match, immutable Cache-Control on version-tagged assets, and dynamic ?v= cache busting for app.js, app.css, and scalar.js.

- **Suite J: Historical Statutory Parameters & Revaluations (1990–2022)** ([`tests/integration_tests/suite_j_historical.rs`](tests/integration_tests/suite_j_historical.rs) — 12 tests):
  - `test_historical_calculation_1990_91_inaugural_nndr`: Validates inaugural non-domestic rating calculation under LGFA 1988 (1990/91) with zero SBRR discount.
  - `test_historical_transitional_phasing_1991_92`: Validates 1991/92 revaluation year 2 transitional upward cap (+15%) based on 1990/91 baseline.
  - `test_historical_transitional_phasing_1993_94`: Validates 1993/94 revaluation upward cap (+15%) under SI 1989/1358 based on 1992/93 baseline.
  - `test_historical_transitional_phasing_1995_list_revaluation`: Validates 1995/96 large property revaluation upward cap (+20%) under SI 1994/3279.
  - `test_historical_sbrr_pre_2005_returns_no_relief`: Validates 2003/04 sole-property small business occupier receives 0% discount (prior to LGA 2003 s.61).
  - `test_historical_sbrr_2005_list_england_full_and_taper`: Verifies 2007/08 full 50% discount and 25% linear taper under SI 2004/3315.
  - `test_historical_sbrr_2010_list_england_and_scotland`: Verifies 2012/13 doubled 100% taper under SI 2010/1959 and Scottish 3-tier SBBS under SSI 2010/37.
  - `test_historical_transitional_phasing_2000_list`: Validates 2000/01 revaluation upward cap (+12.5%) under SI 1999/3379.
  - `test_historical_welsh_transitional_relief_2017_list`: Validates 2017/18 Welsh small business transitional relief (75% phasing) under W.S.I. 2016/1247.
  - `test_historical_transitional_phasing_2017_list_revaluation`: Verifies 2017/18 large property revaluation upward cap (+42%) under SI 2016/1265.
  - `test_england_2017_list_transitional_relief_large_cap_2021_22_is_6_percent`: Validates 2021/22 and 2022/23 large property upward phasing cap of 6% under S.I. 2016/1265 Schedule Table 1 and S.I. 2022/339 reg 2.
  - `test_historical_sbrr_2010_11_england_pre_and_post_october`: Validates English SBRR in 2010/11: 50% max relief (S.I. 2004/3315 as amended by S.I. 2010/145) from 1 April to 30 September 2010, temporarily doubled to 100% from 1 October 2010 to 31 March 2011 under S.I. 2010/1659, and validates automatic billing segment slicing across the 1 October 2010 boundary.

---

## 5. Execution & Verification Guide

To build, verify, and run the project:

```bash
# 1. Build the Svelte 5 frontend with Vite
./build-frontend.sh

# 2. Run the integration test suite (161 tests)
cargo test --test integration_tests

# 3. Start the HTTP server (default port 8080)
cargo run --bin business_rates_engine

# 4. Execute a calculation via curl (Public REST API)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_1_sbrr_rhl_england.json

# 5. Execute with language content negotiation (e.g. Welsh, Scots, or Scottish Gaelic)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -H "Accept-Language: cy" \
  -d @tests/fixtures/scenario_1_sbrr_rhl_england.json

# 6. Retrieve statutory parameters
curl -s http://localhost:8080/api/v1/parameters

# 7. Access the web UI in your browser
open http://localhost:8080/

# 8. Access the Scalar API reference in your browser
open http://localhost:8080/docs

# 9. Access third-party software licenses in your browser
open http://localhost:8080/licenses
```
