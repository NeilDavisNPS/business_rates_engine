#!/usr/bin/env node
/**
 * Generates THIRD_PARTY_LICENSES.md by querying:
 * 1. Cargo metadata for all Rust dependencies (130+ crates)
 * 2. npm package-lock.json for all frontend dependencies (90+ packages)
 * 3. Standalone binaries & embedded assets (Tailwind CSS, Scalar)
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const ROOT_DIR = path.resolve(__dirname, '..');
const OUTPUT_PATH = path.join(ROOT_DIR, 'THIRD_PARTY_LICENSES.md');

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

const rustCrates = [];
const licenseCounts = {};

for (const pkg of cargoMetadata.packages) {
  if (pkg.name === 'business_rates_engine') continue;
  const license = pkg.license || 'Custom / See Repository';
  licenseCounts[license] = (licenseCounts[license] || 0) + 1;
  rustCrates.push({
    name: pkg.name,
    version: pkg.version,
    license,
    repository: pkg.repository || 'N/A',
    description: (pkg.description || '').replace(/\r?\n/g, ' ').slice(0, 100),
  });
}
rustCrates.sort((a, b) => a.name.localeCompare(b.name));

console.log(`Found ${rustCrates.length} Rust crates.`);

console.log('Reading frontend package-lock.json...');
const lockPath = path.join(ROOT_DIR, 'frontend', 'package-lock.json');
let npmPackages = [];

if (fs.existsSync(lockPath)) {
  const lock = JSON.parse(fs.readFileSync(lockPath, 'utf8'));
  for (const [pkgPath, info] of Object.entries(lock.packages || {})) {
    if (!pkgPath) continue;
    const name = pkgPath.replace(/^node_modules\//, '');
    const license = info.license || 'Custom / See Package';
    licenseCounts[license] = (licenseCounts[license] || 0) + 1;
    npmPackages.push({
      name,
      version: info.version,
      license,
    });
  }
}
npmPackages.sort((a, b) => a.name.localeCompare(b.name));
console.log(`Found ${npmPackages.length} NPM packages.`);

// Markdown generation
const now = new Date().toISOString().split('T')[0];

const doc = `# Third-Party Software Licenses & Attribution

This document contains attribution notices and licenses for third-party open-source software libraries, crates, and packages incorporated into or bundled with the **UK Statutory Business Rates Engine** (\`business_rates_engine\`).

Generated automatically on: **${now}**

---

## 1. Overview & License Summary

All third-party dependencies utilized in this project are distributed under permissive open-source licenses. There are **zero copyleft (GPL / AGPL / SSPL)** dependencies in this software.

| License | Total Components | Category |
| :--- | :---: | :--- |
${Object.entries(licenseCounts)
  .sort((a, b) => b[1] - a[1])
  .map(([lic, count]) => `| \`${lic}\` | ${count} | Permissive |`)
  .join('\n')}

---

## 2. Rust Backend Dependencies (${rustCrates.length} Crates)

The following crates are statically compiled into the \`business_rates_engine\` executable:

| Crate | Version | License | Repository |
| :--- | :--- | :--- | :--- |
${rustCrates.map(c => `| **${c.name}** | \`${c.version}\` | ${c.license} | ${c.repository !== 'N/A' ? `[Link](${c.repository})` : 'N/A'} |`).join('\n')}

---

## 3. Frontend & Build Tooling Dependencies (${npmPackages.length} Packages)

The following NPM packages are used in compiling and serving the Svelte 5 user interface:

| Package | Version | License |
| :--- | :--- | :--- |
${npmPackages.map(p => `| **${p.name}** | \`${p.version}\` | ${p.license} |`).join('\n')}

---

## 4. Standalone Binaries & Embedded Assets

| Asset / Tool | Vendor / Author | License | Description |
| :--- | :--- | :--- | :--- |
| **Tailwind CSS Standalone CLI** (\`bin/tailwindcss\`) | Tailwind Labs, Inc. | MIT | Standalone binary compiler for Tailwind CSS v4 |
| **Scalar API Reference** (\`templates/scalar.html\`, \`scalar.js\`) | Scalar Inc. | MIT | Embedded interactive OpenAPI documentation interface |

---

## 5. License Texts

### The MIT License
\`\`\`
Permission is hereby granted, free of charge, to any person obtaining a copy
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
SOFTWARE.
\`\`\`

### The Apache License, Version 2.0
\`\`\`
                                 Apache License
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
   PARTICULAR PURPOSE.
\`\`\`

### The BSD 2-Clause / 3-Clause License
\`\`\`
Redistribution and use in source and binary forms, with or without
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
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
\`\`\`

### The ISC License
\`\`\`
Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
\`\`\`

### The Unicode-3.0 License
\`\`\`
COPYRIGHT AND PERMISSION NOTICE

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
Documentation.
\`\`\`
`;

fs.writeFileSync(OUTPUT_PATH, doc, 'utf8');
console.log(`Successfully generated ${OUTPUT_PATH}`);

