# Project Standards & Agent Guidelines

## 1. UI & Front-End Accessibility Standards (WCAG 2.1 / 2.2 Level AAA)

All front-end UI modifications in this repository MUST strictly enforce and comply with WCAG Level AAA criteria:

### A. Contrast Ratio (Enhanced — WCAG 1.4.6 Level AAA)
- **Normal Text** (< 18pt / 24px regular, < 14pt / 18.5px bold): Minimum **7:1** contrast ratio against its effective background in all states.
- **Large Text** (≥ 18pt / 24px regular, ≥ 14pt / 18.5px bold): Minimum **4.5:1** contrast ratio against its background.
- **UI Components & Graphical Objects** (WCAG 1.4.11): Minimum **3:1** contrast ratio for button borders, input borders, icons, and focus indicators against adjacent surfaces.

### B. Interactive State Stability (Hover, Active, Focus, Disabled)
- Hover, active, and focus states must **never** reduce text or border contrast below the required 7:1 threshold.
- Interactive elements must never lighten or fade into backgrounds on hover. Destructive controls (e.g. Remove / Delete) must invert to solid high-contrast backgrounds (e.g. `bg-red-800` with `text-white`) or darken cleanly.
- Disabled states must retain legible context and include `aria-disabled="true"` or `disabled` attributes.

### C. Dual Theme Parity (Light & Dark Modes)
- Every color pair must be independently verified for both Light mode and Dark mode (`dark:` variants).
- Never use transparent backgrounds on colored text that could render unpredictably across parent containers with different theme backgrounds.
- Explicitly pair text and background colors on interactive controls.

### D. Focus Indicators (Enhanced — WCAG 2.4.7 / 2.4.13)
- All interactive controls must feature clearly visible focus outlines (e.g., `focus-visible:ring-2 focus-visible:ring-offset-2`) with at least 3:1 contrast against adjacent background and non-focused states.
- Focus outlines must not be suppressed (`outline: none` without replacement is strictly forbidden).

### E. Target Size & Spacing (WCAG 2.5.5 Level AAA)
- All interactive touch targets must meet a minimum size of 44×44 CSS pixels, or provide equivalent hit area via padding and row height (`min-h-[36px]` or `min-h-[44px]` with adequate touch spacing).

### F. Non-Color Dependence & Accessible Context (WCAG 1.4.1, 4.1.2)
- Never convey state, selection, or destructive action through color alone.
- Buttons, icons, and table row actions must always have descriptive, accessible names (e.g. `aria-label="{calcStore.t.btn_remove} — {context} #{idx + 1}"`).

---

## 2. Text Externalisation & Internationalization (i18n) Standards

All user-facing, regulatory, and audit text across both the front end and backend MUST be externalised out of source code so that translations and statutory language can be reviewed, edited, and audited without reading or modifying code.

### A. Front-End (Svelte & TypeScript) Text Externalisation
- **Zero Hardcoded Strings**: No raw English or natural-language strings in `.svelte`, `.ts`, or `.js` source files.
- **Scope of Externalisation**:
  - Headings, subtitles, section titles, card labels, and tab labels.
  - Button text, form labels, dropdown option labels, placeholders, and tooltips.
  - Table headers, column labels, badge text, status indicators, and empty state messages.
  - Accessible names (`aria-label`, `aria-description`, `title` attributes).
  - External link descriptors (e.g. `title={b.title + calcStore.t.opens_legislation_link}`).
  - Static statutory timelines, historical narrative summaries, statutory rules, and legislation catalog entries.
- **Storage Locations**:
  - All UI strings and statutory framework datasets MUST reside in external JSON catalogs:
    - English: `frontend/src/locales/en.json`
    - Welsh (Cymraeg): `frontend/src/locales/cy.json`
    - Scots: `frontend/src/locales/sco.json`
    - Scottish Gaelic (Gàidhlig): `frontend/src/locales/gd.json`
- **Catalog Parity**:
  - Any key added, modified, or removed in `en.json` MUST be synchronously mirrored across `cy.json`, `sco.json`, and `gd.json`.
  - Keys must maintain identical hierarchical structures across all 4 files.
- **Component Access**:
  - Svelte components must access strings exclusively through the reactive store dictionary: `{calcStore.t.<key>}` or `$derived(calcStore.t.<section>)`.

