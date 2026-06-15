# Sandbanks DevTools 🦀 ⚡ 🌐

A lightweight, local-first developer utility suite built entirely in Rust, compiled to Client-Side WebAssembly (Wasm), and powered by Leptos. 

**Live App:** [tools.sandbanks.tech](https://tools.sandbanks.tech/)

---

## Why this exists

Most online developer utilities are bloated, tracking-heavy, or require sending your data to a backend server. Pasting production JSON payloads, API keys, or database dumps into standard web tools is a massive security risk. 

**Sandbanks DevTools** fixes this by operating on a strict **local-first, zero-egress architecture**. Because the entire application runs inside your browser's native WebAssembly sandbox, **your data never leaves your machine.** 

## Features

### 🛠️ Base64 Codec
* Instantly encode or decode plain text and URL-safe Base64 strings.
* Handles large strings entirely in Rust memory, bypassing JavaScript engine allocation bottlenecks.

###  JSON Formatter & Validator
* Prettify (2-space/4-space indentation) or minify raw JSON structures.
* Powered by client-side Rust serialization (`serde_json`) for instantaneous syntax validation and zero UI-blocking rendering lag on massive minified payloads.

---

## Technical Architecture

Unlike traditional developer toolkits wrapped in heavy JavaScript frameworks like React, this suite is engineered for raw speed and minimal resource usage:

* **Leptos Framework:** Uses fine-grained reactive signals to manipulate the DOM directly. No Virtual DOM diffing overhead, no heavy `node_modules` runtime baggage.
* **Pure Client-Side Rendering (CSR):** Compiles down to flat static assets (`index.html`, a lightweight JS glue file, and a raw `.wasm` binary). Zero server runtimes, zero hydration mismatches, and 100% offline capable once loaded.
* **Zero Garbage Collection Pauses:** Bypasses managed runtime delays entirely by using Rust's compile-time memory management.

---

## Local Development & Deployment

### Prerequisites
Ensure you have Rust and the WebAssembly target installed:
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Run Locally

To run the development server with hot-reloading:

```bash
trunk serve
```

## Build for Production

To generate the optimized static assets for hosting:

```bash
trunk build --release
```

The resulting output in the /dist folder can be served by any static file host (GitHub Pages, Cloudflare Pages, S3, etc.).

***

### License
MIT / Apache 2.0
