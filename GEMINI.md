# Project Standards & Agent Guidelines

## 1. UI & Front-End Accessibility (WCAG 2.1 / 2.2 Level AAA)
All UI modifications MUST strictly enforce WCAG Level AAA criteria:
- **Contrast Ratios (WCAG 1.4.6 / 1.4.11)**:
  - Normal text (< 18pt / 24px, < 14pt / 18.5px bold): Minimum **7:1** against effective background across all states.
  - Large text (≥ 18pt / 24px, ≥ 14pt / 18.5px bold): Minimum **4.5:1**.
  - UI components, borders, icons, and focus indicators: Minimum **3:1**.
- **Interactive State Stability**: Hover, active, and focus states must never drop below 7:1. Destructive controls must invert to solid high-contrast backgrounds (e.g. `bg-red-800 text-white`). Disabled states must retain context and include `aria-disabled="true"` or `disabled`.
- **Dual Theme Parity**: Every color pair must be independently verified for Light and Dark modes (`dark:`). No transparent backgrounds on colored text.
- **Focus Indicators (WCAG 2.4.7 / 2.4.13)**: Clearly visible focus rings (`focus-visible:ring-2 focus-visible:ring-offset-2`) with ≥ 3:1 contrast against adjacent surfaces. `outline: none` without replacement is forbidden.
- **Target Size & Spacing (WCAG 2.5.5)**: Interactive touch targets must meet ≥ 44×44 CSS px or `min-h-[36px]` / `min-h-[44px]` with adequate touch spacing.
- **Non-Color Dependence (WCAG 1.4.1 / 4.1.2)**: Never convey state/action through color alone. All interactive elements require descriptive, accessible names (`aria-label`).

---

## 2. Text Externalisation & Internationalization (i18n)
All user-facing, regulatory, error, and audit text MUST be externalised from source code into JSON catalogs:
- **Zero Hardcoded Strings**: No raw English/natural-language strings in `.svelte`, `.ts`, `.js`, or `.rs` files.
- **Catalogs & 4-Locale Parity**:
  - Frontend: `frontend/src/locales/{en,cy,sco,gd}.json` (English, Welsh/Cymraeg, Scots, Scottish Gaelic/Gàidhlig).
  - Backend: `locales/{en,cy,sco,gd}.json`.
  - Any key added, modified, or removed in `en.json` MUST be synchronously mirrored across `cy.json`, `sco.json`, and `gd.json` with 100% key and hierarchy parity.
  - Svelte components access strings exclusively via reactive store: `{calcStore.t.<key>}` or `$derived(calcStore.t.<section>)`.
- **Statutory Audit Narrative & Decision Traceability**:
  - The engine must generate comprehensive step-by-step legislative narratives detailing all decisions and branch points: multiplier determination, RV threshold boundaries, relief qualification/disqualification, SBRR/SBBS tapers (citing divisor $E$), transitional phasing caps, and instalment schedules.
  - Every decision and branch point explanation MUST cite its authoritative statutory source: primary Acts (LGFA 1988, NDRA 2023), Statutory Instruments (SIs, SSIs, WSIs), schedules, and regulations.
  - All narrative templates and statutory citations must load from external locale catalogs with zero hardcoded English in Rust code.

---

## 3. Statutory Parameters & Configuration (Zero Magic Numbers)
All rating values, multipliers, rateable value thresholds, tapers, divisor $E$, and transitional caps MUST be dynamically resolved from configuration/repository tables (`ParameterRepository` in `src/db/repository.rs`), NEVER hardcoded in `.rs` files.
- **Ambiguity Protocol**: If unsure whether a financial value, threshold, or taper rule should be hardcoded or parameterized, you MUST obtain explicit user confirmation before hardcoding.
- **Mandatory End-to-End Parameter Exposure**: Whenever a seeded dataset/table/record is added or modified in `ParameterRepository`:
  1. **API & OpenAPI**: Expose in aggregated `GET /api/v1/parameters` (`AllParametersResponse`) and a dedicated `GET /api/v1/parameters/<dataset>` endpoint. Synchronously document in `docs/openapi.json` and `docs/openapi.yaml`.
  2. **Frontend Store**: Load and type in `calcStore.loadParameters()` (`frontend/src/stores/calculation.svelte.ts`) and TypeScript domain types (`frontend/src/types/index.ts`).
  3. **Frontend UI Visualization**: Add an accessible, collapsible accordion catalog under "Explore in-memory seeded statutory rates..." in `frontend/src/App.svelte` displaying all fields, nation filters (`ALL`, `ENGLAND`, `WALES`, `SCOTLAND`), sub-tier tags, search filter, and WCAG AAA badges (England blue, Wales emerald, Scotland purple).
  4. **Text Externalisation**: Mirror catalog headers, titles, filter labels, and badges across `frontend/src/locales/{en,cy,sco,gd}.json` (100% key parity).
  5. **Documentation**: Document dataset, fields, seed functions, and lookup methods in `business-rates-engine-spec-and-tests.md`.

