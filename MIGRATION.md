# Migration & Route Deprecation Guide

This document details breaking route changes, decommissioned aliases, and migration instructions for clients and integrations migrating from earlier versions of the Business Rates Engine.

---

## 1. Route Consolidation & Canonical Namespaces

All public API endpoints have been consolidated into a clean, uniform `/api/v1/` single namespace. Redundant route aliases, legacy vendor-specific documentation paths, and deprecated service slugs return `404 Not Found`.

### REST API Endpoints

| Legacy / Deprecated Route | Canonical Route | Method | Description |
| :--- | :--- | :--- | :--- |
| `/api/v1/business-rates/calculate-liability` | `/api/v1/calculate-liability` | `POST` | Core statutory liability calculation engine |
| `/api/v1/business-rates/reference-codes` | `/api/v1/reference-codes` | `GET` | Statutory dictionary and reference codes catalog |
| `/openapi.json` | `/api/v1/openapi.json` | `GET` | OpenAPI 3.1.0 specification (JSON) |
| `/openapi.yaml` | `/api/v1/openapi.yaml` | `GET` | OpenAPI 3.1.0 specification (YAML) |

---

### Interactive UI & Documentation Endpoints

| Deprecated Alias | Canonical Route | Description |
| :--- | :--- | :--- |
| `/admin`, `/component-ui`, `/app` | `/` | Main interactive calculation sandbox & audit interface (Svelte 5) |
| `/scalar`, `/api-reference`, `/swagger`, `/swagger-ui` | `/docs` | Self-hosted interactive Scalar API documentation |

---

## 2. Authentication & Authorization Changes

### Unauthenticated Public API Access
* **Removed**: Multi-tenant headers, tenant API keys (`X-API-Key`), bearer tokens, and SSO provider endpoints (`/api/v1/auth/sso/login`, `/api/v1/auth/sso/callback`).
* **Current Behavior**: All API endpoints operate as a **high-performance, unauthenticated public service**. No API keys or authentication headers are required. Requests passing `X-API-Key` or `Authorization` headers are accepted without rejection, but credentials are not validated or required.
* **Security**: OWASP-compliant security headers (strict `Content-Security-Policy`, `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Strict-Transport-Security`, `Referrer-Policy`) are enforced on all HTTP responses.

---

## 3. Migration Checklist for API Clients

1. **Update Base Calculation URL**:
   Change `POST /api/v1/business-rates/calculate-liability` to `POST /api/v1/calculate-liability`.
2. **Update Reference Codes URL**:
   Change `GET /api/v1/business-rates/reference-codes` to `GET /api/v1/reference-codes`.
3. **Remove Authentication Headers**:
   You may safely remove `X-API-Key` headers and bearer tokens from your HTTP request client.
4. **Update Documentation Bookmarks**:
   Access the interactive API reference at `/docs` (replaces `/swagger`, `/swagger-ui`, `/scalar`, and `/api-reference`).

