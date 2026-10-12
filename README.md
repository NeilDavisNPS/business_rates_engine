# Business Rates Calculation Engine (1990–2027)

> **High-Performance Rust Axum REST API and Modern Svelte 5 Component Framework for Statutory Non-Domestic Rating Calculations across England, Wales, and Scotland.**

---

## Key Features

- **Zero Hardcoding Temporal Parameter Engine**: Covers rating rules and multipliers from 1 April 1990 to 31 March 2027.
- **Cross-Financial-Year & Multi-Occupancy Slicing**: Slices complex liability periods into discrete billing segments; fully models Section 44A partial-occupancy dual apportionments and statutory 13-week/6-week empty property reset rules.
- **Statutory Devolved Reliefs**: Small Business Rate Relief (SBRR England/Wales), Small Business Bonus Scheme (SBBS Scotland), Retail, Hospitality & Leisure (RHL), Mandatory Charitable Relief, NDRA 2023 Improvement Relief (G-factor), Rural Settlement Relief, Public Lavatories, and Heat Network exemptions.
- **Devolved Anti-Avoidance Enforcement**: Implements Welsh Statutory Instrument 2026/43 artificial avoidance penalties (£500 fixed + 3% rateable value).
- **4-Language Native Internationalization**: Full localization across **English (`en`)**, **Welsh (`cy`)**, **Scots (`sco`)**, and **Scottish Gaelic (`gd`)** for statutory narratives, relief names, error messages, and reference codes.
- **Modern Svelte 5 Component Framework**: Interactive web sandbox built with Svelte 5 Runes (`$state`, `$derived`), Vite 6, TypeScript 5.7, Standalone Tailwind CSS, and embedded parameter catalog explorers.
- **Unified Unauthenticated REST API**: Clean single-namespace endpoints (`/api/v1/calculate-liability`, `/api/v1/reference-codes`, `/api/v1/parameters/...`) with sub-millisecond execution and zero authentication overhead.
- **OpenAPI 3.1.0 & Self-Hosted Scalar Reference**: Interactive API reference at `/docs` powered by locally hosted `scalar.js` (zero external CDN dependencies; air-gapped compliant) with templated customization and official specs at `/api/v1/openapi.json` and `/api/v1/openapi.yaml`.
- **OWASP Security Baseline**: Strict Content Security Policy (CSP) with `script-src 'self'` (zero `'unsafe-inline'` and zero CDN origins), HSTS, and comprehensive security headers.

---

## Quick Start

```bash
# 1. Build the Svelte 5 frontend bundle
./build-frontend.sh

# 2. Run the integration test suite (78 passing tests)
cargo test

# 3. Start the application server (http://localhost:8080)
cargo run
```

---

## Documentation

- **[Installation & Running Guide (INSTALL_AND_RUN.md)](INSTALL_AND_RUN.md)** — Comprehensive installation, development workflow, cURL examples, and deployment guide.
- **[Specification, Test Cases & Source Index (business-rates-engine-spec-and-tests.md)](business-rates-engine-spec-and-tests.md)** — In-depth statutory architecture, SRS, built-in parameter repository specification, test case breakdowns, and legal citations.
- **[Migration & Route Deprecation Guide (MIGRATION.md)](MIGRATION.md)** — Guide for upgrading clients, route consolidations, and legacy endpoint deprecations.
