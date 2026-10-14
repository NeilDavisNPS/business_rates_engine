# Business Rates Calculation Engine (1990–2027)

> **Rust Axum REST API and Svelte 5 web interface for statutory non-domestic rating calculations across England, Wales, and Scotland.**

---

## Key Capabilities

- **Statutory Parameter Engine**: Dynamic repository-backed rating rules, thresholds, and multipliers from 1 April 1990 through 31 March 2027.
- **Multi-Year & Multi-Occupancy Apportionment**: Apportions liability across financial year boundaries, leap years, and successive occupiers; models Section 44A part-occupied dual apportionments and statutory empty property exemption periods.
- **Statutory Devolved Reliefs**: Small Business Rate Relief (SBRR England/Wales), Small Business Bonus Scheme (SBBS Scotland), Retail, Hospitality & Leisure (RHL), Mandatory Charitable Relief, NDRA 2023 Improvement Relief (G-factor), Rural Settlement Relief, Public Lavatories, and Heat Network exemptions.
- **Devolved Anti-Avoidance Enforcement**: Implements Welsh Statutory Instrument 2026/43 artificial avoidance civil penalties (£500 fixed + 3% rateable value).
- **4-Language Native Internationalization**: Localization across English (`en`), Welsh (`cy`), Scots (`sco`), and Scottish Gaelic (`gd`) for statutory narratives, relief names, error messages, and reference codes.
- **Svelte 5 User Interface**: Interactive calculation interface built with Svelte 5 Runes (`$state`, `$derived`), Vite 6, TypeScript 5.7, Tailwind CSS, and collapsible parameter catalog explorers.
- **REST API**: Single-namespace endpoints (`/api/v1/calculate-liability`, `/api/v1/reference-codes`, `/api/v1/parameters/...`) providing deterministic calculation responses.
- **OpenAPI 3.1.0 & Self-Hosted Scalar Reference**: Interactive API reference at `/docs` powered by self-hosted `scalar.js` (zero external CDN dependencies; air-gapped compliant) with templated customization, specifications at `/api/v1/openapi.json` and `/api/v1/openapi.yaml`, and license attributions at `/licenses`.
- **Security Headers & CSP**: Content Security Policy (CSP), HTTP Strict Transport Security (HSTS), and defense-in-depth HTTP headers without external CDN dependencies.

---

## Quick Start

```bash
# 1. Build the Svelte 5 frontend bundle
./build-frontend.sh

# 2. Run the integration test suite (161 passing tests)
cargo test

# 3. Start the application server (http://localhost:8080)
cargo run
```

---

## Documentation

- **[Installation & Running Guide (INSTALL_AND_RUN.md)](INSTALL_AND_RUN.md)** — Installation, development workflow, cURL examples, and deployment guide.
- **[Specification, Test Cases & Source Index (business-rates-engine-spec-and-tests.md)](business-rates-engine-spec-and-tests.md)** — Statutory architecture, software requirements, parameter repository specification, test case breakdowns, and legal citations.
- **[Third-Party Software Licenses (THIRD_PARTY_LICENSES.md)](THIRD_PARTY_LICENSES.md)** — Attribution notices, software licenses, and third-party dependencies incorporated into the engine.
