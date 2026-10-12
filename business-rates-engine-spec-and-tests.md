# Business Rates Calculation Engine — Specification, Test Cases & Source Index

> **Target Environment:** Full-Stack Rust Workspace & Svelte 5 Component Framework  
> **Backend Architecture:** Axum 0.7 + Tokio 1.38 + Built-In Parameter Repository (Seeded 1990–2027)  
> **Frontend Architecture:** Svelte 5 (Runes `$state`, `$derived`) + Vite 6 + TypeScript 5.7 + Standalone Tailwind CSS (`frontend/`)  
> **API Access Mode:** Public REST API  
> **API Documentation:** Interactive Scalar API Reference (`/docs`) with Self-Hosted `scalar.js` (Zero CDN) & OpenAPI 3.1.0 (`/api/v1/openapi.json`)  
> **Internationalization (i18n):** Native 4-language support: English (`en`), Welsh (`cy`), Scots (`sco`), and Scottish Gaelic (`gd`) across UI, statutory narratives, relief names, error payloads, and reference codes  
> **Security Baseline:** OWASP-compliant HTTP security headers with strict Content Security Policy (zero `'unsafe-inline'` scripts, zero external CDNs)  
> **Coverage Period:** 1 April 1990 – 31 March 2027 (England, Wales, and Scotland)  

---

## 1. Statutory Source Index & Legislative References

The calculation engine's business logic, zero-hardcoding parameter tables, and statutory audit narrative rules are grounded in primary legislation acts, secondary statutory instruments (SIs / SSIs / WSIs), and executive guidance manuals across England, Wales, and Scotland:

