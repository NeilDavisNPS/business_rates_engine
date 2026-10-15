# Universal Agent Engineering Standards & Defect Resolution Protocol

A standardized, technology-agnostic engineering protocol and defect resolution workflow designed for autonomous AI agents and human developers across software repositories.

---

## 1. UI & Front-End Accessibility (WCAG 2.1 / 2.2 Level AAA)
All user interface changes MUST strictly comply with WCAG Level AAA criteria:

| Standard / Element | Criteria | Minimum Threshold & Operational Rule |
|---|---|---|
| **Normal Text** | WCAG 1.4.6 | **$\ge$ 7:1** contrast against background across all states (< 18pt / 24px, < 14pt / 18.5px bold) |
| **Large Text** | WCAG 1.4.6 | **$\ge$ 4.5:1** contrast ($\ge$ 18pt / 24px, $\ge$ 14pt / 18.5px bold) |
| **UI Components & Borders** | WCAG 1.4.11 | **$\ge$ 3:1** contrast for boundaries, icons, badges, and state indicators |
| **Focus Rings** | WCAG 2.4.7 / 2.4.13 | **$\ge$ 3:1** contrast against adjacent surfaces; `outline: none` forbidden without visible ring |
| **Target Size & Spacing** | WCAG 2.5.5 | **$\ge 44 \times 44$ CSS px** (or min-h-[36px]/min-h-[44px] with adequate touch spacing) |
| **State Stability** | Operational | Hover, active, focus, and disabled states must maintain required contrast; destructive controls invert |
| **Dual Theme Parity** | Operational | Independent verification for Light & Dark modes; no transparent backgrounds on text |
| **Non-Color Dependence** | WCAG 1.4.1 / 4.1.2 | Never convey state/action by color alone; all controls require descriptive `aria-label` |

---

## 2. Text Externalisation & Internationalization (i18n)
All user-facing, regulatory, error, and audit text MUST be externalised into structured locale catalogs:
- **Zero Hardcoded Strings**: No raw strings in view templates, component files, client scripts, or engine logic.
- **100% Key Parity**: Any key added, modified, or deleted in the primary locale MUST be synchronously mirrored across all configured locale catalogs with identical hierarchy.
- **Audit Narrative**: Regulated/financial engines must generate step-by-step explanatory narratives detailing every decision, branch point, and calculation formula citing governing clauses, loaded entirely from locale catalogs.

---

## 3. Configuration Over Hardcoding (Zero Magic Numbers)
All domain values, thresholds, multipliers, rates, caps, and constants MUST be dynamically resolved from configuration tables or repository parameters, NEVER hardcoded in source logic:
- **Ambiguity Protocol**: If unsure whether a value or rule should be hardcoded or parameterized, obtain explicit user confirmation before hardcoding.
- **End-to-End Parameter Exposure**: When adding/modifying parameters, synchronously update: API query endpoints, schema contracts (OpenAPI/JSON schema), client stores & domain types, UI inspection catalogs, and documentation.

---

## 4. In-Code Domain Traceability Comments
Calculation logic, algorithms, and branch evaluations MUST include code comments citing governing specifications:
- **Precision Citations**: Primary specification/statute with section, schedule, or clause number.
- **Intent**: Explain non-obvious mathematical, regulatory, or business logic for auditors and maintainers.

---

## 5. Verification & Quality Gates
Before committing code changes, verify all quality gates (*Note: Test, linter, and build execution is not required if only documentation or `.md` files have changed*):
1. **Frontend**: Type check passes with 0 errors and 0 warnings; production bundle build succeeds.
2. **Backend**: All unit/integration tests pass (0 failures, 0 regressions); linter passes with 0 warnings.
3. **Accessibility**: Text contrast ≥ 7:1 (AAA), focus rings ≥ 3:1, and touch targets ≥ 44px across all themes.
4. **i18n**: Zero hardcoded strings; 100% key parity across all configured locale catalogs.
5. **Zero Magic Numbers**: Dynamic parameter resolution verified; no hardcoded business values.
6. **Documentation & API Contracts**: Synchronously update specifications, API schemas, and narrative catalogs.

---

## 6. Issue Resolution & Defect Triage Workflow
When investigating, fixing, or resolving issues raised in issue trackers (e.g. GitHub Issues), strictly follow this sequence:

1. **Review Authoritative Context**: Analyze the issue description against governing specifications, laws, or technical standards. Where there are comments on the issue from users other than the person who raised the issue, analyse the issue using the information provided by the person who raised the issue before taking into account any other comments.
2. **Dispute / Explanation Protocol**:
   - If disagreeing with the issue following analysis in Step 1, update the issue with supporting specification evidence explaining why the reported behavior is correct, do not modify the code, and stop the workflow. Only continue if you agree with the issue.
   - If multiple problems are raised in one issue and you disagree with some of them, split the disputed problems into a new issue linked to the original issue, update the new issue with justification as to why they are not defects, and proceed to fix only the agreed problems under the original issue.
   - Where you dispute an issue, **do not close it** (including where the issue has been split from the original issue); instead, keep the issue open and tag in the original author for comment.
3. **Reproduce via Failing Test**: Write a failing integration test (or frontend suite test) demonstrating the defect before making code changes.
4. **Implement Fix**: Address the defect adhering to Level AAA accessibility, i18n externalisation, zero magic numbers, narrative traceability, and in-code specification comments.
5. **Verify Test Passes**: Confirm the reproducing test passes cleanly.
6. **Update Documentation & API Contracts**: Synchronously update technical specifications, OpenAPI schemas, and narrative catalogs.
7. **Run Full Verification Suite**: Execute full test suites, linters with zero warnings, and production builds (not required if only documentation has changed).
8. **Atomic Commit**: Commit each fix locally referencing the issue: `git commit -m "fix: ... (fixes #<id>)"`.
9. **Post-Fix Analysis**: Comment on the issue with comprehensive analysis: authoritative context, reproducing test details, implementation summary, and test verification.
10. **Batched Push Policy & User Confirmation**: Commit each fix individually. Only push to the remote repository once ALL targeted issues are fixed and verified. **Always obtain explicit user confirmation before batch pushing.**
