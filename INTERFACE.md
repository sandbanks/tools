# Interface Contract — Sandbanks DevTools

## Scope

Client-side WebAssembly (CSR) developer utility suite written in Rust with Leptos `0.8` and `leptos_router`, compiled via `trunk` to static assets (`index.html`, JS glue, `.wasm`, and Tailwind CSS). All data processing runs locally in the browser with no backend API.

## Binaries & Entrypoints

- **Crate**: `devtools` (`0.1.0`, Rust edition `2021`) in `Cargo.toml`
- **Wasm Entrypoint**: `src/main.rs` (`main()` attaches `console_error_panic_hook` and mounts `app::App` to `<body>`)
- **Build Artifact**: Static bundle output to `dist/` (via `trunk build --release` targeting `wasm32-unknown-unknown`)

## Client-Side Routes (`src/app.rs`)

| Path | View Component | Purpose |
| --- | --- | --- |
| `/` | `Dashboard` | Tool directory / home |
| `/base64` | `Base64Tool` | Base64 encode/decode |
| `/json` | `JsonTool` | JSON formatter/validator |
| `/sql-format` | `SqlFormatter` | SQL formatter |
| `/url` | `UrlCodec` | URL encode/decode |
| `/uuid` | `UuidGen` | UUID v4/v7 generator |
| `/jwt` | `JwtDecoder` | JWT header/payload/signature decoder |
| `/mermaid` | `MermaidEditor` | Mermaid diagram editor & preview |
| `/timestamp` | `TimestampConv` | Unix timestamp ↔ UTC/Local/ISO converter |
| `/json-to-go` | `JsonToGo` | JSON to Go struct converter |
| `/json-to-rust` | `JsonToRust` | JSON to Rust struct converter |
| `/yaml-json` | `YamlJsonConverter` | YAML ↔ JSON converter |
| `/csv-json` | `CsvJsonConverter` | CSV ↔ JSON converter |
| `/qr-generator` | `QrGenerator` | QR code generator |

## Browser / JS Interop (`index.html`)

- `window.renderMermaid(id, code, isDark)` — Renders Mermaid diagrams using `mermaid@10` loaded from `cdn.jsdelivr.net`.
- `window.downloadQrCode(svgId, filename, format)` — Exports an SVG element as `.svg` or rasterized `.png` (`1000x1000`).
- External font stylesheet: Google Fonts (`Outfit`, `Fira Code`).

## Configuration Paths

| File | Role |
| --- | --- |
| `Cargo.toml` | Rust dependencies (`wasm-bindgen = "0.2.125"`) and `[profile.release]` (`opt-level = 'z'`, `lto = true`, `codegen-units = 1`, `panic = "abort"`) |
| `Trunk.toml` | Trunk build (`target = "index.html"`, `dist = "dist"`), dev server, and `tailwindcss = "4.3.0"` |
| `index.html` | HTML shell, Trunk `tailwind-css` link, and JS interop helpers |
| `tailwind.css` | Tailwind CSS v4 entrypoint |
| `renovate.json` | Automated dependency update configuration |

## Ports

- **Local dev server (`trunk serve`)**: `0.0.0.0:3034` (`open = false`, configured in `Trunk.toml`)

## Deployment Targets

- **Live URL**: `https://tools.sandbanks.tech/` (per `README.md`)
- **Static Host / Cloudflare**: Serves `dist/` as static assets. Requires installing `rustup` (`wasm32-unknown-unknown`) and `trunk` in CI/build environments where Rust is not pre-installed.
