#!/usr/bin/env node
/**
 * Automated CI License Guardrail:
 * Verifies that 100% of Rust crates and NPM packages comply with the
 * permissive open-source license policy (zero copyleft, zero unapproved licenses).
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const ROOT_DIR = path.resolve(__dirname, '..');

const ALLOWED_LICENSE_PATTERNS = [
  /^MIT$/,
  /^Apache-2\.0$/,
  /^ISC$/,
  /^BSD-2-Clause$/,
  /^BSD-3-Clause$/,
  /^Unlicense$/,
  /^Unicode-3\.0$/,
  /^BSL-1\.0$/,
  /^Apache-2\.0 WITH LLVM-exception$/,
];

function isPermissive(licenseStr) {
  if (!licenseStr || licenseStr === 'UNKNOWN') return false;

  // Split OR clauses: if any option in an OR group is permissive, it can be chosen
  // e.g. "MIT OR Apache-2.0", "MIT OR Apache-2.0 OR LGPL-2.1-or-later"
  const orOptions = licenseStr.split(/\s+OR\s+/i);
  for (const opt of orOptions) {
    const cleaned = opt.replace(/[()]/g, '').trim();
    // Split AND clauses: in an AND expression, all parts must be permissive
    const andParts = cleaned.split(/\s+AND\s+/i);
    const allAndPermissive = andParts.every(part => {
      const pClean = part.replace(/[()]/g, '').trim();
      return ALLOWED_LICENSE_PATTERNS.some(pat => pat.test(pClean));
    });
    if (allAndPermissive) {
      return true;
    }
  }

  // Also check slash separated like MIT/Apache-2.0
  if (licenseStr.includes('/')) {
    const slashParts = licenseStr.split('/');
    if (slashParts.some(p => ALLOWED_LICENSE_PATTERNS.some(pat => pat.test(p.trim())))) {
      return true;
    }
  }

  return false;
}

console.log('--- License Guardrail Audit ---');
let violations = [];

// 1. Audit Rust crates
try {
  const raw = execSync('cargo metadata --format-version 1', {
    cwd: ROOT_DIR,
    encoding: 'utf8',
    maxBuffer: 50 * 1024 * 1024,
  });
  const metadata = JSON.parse(raw);
  let rustCount = 0;

  for (const pkg of metadata.packages) {
    if (pkg.name === 'business_rates_engine') continue;
    rustCount++;
    if (!isPermissive(pkg.license)) {
      violations.push({
        ecosystem: 'Rust',
        name: pkg.name,
        version: pkg.version,
        license: pkg.license || 'NONE',
      });
    }
  }
  console.log(`Audited ${rustCount} Rust crates.`);
} catch (err) {
  console.error('Failed to audit Rust dependencies:', err.message);
  process.exit(1);
}

// 2. Audit NPM packages
const lockPath = path.join(ROOT_DIR, 'frontend', 'package-lock.json');
if (fs.existsSync(lockPath)) {
  const lock = JSON.parse(fs.readFileSync(lockPath, 'utf8'));
  let npmCount = 0;
  for (const [pkgPath, info] of Object.entries(lock.packages || {})) {
    if (!pkgPath) continue;
    const name = pkgPath.replace(/^node_modules\//, '');
    npmCount++;
    if (!isPermissive(info.license)) {
      violations.push({
        ecosystem: 'NPM',
        name,
        version: info.version,
        license: info.license || 'NONE',
      });
    }
  }
  console.log(`Audited ${npmCount} NPM packages.`);
}

if (violations.length > 0) {
  console.error('\n❌ LICENSE POLICY VIOLATIONS DETECTED:');
  for (const v of violations) {
    console.error(`  [${v.ecosystem}] ${v.name}@${v.version} - License: "${v.license}" (Not in approved permissive list)`);
  }
  console.error('\nBuild failed: Copyleft or unapproved third-party licenses are forbidden.');
  process.exit(1);
} else {
  console.log('✅ License Guardrail Passed: 100% of dependencies use approved permissive licenses.');
  process.exit(0);
}

