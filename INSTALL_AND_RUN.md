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
   - [Option A: Multi-Stage Container Deployment (Docker / Kubernetes)](#option-a-multi-stage-container-deployment-docker--kubernetes)
   - [Option B: Bare-Metal / VM Linux Systemd Service](#option-b-bare-metal--vm-linux-systemd-service)
   - [Reverse Proxy & SSL/TLS Configuration (Nginx & Caddy)](#reverse-proxy--ssltls-configuration)
9. [REST API Endpoints & cURL Reference](#9-rest-api-endpoints--curl-reference)
10. [Project Directory Structure](#10-project-directory-structure)

---

## 1. Prerequisites

Before getting started, ensure you have the following installed on your machine:

- **Rust & Cargo**: Rust 1.75+ (2021 Edition)  
  *Install:* `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js & npm**: Node.js 18+ or 20+ (LTS)  
  *Verify:* `node -v` and `npm -v`  
  *(Note: In WSL/Linux environments without global Node, run `./setup-node.sh` to install a local portable Node binary into `./bin/`).*
- **(Optional) Docker**: For standalone containerized deployment using the multi-stage [`Dockerfile`](Dockerfile).  
  *(Note: The calculation engine includes an in-memory repository pre-seeded with all 1993–2027 statutory parameters across Great Britain, operating with zero external database dependencies).*

---

## 2. Quick Start (Local Development)

From the project root directory:

```bash
# 1. Compile Tailwind styles and the Svelte 5 frontend into a single bundle
./build-frontend.sh

# 2. Run the full automated integration test suite (45 tests)
cargo test

# 3. Launch the application server
cargo run
```

Once running, navigate your web browser to:
👉 **[http://localhost:8080/](http://localhost:8080/)** (or **[http://localhost:8080/admin](http://localhost:8080/admin)**)  
👉 **[http://localhost:8080/docs](http://localhost:8080/docs)** (Interactive Scalar API Reference)

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
2. Svelte 5 components and styles into a **single unified CSS bundle** (`dist/assets/index-[hash].css`) and JS bundle (`dist/assets/index-[hash].js`).
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
| `http://localhost:8080/admin` | Direct alias to the Component Sandbox UI | Public (GET) |
| `http://localhost:8080/component-ui` | Direct alias to the Component Sandbox UI | Public (GET) |
| `http://localhost:8080/app` | Direct alias to the Component Sandbox UI | Public (GET) |
| `http://localhost:8080/docs` | Interactive Scalar API Reference (zero-CDN self-hosted) | Public (GET) |
| `http://localhost:8080/scalar` | Direct alias to the Scalar API Reference | Public (GET) |
| `http://localhost:8080/api-reference` | Direct alias to the Scalar API Reference | Public (GET) |
| `http://localhost:8080/swagger` | Direct alias to the Scalar API Reference | Public (GET) |
| `http://localhost:8080/swagger-ui` | Direct alias to the Scalar API Reference | Public (GET) |
| `http://localhost:8080/api/v1/openapi.json` | OpenAPI 3.1.0 Specification (JSON format, alias `/openapi.json`) | Public (GET) |
| `http://localhost:8080/api/v1/openapi.yaml` | OpenAPI 3.1.0 Specification (YAML format, alias `/openapi.yaml`) | Public (GET) |
| `http://localhost:8080/api/v1/calculate-liability` | Core Statutory Calculation REST Endpoint | Public (POST) |
| `http://localhost:8080/api/v1/reference-codes` | Statutory Reference Codes & Allowable Values Catalog | Public (GET) |
| `http://localhost:8080/api/v1/parameters` | Aggregated Statutory Parameter Repository Catalog | Public (GET) |
| `http://localhost:8080/api/v1/parameters/multipliers` | Rating Multipliers Catalog (1993–2027) | Public (GET) |
| `http://localhost:8080/api/v1/parameters/sbrr-thresholds` | SBRR / SBBS Statutory Thresholds Catalog | Public (GET) |
| `http://localhost:8080/api/v1/parameters/sector-reliefs` | Devolved Sector Relief Schemes Catalog | Public (GET) |
| `http://localhost:8080/api/v1/parameters/transitional-caps` | Statutory Transitional Phasing Caps Catalog | Public (GET) |

> **Note on Deprecated Routes**: Legacy endpoints with the `/business-rates/` slug (such as `/api/v1/business-rates/calculate-liability` and `/api/v1/business-rates/reference-codes`) have been removed and return `404 Not Found`. Use the clean, unified `/api/v1/...` routes listed above.

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
# Run all 45 integration tests
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

### Option A: Multi-Stage Container Deployment (Docker / Kubernetes)

A production-ready [`Dockerfile`](Dockerfile) is provided in the root directory. It uses a **3-stage build process**:
1. Builds the Svelte 5 frontend with Node 20.
2. Compiles the Rust backend with `--release` optimizations.
3. Packages the binary and static assets into an ultra-lean Debian runtime container (~90 MB) running as an **unprivileged non-root user** (`appuser`).

#### 1. Build the Docker Image
```bash
docker build -t business-rates-engine:latest .
```

#### 2. Run the Container
```bash
docker run -d \
  --name business-rates-engine \
  -p 8080:8080 \
  -e PORT=8080 \
  -e RUST_LOG=info \
  --restart unless-stopped \
  business-rates-engine:latest
```

#### 3. Kubernetes Deployment Manifest (`k8s-deployment.yaml`)
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

The REST API is **100% unauthenticated**; no API keys (`X-API-Key`) or bearer tokens are required.

### 1. Calculate Liability (Standard Scenario)
```bash
curl -s -X POST http://localhost:8080/api/v1/calculate-liability \
  -H "Content-Type: application/json" \
  -d @tests/fixtures/scenario_1_sbrr_rhl_england.json
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

# Rating Multipliers (1993–2027)
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

---

## 10. Project Directory Structure

```
business_rates_engine/
├── docs/                         # Official API Specifications
│   ├── openapi.yaml              # OpenAPI 3.1.0 specification (YAML)
│   └── openapi.json              # OpenAPI 3.1.0 specification (JSON)
├── templates/                    # Server-rendered HTML templates
│   └── scalar.html               # Configurable, offline Scalar UI template
├── frontend/                     # Modern Svelte 5 SPA framework
│   ├── public/assets/            # Self-hosted static assets (scalar.js)
│   ├── src/
│   │   ├── components/           # UI Components (Navbar, S.44A Table, Matrix, Catalogs)
│   │   ├── stores/               # Reactive calculation & form state ($state Runes)
│   │   ├── i18n/                 # Dictionaries (en, cy, sco, gd)
│   │   └── types/                # TypeScript statutory interfaces
│   ├── vite.config.ts            # Vite 6 configuration (with API proxy)
│   └── package.json
├── src/                          # High-performance Axum backend
│   ├── api/                      # REST Handlers, Routers & OWASP middleware
│   ├── db/                       # ParameterRepository (1993–2027 statutory parameters)
│   ├── engine/                   # CalculationEngine (Statutory slicer, multipliers, reliefs)
│   ├── models/                   # Calculation requests, responses, enums
│   ├── error.rs                  # Localized AppError handling
│   ├── i18n.rs                   # Multi-language resolution & narrative generation
│   ├── lib.rs
│   └── main.rs                   # Server entrypoint (supports PORT env var)
├── tests/
│   ├── fixtures/                 # Pre-configured test scenario JSON files (Scenarios 1-5)
│   └── integration_tests.rs      # Complete test suite (45 passing tests)
├── build-frontend.sh             # Build script for Svelte 5 frontend
├── build-css.sh                  # Standalone Tailwind CSS compiler
├── watch-css.sh                  # Tailwind CSS watch mode
├── Dockerfile                    # Multi-stage production container build
└── Cargo.toml                    # Rust dependencies & package manifest
```