### B. Backend (Rust Engine) Text Externalisation
- **Zero Hardcoded Text in Code**: No hardcoded statutory narratives, audit explanations, error messages, or reference code descriptions in `.rs` source code.
- **External Catalogs**:
  - Statutory narratives, relief audit notes, and descriptions must be loaded from external data catalogs (e.g., `locales/` JSON/YAML catalogs or external data files).
  - Translators, legal reviewers, and rating auditors must be able to verify or edit statutory narratives without compiling Rust source code.
- **Multilingual Support**:
  - The calculation engine must support English (`en`), Welsh (`cy`), Scots (`sco`), and Scottish Gaelic (`gd`) across:
    - Calculation audit narratives and step-by-step statutory justifications.
    - Statutory relief names, authority citations, and qualification notes.
    - Domain error messages and API error details.
    - Reference code descriptions (e.g. SCAT codes, relief codes, occupancy types).

---

## 3. Verification & Quality Gates

Before committing any changes:
1. **Frontend Build & Diagnostics**:
   - Run `svelte-check` — 0 errors, 0 warnings.
   - Run `vite build` — production bundle completes successfully.
2. **Backend Build & Test Suite**:
   - Run `cargo test` — all integration tests pass with 0 regressions.
3. **Accessibility Audit**:
   - Verify that all modified text/background pairs exceed **7:1** contrast ratio (Level AAA) in default, hover, active, and focus states.
   - Verify focus indicator visibility (≥ 3:1 contrast) and touch target spacing.
4. **i18n & Externalisation Audit**:
   - Verify zero hardcoded strings were introduced in `.svelte` or `.rs` files.
   - Verify key parity across `frontend/src/locales/{en,cy,sco,gd}.json` (no missing keys).
5. **Documentation & OpenAPI Specification Audit**:
   - Verify that all newly cited legislation, Acts of Parliament, statutory instruments (SIs, SSIs, WSIs), regulations, and new tests are recorded in `business-rates-engine-spec-and-tests.md`.
   - Verify that `docs/openapi.json` and `docs/openapi.yaml` accurately document any new domain interface fields, request/response models, or relief flags.

---

## 4. GitHub Issue Resolution Workflow

Whenever investigating, fixing, or resolving an issue raised on GitHub against this project, you MUST strictly adhere to the following sequence:

1. **Review Statutory & Legislative Context**:
   - Thoroughly review the issue description, including any relevant legislation, Acts of Parliament (e.g. LGFA 1988, NDRA 2023), statutory instruments (SIs), or devolved Scottish/Welsh regulations.
2. **Dispute / Explanation Protocol**:
   - If you do not agree with the issue based on statutory provisions or engine architecture, update the GitHub issue with a clear, evidenced explanation citing the relevant legislation or rationale before taking any code action.
3. **Reproduce via Failing Test Case**:
   - If you agree with the issue, first create a failing test case in `tests/integration_tests.rs` (or the relevant frontend test suite) that explicitly reproduces and demonstrates the reported issue.
4. **Implement the Fix**:
   - Fix the issue in the code, ensuring strict adherence to WCAG AAA accessibility, text externalisation (i18n), and zero-hardcoding standards.
5. **Verify the Failing Test Passes**:
   - Re-run the specific test case created in Step 3 to confirm that the issue is resolved and the test now passes cleanly.
6. **Review Documentation, Specifications & OpenAPI Interfaces**:
   - Review and update all documentation before committing the fix.
   - Ensure that any citations, legislation, statutory instruments (SIs, SSIs, WSIs), regulations, and newly created tests are added to `business-rates-engine-spec-and-tests.md`.
   - Ensure the OpenAPI documents (`docs/openapi.json` and `docs/openapi.yaml`) are updated synchronously with any changes to the interfaces, data models, or relief flags.
7. **Run Full Test Suite (Zero Regressions)**:
   - Run the complete test suite (`cargo test`, `cargo clippy --all-targets -- -D warnings`, `npm run check`, `npm run build`) to confirm there are no regression issues.
8. **Commit the Fix Referencing the Issue**:
   - Commit the fix referencing the GitHub issue number in the commit message (e.g. `fix: ... (fixes #123)`). Always obtain explicit user confirmation before committing.
9. **Batched Push Policy**:
   - If multiple issues are being fixed in a batch or session, commit each fix individually, but only push to GitHub once ALL of the targeted issues have been completely fixed and verified.
