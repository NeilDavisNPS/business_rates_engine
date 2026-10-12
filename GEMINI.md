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

## 2. Internationalization (i18n) Standards
- All user-facing text, button labels, placeholders, titles, and statutory narratives must be externalized into locale catalogs in `frontend/src/locales/{en,cy,sco,gd}.json`.
- Zero hardcoded English strings in components.
- Any new translation key added to `en.json` must be mirrored across `cy.json` (Welsh), `sco.json` (Scots), and `gd.json` (Scottish Gaelic).

---

## 3. Verification & Quality Gates
Before proposing any front-end commits:
1. Run `svelte-check` and `vite build` to guarantee zero errors and zero warnings.
2. Run `cargo test` to verify zero backend regressions.
3. Verify that contrast ratios meet or exceed 7:1 across all interactive states.

