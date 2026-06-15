# Sandbanks DevTools 🦀 ⚡ 🌐

A lightweight, local-first developer utility suite built entirely in Rust, compiled to Client-Side WebAssembly (Wasm), and powered by Leptos. 

**Live App:** [tools.sandbanks.tech](https://tools.sandbanks.tech/)

---

## Why this exists

Most online developer utilities are bloated, tracking-heavy, or require sending your data to a backend server. Pasting production JSON payloads, API keys, or database dumps into standard web tools is a massive security risk. 

**Sandbanks DevTools** fixes this by operating on a strict **local-first, zero-egress architecture**. Because the entire application runs inside your browser's native WebAssembly sandbox, **your data never leaves your machine.** 

## Features

### 🔏 Encoders & Decoders
* **Base64 Codec:** Instantly encode or decode plain text or URL-safe Base64 strings without JavaScript engine allocation bottlenecks.
* **URL Codec:** Safely encode and decode URL parameters locally to prevent character conflicts.
* **JWT Decoder:** Decode and inspect JSON Web Token (JWT) headers, payloads, and signatures. Paste production access tokens with complete confidence that your keys are never transmitted to a third party.

### 📄 Formatters & Beautifiers
* **JSON Formatter:** Format, prettify, and minify complex JSON structures with customizable indentation styles. Powered by client-side `serde_json` for lag-free rendering.
* **SQL Formatter:** Instantly pretty-print messy, raw SQL queries into clean, highly readable, structured layouts.

### 🔄 Converters
* **JSON to Go Struct:** Convert raw JSON objects into fully typed, nested Go struct definitions instantly.
* **JSON to Rust Struct:** Convert JSON payloads into idiomatic, nested Rust struct definitions complete with `#[derive(Serialize, Deserialize)]` attributes.
* **YAML ↔ JSON:** Seamlessly convert configuration files between YAML syntax and JSON format completely client-side.
* **CSV ↔ JSON:** Transform tabular data between raw CSV sheets and structured JSON arrays.
* **Timestamp Converter:** Convert Unix epoch timestamps to UTC, Local, and ISO dates (and vice versa) without worrying about system timezone leaks.

### 🎲 Generators
* **UUID Generator:** Generate cryptographically secure Universally Unique Identifiers (**UUIDv4** and **UUIDv7**) in bulk using native client-side entropy.
* **Mermaid Diagrams:** Render flowcharts, sequence diagrams, and class diagrams instantly using simple Markdown-like text syntax directly in the browser runtime.
* **QR Code Generator:** Generate customized QR codes with custom colors, gradients, logos, and eye patterns entirely offline without reliance on sketchy tracking APIs.

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
