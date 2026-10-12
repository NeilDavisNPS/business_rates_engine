#!/usr/bin/env node
/**
 * Generates:
 * 1. THIRD_PARTY_LICENSES.md (Raw markdown document for repository & containers)
 * 2. templates/licenses.html (Accessible HTML page served at /licenses using the existing stylesheet)
 *
 * Scans:
 * - Cargo metadata for all Rust dependencies
 * - npm package.json for runtime production frontend dependencies (excluding dev/build tools)
 * - Standalone binaries & embedded assets (Tailwind CSS, Scalar)
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const ROOT_DIR = path.resolve(__dirname, '..');
const MD_OUTPUT_PATH = path.join(ROOT_DIR, 'THIRD_PARTY_LICENSES.md');
const HTML_OUTPUT_PATH = path.join(ROOT_DIR, 'templates', 'licenses.html');

console.log('Reading Rust Cargo metadata...');
let cargoMetadata;
try {
  const raw = execSync('cargo metadata --format-version 1', {
    cwd: ROOT_DIR,
    encoding: 'utf8',
    maxBuffer: 50 * 1024 * 1024,
  });
  cargoMetadata = JSON.parse(raw);
} catch (err) {
  console.error('Failed to run cargo metadata. Make sure cargo is in PATH.', err.message);
  process.exit(1);
}

// 1. Traverse Rust runtime dependency graph (excluding build.rs and dev-dependencies)
const rootPkg = cargoMetadata.packages.find(p => p.name === 'business_rates_engine');
const nodeMap = new Map();
for (const n of cargoMetadata.resolve.nodes) {
  nodeMap.set(n.id, n);
}

const runtimePkgIds = new Set();
const queue = [];
const rootNode = nodeMap.get(rootPkg.id);
for (const d of rootNode.deps) {
  if (d.dep_kinds.some(dk => dk.kind === null)) {
    runtimePkgIds.add(d.pkg);
    queue.push(d.pkg);
  }
}
while (queue.length > 0) {
  const currentId = queue.shift();
  const node = nodeMap.get(currentId);
  if (!node) continue;
  for (const d of node.deps) {
    if (d.dep_kinds.some(dk => dk.kind === null) && !runtimePkgIds.has(d.pkg)) {
      runtimePkgIds.add(d.pkg);
      queue.push(d.pkg);
    }
  }
}

const rustCratesMap = new Map();
for (const pkg of cargoMetadata.packages) {
  if (pkg.name === 'business_rates_engine') continue;
  if (runtimePkgIds.has(pkg.id) && !rustCratesMap.has(pkg.name)) {
    rustCratesMap.set(pkg.name, {
      name: pkg.name,
      license: pkg.license || 'Custom / See Repository',
      repository: pkg.repository || 'N/A',
    });
  }
}
const rustCrates = Array.from(rustCratesMap.values()).sort((a, b) => a.name.localeCompare(b.name));
console.log(`Found ${rustCrates.length} unique runtime Rust crates (excluding build-only dependencies).`);

// 2. Traverse frontend runtime dependency graph (including svelte and its runtime dependencies, excluding build tools)
console.log('Reading frontend runtime dependencies from package-lock.json...');
const pkgJsonPath = path.join(ROOT_DIR, 'frontend', 'package.json');
const lockPath = path.join(ROOT_DIR, 'frontend', 'package-lock.json');
const npmPackagesMap = new Map();

if (fs.existsSync(lockPath)) {
  const lock = JSON.parse(fs.readFileSync(lockPath, 'utf8'));
  let prodRoots = [];
  if (fs.existsSync(pkgJsonPath)) {
    const pkgJson = JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'));
    prodRoots = Object.keys(pkgJson.dependencies || {});
  }
  if (prodRoots.length === 0 && lock.packages['node_modules/svelte']) {
    prodRoots = ['svelte'];
  }

  const visited = new Set();
  const npmQueue = [...prodRoots];

  while (npmQueue.length > 0) {
    const pkgName = npmQueue.shift();
    if (visited.has(pkgName)) continue;
    visited.add(pkgName);

    const lockEntry = lock.packages ? (lock.packages[`node_modules/${pkgName}`] || lock.packages[pkgName]) : null;
    const license = (lockEntry && lockEntry.license) || 'MIT';
    npmPackagesMap.set(pkgName, {
      name: pkgName,
      license,
    });

    if (lockEntry && lockEntry.dependencies) {
      for (const dep of Object.keys(lockEntry.dependencies)) {
        if (!visited.has(dep)) {
          npmQueue.push(dep);
        }
      }
    }
  }
}
const npmPackages = Array.from(npmPackagesMap.values()).sort((a, b) => a.name.localeCompare(b.name));
console.log(`Found ${npmPackages.length} unique runtime frontend packages (including svelte and its runtime dependencies).`);

const now = new Date().toISOString().split('T')[0];

const standaloneAssets = [
  {
    name: 'Tailwind CSS Standalone CLI',
    path: 'bin/tailwindcss',
    vendor: 'Tailwind Labs, Inc.',
    license: 'MIT',
    description: 'Standalone binary compiler for Tailwind CSS v4',
  },
  {
    name: 'Scalar API Reference',
    path: 'templates/scalar.html, scalar.js',
    vendor: 'Scalar Inc.',
    license: 'MIT',
    description: 'Embedded interactive OpenAPI documentation interface',
  },
];

const licenseTexts = {
  MIT: `Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.`,

  Apache2: `                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.
   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

2. Grant of Copyright License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   copyright license to reproduce, prepare Derivative Works of,
   publicly display, publicly perform, sublicense, and distribute the
   Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   (except as stated in this section) patent license to make, have made,
   use, offer to sell, sell, import, and otherwise transfer the Work.

4. Redistribution. You may reproduce and distribute copies of the
   Work or Derivative Works thereof in any medium, with or without
   modifications, and in Source or Object form, provided that You
   meet the following conditions:

   (a) You must give any other recipients of the Work or
       Derivative Works a copy of this License; and

   (b) You must cause any modified files to carry prominent notices
       stating that You changed the files; and

   (c) You must retain, in the Source form of any Derivative Works
       that You distribute, all copyright, patent, trademark, and
       attribution notices from the Source form of the Work; and

   (d) If the Work includes a "NOTICE" text file as part of its
       distribution, then any Derivative Works that You distribute must
       include a readable copy of the attribution notices contained
       within such NOTICE file.

5. Disclaimer of Warranty. Unless required by applicable law or
   agreed to in writing, Licensor provides the Work (and each
   Contributor provides its Contributions) on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
   implied, including, without limitation, any warranties or conditions
   of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
   PARTICULAR PURPOSE.`,

  BSD: `Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors
   may be used to endorse or promote products derived from this software
   without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.`,

  ISC: `Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.`,

  Unicode: `COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2023 Unicode, Inc. All rights reserved.
Distributed under the Terms of Use in https://www.unicode.org/copyright.html.

Permission is hereby granted, free of charge, to any person obtaining
a copy of the Unicode data files and any associated documentation
(the "Data Files") or Unicode software and any associated documentation
(the "Software") to deal in the Data Files or Software
without restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Data Files or Software, and to permit persons to whom
the Data Files or Software are furnished to do so, provided that either
(a) this copyright and permission notice appear with all copies
of the Data Files or Software, or
(b) this copyright and permission notice appear in associated
Documentation.`,
};

// ============================================================================
// 1. Generate Markdown Document
// ============================================================================
const mdContent = `# Third-Party Software Licenses

Third-party open-source libraries and embedded assets incorporated into the **UK Statutory Business Rates Engine**.

Generated automatically on: **${now}**

---

## 1. Rust Backend Dependencies

| Crate | License | Repository |
| :--- | :--- | :--- |
${rustCrates.map(c => `| **${c.name}** | ${c.license} | ${c.repository !== 'N/A' ? `[Link](${c.repository})` : 'N/A'} |`).join('\n')}

---

## 2. Frontend Dependencies

| Package | License |
| :--- | :--- |
${npmPackages.map(p => `| **${p.name}** | ${p.license} |`).join('\n')}

---

## 3. Standalone Binaries & Embedded Assets

| Asset / Tool | Vendor / Author | License | Description |
| :--- | :--- | :--- | :--- |
${standaloneAssets.map(a => `| **${a.name}** (\`${a.path}\`) | ${a.vendor} | ${a.license} | ${a.description} |`).join('\n')}

---

## 4. License Texts

### The MIT License
\`\`\`
${licenseTexts.MIT}
\`\`\`

### The Apache License, Version 2.0
\`\`\`
${licenseTexts.Apache2}
\`\`\`

### The BSD 2-Clause / 3-Clause License
\`\`\`
${licenseTexts.BSD}
\`\`\`

### The ISC License
\`\`\`
${licenseTexts.ISC}
\`\`\`

### The Unicode-3.0 License
\`\`\`
${licenseTexts.Unicode}
\`\`\`
`;

fs.writeFileSync(MD_OUTPUT_PATH, mdContent, 'utf8');
console.log(`Successfully generated Markdown: ${MD_OUTPUT_PATH}`);

// ============================================================================
// 2. Generate Standalone Accessible HTML Document (templates/licenses.html)
// Strictly relies on the existing stylesheet (/assets/app.css) to adhere to CSP style-src 'self'
// ============================================================================
function escapeHtml(str) {
  return String(str)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

const htmlContent = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Third-Party Software Licenses — Business Rates Engine</title>
  <link rel="icon" type="image/svg+xml" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🏛️</text></svg>" />
  <!-- Existing stylesheet served from 'self' under CSP style-src 'self' -->
  <link rel="stylesheet" href="/assets/app.css" />
</head>
<body class="bg-slate-950 text-slate-100 font-sans antialiased min-h-screen">
  <header class="sticky top-0 z-50 bg-slate-900 border-b border-slate-800 px-6 py-3.5 flex flex-wrap items-center justify-between gap-4">
    <div class="flex items-center gap-3">
      <a href="/docs" class="text-sky-400 hover:text-sky-300 font-semibold text-sm transition-colors flex items-center gap-1.5" title="Return to API Reference">&larr; API Reference</a>
      <span class="text-slate-700" aria-hidden="true">|</span>
      <span class="text-slate-100 font-bold text-sm flex items-center gap-2"><span aria-hidden="true">🏛️</span> Business Rates Engine</span>
    </div>
    <div>
      <a href="/licenses?format=md" download="THIRD_PARTY_LICENSES.md" class="px-4 py-1.5 rounded-md bg-sky-950/80 text-sky-400 border border-sky-600/40 hover:bg-sky-900 hover:text-sky-200 text-xs font-semibold transition-colors flex items-center gap-1.5" title="Download raw Markdown format">
        <span aria-hidden="true">&#x2193;</span> Download Raw Markdown
      </a>
    </div>
  </header>

  <main class="max-w-7xl mx-auto px-6 py-8">
    <div class="bg-slate-900 border border-slate-800 rounded-xl p-6 mb-8 shadow-sm">
      <h1 class="text-2xl font-extrabold text-white tracking-tight mb-2">Third-Party Software Licenses</h1>
      <p class="text-sm text-slate-400 max-w-3xl leading-relaxed mb-5">
        Third-party open-source libraries and embedded assets incorporated into the <strong>UK Statutory Business Rates Engine</strong>. Auto-generated on <strong>${now}</strong>.
      </p>
      <nav class="flex flex-wrap items-center gap-2 pt-4 border-t border-slate-800 text-xs font-medium text-slate-400" aria-label="Section navigation">
        <span class="text-slate-400">Quick Jump:</span>
        <a href="#rust-dependencies" class="px-3 py-1 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-200 hover:text-sky-300 border border-slate-700 transition-colors">Rust Backend</a>
        <a href="#frontend-dependencies" class="px-3 py-1 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-200 hover:text-sky-300 border border-slate-700 transition-colors">Frontend</a>
        <a href="#standalone-assets" class="px-3 py-1 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-200 hover:text-sky-300 border border-slate-700 transition-colors">Embedded Assets</a>
        <a href="#license-texts" class="px-3 py-1 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-200 hover:text-sky-300 border border-slate-700 transition-colors">License Texts</a>
      </nav>
    </div>

    <section id="rust-dependencies" class="mb-10">
      <h2 class="text-xl font-bold text-white mb-4 pb-2 border-b border-slate-800">1. Rust Backend Dependencies</h2>
      <div class="bg-slate-900 border border-slate-800 rounded-lg overflow-x-auto shadow-sm">
        <table class="w-full text-left text-sm border-collapse">
          <thead>
            <tr class="bg-slate-800/80 text-slate-300 text-xs uppercase tracking-wider border-b border-slate-700">
              <th class="px-4 py-3 font-semibold">Crate</th>
              <th class="px-4 py-3 font-semibold">License</th>
              <th class="px-4 py-3 font-semibold">Repository</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-800">
            ${rustCrates.map(c => `<tr class="hover:bg-slate-800/50 transition-colors">
              <td class="px-4 py-3 font-semibold text-white">${escapeHtml(c.name)}</td>
              <td class="px-4 py-3"><span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-mono font-semibold bg-sky-950 text-sky-300 border border-sky-800/50">${escapeHtml(c.license)}</span></td>
              <td class="px-4 py-3 text-slate-400">${c.repository !== 'N/A' ? `<a href="${escapeHtml(c.repository)}" class="text-sky-400 hover:text-sky-300 hover:underline inline-flex items-center gap-1" target="_blank" rel="noopener noreferrer">${escapeHtml(c.repository)} <span aria-hidden="true">&#x2197;</span></a>` : '<span class="text-slate-400">N/A</span>'}</td>
            </tr>`).join('\n            ')}
          </tbody>
        </table>
      </div>
    </section>

    <section id="frontend-dependencies" class="mb-10">
      <h2 class="text-xl font-bold text-white mb-4 pb-2 border-b border-slate-800">2. Frontend Dependencies</h2>
      <div class="bg-slate-900 border border-slate-800 rounded-lg overflow-x-auto shadow-sm">
        <table class="w-full text-left text-sm border-collapse">
          <thead>
            <tr class="bg-slate-800/80 text-slate-300 text-xs uppercase tracking-wider border-b border-slate-700">
              <th class="px-4 py-3 font-semibold">Package</th>
              <th class="px-4 py-3 font-semibold">License</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-800">
            ${npmPackages.map(p => `<tr class="hover:bg-slate-800/50 transition-colors">
              <td class="px-4 py-3 font-semibold text-white">${escapeHtml(p.name)}</td>
              <td class="px-4 py-3"><span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-mono font-semibold bg-indigo-950 text-indigo-300 border border-indigo-800/50">${escapeHtml(p.license)}</span></td>
            </tr>`).join('\n            ')}
          </tbody>
        </table>
      </div>
    </section>

    <section id="standalone-assets" class="mb-10">
      <h2 class="text-xl font-bold text-white mb-4 pb-2 border-b border-slate-800">3. Standalone Binaries &amp; Embedded Assets</h2>
      <div class="bg-slate-900 border border-slate-800 rounded-lg overflow-x-auto shadow-sm">
        <table class="w-full text-left text-sm border-collapse">
          <thead>
            <tr class="bg-slate-800/80 text-slate-300 text-xs uppercase tracking-wider border-b border-slate-700">
              <th class="px-4 py-3 font-semibold">Asset / Tool</th>
              <th class="px-4 py-3 font-semibold">Vendor / Author</th>
              <th class="px-4 py-3 font-semibold">License</th>
              <th class="px-4 py-3 font-semibold">Description</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-800">
            ${standaloneAssets.map(a => `<tr class="hover:bg-slate-800/50 transition-colors">
              <td class="px-4 py-3 font-semibold text-white">${escapeHtml(a.name)} <code class="text-xs bg-slate-950 text-slate-300 px-1.5 py-0.5 rounded border border-slate-800">${escapeHtml(a.path)}</code></td>
              <td class="px-4 py-3 text-slate-300">${escapeHtml(a.vendor)}</td>
              <td class="px-4 py-3"><span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-mono font-semibold bg-teal-950 text-teal-300 border border-teal-800/50">${escapeHtml(a.license)}</span></td>
              <td class="px-4 py-3 text-slate-400 text-xs">${escapeHtml(a.description)}</td>
            </tr>`).join('\n            ')}
          </tbody>
        </table>
      </div>
    </section>

    <section id="license-texts" class="mb-12">
      <h2 class="text-xl font-bold text-white mb-4 pb-2 border-b border-slate-800">4. License Texts</h2>

      <details class="bg-slate-900 border border-slate-800 rounded-lg mb-4 overflow-hidden" open>
        <summary class="px-5 py-3.5 cursor-pointer font-bold text-slate-200 hover:bg-slate-800/70 flex items-center justify-between select-none transition-colors">
          <span class="flex items-center gap-3">
            <span>The MIT License</span>
            <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-mono font-semibold bg-sky-950 text-sky-300 border border-sky-800/50">MIT</span>
          </span>
        </summary>
        <div class="border-t border-slate-800">
          <pre class="p-4 bg-slate-950 font-mono text-xs text-slate-300 leading-relaxed overflow-x-auto whitespace-pre-wrap">${escapeHtml(licenseTexts.MIT)}</pre>
        </div>
      </details>

      <details class="bg-slate-900 border border-slate-800 rounded-lg mb-4 overflow-hidden">
        <summary class="px-5 py-3.5 cursor-pointer font-bold text-slate-200 hover:bg-slate-800/70 flex items-center justify-between select-none transition-colors">
          <span class="flex items-center gap-3">
            <span>The Apache License, Version 2.0</span>
            <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-mono font-semibold bg-indigo-950 text-indigo-300 border border-indigo-800/50">Apache-2.0</span>
          </span>
        </summary>
        <div class="border-t border-slate-800">
          <pre class="p-4 bg-slate-950 font-mono text-xs text-slate-300 leading-relaxed overflow-x-auto whitespace-pre-wrap">${escapeHtml(licenseTexts.Apache2)}</pre>
        </div>
      </details>

      <details class="bg-slate-900 border border-slate-800 rounded-lg mb-4 overflow-hidden">
        <summary class="px-5 py-3.5 cursor-pointer font-bold text-slate-200 hover:bg-slate-800/70 flex items-center justify-between select-none transition-colors">
          <span class="flex items-center gap-3">
            <span>The BSD 2-Clause / 3-Clause License</span>
            <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-mono font-semibold bg-teal-950 text-teal-300 border border-teal-800/50">BSD</span>
          </span>
        </summary>
        <div class="border-t border-slate-800">
          <pre class="p-4 bg-slate-950 font-mono text-xs text-slate-300 leading-relaxed overflow-x-auto whitespace-pre-wrap">${escapeHtml(licenseTexts.BSD)}</pre>
        </div>
      </details>

      <details class="bg-slate-900 border border-slate-800 rounded-lg mb-4 overflow-hidden">
        <summary class="px-5 py-3.5 cursor-pointer font-bold text-slate-200 hover:bg-slate-800/70 flex items-center justify-between select-none transition-colors">
          <span class="flex items-center gap-3">
            <span>The ISC License</span>
            <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-mono font-semibold bg-amber-950 text-amber-300 border border-amber-800/50">ISC</span>
          </span>
        </summary>
        <div class="border-t border-slate-800">
          <pre class="p-4 bg-slate-950 font-mono text-xs text-slate-300 leading-relaxed overflow-x-auto whitespace-pre-wrap">${escapeHtml(licenseTexts.ISC)}</pre>
        </div>
      </details>

      <details class="bg-slate-900 border border-slate-800 rounded-lg mb-4 overflow-hidden">
        <summary class="px-5 py-3.5 cursor-pointer font-bold text-slate-200 hover:bg-slate-800/70 flex items-center justify-between select-none transition-colors">
          <span class="flex items-center gap-3">
            <span>The Unicode-3.0 License</span>
            <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-mono font-semibold bg-purple-950 text-purple-300 border border-purple-800/50">Unicode-3.0</span>
          </span>
        </summary>
        <div class="border-t border-slate-800">
          <pre class="p-4 bg-slate-950 font-mono text-xs text-slate-300 leading-relaxed overflow-x-auto whitespace-pre-wrap">${escapeHtml(licenseTexts.Unicode)}</pre>
        </div>
      </details>
    </section>
  </main>
</body>
</html>
`;

fs.writeFileSync(HTML_OUTPUT_PATH, htmlContent, 'utf8');
console.log(`Successfully generated HTML: ${HTML_OUTPUT_PATH}`);