---

## 4. Statutory In-Code Comments & Cross-Referencing
All calculation engine logic, formulas, algorithms, and branch evaluations in `src/engine/` MUST include detailed code comments citing governing legislation:
- **Precision Citations**: Primary Acts with section/schedule/paragraph (e.g. `// LGFA 1988 Sched 7 para A4`), secondary legislation with year/number/regulation (e.g. `// Non-Domestic Rating (Unoccupied Property) (England) Regs 2008 (S.I. 2008/386) reg 4`), and devolved Scottish/Welsh SIs.
- **Statutory Intent**: Explain non-obvious rating logic (e.g. whole-pound instalment truncation under S.I. 1989/1058, divisor $E$ rounding under S.I. 2017/102, calendar-month leap year boundaries, relief ordering hierarchy) so auditors can verify mathematical logic directly against legislation.

---

## 5. Verification & Quality Gates
Before committing any changes, verify all quality gates:
1. **Frontend**: `svelte-check` (0 errors, 0 warnings), `vite build` (bundle completes).
2. **Backend**: `cargo test` (all tests pass, 0 regressions), `cargo clippy --all-targets -- -D warnings` (0 warnings).
3. **Accessibility**: All modified text/background pairs ≥ 7:1 (AAA) in default/hover/active/focus/disabled across light and dark modes; focus rings ≥ 3:1; touch targets ≥ 44px.
4. **i18n**: Zero hardcoded strings in `.svelte` or `.rs`; 100% key parity across `frontend/src/locales/{en,cy,sco,gd}.json` and `locales/{en,cy,sco,gd}.json`.
5. **Zero Magic Numbers**: Dynamic parameter resolution verified; no hardcoded rating values in `.rs`.
6. **Audit Narrative**: Step-by-step legislative justification with statutory citations verified across all 4 locales.
7. **In-Code Comments**: Governing legislation cited for every formula and branch point in modified code.
8. **Documentation & OpenAPI**: Synchronously update `business-rates-engine-spec-and-tests.md`, `docs/openapi.{json,yaml}`, and `statutory_framework` in `frontend/src/locales/` (including `currentRegimeSummary`, `preDevolution`, `ratingLists`).
9. **Seeded Parameters**: 100% exposure in API, OpenAPI, `calcStore`, and UI catalogs with filtering and 4-locale i18n parity.

---

## 6. GitHub Issue Resolution Workflow
When investigating, fixing, or resolving issues raised on GitHub, strictly follow this sequence:
1. **Review Statutory Context**: Analyze issue description against governing Acts (LGFA 1988, NDRA 2023), SIs, SSIs, and WSIs.
2. **Dispute / Explanation Protocol**: If disagreeing based on legislation or architecture, update GitHub issue with clear statutory justification before modifying code.
3. **Reproduce via Failing Test**: Write a failing integration test in `tests/integration_tests/` (or frontend suite) demonstrating the issue.
4. **Implement Fix**: Address issue adhering to WCAG AAA, i18n externalisation, zero magic numbers, narrative traceability, and in-code statutory comments.
5. **Verify Test Passes**: Confirm the reproducing test passes cleanly.
6. **Update Documentation & OpenAPI**: Update `business-rates-engine-spec-and-tests.md`, `docs/openapi.{json,yaml}`, narrative catalogs, and `statutory_framework` across all 4 locales.
7. **Run Full Test Suite**: Execute `cargo test`, `cargo clippy --all-targets -- -D warnings`, `npm run check`, `npm run build` (0 errors, 0 warnings, 0 regressions).
8. **Commit Fix**: Commit locally referencing the issue: `git commit -m "fix: ... (fixes #<id>)"`.
9. **Post Issue Fix Analysis**: Comment on the GitHub issue with comprehensive analysis: statutory context, reproducing test, implementation details, documentation updates, and test verification.
10. **Batched Push Policy & User Confirmation**: Commit each fix individually. Only push to `origin/main` once ALL targeted issues are fixed and verified. **Always obtain explicit user confirmation before batch pushing to GitHub.**
