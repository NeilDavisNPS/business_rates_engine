# Business Rates Engine — Project Guidelines

Inherits [`CORE_ENGINEERING_STANDARDS.md`](./CORE_ENGINEERING_STANDARDS.md). Follow all baseline standards.

## 1. Statutory Authorities & Reference Sync
- **Statutory Inventory**: Primary Acts (LGFA 1988, NDRA 2023, LGF(W)A 2024, NDR(S)A 2020) & secondary SIs/SSIs/WSIs in [`STATUTORY_SOURCES.md`](./STATUTORY_SOURCES.md). Synchronously update with any new citations when adjusting rating logic or resolving issues.

## 2. i18n & Locales (4-Nation Parity)
- **Catalogs**: `frontend/src/locales/{en,cy,sco,gd}.json` and `locales/{en,cy,sco,gd}.json` (English, Welsh, Scots, Gaelic).
- **Rules**: Zero hardcoded strings. 100% key parity across all 4 files. Svelte access ONLY via `{calcStore.t.<key>}`.

## 3. Parameterization & Zero Magic Numbers
- All rating values/multipliers/thresholds/caps MUST be dynamically loaded from `ParameterRepository` (`src/db/repository.rs`), NEVER hardcoded in `.rs`.
- **Exposure Pipeline**: When modifying seeded tables, synchronously update: API (`/api/v1/parameters`), specs (`docs/openapi.{json,yaml}`), frontend types & store, UI catalog accordion (with AAA badges: England blue, Wales emerald, Scotland purple), 4 locale catalogs, and [`BUSINESS_RATES_ENGINE_SPEC_AND_TESTS.md`](./BUSINESS_RATES_ENGINE_SPEC_AND_TESTS.md).

## 4. In-Code Statutory Comments
- Logic in `src/engine/` MUST cite governing Act/SI/SSI/WSI (e.g. `// S.I. 2026/81 reg 10(5)`, `// LGFA 1988 s.47`). Explain non-obvious math (S.I. 1989/1058 whole-pound rounding, divisor $E$, leap years, TRS exclusivity under S.I. 2026/81 reg 16).

## 5. Quality Verification Gates
All commands MUST pass cleanly before commit (exempt when only documentation or `.md` files have changed):
1. `cargo test` (Suites A–J pass, 0 regressions) | 2. `cargo clippy --all-targets -- -D warnings` (0 warnings)
3. `npm --prefix frontend run check` (0 errors) | 4. `npm --prefix frontend run build` (build succeeds)
5. `docs/openapi.{json,yaml}` & `BUSINESS_RATES_ENGINE_SPEC_AND_TESTS.md` updated

## 6. Defect Resolution Workflow Bindings
Execute the 10-step workflow in [`CORE_ENGINEERING_STANDARDS.md`](./CORE_ENGINEERING_STANDARDS.md) with these project bindings:
- **Step 1 (Context)**: Verify against [`STATUTORY_SOURCES.md`](./STATUTORY_SOURCES.md). Prioritize original reporter's issue description over third-party comments.
- **Step 2 (Dispute)**: If disagreeing, update issue with statutory evidence, **do not close the issue** (even if split), and tag the original author. If multi-part, split disputed items into a linked issue.
- **Step 3 (Reproduce)**: Write failing integration test in `tests/integration_tests/` (Suites A–J).
- **Step 8 (Commit)**: Format: `git commit -m "fix: ... (fixes #<id>)"`.
- **Step 10 (Push)**: **Always obtain explicit user confirmation before batch pushing to GitHub.**
