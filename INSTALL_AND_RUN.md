# Installation, Running & Deployment Guide

This guide details how to install, build, configure, test, and deploy the **Business Rates Statutory Calculation Engine & Interactive Component Sandbox** across Great Britain (England, Wales, and Scotland) for both **Testing / QA** and **High-Performance Production** environments.

---

## Table of Contents
1. [Prerequisites](#1-prerequisites)
2. [Quick Start (Local Development)](#2-quick-start-local-development)
3. [Building the Application](#3-building-the-application)
4. [Running Locally](#4-running-locally)
5. [Frontend Development Mode (Vite HMR)](#5-frontend-development-mode-vite-hmr)
6. [Automated Testing & Quality Verification](#6-automated-testing--quality-verification)
7. [Deploying for Testing / Staging](#7-deploying-for-testing--staging)
8. [Deploying for Production](#8-deploying-for-production)
   - [Option A: Multi-Stage Container Deployment (Podman / Docker / Kubernetes)](#option-a-multi-stage-container-deployment-podman--docker--kubernetes)
   - [Option B: Bare-Metal / VM Linux Systemd Service](#option-b-bare-metal--vm-linux-systemd-service)
   - [Reverse Proxy & SSL/TLS Configuration (Nginx & Caddy)](#reverse-proxy--ssltls-configuration)
9. [REST API Endpoints & cURL Reference](#9-rest-api-endpoints--curl-reference)
10. [Project Directory Structure](#10-project-directory-structure)

---

## 1. Prerequisites

Before getting started, ensure you have the following installed on your machine:

- **Rust & Cargo**: Rust 1.82+ (2021 Edition; Recommended: Latest Stable)  
  *Install:* `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js & npm**: Node.js 20+ or 24+ (LTS)  
  *Verify:* `node -v` and `npm -v`  
  *(Note: In WSL/Linux environments without global Node, run `./setup-node.sh` to install a local portable Node binary into `./bin/`).*
- **(Optional) Podman (or Docker)**: For rootless, daemonless containerized deployment using Podman with the multi-stage [`Dockerfile`](Dockerfile).  
  *(Note: The calculation engine includes an in-memory repository pre-seeded with all 1990–2027 statutory parameters across Great Britain, operating with zero external database dependencies).*

---

## 2. Quick Start (Local Development)

From the project root directory:

```bash
# 1. Compile Tailwind styles and the Svelte 5 frontend into a single bundle
./build-frontend.sh

# 2. Run the full automated integration test suite (83 tests)
cargo test

# 3. Launch the application server
cargo run
```

Once running, navigate your web browser to:
👉 **[http://localhost:8080/](http://localhost:8080/)** (Interactive Calculation Sandbox)  
👉 **[http://localhost:8080/docs](http://localhost:8080/docs)** (Interactive Scalar API Reference)  
👉 **[http://localhost:8080/licenses](http://localhost:8080/licenses)** (Third-Party Software Licenses)

---

## 3. Building the Application

### Step A: Build the Svelte 5 Frontend

The user interface is a modern Single-Page Application (SPA) located in `frontend/`, built with **Svelte 5 Runes**, **Vite 6**, **TypeScript 5.7**, and **Tailwind CSS**.

```bash
# Automated single-command build:
./build-frontend.sh
```

This compiles:
1. Tailwind utility classes from `styles/input.css` into `frontend/src/tailwind.css`.
2. Svelte 5 components and styles into a **unified stylesheet** (`dist/assets/app.css`) and JS bundle (`dist/assets/app.js`).
3. Output directory: `frontend/dist/`.

### Step B: Build the Rust Backend

```bash
# Debug build (fast compile for testing):
cargo build

# Release build (heavily optimized for production):
cargo build --release
```

The release binary will be created at: `target/release/business_rates_engine`.

---

## 4. Running Locally

```bash
# Run with default port 8080
cargo run

# Run with custom port via environment variable
PORT=9090 cargo run
```

### Application URLs & Routes

| Path | Description | Access |
| :--- | :--- | :--- |
| `http://localhost:8080/` | Main Interactive Calculation Sandbox & Audit Interface | Public (GET) |
| `http://localhost:8080/docs` | Interactive Scalar API Reference (zero-CDN self-hosted) | Public (GET) |
| `http://localhost:8080/licenses` | Third-Party Software Licenses Web Page | Public (GET) |
| `http://localhost:8080/assets/:filename` | Bundled Static Frontend & Scalar Assets | Public (GET) |
| `http://localhost:8080/api/v1/calculate-liability` | Core Statutory Calculation REST Endpoint | Public (POST) |
| `http://localhost:8080/api/v1/reference-codes` | Statutory Reference Codes & Allowable Values Catalog | Public (GET) |
| `http://localhost:8080/api/v1/parameters` | Aggregated Statutory Parameter Repository Catalog | Public (GET) |
| `http://localhost:8080/api/v1/parameters/multipliers` | Rating Multipliers Catalog (1990–2027) | Public (GET) |
| `http://localhost:8080/api/v1/parameters/sbrr-thresholds` | SBRR / SBBS Statutory Thresholds Catalog | Public (GET) |
| `http://localhost:8080/api/v1/parameters/sector-reliefs` | Devolved Sector Relief Schemes Catalog | Public (GET) |
| `http://localhost:8080/api/v1/parameters/transitional-caps` | Statutory Transitional Phasing Caps Catalog | Public (GET) |
| `http://localhost:8080/api/v1/licenses` | Third-Party Licenses (Markdown or JSON format) | Public (GET) |
| `http://localhost:8080/api/v1/openapi.json` | OpenAPI 3.1.0 Specification (JSON format) | Public (GET) |
| `http://localhost:8080/api/v1/openapi.yaml` | OpenAPI 3.1.0 Specification (YAML format) | Public (GET) |
| `http://localhost:8080/healthz` | Service Health Check | Public (GET) |

---

## 5. Frontend Development Mode (Vite HMR)

For live code editing with instant Hot Module Replacement:

1. Start the backend server in terminal 1:
   ```bash
   cargo run
   ```
2. Start the Vite dev server in terminal 2:
   ```bash
   cd frontend
   npm run dev
   ```
3. Open **`http://localhost:5173/`**. Vite automatically proxies calculation API requests directly to Axum at `http://localhost:8080`.

---

## 6. Automated Testing & Quality Verification

Run the test suite verifying multi-jurisdiction calculations, multi-occupancy slicing, Section 44A apportionments, 4-language i18n, parameter endpoints, Scalar documentation, and OWASP security headers:

```bash
# Run all 83 integration tests
cargo test

# Run tests with detailed backtrace on failure
RUST_BACKTRACE=1 cargo test

# Run a specific scenario test
cargo test --test integration_tests test_scenario_1_england_sbrr_and_rhl_multi_year

# Typecheck and lint the Svelte 5 frontend codebase
cd frontend && npm run check
```

---

## 7. Deploying for Testing / Staging

When deploying to a dedicated testing, QA, or staging server:

### A. Environment Configuration for Testing
Set verbose logging and run in testing mode:
```bash
export RUST_LOG=debug
export PORT=8080
```

### B. Automated Build & Test in CI/CD Pipeline
In GitHub Actions, GitLab CI, or Jenkins:
```bash
# 1. Ensure toolchains are installed
rustup default stable
node -v && npm -v

# 2. Build frontend production assets
./build-frontend.sh

# 3. Execute all unit and integration tests
cargo test --verbose

# 4. Compile the testing release binary
cargo build --release
```

### C. Smoke Testing & Verification
After launching the staging service, run automated smoke tests against the instance:
```bash
# 1. Health check & UI mount verification
curl -f -s http://staging-server:8080/ | grep -q 'id="app"' && echo "UI Mount OK"

# 2. OWASP Strict CSP verification (verify zero 'unsafe-inline' on core endpoints)
curl -s -I http://staging-server:8080/ | grep -i 'content-security-policy' | grep -v "'unsafe-inline'" && echo "Strict CSP OK"

# 3. Calculation smoke test (canonical unified route)
curl -f -s -X POST http://staging-server:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_1_sbrr_rhl_england.json \
  | grep -q "calculation_id" && echo "Calculation API OK"

# 4. Scalar API Reference & OpenAPI spec verification
curl -f -s http://staging-server:8080/api/v1/openapi.json | grep -q '"openapi":"3.1.0"' && echo "OpenAPI 3.1.0 OK"
curl -f -s http://staging-server:8080/docs | grep -q 'scalar.js' && echo "Scalar Docs OK"
```

---

## 8. Deploying for Production

### Option A: Multi-Stage Container Deployment (Podman / Docker / Kubernetes)

A production-ready [`Containerfile`](Containerfile) is provided in the root directory. It uses a **3-stage multi-stage build**:
1. **Frontend Builder (`node:24-slim`)**: Automatically fetches verified standalone Tailwind CLI and compiles Svelte 5 frontend assets via Vite.
2. **Backend Builder (`rust:1-slim-bookworm`)**: Compiles the Rust binary with `--release` optimizations, baking OpenAPI definitions and Scalar templates directly into the binary.
3. **Runtime (`debian:bookworm-slim`)**: Minimal (~90 MB) secure runtime executing as an **unprivileged non-root service account** (`appuser`, UID/GID 1001).

Podman is daemonless and rootless by default, requiring no root privileges or background daemon.

#### 1. Build the Container Image with Podman
```bash
# Build using rootless Podman (automatically detects Containerfile):
podman build -t business-rates-engine:latest .
```
*(Docker drop-in equivalent: `docker build -f Containerfile -t business-rates-engine:latest .`)*

#### 2. Run the Container
```bash
podman run -d \
  --name business-rates-engine \
  -p 8080:8080 \
  -e PORT=8080 \
  -e RUST_LOG=info \
  --restart unless-stopped \
  business-rates-engine:latest
```
*(Docker drop-in equivalent: replace `podman` with `docker`).*

#### 3. Container Management & Health Inspection
```bash
# Stream container logs
podman logs -f business-rates-engine

# Verify built-in container healthcheck
podman healthcheck run business-rates-engine

# Graceful stop (triggers SIGTERM; shuts down cleanly in milliseconds)
podman stop business-rates-engine
```

#### 4. Run Directly with Podman Play Kube (Local Kubernetes Without a Cluster)
Podman can directly run Kubernetes pod and service manifests locally:

```bash
# Launch pods directly from the manifest
podman play kube k8s-deployment.yaml

# Tear down the pod
podman play kube --down k8s-deployment.yaml
```

#### 5. Kubernetes Deployment Manifest (`k8s-deployment.yaml`)
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: business-rates-engine
  labels:
    app: business-rates-engine
spec:
  replicas: 3
  selector:
    matchLabels:
      app: business-rates-engine
  template:
    metadata:
      labels:
        app: business-rates-engine
    spec:
      containers:
      - name: engine
        image: business-rates-engine:latest
        imagePullPolicy: IfNotPresent
        ports:
        - containerPort: 8080
        env:
        - name: PORT
          value: "8080"
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "64Mi"
            cpu: "50m"
          limits:
            memory: "256Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 15
        readinessProbe:
          httpGet:
            path: /
            port: 8080
          initialDelaySeconds: 3
          periodSeconds: 10
---
apiVersion: v1
kind: Service
metadata:
  name: business-rates-service
spec:
  type: ClusterIP
  selector:
    app: business-rates-engine
  ports:
  - port: 80
    targetPort: 8080
```

---

### Option B: Bare-Metal / VM Linux Systemd Service

For direct deployment on Ubuntu / Debian / RHEL virtual machines:

#### 1. Prepare Release Files
```bash
# 1. Build frontend and backend on build server
./build-frontend.sh
cargo build --release

# 2. Deploy to production directory (e.g. /opt/business_rates_engine)
sudo mkdir -p /opt/business_rates_engine/frontend
sudo cp target/release/business_rates_engine /opt/business_rates_engine/
sudo cp -r frontend/dist /opt/business_rates_engine/frontend/

# 3. Create dedicated non-root user
sudo useradd -r -s /bin/false -d /opt/business_rates_engine businessrates
sudo chown -R businessrates:businessrates /opt/business_rates_engine
```

#### 2. Create Systemd Unit (`/etc/systemd/system/business_rates_engine.service`)
```ini
[Unit]
Description=Business Rates Statutory Calculation Engine
After=network.target

[Service]
Type=simple
User=businessrates
Group=businessrates
WorkingDirectory=/opt/business_rates_engine
ExecStart=/opt/business_rates_engine/business_rates_engine
Restart=always
RestartSec=5
Environment=PORT=8080
Environment=RUST_LOG=info

# Sandboxing & Security Hardening
ProtectSystem=strict
ProtectHome=true
NoNewPrivileges=true
PrivateTmp=true
ReadWritePaths=/opt/business_rates_engine

[Install]
WantedBy=multi-user.target
```

#### 3. Enable and Start the Service
```bash
sudo systemctl daemon-reload
sudo systemctl enable business_rates_engine
sudo systemctl start business_rates_engine

# Check status and logs
sudo systemctl status business_rates_engine
sudo journalctl -u business_rates_engine -f
```

---

### Reverse Proxy & SSL/TLS Configuration

Place the Axum service behind a production reverse proxy for TLS termination, HTTP/2, and domain routing.

#### Nginx Configuration (`/etc/nginx/sites-available/business_rates.conf`)
```nginx
server {
    listen 80;
    server_name rates.example.gov.uk;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    server_name rates.example.gov.uk;

    ssl_certificate /etc/letsencrypt/live/rates.example.gov.uk/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/rates.example.gov.uk/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    # Gzip compression for static Vite assets and API JSON
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml;
    gzip_min_length 1000;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Keepalive and timeouts
        proxy_connect_timeout 5s;
        proxy_read_timeout 60s;
        proxy_send_timeout 60s;
    }
}
```

#### Caddy Configuration (`/etc/caddy/Caddyfile`)
```caddy
rates.example.gov.uk {
    encode gzip zstd
    reverse_proxy 127.0.0.1:8080
}
```

---

## 9. REST API Endpoints & cURL Reference

The REST API operates as an **unauthenticated public service**; requests do not require API keys (`X-API-Key`) or bearer tokens.

### 1. Calculate Liability via Canonical Test Fixtures (Scenarios 1–11)

The test suite includes 11 canonical statutory scenario fixtures under `tests/fixtures/` covering major non-domestic rating workflows across England, Wales, and Scotland:

```bash
# Scenario 1: England Small Business (SBRR) & RHL Multi-Year Calculation
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_1_sbrr_rhl_england.json

# Scenario 2: Wales Commercial Hereditament (Petrol Station / SCAT 211)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_2_wales_petrol_station.json

# Scenario 3: Scotland Self-Catering Accommodation (SBBS & 182-Day Letting Test)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_3_scotland_self_catering.json

# Scenario 4: Welsh Section 63B Artificial Avoidance Civil Penalty (£500 + 3% of RV)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_4_welsh_artificial_avoidance.json

# Scenario 5: England High-Value Hereditament (> £500,000 RV High-Value Multiplier)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_5_airport_high_value.json

# Scenario 6: Mandatory 80% Charitable Relief (LGFA 1988 s.43(5) & LG(R)S 1962 s.4(2))
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_6_charity_mandatory_80.json

# Scenario 7: Multi-Occupancy Episodes (Unoccupied → Occupied SBRR → S.44A Part-Occupied)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_7_multi_occupancy_episodes.json

# Scenario 8: Empty Industrial Hereditament Section 45 Exemption (6-Month Rate-Free Period)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_8_empty_property_exemption.json

# Scenario 9: NDRA 2023 Certified Improvement Relief (G-Factor RV Increment Exemption)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_9_improvement_relief_g_factor.json

# Scenario 10: Statutory 12-Month Installment Plan (LGA 2003 / SI 2014/466 Statutory Right)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_10_statutory_12_installments.json

# Scenario 11: Mandatory 100% Rural Settlement Relief (Sole Village Shop / Post Office / Pub)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_11_rural_settlement_relief.json
```

### 2. Multi-Language Content Negotiation
Request statutory narratives and relief names in any of the 4 supported languages via the `Accept-Language` header (or payload `"language"` field):

```bash
# Welsh (Cymraeg)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -H "Accept-Language: cy" \
  -d @tests/fixtures/scenario_1_sbrr_rhl_england.json

# Scots
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -H "Accept-Language: sco" \
  -d @tests/fixtures/scenario_1_sbrr_rhl_england.json

# Scottish Gaelic (Gàidhlig)
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -H "Accept-Language: gd" \
  -d @tests/fixtures/scenario_1_sbrr_rhl_england.json
```

### 3. Retrieve Statutory Reference Codes Catalog
```bash
# All reference codes
curl -s http://localhost:8080/api/v1/reference-codes

# Welsh localized reference codes
curl -s "http://localhost:8080/api/v1/reference-codes?lang=cy"
```

### 4. Retrieve Statutory Parameters Catalogs
```bash
# Full aggregated parameter repository catalog
curl -s http://localhost:8080/api/v1/parameters

# Rating Multipliers (1990–2027)
curl -s http://localhost:8080/api/v1/parameters/multipliers

# Small Business Rate Relief (SBRR / SBBS) thresholds
curl -s http://localhost:8080/api/v1/parameters/sbrr-thresholds

# Devolved sector relief schemes
curl -s http://localhost:8080/api/v1/parameters/sector-reliefs

# Statutory transitional phasing caps
curl -s http://localhost:8080/api/v1/parameters/transitional-caps
```

### 5. OpenAPI 3.1.0 Specification & Self-Hosted Scalar Reference
```bash
# Fetch OpenAPI 3.1.0 JSON specification
curl -s http://localhost:8080/api/v1/openapi.json

# Fetch OpenAPI 3.1.0 YAML specification
curl -s http://localhost:8080/api/v1/openapi.yaml

# Open Interactive Scalar Reference in browser (default solarized dark theme)
open http://localhost:8080/docs

# Open Scalar Reference with custom theme & layout query parameters
open "http://localhost:8080/docs?theme=kepler&layout=modern"
```

### 6. Third-Party Licenses & Health Check
```bash
# Fetch third-party licenses web page
curl -s http://localhost:8080/licenses

# Fetch raw third-party licenses Markdown
curl -s http://localhost:8080/api/v1/licenses

# Check service health status
curl -s http://localhost:8080/healthz
```

---

## 10. Project Directory Structure

```
business_rates_engine/
├── docs/                         # Official API Specifications
│   ├── openapi.yaml              # OpenAPI 3.1.0 specification (YAML)
│   └── openapi.json              # OpenAPI 3.1.0 specification (JSON)
├── templates/                    # Server-rendered HTML templates
│   ├── licenses.html             # CSP-compliant third-party licenses template
│   └── scalar.html               # Configurable, offline Scalar UI template
├── frontend/                     # Modern Svelte 5 SPA framework
│   ├── public/assets/            # Self-hosted static assets (scalar.js)
│   ├── src/
│   │   ├── components/           # UI Components (Navbar, S.44A Table, Matrix, Catalogs)
│   │   ├── stores/               # Reactive calculation & form state ($state Runes)
│   │   ├── locales/              # i18n Catalogs (en, cy, sco, gd JSON)
│   │   └── types/                # TypeScript statutory interfaces
│   ├── vite.config.ts            # Vite 6 configuration (with API proxy)
│   └── package.json
├── src/                          # High-performance Axum backend
│   ├── api/                      # REST Handlers, Routers & OWASP middleware
│   ├── db/                       # ParameterRepository (1990–2027 statutory parameters)
│   ├── engine/                   # CalculationEngine (Statutory slicer, multipliers, reliefs)
│   ├── models/                   # Calculation requests, responses, enums
│   ├── error.rs                  # Localized AppError handling
│   ├── i18n.rs                   # Multi-language resolution & narrative generation
│   ├── lib.rs
│   └── main.rs                   # Server entrypoint (supports PORT env var)
├── tests/
│   ├── fixtures/                 # Pre-configured test scenario JSON files (Scenarios 1-11)
│   └── integration_tests.rs      # Complete test suite (83 passing tests)
├── scripts/                      # Build, licensing & toolchain helper scripts
│   ├── check-licenses.js         # License compliance validator
│   ├── generate-licenses.js      # License catalog & attribution generator
│   ├── generate-sbom.sh          # CycloneDX SBOM generator script
│   └── ensure-tailwind.sh        # Pinned standalone Tailwind CLI downloader
├── build-frontend.sh             # Build script for Svelte 5 frontend (Tailwind + Vite bundle)
├── watch-css.sh                  # Tailwind CSS watch mode for Vite development
├── setup-node.sh                 # Local Node.js environment setup helper
├── LICENSE.md                    # MIT License specification
├── THIRD_PARTY_LICENSES.md       # Third-party open-source attribution notices
├── Dockerfile                    # Multi-stage production container build (Podman / Docker)
└── Cargo.toml                    # Rust dependencies & package manifest
```
