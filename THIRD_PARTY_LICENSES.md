# Third-Party Software Licenses & Attribution

This document contains attribution notices and licenses for third-party open-source software libraries, crates, and packages incorporated into or bundled with the **UK Statutory Business Rates Engine** (`business_rates_engine`).

Generated automatically on: **2026-10-12**

---

## 1. Overview & License Summary

All third-party dependencies utilized in this project are distributed under permissive open-source licenses. There are **zero copyleft (GPL / AGPL / SSPL)** dependencies in this software.

| License | Total Components | Category |
| :--- | :---: | :--- |
| `MIT` | 121 | Permissive |
| `MIT OR Apache-2.0` | 79 | Permissive |
| `Apache-2.0 OR MIT` | 6 | Permissive |
| `Apache-2.0` | 5 | Permissive |
| `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | 3 | Permissive |
| `Unlicense OR MIT` | 2 | Permissive |
| `MIT OR Apache-2.0 OR LGPL-2.1-or-later` | 2 | Permissive |
| `BSD-2-Clause OR Apache-2.0 OR MIT` | 2 | Permissive |
| `MIT AND BSD-3-Clause` | 1 | Permissive |
| `Apache-2.0 OR BSL-1.0` | 1 | Permissive |
| `MIT/Apache-2.0` | 1 | Permissive |
| `(MIT OR Apache-2.0) AND Unicode-3.0` | 1 | Permissive |
| `ISC` | 1 | Permissive |
| `BSD-3-Clause` | 1 | Permissive |

---

## 2. Rust Backend Dependencies (130 Crates)

The following crates are statically compiled into the `business_rates_engine` executable:

| Crate | Version | License | Repository |
| :--- | :--- | :--- | :--- |
| **aho-corasick** | `1.1.5` | Unlicense OR MIT | [Link](https://github.com/BurntSushi/aho-corasick) |
| **android_system_properties** | `0.1.6` | MIT OR Apache-2.0 | [Link](https://github.com/nical/android_system_properties) |
| **arrayvec** | `0.7.8` | MIT OR Apache-2.0 | [Link](https://github.com/bluss/arrayvec) |
| **async-trait** | `0.1.92` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/async-trait) |
| **atomic-waker** | `1.1.2` | Apache-2.0 OR MIT | [Link](https://github.com/smol-rs/atomic-waker) |
| **autocfg** | `1.5.1` | Apache-2.0 OR MIT | [Link](https://github.com/cuviper/autocfg) |
| **axum** | `0.7.9` | MIT | [Link](https://github.com/tokio-rs/axum) |
| **axum-core** | `0.4.5` | MIT | [Link](https://github.com/tokio-rs/axum) |
| **axum-macros** | `0.4.2` | MIT | [Link](https://github.com/tokio-rs/axum) |
| **bitflags** | `2.13.2` | MIT OR Apache-2.0 | [Link](https://github.com/bitflags/bitflags) |
| **borsh** | `1.8.1` | MIT OR Apache-2.0 | [Link](https://github.com/near/borsh-rs) |
| **borsh-derive** | `1.8.1` | Apache-2.0 | [Link](https://github.com/near/borsh-rs) |
| **bumpalo** | `3.20.3` | MIT OR Apache-2.0 | [Link](https://github.com/fitzgen/bumpalo) |
| **bytes** | `1.12.1` | MIT | [Link](https://github.com/tokio-rs/bytes) |
| **cc** | `1.7.0` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/cc-rs) |
| **cfg_aliases** | `0.2.2` | MIT | [Link](https://github.com/katharostech/cfg_aliases) |
| **cfg-if** | `1.0.5` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/cfg-if) |
| **chrono** | `0.4.45` | MIT OR Apache-2.0 | [Link](https://github.com/chronotope/chrono) |
| **core-foundation-sys** | `0.8.7` | MIT OR Apache-2.0 | [Link](https://github.com/servo/core-foundation-rs) |
| **equivalent** | `1.0.2` | Apache-2.0 OR MIT | [Link](https://github.com/indexmap-rs/equivalent) |
| **errno** | `0.3.14` | MIT OR Apache-2.0 | [Link](https://github.com/lambda-fairy/rust-errno) |
| **find-msvc-tools** | `0.1.14` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/cc-rs) |
| **form_urlencoded** | `1.2.2` | MIT OR Apache-2.0 | [Link](https://github.com/servo/rust-url) |
| **futures-core** | `0.3.34` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/futures-rs) |
| **futures-task** | `0.3.34` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/futures-rs) |
| **futures-util** | `0.3.34` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/futures-rs) |
| **getrandom** | `0.2.17` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/getrandom) |
| **getrandom** | `0.3.4` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/getrandom) |
| **getrandom** | `0.4.3` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/getrandom) |
| **hashbrown** | `0.17.1` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/hashbrown) |
| **http** | `1.5.0` | MIT OR Apache-2.0 | [Link](https://github.com/hyperium/http) |
| **http-body** | `1.1.0` | MIT | [Link](https://github.com/hyperium/http-body) |
| **http-body-util** | `0.1.5` | MIT | [Link](https://github.com/hyperium/http-body) |
| **httparse** | `1.10.1` | MIT OR Apache-2.0 | [Link](https://github.com/seanmonstar/httparse) |
| **httpdate** | `1.0.3` | MIT OR Apache-2.0 | [Link](https://github.com/pyfisch/httpdate) |
| **hyper** | `1.12.0` | MIT | [Link](https://github.com/hyperium/hyper) |
| **hyper-util** | `0.1.21` | MIT | [Link](https://github.com/hyperium/hyper-util) |
| **iana-time-zone** | `0.1.65` | MIT OR Apache-2.0 | [Link](https://github.com/strawlab/iana-time-zone) |
| **iana-time-zone-haiku** | `0.1.2` | MIT OR Apache-2.0 | [Link](https://github.com/strawlab/iana-time-zone) |
| **indexmap** | `2.14.2` | Apache-2.0 OR MIT | [Link](https://github.com/indexmap-rs/indexmap) |
| **itoa** | `1.0.18` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/itoa) |
| **js-sys** | `0.3.106` | MIT OR Apache-2.0 | [Link](https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/js-sys) |
| **lazy_static** | `1.5.1` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang-nursery/lazy-static.rs) |
| **libc** | `0.2.190` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/libc) |
| **lock_api** | `0.4.14` | MIT OR Apache-2.0 | [Link](https://github.com/Amanieu/parking_lot) |
| **log** | `0.4.34` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/log) |
| **matchers** | `0.2.0` | MIT | [Link](https://github.com/hawkw/matchers) |
| **matchit** | `0.7.3` | MIT AND BSD-3-Clause | [Link](https://github.com/ibraheemdev/matchit) |
| **memchr** | `2.8.3` | Unlicense OR MIT | [Link](https://github.com/BurntSushi/memchr) |
| **mime** | `0.3.17` | MIT OR Apache-2.0 | [Link](https://github.com/hyperium/mime) |
| **mio** | `1.2.4` | MIT | [Link](https://github.com/tokio-rs/mio) |
| **nu-ansi-term** | `0.50.3` | MIT | [Link](https://github.com/nushell/nu-ansi-term) |
| **num-traits** | `0.2.19` | MIT OR Apache-2.0 | [Link](https://github.com/rust-num/num-traits) |
| **once_cell** | `1.21.4` | MIT OR Apache-2.0 | [Link](https://github.com/matklad/once_cell) |
| **parking_lot** | `0.12.5` | MIT OR Apache-2.0 | [Link](https://github.com/Amanieu/parking_lot) |
| **parking_lot_core** | `0.9.12` | MIT OR Apache-2.0 | [Link](https://github.com/Amanieu/parking_lot) |
| **percent-encoding** | `2.3.2` | MIT OR Apache-2.0 | [Link](https://github.com/servo/rust-url/) |
| **pin-project-lite** | `0.2.17` | Apache-2.0 OR MIT | [Link](https://github.com/taiki-e/pin-project-lite) |
| **ppv-lite86** | `0.2.21` | MIT OR Apache-2.0 | [Link](https://github.com/cryptocorrosion/cryptocorrosion) |
| **proc-macro-crate** | `3.5.0` | MIT OR Apache-2.0 | [Link](https://github.com/bkchr/proc-macro-crate) |
| **proc-macro2** | `1.0.107` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/proc-macro2) |
| **quote** | `1.0.47` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/quote) |
| **r-efi** | `5.3.0` | MIT OR Apache-2.0 OR LGPL-2.1-or-later | [Link](https://github.com/r-efi/r-efi) |
| **r-efi** | `6.0.0` | MIT OR Apache-2.0 OR LGPL-2.1-or-later | [Link](https://github.com/r-efi/r-efi) |
| **rand** | `0.8.8` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/rand) |
| **rand** | `0.9.5` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/rand) |
| **rand_chacha** | `0.3.1` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/rand) |
| **rand_chacha** | `0.9.0` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/rand) |
| **rand_core** | `0.6.4` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/rand) |
| **rand_core** | `0.9.5` | MIT OR Apache-2.0 | [Link](https://github.com/rust-random/rand) |
| **redox_syscall** | `0.5.18` | MIT | [Link](https://gitlab.redox-os.org/redox-os/syscall) |
| **regex-automata** | `0.4.18` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/regex) |
| **regex-syntax** | `0.8.11` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/regex) |
| **rust_decimal** | `1.43.0` | MIT | [Link](https://github.com/paupino/rust-decimal) |
| **rust_decimal_macros** | `1.40.0` | MIT | [Link](https://github.com/paupino/rust-decimal) |
| **rustversion** | `1.0.23` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/rustversion) |
| **ryu** | `1.0.23` | Apache-2.0 OR BSL-1.0 | [Link](https://github.com/dtolnay/ryu) |
| **scopeguard** | `1.2.0` | MIT OR Apache-2.0 | [Link](https://github.com/bluss/scopeguard) |
| **serde** | `1.0.229` | MIT OR Apache-2.0 | [Link](https://github.com/serde-rs/serde) |
| **serde_core** | `1.0.229` | MIT OR Apache-2.0 | [Link](https://github.com/serde-rs/serde) |
| **serde_derive** | `1.0.229` | MIT OR Apache-2.0 | [Link](https://github.com/serde-rs/serde) |
| **serde_json** | `1.0.152` | MIT OR Apache-2.0 | [Link](https://github.com/serde-rs/json) |
| **serde_path_to_error** | `0.1.20` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/path-to-error) |
| **serde_urlencoded** | `0.7.1` | MIT/Apache-2.0 | [Link](https://github.com/nox/serde_urlencoded) |
| **sharded-slab** | `0.1.7` | MIT | [Link](https://github.com/hawkw/sharded-slab) |
| **shlex** | `2.0.1` | MIT OR Apache-2.0 | [Link](https://github.com/comex/rust-shlex) |
| **signal-hook-registry** | `1.4.8` | MIT OR Apache-2.0 | [Link](https://github.com/vorner/signal-hook) |
| **slab** | `0.4.12` | MIT | [Link](https://github.com/tokio-rs/slab) |
| **smallvec** | `1.16.3` | MIT OR Apache-2.0 | [Link](https://github.com/servo/rust-smallvec) |
| **socket2** | `0.6.5` | MIT OR Apache-2.0 | [Link](https://github.com/rust-lang/socket2) |
| **syn** | `2.0.119` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/syn) |
| **syn** | `3.0.7` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/syn) |
| **sync_wrapper** | `1.0.2` | Apache-2.0 | [Link](https://github.com/Actyx/sync_wrapper) |
| **thiserror** | `2.0.21` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/thiserror) |
| **thiserror-impl** | `2.0.21` | MIT OR Apache-2.0 | [Link](https://github.com/dtolnay/thiserror) |
| **thread_local** | `1.1.10` | MIT OR Apache-2.0 | [Link](https://github.com/Amanieu/thread_local-rs) |
| **tokio** | `1.53.2` | MIT | [Link](https://github.com/tokio-rs/tokio) |
| **tokio-macros** | `2.7.2` | MIT | [Link](https://github.com/tokio-rs/tokio) |
| **toml_datetime** | `1.1.2+spec-1.1.0` | MIT OR Apache-2.0 | [Link](https://github.com/toml-rs/toml) |
| **toml_edit** | `0.25.17+spec-1.1.0` | MIT OR Apache-2.0 | [Link](https://github.com/toml-rs/toml) |
| **toml_parser** | `1.1.5+spec-1.1.0` | MIT OR Apache-2.0 | [Link](https://github.com/toml-rs/toml) |
| **tower** | `0.5.3` | MIT | [Link](https://github.com/tower-rs/tower) |
| **tower-layer** | `0.3.3` | MIT | [Link](https://github.com/tower-rs/tower) |
| **tower-service** | `0.3.3` | MIT | [Link](https://github.com/tower-rs/tower) |
| **tracing** | `0.1.44` | MIT | [Link](https://github.com/tokio-rs/tracing) |
| **tracing-attributes** | `0.1.31` | MIT | [Link](https://github.com/tokio-rs/tracing) |
| **tracing-core** | `0.1.36` | MIT | [Link](https://github.com/tokio-rs/tracing) |
| **tracing-log** | `0.2.0` | MIT | [Link](https://github.com/tokio-rs/tracing) |
| **tracing-subscriber** | `0.3.23` | MIT | [Link](https://github.com/tokio-rs/tracing) |
| **unicode-ident** | `1.0.26` | (MIT OR Apache-2.0) AND Unicode-3.0 | [Link](https://github.com/dtolnay/unicode-ident) |
| **uuid** | `1.28.0` | Apache-2.0 OR MIT | [Link](https://github.com/uuid-rs/uuid) |
| **valuable** | `0.1.1` | MIT | [Link](https://github.com/tokio-rs/valuable) |
| **wasi** | `0.11.1+wasi-snapshot-preview1` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | [Link](https://github.com/bytecodealliance/wasi) |
| **wasip2** | `1.0.4+wasi-0.2.12` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | [Link](https://github.com/bytecodealliance/wasi-rs) |
| **wasm-bindgen** | `0.2.129` | MIT OR Apache-2.0 | [Link](https://github.com/wasm-bindgen/wasm-bindgen) |
| **wasm-bindgen-macro** | `0.2.129` | MIT OR Apache-2.0 | [Link](https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro) |
| **wasm-bindgen-macro-support** | `0.2.129` | MIT OR Apache-2.0 | [Link](https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/macro-support) |
| **wasm-bindgen-shared** | `0.2.129` | MIT OR Apache-2.0 | [Link](https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared) |
| **windows-core** | `0.62.2` | MIT OR Apache-2.0 | [Link](https://github.com/microsoft/windows-rs) |
| **windows-implement** | `0.60.2` | MIT OR Apache-2.0 | [Link](https://github.com/microsoft/windows-rs) |
| **windows-interface** | `0.59.3` | MIT OR Apache-2.0 | [Link](https://github.com/microsoft/windows-rs) |
| **windows-link** | `0.2.1` | MIT OR Apache-2.0 | [Link](https://github.com/microsoft/windows-rs) |
| **windows-result** | `0.4.1` | MIT OR Apache-2.0 | [Link](https://github.com/microsoft/windows-rs) |
| **windows-strings** | `0.5.1` | MIT OR Apache-2.0 | [Link](https://github.com/microsoft/windows-rs) |
| **windows-sys** | `0.61.2` | MIT OR Apache-2.0 | [Link](https://github.com/microsoft/windows-rs) |
| **winnow** | `1.0.4` | MIT | [Link](https://github.com/winnow-rs/winnow) |
| **wit-bindgen** | `0.57.1` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | [Link](https://github.com/bytecodealliance/wit-bindgen) |
| **zerocopy** | `0.8.62` | BSD-2-Clause OR Apache-2.0 OR MIT | [Link](https://github.com/google/zerocopy) |
| **zerocopy-derive** | `0.8.62` | BSD-2-Clause OR Apache-2.0 OR MIT | [Link](https://github.com/google/zerocopy) |
| **zmij** | `1.0.23` | MIT | [Link](https://github.com/dtolnay/zmij) |

---

## 3. Frontend & Build Tooling Dependencies (96 Packages)

The following NPM packages are used in compiling and serving the Svelte 5 user interface:

| Package | Version | License |
| :--- | :--- | :--- |
| **@esbuild/aix-ppc64** | `0.25.12` | MIT |
| **@esbuild/android-arm** | `0.25.12` | MIT |
| **@esbuild/android-arm64** | `0.25.12` | MIT |
| **@esbuild/android-x64** | `0.25.12` | MIT |
| **@esbuild/darwin-arm64** | `0.25.12` | MIT |
| **@esbuild/darwin-x64** | `0.25.12` | MIT |
| **@esbuild/freebsd-arm64** | `0.25.12` | MIT |
| **@esbuild/freebsd-x64** | `0.25.12` | MIT |
| **@esbuild/linux-arm** | `0.25.12` | MIT |
| **@esbuild/linux-arm64** | `0.25.12` | MIT |
| **@esbuild/linux-ia32** | `0.25.12` | MIT |
| **@esbuild/linux-loong64** | `0.25.12` | MIT |
| **@esbuild/linux-mips64el** | `0.25.12` | MIT |
| **@esbuild/linux-ppc64** | `0.25.12` | MIT |
| **@esbuild/linux-riscv64** | `0.25.12` | MIT |
| **@esbuild/linux-s390x** | `0.25.12` | MIT |
| **@esbuild/linux-x64** | `0.25.12` | MIT |
| **@esbuild/netbsd-arm64** | `0.25.12` | MIT |
| **@esbuild/netbsd-x64** | `0.25.12` | MIT |
| **@esbuild/openbsd-arm64** | `0.25.12` | MIT |
| **@esbuild/openbsd-x64** | `0.25.12` | MIT |
| **@esbuild/openharmony-arm64** | `0.25.12` | MIT |
| **@esbuild/sunos-x64** | `0.25.12` | MIT |
| **@esbuild/win32-arm64** | `0.25.12` | MIT |
| **@esbuild/win32-ia32** | `0.25.12` | MIT |
| **@esbuild/win32-x64** | `0.25.12` | MIT |
| **@jridgewell/gen-mapping** | `0.3.13` | MIT |
| **@jridgewell/remapping** | `2.3.5` | MIT |
| **@jridgewell/resolve-uri** | `3.1.2` | MIT |
| **@jridgewell/sourcemap-codec** | `1.6.0` | MIT |
| **@jridgewell/trace-mapping** | `0.3.31` | MIT |
| **@napi-rs/lzma-linux-x64-gnu** | `1.5.1` | MIT |
| **@rollup/rollup-android-arm-eabi** | `4.64.5` | MIT |
| **@rollup/rollup-android-arm64** | `4.64.5` | MIT |
| **@rollup/rollup-darwin-arm64** | `4.64.5` | MIT |
| **@rollup/rollup-darwin-x64** | `4.64.5` | MIT |
| **@rollup/rollup-freebsd-arm64** | `4.64.5` | MIT |
| **@rollup/rollup-freebsd-x64** | `4.64.5` | MIT |
| **@rollup/rollup-linux-arm-gnueabihf** | `4.64.5` | MIT |
| **@rollup/rollup-linux-arm-musleabihf** | `4.64.5` | MIT |
| **@rollup/rollup-linux-arm64-gnu** | `4.64.5` | MIT |
| **@rollup/rollup-linux-arm64-musl** | `4.64.5` | MIT |
| **@rollup/rollup-linux-loong64-gnu** | `4.64.5` | MIT |
| **@rollup/rollup-linux-loong64-musl** | `4.64.5` | MIT |
| **@rollup/rollup-linux-ppc64-gnu** | `4.64.5` | MIT |
| **@rollup/rollup-linux-ppc64-musl** | `4.64.5` | MIT |
| **@rollup/rollup-linux-riscv64-gnu** | `4.64.5` | MIT |
| **@rollup/rollup-linux-riscv64-musl** | `4.64.5` | MIT |
| **@rollup/rollup-linux-s390x-gnu** | `4.64.5` | MIT |
| **@rollup/rollup-linux-x64-gnu** | `4.64.5` | MIT |
| **@rollup/rollup-linux-x64-musl** | `4.64.5` | MIT |
| **@rollup/rollup-openbsd-x64** | `4.64.5` | MIT |
| **@rollup/rollup-openharmony-arm64** | `4.64.5` | MIT |
| **@rollup/rollup-win32-arm64-msvc** | `4.64.5` | MIT |
| **@rollup/rollup-win32-ia32-msvc** | `4.64.5` | MIT |
| **@rollup/rollup-win32-x64-gnu** | `4.64.5` | MIT |
| **@rollup/rollup-win32-x64-msvc** | `4.64.5` | MIT |
| **@sveltejs/acorn-typescript** | `1.0.13` | MIT |
| **@sveltejs/load-config** | `0.2.3` | MIT |
| **@sveltejs/vite-plugin-svelte** | `5.1.1` | MIT |
| **@sveltejs/vite-plugin-svelte-inspector** | `4.0.1` | MIT |
| **@types/estree** | `1.0.9` | MIT |
| **acorn** | `8.19.0` | MIT |
| **aria-query** | `5.3.1` | Apache-2.0 |
| **axobject-query** | `4.1.0` | Apache-2.0 |
| **chokidar** | `4.0.3` | MIT |
| **clsx** | `2.1.1` | MIT |
| **debug** | `4.4.3` | MIT |
| **deepmerge** | `4.3.1` | MIT |
| **devalue** | `5.9.4` | MIT |
| **esbuild** | `0.25.12` | MIT |
| **esm-env** | `1.2.2` | MIT |
| **esrap** | `2.4.0` | MIT |
| **fdir** | `6.5.0` | MIT |
| **fsevents** | `2.3.3` | MIT |
| **is-reference** | `3.0.3` | MIT |
| **kleur** | `4.1.5` | MIT |
| **locate-character** | `3.0.0` | MIT |
| **magic-string** | `0.30.21` | MIT |
| **mri** | `1.2.0` | MIT |
| **ms** | `2.1.3` | MIT |
| **nanoid** | `3.3.20` | MIT |
| **picocolors** | `1.1.1` | ISC |
| **picomatch** | `4.0.7` | MIT |
| **postcss** | `8.5.29` | MIT |
| **readdirp** | `4.1.2` | MIT |
| **rollup** | `4.64.5` | MIT |
| **sade** | `1.8.1` | MIT |
| **source-map-js** | `1.2.2` | BSD-3-Clause |
| **svelte** | `5.57.2` | MIT |
| **svelte-check** | `4.7.6` | MIT |
| **tinyglobby** | `0.2.17` | MIT |
| **typescript** | `5.9.3` | Apache-2.0 |
| **vite** | `6.4.4` | MIT |
| **vitefu** | `1.1.3` | MIT |
| **zimmerframe** | `1.1.5` | MIT |

---

## 4. Standalone Binaries & Embedded Assets

| Asset / Tool | Vendor / Author | License | Description |
| :--- | :--- | :--- | :--- |
| **Tailwind CSS Standalone CLI** (`bin/tailwindcss`) | Tailwind Labs, Inc. | MIT | Standalone binary compiler for Tailwind CSS v4 |
| **Scalar API Reference** (`templates/scalar.html`, `scalar.js`) | Scalar Inc. | MIT | Embedded interactive OpenAPI documentation interface |

---

## 5. License Texts

### The MIT License
```
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
```

### The Apache License, Version 2.0
```
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
```

### The BSD 2-Clause / 3-Clause License
```
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
```

### The ISC License
```
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
```

### The Unicode-3.0 License
```
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
```