### 1.1 Primary Legislation & Core Rating Acts
1. [**Local Government Finance Act 1988 (c. 41)**](https://www.legislation.gov.uk/ukpga/1988/41/contents) — Foundation of national non-domestic rating across England and Wales; sections 43 (occupied liability), 44A (partly occupied apportionments), 45 (unoccupied liability), 47 (discretionary relief), 57A (transitional relief power), and 64 (hereditament definition).
2. [**Non-Domestic Rating Act 2023 (c. 53)**](https://www.legislation.gov.uk/ukpga/2023/53/contents/enacted) — English 3-year revaluations, SBRR statutory amendments, Improvement Relief (s. 1 / Schedule 1 G-factor certificates), Heat Network exemptions (s. 2), high-value rating multipliers ($\ge £500,000$ RV), and empty rate reset rules (s. 15: 13-week occupation requirement).
3. [**Local Government Finance (Wales) Act 2024 (asc 6)**](https://www.legislation.gov.uk/asc/2024/6/contents/enacted) — Welsh 3-year revaluation synchronization, Welsh differential multipliers ($M \times N$), bilingual statutory audit trails, and anti-avoidance reforms.
4. [**Non-Domestic Rates (Scotland) Act 2020 (asp 4)**](https://www.legislation.gov.uk/asp/2020/4/contents) — Scottish 3-year revaluations, Valuation Roll transfer rules, and statutory three-tier multiplier system (Basic, Intermediate, Higher Property Rates).
5. [**Local Government (Financial Provisions) (Scotland) Act 1962 (c. 9, s. 4)**](https://www.legislation.gov.uk/ukpga/1962/9/section/4) — Mandatory 80% charitable relief and Scottish local authority discretionary rating relief powers.
6. [**Islands (Scotland) Act 2018 (asp 12)**](https://www.legislation.gov.uk/asp/2018/12/contents) — Statutory island definition conferring 100% Retail, Hospitality & Leisure relief for island commercial properties up to £110,000 statutory cash cap.
7. [**Rating (Coronavirus and Public Lavatories) Act 2021 (c. 12)**](https://www.legislation.gov.uk/ukpga/2021/12/contents) — 100% statutory rating exemption for standalone public conveniences in England and Wales.
8. [**Local Government and Rating Act 1997 (c. 29)**](https://www.legislation.gov.uk/ukpga/1997/29/contents) — Rural commercial settlement relief framework under LGFA 1988 s.43(6A)–(6E).

### 1.2 Statutory Instruments — Transitional Relief & Phasing Schemes
- **Scotland**:
  - [**The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2023 (S.S.I. 2023/31)**](https://www.legislation.gov.uk/ssi/2023/31/contents/made):
    - [**Part 2 (Regs 4–9)**](https://www.legislation.gov.uk/ssi/2023/31/part/2/made): Scottish General Revaluation Transitional Relief capping gross liability for Small ($\le £20\text{k}$), Medium ($\le £100\text{k}$), and Large ($> £100\text{k}$) properties across 2023/24 (+12.5%/+25%/+37.5%), 2024/25 (+25%/+50%/+75%), and 2025/26 (+37.5%/+75%/+112.5%).
    - [**Part 3 (Regs 10–14)**](https://www.legislation.gov.uk/ssi/2023/31/part/3/made): Small Business Bonus Scheme Transitional Relief (SBBS TR) capping net bill increases over the 31 March 2023 baseline bill to £600 in 2023/24, £1,200 in 2024/25, and £1,800 in 2025/26.
    - [**Part 4 (Regs 15–18)**](https://www.legislation.gov.uk/ssi/2023/31/part/4/made): Parks Transitional Relief.
  - [**The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2026**](https://www.legislation.gov.uk/ssi):
    - 2026 Revaluation List General Transitional Relief capping gross liability increases for Small (+12.5%), Medium (+25%), and Large (+37.5%) properties.
    - Part 3: 2026 Small Business Bonus Scheme Transitional Relief (SBBS TR) limiting the ratepayer's liability on net bill escalation to 25% in 2026/27 (relief deducts 75% of increase), stepping up to 50% in 2027/28 and 75% in 2028/29.
- **England**:
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2026**](https://www.legislation.gov.uk/uksi) — 2026 Revaluation List upward transitional phasing caps: Small (+5%), Medium (+15%), Large (+30%) under LGFA 1988 s.57A / NDRA 2023.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2022 (S.I. 2022/1263)**](https://www.legislation.gov.uk/uksi/2022/1263/contents/made) — 2023 Revaluation List upward phasing caps: Small (5%/15%/25%), Medium (15%/25%/40%), and Large (30%/40%/55%).
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2016 (S.I. 2016/1265)**](https://www.legislation.gov.uk/uksi/2016/1265/contents/made) — 2017 Revaluation List upward and downward transitional phasing percentages.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2009 (S.I. 2009/3178)**](https://www.legislation.gov.uk/uksi/2009/3178/contents/made) — 2010 Revaluation List transitional phasing caps.
  - [**The Non-Domestic Rating (Chargeable Amounts) (England) Regulations 2004 (S.I. 2004/3312)**](https://www.legislation.gov.uk/uksi/2004/3312/contents/made) — 2005 Revaluation List transitional phasing caps.
  - [**The Non-Domestic Rating (Chargeable Amounts) Regulations 1994 (S.I. 1994/3279)**](https://www.legislation.gov.uk/uksi/1994/3279/contents/made) — 1995 Revaluation List transitional phasing caps.
  - [**The Non-Domestic Rating (Transitional Period) Regulations 1990 (S.I. 1990/608)**](https://www.legislation.gov.uk/uksi/1990/608/contents/made) & [**The Non-Domestic Rating (Transitional Period) (Amendment) Regulations 1993 (S.I. 1993/616)**](https://www.legislation.gov.uk/uksi/1993/616/contents/made) — Inaugural national non-domestic rating transitional phasing caps.
- **Wales**:
  - [**The Non-Domestic Rating (Chargeable Amounts) (Wales) Regulations 2025 (W.S.I. 2025/1371 / W. 226)**](https://www.legislation.gov.uk/wsi/2025/1371/contents/made) — 2026 Revaluation List 2-Year Transitional Relief Scheme: applies to net bill increases exceeding **£300** over the 31 March 2026 baseline bill; grants **67% relief** on the excess in Year 1 (2026/27, ratepayer pays 33%), and **34% relief** on the excess in Year 2 (2027/28, ratepayer pays 66%).
  - [**The Non-Domestic Rating (Chargeable Amounts) (Wales) Regulations 2022 (W.S.I. 2022/1350 / W. 272)**](https://www.legislation.gov.uk/wsi/2022/1350/contents/made) — 2023 Revaluation List 2-Year Transitional Relief Scheme: 67% relief in 2023/24 and 34% relief in 2024/25 on net bill increases exceeding £300.

### 1.3 Statutory Instruments — Small Business Rate Relief (SBRR & SBBS) & Multiplier Frameworks
- **Scotland (SBBS)**:
  - [**The Non-Domestic Rates (Small Business Bonus Scheme) (Scotland) Regulations 2023 (S.S.I. 2023/34)**](https://www.legislation.gov.uk/ssi/2023/34/contents/made) — 2023 List: 100% relief for $\text{RV} \le £12,000$; tapered relief for $£12,001 \le \text{RV} \le £20,000$.
  - [**The Non-Domestic Rates (Small Business Bonus Scheme) (Scotland) Regulations 2017 (S.S.I. 2017/36)**](https://www.legislation.gov.uk/ssi/2017/36/contents/made) — 2017 List: 100% relief for $\text{RV} \le £15,000$; 25% for $£15,001 \le \text{RV} \le £18,000$.
  - [**The Non-Domestic Rates (Small Business Bonus Scheme) (Scotland) Regulations 2010 (S.S.I. 2010/37)**](https://www.legislation.gov.uk/ssi/2010/37/contents/made) — 2010 List 3-tier SBBS framework.
  - [**The Non-Domestic Rates (Small Business Bonus Scheme) (Scotland) Regulations 2008 (S.S.I. 2008/106)**](https://www.legislation.gov.uk/ssi/2008/106/contents/made) — Inaugural SBBS regulations.
- **England (SBRR, 2026/27 5-Multiplier Regime & SSBR)**:
  - [**The Non-Domestic Rating (Small Business Rate Relief) (England) Order 2017 (S.I. 2017/411)**](https://www.legislation.gov.uk/uksi/2017/411/contents/made) — 100% relief for $\text{RV} \le £12,000$; linear taper between £12,001 and £15,000 RV.
  - [**S.I. 2017/411 art. 3(4)**](https://www.legislation.gov.uk/uksi/2017/411/article/3/made) & [**S.I. 2014/43**](https://www.legislation.gov.uk/uksi/2014/43/contents/made) under LGFA 1988 s.43(4B) — Statutory **12-month grace period** allowing a ratepayer who takes on an additional hereditament to retain full SBRR on their primary hereditament for 12 months.
  - [**Autumn Budget 2024 / Non-Domestic Rating Act 2023 Multiplier Framework**](https://www.gov.uk/government/publications/autumn-budget-2024) — Permanent 5-multiplier rating system effective 1 April 2026 replacing temporary RHL relief:
    - Small Business RHL: `38.2p` ($\text{RV} < £51,000$ for qualifying retail, hospitality, leisure)
    - Small Business non-RHL: `43.2p` ($\text{RV} < £51,000$ standard small business rate)
    - Standard RHL: `43.0p` ($£51,000 \le \text{RV} < £500,000$ qualifying retail, hospitality, leisure)
    - Standard non-RHL: `48.0p` ($£51,000 \le \text{RV} < £500,000$ standard national rate)
    - High-Value: `50.8p` ($\text{RV} \ge £500,000$ premium rate)
  - [**Supporting Small Business Relief Scheme (SSBR) under LGFA 1988 s.47**](https://www.gov.uk/government/publications/business-rates-supporting-small-business-relief-guidance) — Statutory discretionary relief capping annual bill increases to **£600/year** (2023/24–2025/26) and **£800/year** (2026/27) for ratepayers losing or experiencing reduced SBRR, Rural Rate Relief, or 2022/23 RHL relief due to revaluation.
  - [**The Non-Domestic Rating (Small Business Rate Relief) (England) Order 2012 (S.I. 2012/148)**](https://www.legislation.gov.uk/uksi/2012/148/contents/made) & [**S.I. 2010/1959**](https://www.legislation.gov.uk/uksi/2010/1959/contents/made) — 2010 list SBRR doubled taper.
  - [**The Non-Domestic Rating (Small Business Rate Relief) (England) Order 2004 (S.I. 2004/3315)**](https://www.legislation.gov.uk/uksi/2004/3315/contents/made) — Inaugural SBRR order establishing 50% max relief and linear taper.
- **Wales (SBRR, 2026/27 3-Tier Multipliers & Food and Drink Hospitality Rates Relief)**:
  - [**The Non-Domestic Rating (Small Business Relief) (Wales) Order 2017 (W.S.I. 2017/1229 / W. 293)**](https://www.legislation.gov.uk/wsi/2017/1229/contents/made) — 100% relief for $\text{RV} \le £6,000$; linear taper between £6,001 and £12,000 RV.
  - [**Local Government Finance (Wales) Act 2024 (asc 6)**](https://www.legislation.gov.uk/asc/2024/6/contents/enacted) — 2026/27 3-Tier Multiplier Framework replacing the historical single multiplier:
    - **Retail Multiplier**: `35.0p` (`0.350`) for retail premises with $\text{RV} < £51,000$.
    - **Standard Multiplier**: `50.2p` (`0.502`) for commercial premises with $\text{RV} \le £100,000$ (non-retail or general commercial).
    - **Higher Multiplier**: `51.5p` (`0.515`) for high-value properties with $\text{RV} > £100,000$.
  - [**Welsh Food and Drink Hospitality Rates Relief Scheme (2026/27)**](https://www.gov.wales/non-domestic-rates-relief) under LGFA 1988 s.47 — Successor to Welsh RHL relief providing a **15% discount** capped at £110,000 per business; targeted specifically at qualifying food and drink hospitality premises (pubs, restaurants, cafés, bars, live music venues); excludes petrol filling stations (SCAT 211) and general accommodation.
  - [**The Non-Domestic Rating (Small Business Relief) (Wales) Order 2015 (W.S.I. 2015/976 / W. 69)**](https://www.legislation.gov.uk/wsi/2015/976/contents/made), [**W.S.I. 2010/248 (W. 34)**](https://www.legislation.gov.uk/wsi/2010/248/contents/made), and [**W.S.I. 2008/707 (W. 75)**](https://www.legislation.gov.uk/wsi/2008/707/contents/made).

### 1.4 Statutory Instruments — Relief Exemptions, Unoccupied Rates & Compliance
- **Empty Property & Unoccupied Rating**:
  - [**The Non-Domestic Rating (Unoccupied Property) (England) Regulations 2008 (S.I. 2008/386)**](https://www.legislation.gov.uk/uksi/2008/386/contents/made) — 3-month commercial / 6-month industrial rate-free periods; low value exemption below £2,900 RV.
  - [**The Non-Domestic Rating (Unoccupied Property) (Wales) Regulations 2008 (W.S.I. 2008/2499 / W. 217)**](https://www.legislation.gov.uk/wsi/2008/2499/contents/made) — Welsh unoccupied property exemptions and reset rules.
  - [**The Non-Domestic Rating (Unoccupied Property) (Scotland) Regulations 2018 (S.S.I. 2018/77)**](https://www.legislation.gov.uk/ssi/2018/77/contents/made) — Scottish unoccupied property relief rules.
- **Improvement Relief (G-Factor Certificates)**:
  - [**The Non-Domestic Rating (Improvement Relief) (England) Regulations 2024 (S.I. 2024/22)**](https://www.legislation.gov.uk/uksi/2024/22/contents/made) — 12-month qualifying improvement relief under NDRA 2023 s. 1.
  - [**The Non-Domestic Rating (Improvement Relief) (Wales) Regulations 2024 (W.S.I. 2024/302 / W. 57)**](https://www.legislation.gov.uk/wsi/2024/302/contents/made) — Welsh improvement relief.
- **Billing, Appeals & Anti-Avoidance**:
  - [**The Non-Domestic Rating (Collection and Enforcement) (Local Lists) Regulations 1989 (S.I. 1989/1058)**](https://www.legislation.gov.uk/uksi/1989/1058/contents/made) & [**Schedule 1**](https://www.legislation.gov.uk/uksi/1989/1058/schedule/1/made) — Statutory 10-installment billing payment schedules, adjustment rules, and mid-year cessation refunds.
  - [**The Non-Domestic Rating (Alteration of Lists and Appeals) (Wales) Regulations 2023 (W.S.I. 2023/1162)**](https://www.legislation.gov.uk/wsi/2023/1162/contents/made) — Welsh Check, Challenge, Appeal (CCA) procedures and Valuation Tribunal for Wales appeals.
  - [**The Non-Domestic Rating (Artificial Avoidance Arrangements) (Local Lists) (Wales) Regulations 2026 (W.S.I. 2026/43)**](https://www.legislation.gov.uk/wsi/2026/43/contents/made) — Welsh artificial avoidance countermeasures; WSI 2026/43 civil financial penalties (£500 flat + 3% RV).
  - [**The Council Tax (Dwellings and Part Residential Subjects) (Scotland) Amendment Regulations 2026 (S.S.I. 2026/50)**](https://www.legislation.gov.uk/ssi/2026/50/contents/made) — Scottish self-catering 182-day actual letting test (and 140 days available to let) and Valuation Roll transfers.

### 1.5 Valuation Office Agency (VOA) Manuals & Policy Guidance
1. [**Common Legislative Solutions: A Guide to Tackling Recurring Policy Issues**](https://www.gov.uk/government/publications/common-legislative-solutions) (Guidance) — Statutory drafting guidelines, licensing frameworks, and administrative powers.
2. [**Rating Manual Section 5a: Airports Guidance**](https://www.gov.uk/guidance/rating-manual-section-5a-valuation-of-all-property-classes/airports) (VOA Manual) — SCAT 003 Airport valuation rules; Receipts & Expenditure for major hubs vs Contractor's Basis for airfields.
3. [**Rating Manual Section 2: Domestic & Non-Domestic Borderline**](https://www.gov.uk/guidance/rating-manual-section-2-valuation-principles/domestic-and-non-domestic-borderline) (VOA Manual) — Composite hereditament rules, private garages, and holiday accommodation classification criteria.
4. [**Rating Manual Section 5a: Petrol Filling Stations Guidance**](https://www.gov.uk/guidance/rating-manual-section-5a-valuation-of-all-property-classes/petrol-filling-stations) (VOA Manual) — SCAT 211 valuation methods; fuel throughput, forecourt shop turnover, valeting, and Fair Maintainable Trade.
5. [**Rating Manual Section 4: 2023 Rental Adjustment Practice Note**](https://www.gov.uk/guidance/rating-manual-section-4-valuation-principles/2023-rental-adjustment-practice-note) (VOA Manual) — Virtual rent conversions, lease incentives, VAT adjustments, rent-free period capitalisation, and yields.

---

## 2. Software Requirements Specification (SRS)

### 2.1 Architecture Overview

The system operates with a decoupled modern architecture:
- **Interactive Component-Based UI (`frontend/`)**: A modular Svelte 5 Single-Page Application (SPA) compiled with Vite 6 and TypeScript 5.7 into static assets served directly by the backend at `/`. Includes embedded statutory parameter explorers and top navbar link to API reference.
- **Interactive Documentation Plane (`/docs`, `/api/v1/openapi.json`)**: Self-hosted Scalar API reference rendered via `templates/scalar.html` with zero external CDN dependencies, dynamic query parameter templating, and official OpenAPI 3.1.0 specifications.
- **Statutory Data Plane (REST API)**: High-performance, unauthenticated Axum endpoints executing sub-millisecond statutory rating calculations, cataloging statutory reference data, and serving parameter repository tables.

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
                                    │ Unauthenticated HTTP / JSON
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     Axum 0.7 REST API Server                           │
│                                                                        │
│   GET  /                                -> Svelte 5 SPA index.html     │
│   GET  /docs                            -> Scalar API Reference (HTML) │
│   GET  /assets/:filename                -> Vite Assets / scalar.js     │
│   POST /api/v1/calculate-liability      -> Statutory Calculation API   │
│   GET  /api/v1/reference-codes          -> Statutory Reference Codes   │
│   GET  /api/v1/parameters               -> Aggregated Parameter Repo   │
│   GET  /api/v1/parameters/multipliers   -> Rating Multipliers (93-27)  │
│   GET  /api/v1/parameters/sbrr-thresholds                              │
│   GET  /api/v1/parameters/sector-reliefs                               │
│   GET  /api/v1/parameters/transitional-caps                            │
│   GET  /api/v1/openapi.json             -> OpenAPI 3.1.0 Spec (JSON)   │
│   GET  /api/v1/openapi.yaml             -> OpenAPI 3.1.0 Spec (YAML)   │
│                                                                        │
│   OWASP Security Headers Middleware (Strict CSP, Zero 'unsafe-inline') │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│              Statutory Calculation Engine & Built-In Repo              │
│                                                                        │
│   - CalculationEngine (Statutory Slicer, Multi-Occupancy, Narratives)  │
│   - ParameterRepository (Built-in In-Memory Multipliers 1993–2027,     │
│     SBRR/SBBS Tapers, Devolved Sector Reliefs, Caps & Reference Codes) │
└────────────────────────────────────────────────────────────────────────┘
```

#### Key Technical Decisions & Choices:
1. **Svelte 5 Component UI Architecture**:
   - Modern component-driven architecture built with Svelte 5 Runes (`$state`, `$derived`).
   - Clean, modular component decomposition for reactive form state, multi-episode tables, live audit narrative preview, parameter catalog accordions, and bidirectional Form-to-JSON editing.
   - Built to production dist via `./build-frontend.sh` (`frontend/dist/`).
2. **Public REST API Access**:
   - All REST APIs and calculation endpoints are directly accessible as a public service with zero authentication overhead.
3. **4-Language Internationalization (i18n)**:
   - Full support for **English (`en`)**, **Welsh (`cy`)**, **Scots (`sco`)**, and **Scottish Gaelic (`gd`)**.
   - Language is negotiated dynamically based on: Request body `"language"` > Query parameter `?lang=` > `Accept-Language` header > Default `en`.
4. **Decoupled Calculation Contract**:
   - Informational metadata fields (`liable_party_id`, `uarn`, street addresses) are decoupled from the statutory math core.
5. **Multi-Occupancy Section 44A Support**:
   - Liable parties can submit multiple occupancy periods (`OCCUPIED`, `UNOCCUPIED`, `PART_OCCUPIED`) in a single calculation request.
   - Automatically handles statutory occupation reset rules ($\ge 91$ days in England post-April 2024; $\ge 42$ days in Wales/Scotland) and Section 44A dual-portion apportionments.
6. **OpenAPI 3.1.0 & Self-Hosted Scalar Reference**:
   - Official OpenAPI 3.1.0 specification stored in [`docs/openapi.yaml`](docs/openapi.yaml) and [`docs/openapi.json`](docs/openapi.json), served strictly at `/api/v1/openapi.json` and `/api/v1/openapi.yaml`.
   - Built-in interactive documentation served at `/docs` rendered via [`templates/scalar.html`](templates/scalar.html) using self-hosted `/assets/scalar.js`. Operates 100% offline with zero external CDN dependencies and supports dynamic query parameter templating (`title`, `spec_url`, `theme`, `layout`).
7. **Unified Single REST API Routes**:
   - Standardized single-namespace REST endpoints directly under `/api/v1/` (`/calculate-liability`, `/reference-codes`, `/parameters/*`, `/openapi.*`).

---

### 2.2 OWASP Secure Headers Specification

All HTTP responses (HTML web pages, API JSON responses, static assets, and error handlers) are protected by centralized security middleware enforcing OWASP recommended headers:

| Header | Production Value | Purpose |
| :--- | :--- | :--- |
| `Content-Security-Policy` | `default-src 'self'; script-src 'self'; style-src 'self'; font-src 'self' data:; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; form-action 'self'; base-uri 'self';` | Strict XSS, script injection, and clickjacking prevention. Zero `'unsafe-inline'` script compliance; zero external CDN dependencies. |
| `X-Content-Type-Options` | `nosniff` | Prevents MIME-type sniffing. |
| `X-Frame-Options` | `DENY` | Eliminates clickjacking by disallowing framing. |
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Enforces secure transport. |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Protects URI query data on navigation. |
| `Permissions-Policy` | `camera=(), microphone=(), geolocation=(), payment=()` | Restricts privileged browser hardware APIs. |
| `Cross-Origin-Opener-Policy` | `same-origin` | Isolates browsing contexts against Spectre attacks. |
| `Cross-Origin-Resource-Policy` | `same-origin` | Restricts cross-origin resource sharing. |
| `X-Permitted-Cross-Domain-Policies` | `none` | Blocks Flash/PDF policy leaks. |

> **Route-Scoped Style CSP Policy:** The `/docs` and `/scalar` reference routes scope `style-src 'self' 'unsafe-inline'` exclusively to accommodate Scalar's dynamic CSS variable theme injection, while strictly retaining `script-src 'self'` (zero `'unsafe-inline'` scripts and zero CDN origins).

---

### 2.3 Internationalization (i18n) & Language Resolution

The engine implements compile-time verified multi-lingual support via [`src/i18n.rs`](src/i18n.rs) and the frontend TypeScript dictionary store [`frontend/src/i18n/`](frontend/src/i18n/):

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

> **Single Canonical Calculation Route:** All liability calculations are handled exclusively by `POST /api/v1/calculate-liability`.

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

> **Statutory Slicing & Reset Mechanics:**
> - **Section 43 (Occupied)**: Standard liability with SBRR, RHL, charitable relief, and G-factor improvement relief.
> - **Section 45 (Unoccupied)**: Initial 3-month (commercial) or 6-month (industrial) empty property rate-free exemption.
> - **Statutory Occupation Reset Rule**: Under Non-Domestic Rating Act 2023 s.15, consecutive occupation of $\ge 13$ weeks (91 days) in England post-1 April 2024 (or $\ge 6$ weeks / 42 days in Wales/Scotland or pre-2024) resets the empty rate-free period to 0.
> - **Section 44A (Part-Occupied)**: Apportions rateable value between occupied portion and unoccupied portion. The unoccupied portion receives Section 45 empty relief, while the occupied portion receives occupied reliefs (e.g. SBRR).
> - **Backward Compatibility**: Requests with a single `occupation_period` and `occupancy_type` remain 100% supported via automatic internal mapping.

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

The calculation engine operates with a self-contained, in-memory **Built-In Statutory Parameter Repository** (`ParameterRepository` in `src/db/repository.rs`). This architecture provides:

- **Zero External Infrastructure**: Operates independently with zero database services, connection strings, or migration scripts.
- **Deterministic Sub-Millisecond Latency**: Microsecond in-memory indexing and lookups with zero network overhead.
- **Statutory Parameter Immutability**: Exact legal parameters pre-seeded from primary acts and secondary regulations (1990–2027) across England, Wales, and Scotland.
- **Turnkey Portability**: The single compiled release binary or OCI container (Podman / Docker) runs anywhere out of the box.

### 3.1 Repository Structure & Seeded Models

The built-in repository encapsulates 5 core statutory parameter domains:

```rust
pub struct ParameterRepository {
    multipliers: Vec<ParamMultiplier>,
    sbrr_sbbs_thresholds: Vec<ParamSbrrSbbsThreshold>,
    sector_reliefs: Vec<ParamSectorRelief>,
    transitional_caps: Vec<ParamTransitionalCap>,
    reference_codes: Vec<ParamReferenceCode>,
}
```

#### 1. Multipliers (`ParamMultiplier`)
- **Coverage**: Complete temporal coverage from **1 April 1990 to 31 March 2027**.
- **Devolved Disaggregation**:
  - **England**: Standard Multiplier, Small Business Multiplier, City of London Standard, City of London Small Business.
  - **Wales**: National Welsh Non-Domestic Multiplier.
  - **Scotland**: Basic Property Rate, Intermediate Property Rate, Higher Property Rate.

#### 2. Small Business Relief Thresholds (`ParamSbrrSbbsThreshold`)
- **England (SBRR)**: 100% relief for $\text{RV} \le £12,000$; linear taper from 100% to 0% for $£12,001 \le \text{RV} \le £15,000$.
- **Wales (Devolved SBRR)**: 100% relief for $\text{RV} \le £6,000$; linear taper from 100% to 0% for $£6,001 \le \text{RV} \le £12,000$.
- **Scotland (SBBS)**: 100% relief for $\text{RV} \le £12,000$; linear taper from 100% to 25% for $£12,001 \le \text{RV} \le £15,000$, and 25% to 0% up to $£20,000$.

#### 3. Sector Relief Schemes (`ParamSectorRelief`)
- **Retail, Hospitality & Leisure (RHL) / Food & Drink Hospitality**:
  - **England**:
    - 2023/24 & 2024/25: 75% discount (statutory cash cap of £110,000 per business).
    - 2025/26: 40% discount (statutory cash cap of £110,000).
    - 2026/27: Discretionary percentage relief ceased on 31 March 2026 (0% discount). Replaced by permanent lower rating multipliers (38.2p small / 43.0p standard) for high street retail, hospitality, and leisure properties with $\text{RV} < £500,000$ from 1 April 2026 under the Non-Domestic Rating Act 2023.
  - **Wales**:
    - 2023/24: 75% discount (statutory cash cap of £110,000).
    - 2024/25 & 2025/26: 40% discount (statutory cash cap of £110,000; petrol filling stations SCAT 211 excluded).
    - 2026/27: 15% targeted Food & Drink Hospitality scheme (pubs, restaurants, cafés, music venues; statutory cash cap of £110,000; petrol filling stations SCAT 211 excluded), bridging to permanent lower multipliers in 2027/28 under the *Local Government Finance (Wales) Act 2024*.
  - **Scotland**:
    - 2026/27: 15% Retail, Hospitality & Leisure relief on mainland for properties with $\text{RV} \le £100,000$ (statutory cash cap of £110,000 per ratepayer under *Local Government (Financial Provisions) (Scotland) Act 1962 s.4*); 100% relief for island properties under the [*Islands (Scotland) Act 2018*](https://www.legislation.gov.uk/asp/2018/12/contents) (statutory cash cap of £110,000). Properties on the mainland with $\text{RV} > £100,000$ are not eligible.

#### 4. Transitional Phasing Caps (`ParamTransitionalCap` & `SbbsTrResolution`)
- **England 2023 Revaluation List (2023/24, 2024/25, 2025/26)**:
  - Small Hereditament ($\text{RV} \le £20,000$ / London $£28,000$): Upward caps of 5%, 15%, 25%.
  - Medium Hereditament ($\text{RV} \le £100,000$): Upward caps of 15%, 25%, 40%.
  - Large Hereditament ($\text{RV} > £100,000$): Upward caps of 30%, 40%, 55%.
- **Scotland General Revaluation Transitional Relief** ([The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2023 - S.S.I. 2023/31 Regs 4–9](https://www.legislation.gov.uk/ssi/2023/31/contents/made) & *The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2026*):
  - Statutory upward phasing caps on gross liability escalation over baseline bill:
    - Small Hereditament ($\text{RV} \le £20,000$): Upward caps of 12.5% (2023/24), 25% (2024/25), 37.5% (2025/26), and 12.5% (2026/27).
    - Medium Hereditament ($\text{RV} \le £100,000$): Upward caps of 25% (2023/24), 50% (2024/25), 75% (2025/26), and 25% (2026/27).
    - Large Hereditament ($\text{RV} > £100,000$): Upward caps of 37.5% (2023/24), 75% (2024/25), 112.5% (2025/26), and 37.5% (2026/27).
- **Scotland Small Business Bonus Scheme Transitional Relief (SBBS TR)** ([The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2023 Part 3 Regs 10–14](https://www.legislation.gov.uk/ssi/2023/31/part/3/made) & *The Non-Domestic Rates (Transitional Relief) (Scotland) Regulations 2026 Part 3*):
  - Protects ratepayers who lost or experienced reduced Small Business Bonus Scheme (SBBS) entitlement following rating list revaluations:
    - **2023 List (2023/24, 2024/25, 2025/26)**: Fixed cash caps on net bill increase relative to the previous baseline bill (£600 in 2023/24, £1,200 in 2024/25, £1,800 in 2025/26).
    - **2026 List (2026/27, 2027/28, 2028/29)**: Percentage-of-increase cap limiting the ratepayer's liability on net bill escalation to **25%** in 2026/27 (relief deducts 75% of the increase), stepping up to 50% in 2027/28 and 75% in 2028/29.
- **2017 Revaluation List**: Historical upward and downward statutory phasing percentages.

#### 5. Statutory Reference Codes (`ParamReferenceCode`)
- Pre-seeded dictionaries for `JURISDICTION`, `OCCUPATION_STATUS`, `SCAT_CODE`, `BILLING_FREQUENCY`, `RELIEF_TYPE`, and `LANGUAGE` with localized titles and descriptions across English (`en`), Welsh (`cy`), Scots (`sco`), and Scottish Gaelic (`gd`).

### 3.2 Lookup Methods

The repository exposes synchronous, zero-copy query methods:

```rust
impl ParameterRepository {
    pub fn resolve_multiplier(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> MultiplierInfo;
    pub fn calculate_sbrr_sbbs_discount(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> Decimal;
    pub fn get_sector_relief(&self, jurisdiction: Jurisdiction, financial_year: &str, relief_code: &str) -> Option<&ParamSectorRelief>;
    pub fn resolve_transitional_cap(&self, jurisdiction: Jurisdiction, financial_year: &str, rv: Decimal) -> Option<&ParamTransitionalCap>;
    pub fn get_reference_codes(&self, category: &str) -> Vec<&ParamReferenceCode>;
}
```

### 3.3 Statutory Parameter REST Endpoints & OpenAPI Documentation

The in-memory parameter repository and statutory documentation are exposed directly via dedicated REST endpoints:
- `GET /api/v1/parameters`: Aggregated JSON payload containing all temporal multipliers, SBRR/SBBS thresholds, sector reliefs, and transitional caps.
- `GET /api/v1/parameters/multipliers`: Temporal rating multipliers array (1990–2027) disaggregated by jurisdiction.
- `GET /api/v1/parameters/sbrr-thresholds`: SBRR / SBBS rateable value thresholds and statutory linear relief rules.
- `GET /api/v1/parameters/sector-reliefs`: Devolved retail, hospitality, leisure, and special relief schemes.
- `GET /api/v1/parameters/transitional-caps`: Statutory revaluation transitional phasing percentage caps.
- `GET /api/v1/openapi.json` & `GET /api/v1/openapi.yaml`: Official OpenAPI 3.1.0 specifications documenting all statutory models, calculation endpoints, schemas, and parameter filters.
- `GET /docs`: Interactive self-hosted Scalar API reference UI rendered from `templates/scalar.html` with zero external CDN dependencies, strict script CSP, and dynamic query parameter templating (`?title=...&spec_url=...&theme=...&layout=...`).

---

## 4. Test Cases Specification & Scenario Fixtures

### Test Suite Overview (59 Passing Integration Tests)

The test suite in [`tests/integration_tests.rs`](tests/integration_tests.rs) rigorously validates statutory calculations, multi-occupancy slicing, i18n, parameter endpoints, Scalar documentation, and web security:

- **Suite A: Multi-Jurisdiction Scenario Tests**:
  - `test_scenario_1_england_sbrr_and_rhl_multi_year`: Multi-year slicing (2025/26 & 2026/27), SBRR linear taper, England 2026/27 RHL expiration verification, and Transitional Relief caps.
  - `test_scenario_2_wales_petrol_station`: SCAT 211 forecourt valuation under Welsh Local Government Finance Act 2024.
  - `test_scenario_3_scotland_self_catering`: Scottish SBBS £12k threshold and 182-day actual letting compliance.
  - `test_scenario_4_welsh_artificial_avoidance`: Statutory avoidance penalty triggered by `is_artificial_avoidance_suspected` flag (£500 + 3% RV penalty under WSI 2026/43).
  - `test_scenario_5_airport_high_value`: High-value hereditament (RV £1.25m) using English standard national multiplier.

- **Suite B: Multi-Occupancy & Core Statutory Rules**:
  - `test_multi_occupancy_periods_single_liable_party`: 3 episodes across 2024/25 (`UNOCCUPIED` → `OCCUPIED` → `PART_OCCUPIED`), Section 44A dual-portion apportionments, and statutory reset triggers.
  - `test_multi_occupancy_api_endpoint_json_serialization`: JSON payload serialization and billing segment output for multi-occupancy arrays.
  - `test_mandatory_charitable_relief_80_percent`: LGFA 1988 s.43(5) mandatory 80% charity deduction.
  - `test_ndra_2023_improvement_relief_g_factor`: Non-Domestic Rating Act 2023 s.1 G-factor certificate reduction.
  - `test_suite_7_transitional_phasing_relief_cap`: Chargeable amounts ceiling capping year-on-year bill escalation.
  - `test_suite_7_transitional_relief_none_when_under_cap`: Zero transitional adjustment when calculated liability remains under cap.
  - `test_suite_8_empty_property_exemption`: 100% exemption for initial statutory period and "box-shifting" reset rules.
  - `test_open_ended_occupation_empty_string_and_null`: Open-ended occupation handling (`""`, `null`, omitted `end_date`) defaulting cleanly to 2027-03-31.

- **Suite C: 4-Language Internationalization (i18n) & Content Negotiation**:
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
  - `test_domain_error_status_codes_and_codes`: Verifies strongly-typed `AppError` status codes and error identifiers (`InvalidOccupationPeriod` -> 400 `INVALID_OCCUPATION_PERIOD`, `PayloadDeserialization` -> 422 `PAYLOAD_DESERIALIZATION_ERROR`).

- **Suite D: Reference Codes, Parameter Resolution & Public API Access**:
  - `test_app_state_repository_access`: Validates thread-safe parameter repository access on `AppState`.
  - `test_zero_hardcoding_historical_multipliers_1990_to_2026`: Complete verification of English, Welsh, and Scottish rating multipliers from 1990 to 2026.
  - `test_preseeded_reference_codes_and_allowable_values`: Pre-seeded values for jurisdictions, party types, and reliefs.
  - `test_http_reference_codes_endpoints`: Validates public `GET /api/v1/reference-codes` and verifies legacy prefix path `/api/v1/business-rates/reference-codes` returns `404 NOT FOUND`.
  - `test_http_calculate_liability_unauthenticated_access`: Verifies calculation succeeds on `POST /api/v1/calculate-liability` without requiring authorization headers, and confirms legacy path `/api/v1/business-rates/calculate-liability` returns `404 NOT FOUND`.
  - `test_http_sso_endpoints`: Verifies removed SSO endpoints (`/api/v1/auth/sso/login`, `/api/v1/auth/sso/callback`) return `404 NOT FOUND`.
  - `test_http_parameters_endpoints`: Validates all 5 parameter catalog endpoints (`/api/v1/parameters`, `/api/v1/parameters/multipliers`, `/api/v1/parameters/sbrr-thresholds`, `/api/v1/parameters/sector-reliefs`, `/api/v1/parameters/transitional-caps`) return valid HTTP 200 JSON arrays.
  - `test_openapi_and_swagger_endpoints`: Validates OpenAPI 3.1.0 specifications (`/api/v1/openapi.json`, `/api/v1/openapi.yaml`), `/docs` Scalar HTML rendering with query parameter templating (`title`, `spec_url`, `theme`, `layout`), self-hosted zero-CDN `scalar.js`, and confirms unsupported documentation aliases (`/scalar`, `/api-reference`, `/swagger`, `/swagger-ui`) return `404 NOT FOUND`.

- **Suite E: Statutory Relief Extensions & Devolved Sector Schemes**:
  - `test_statutory_heat_network_exemption`: NDRA 2023 s.2 100% heat network discount.
  - `test_statutory_public_lavatory_exemption`: Rating (Public Lavatories) Act 2021 100% discount.
  - `test_statutory_rural_settlement_relief`: LGFA 1988 s.43(6B) rural commercial relief.
  - `test_statutory_sbrr_england_linear_taper`: Exact mathematical formula verification for English £12k–£15k SBRR linear taper.
  - `test_statutory_empty_low_value_exemption`: SI 2008/386 low value exemption for RV < £2,900.
  - `test_statutory_empty_property_prior_unoccupied_days`: Calculation accounting for prior unoccupied days deducted from statutory allowance.
  - `test_sector_relief_england_2025_vs_2026`: Verifies English RHL 40% relief in 2025/26 and statutory expiration (0% percentage relief) in 2026/27 under NDRA 2023.
  - `test_sector_relief_scotland_mainland_vs_island_2026`: Verifies Scottish 15% mainland relief (RV $\le £100,000$), 100% island relief under *Islands (Scotland) Act 2018*, and 0% for mainland RV $> £100,000$.
  - `test_sector_relief_wales_2026_food_drink_hospitality`: Verifies Welsh 15% Food & Drink Hospitality relief and statutory exclusion of petrol stations (SCAT 211).
  - `test_scottish_general_transitional_relief_2023_list`: Verifies Scottish General Revaluation Transitional Relief (+12.5% upward cap in 2023/24 for Small property $\text{RV} \le £20,000$ under SSI 2023/31).
  - `test_scottish_sbbs_tr_2023_list_600_cap`: Verifies Scottish SBBS Transitional Relief under SSI 2023/31 Part 3, capping net bill increase over 2022/23 to £600 for a property losing full SBBS.
  - `test_scottish_sbbs_tr_2026_list_25_percent_cap`: Verifies 2026 Scottish SBBS Transitional Relief limiting liability on net bill escalation to 25% (relief deducts 75%) alongside General Transitional Relief.
  - `test_england_sbrr_second_property_grace_period`: Verifies statutory 12-month grace period on primary hereditament when taking on an additional hereditament (*S.I. 2017/411 art. 3(4)* under LGFA 1988 s.43(4B)).
  - `test_england_2026_five_multipliers_rhl_vs_non_rhl`: Verifies Autumn Budget 2024 / NDRA 2023 five-multiplier regime in England for 2026/27 (Small Business RHL 38.2p, Small Business non-RHL 43.2p, Standard RHL 43.0p, Standard non-RHL 48.0p, and High-Value 50.8p).
  - `test_england_supporting_small_business_relief_2023_cap_600`: Verifies English Supporting Small Business Relief (SSBR) under LGFA 1988 s.47 capping net bill increase to £600/year in 2023/24 for ratepayers losing SBRR.
  - `test_england_supporting_small_business_relief_2026_cap_800`: Verifies English Supporting Small Business Relief (SSBR) under LGFA 1988 s.47 capping net bill increase to £800/year in 2026/27 for ratepayers losing SBRR.
  - `test_england_transitional_relief_opt_out`: Verifies setting `apply_transitional_relief: Some(false)` calculates unphased gross liability without transitional relief deduction.
  - `test_wales_2026_three_multipliers`: Verifies Welsh 2026/27 three-tier multiplier regime under *Local Government Finance (Wales) Act 2024* (Retail 35.0p for RV < £51,000, Standard 50.2p for RV <= £100,000, and Higher 51.5p for RV > £100,000).
  - `test_wales_transitional_relief_2026_year_1_67_percent`: Verifies Welsh 2-year transitional relief scheme under *W.S.I. 2025/1371* granting 67% relief on bill increases exceeding £300 in Year 1 (2026/27).
  - `test_wales_transitional_relief_2027_year_2_34_percent`: Verifies Welsh 2-year transitional relief scheme under *W.S.I. 2025/1371* granting 34% relief on bill increases exceeding £300 in Year 2 (2027/28).
  - `test_wales_transitional_relief_under_300_no_relief`: Verifies Welsh transitional relief under *W.S.I. 2025/1371* awards 0% relief when the bill increase does not exceed the £300 statutory threshold.

- **Suite F: Component UI Serving & OWASP Security**:
  - `test_root_ui_route_and_static_assets_with_owasp_headers`: Verifies `/` and bundled assets `/assets/*` serve with strict OWASP CSP headers.
  - `test_ui_html_rendering_english_default`: Verifies `/` serves the Svelte 5 single-page application mount `<div id="app">`.
  - `test_ui_decommissioned_alias_routes_return_404`: Verifies unsupported UI alias routes (`/admin`, `/component-ui`, `/app`) return `404 NOT FOUND`.
  - `test_owasp_secure_headers_on_html_ui_response`: Verifies complete suite of OWASP headers on HTML responses.
  - `test_owasp_secure_headers_on_api_json_response`: Verifies OWASP security headers on API JSON responses.
  - `test_owasp_secure_headers_on_error_and_unauthorized_responses`: Verifies OWASP security headers on 422/404 error responses.
  - `test_static_assets_served_with_correct_content_types`: CSS and Vite assets served with proper MIME types; unmapped static asset paths return 404.

- **Suite G: Historical Statutory Parameters & Revaluations (1990–2022)**:
  - `test_historical_calculation_1990_91_inaugural_nndr`: Validates inaugural non-domestic rating calculation under LGFA 1988 (1990/91) with zero SBRR discount.
  - `test_historical_transitional_phasing_1991_92`: Validates 1991/92 revaluation year 2 transitional upward cap (+15%) based on 1990/91 baseline.
  - `test_historical_transitional_phasing_1993_94`: Validates 1993/94 revaluation upward cap (+15%) under SI 1989/1358 based on 1992/93 baseline.
  - `test_historical_transitional_phasing_1995_list_revaluation`: Validates 1995/96 large property revaluation upward cap (+20%) under SI 1994/3279.
  - `test_historical_sbrr_pre_2005_returns_no_relief`: Proves 2003/04 sole-property small business occupier receives 0% discount (prior to LGA 2003 s.61).
  - `test_historical_sbrr_2005_list_england_full_and_taper`: Verifies 2007/08 full 50% discount and 25% linear taper under SI 2004/3315.
  - `test_historical_sbrr_2010_list_england_and_scotland`: Verifies 2012/13 doubled 100% taper under SI 2010/1959 and Scottish 3-tier SBBS under SSI 2010/37.
  - `test_historical_transitional_phasing_2017_list_revaluation`: Verifies 2017/18 large property revaluation upward cap (+42%) under SI 2016/1265.

---

## 5. Execution & Verification Guide

To build, verify, and run the project:

```bash
# 1. Compile the Svelte 5 frontend component framework with Vite
./build-frontend.sh

# 2. Run the automated test suite (68 integration tests)
cargo test --test integration_tests

# 3. Start the HTTP API & Single-Page Application server (default port 8080)
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

# 7. Access the Component Framework UI in your browser
open http://localhost:8080/

# 8. Access the Interactive Scalar API Reference in your browser
open http://localhost:8080/docs
```
