use leptos::prelude::*;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use std::time::Duration;

// Formats Unix epoch seconds into a human-readable UTC string
fn format_epoch_seconds(sec: u64) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64((sec * 1000) as f64));
    String::from(date.to_utc_string())
}

#[derive(Clone, Debug, PartialEq)]
struct DecodedJwt {
    header_json: String,
    payload_json: String,
    signature_hex: String,
    signature_base64: String,
    // Claim details
    exp: Option<u64>,
    iat: Option<u64>,
    nbf: Option<u64>,
    iss: Option<String>,
    sub: Option<String>,
    aud: Option<String>,
    is_expired: bool,
    time_diff_msg: Option<String>,
}

fn decode_jwt(token: &str) -> Result<DecodedJwt, String> {
    let parts: Vec<&str> = token.trim().split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT format. A valid token must have 3 parts separated by dots (header.payload.signature).".to_string());
    }

    // 1. Decode Header
    let header_bytes = URL_SAFE_NO_PAD
        .decode(parts[0])
        .map_err(|e| format!("Failed to base64-decode header: {}", e))?;
    let header_str = String::from_utf8(header_bytes)
        .map_err(|e| format!("Failed to read header as UTF-8: {}", e))?;
    let header_val: Value = serde_json::from_str(&header_str)
        .map_err(|e| format!("Failed to parse header as JSON: {}", e))?;
    let header_json = serde_json::to_string_pretty(&header_val)
        .unwrap_or(header_str);

    // 2. Decode Payload
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(parts[1])
        .map_err(|e| format!("Failed to base64-decode payload: {}", e))?;
    let payload_str = String::from_utf8(payload_bytes)
        .map_err(|e| format!("Failed to read payload as UTF-8: {}", e))?;
    let payload_val: Value = serde_json::from_str(&payload_str)
        .map_err(|e| format!("Failed to parse payload as JSON: {}", e))?;
    let payload_json = serde_json::to_string_pretty(&payload_val)
        .unwrap_or(payload_str);

    // 3. Decode Signature
    let sig_bytes = URL_SAFE_NO_PAD
        .decode(parts[2])
        .map_err(|e| format!("Failed to base64-decode signature: {}", e))?;
    let signature_hex = sig_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    let signature_base64 = parts[2].to_string();

    // Extract claims from payload
    let exp = payload_val.get("exp").and_then(|v| v.as_u64());
    let iat = payload_val.get("iat").and_then(|v| v.as_u64());
    let nbf = payload_val.get("nbf").and_then(|v| v.as_u64());
    let iss = payload_val.get("iss").and_then(|v| v.as_str().map(|s| s.to_string()));
    let sub = payload_val.get("sub").and_then(|v| v.as_str().map(|s| s.to_string()));
    
    // Audience can be string or array
    let aud = payload_val.get("aud").and_then(|v| {
        if let Some(s) = v.as_str() {
            Some(s.to_string())
        } else if let Some(arr) = v.as_array() {
            let elements: Vec<String> = arr.iter().filter_map(|el| el.as_str().map(|s| s.to_string())).collect();
            Some(elements.join(", "))
        } else {
            None
        }
    });

    // Check expiration against current browser time
    let now_sec = (js_sys::Date::now() / 1000.0) as u64;
    let mut is_expired = false;
    let mut time_diff_msg = None;

    if let Some(exp_val) = exp {
        if exp_val < now_sec {
            is_expired = true;
            let diff = now_sec - exp_val;
            time_diff_msg = Some(format!("expired {} ago", format_duration(diff)));
        } else {
            let diff = exp_val - now_sec;
            time_diff_msg = Some(format!("expires in {}", format_duration(diff)));
        }
    }

    Ok(DecodedJwt {
        header_json,
        payload_json,
        signature_hex,
        signature_base64,
        exp,
        iat,
        nbf,
        iss,
        sub,
        aud,
        is_expired,
        time_diff_msg,
    })
}

// Helpers for relative duration formatting
fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    } else {
        format!("{}d {}h", seconds / 86400, (seconds % 86400) / 3600)
    }
}

#[component]
pub fn JwtDecoder() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (copied_header, set_copied_header) = signal(false);
    let (copied_payload, set_copied_payload) = signal(false);

    // Reactive Memo to compute JWT decode
    let decode_result = Memo::new(move |_| {
        let text = input.get();
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return None;
        }
        Some(decode_jwt(trimmed))
    });

    // Clipboard copy helpers
    let handle_copy_header = move |_| {
        if let Some(Ok(ref jwt)) = decode_result.get() {
            let text = jwt.header_json.clone();
            if let Some(win) = window() {
                let nav = win.navigator();
                let clipboard = nav.clipboard();
                let promise = clipboard.write_text(&text);
                spawn_local(async move {
                    if wasm_bindgen_futures::JsFuture::from(promise).await.is_ok() {
                        set_copied_header.set(true);
                        set_timeout(move || set_copied_header.set(false), Duration::from_millis(1500));
                    }
                });
            }
        }
    };

    let handle_copy_payload = move |_| {
        if let Some(Ok(ref jwt)) = decode_result.get() {
            let text = jwt.payload_json.clone();
            if let Some(win) = window() {
                let nav = win.navigator();
                let clipboard = nav.clipboard();
                let promise = clipboard.write_text(&text);
                spawn_local(async move {
                    if wasm_bindgen_futures::JsFuture::from(promise).await.is_ok() {
                        set_copied_payload.set(true);
                        set_timeout(move || set_copied_payload.set(false), Duration::from_millis(1500));
                    }
                });
            }
        }
    };

    let handle_clear = move |_| {
        set_input.set(String::new());
    };

    view! {
        <div class="space-y-6">
            // Tool Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"JWT Decoder"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">
                    "Decode, inspect, and validate JSON Web Tokens (JWT) locally. Your data remains in the browser."
                </p>
            </div>

            // Token Input Area
            <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-4">
                <div class="flex flex-col space-y-2">
                    <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Encoded Token (Paste JWT)"</label>
                    <textarea
                        rows="4"
                        placeholder="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
                        prop:value=input
                        on:input=move |ev| set_input.set(event_target_value(&ev))
                        class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar break-all"
                    />
                </div>
                
                <div class="flex items-center justify-between">
                    <button
                        on:click=handle_clear
                        class="px-4 py-2 border border-slate-200 dark:border-slate-800 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 font-semibold rounded-lg text-sm transition duration-200"
                    >
                        "Clear"
                    </button>
                </div>
            </div>

            // Decoded Content View
            {move || decode_result.get().map(|res| {
                match res {
                    Err(err) => view! {
                        <div class="flex items-start space-x-3 p-4 bg-rose-50 dark:bg-rose-950/20 border border-rose-200 dark:border-rose-800/40 rounded-lg text-rose-800 dark:text-rose-400 transition-all duration-300">
                            <svg class="w-5 h-5 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                            </svg>
                            <div class="text-sm font-medium">
                                {err}
                            </div>
                        </div>
                    }.into_any(),
                    Ok(jwt) => {
                        let expired_badge = if jwt.exp.is_some() {
                            if jwt.is_expired {
                                view! {
                                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold bg-rose-100 text-rose-800 dark:bg-rose-950/40 dark:text-rose-400 border border-rose-200 dark:border-rose-800/40">
                                        "Expired"
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold bg-emerald-100 text-emerald-800 dark:bg-emerald-950/40 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-800/40">
                                        "Active"
                                    </span>
                                }.into_any()
                            }
                        } else {
                            view! {
                                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold bg-slate-100 text-slate-800 dark:bg-slate-800 dark:text-slate-300 border border-slate-200 dark:border-slate-700">
                                    "No Expiration"
                                </span>
                            }.into_any()
                        };

                        view! {
                            <div class="space-y-6">
                                // Metadata / Claims Summary
                                <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-4">
                                    <h3 class="text-sm font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Token Claims Summary"</h3>
                                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                                        // Status
                                        <div class="space-y-1">
                                            <span class="text-xs text-slate-400 dark:text-slate-500">"Status"</span>
                                            <div class="flex items-center space-x-2 mt-0.5">
                                                {expired_badge}
                                                {jwt.time_diff_msg.map(|msg| view! { <span class="text-xs text-slate-500 dark:text-slate-400">"("{msg}")"</span> })}
                                            </div>
                                        </div>

                                        // Subject
                                        <div class="space-y-1">
                                            <span class="text-xs text-slate-400 dark:text-slate-500">"Subject (sub)"</span>
                                            <div class="font-mono text-sm font-semibold text-slate-800 dark:text-slate-200 truncate mt-0.5">
                                                {jwt.sub.clone().unwrap_or_else(|| "N/A".to_string())}
                                            </div>
                                        </div>

                                        // Issuer
                                        <div class="space-y-1">
                                            <span class="text-xs text-slate-400 dark:text-slate-500">"Issuer (iss)"</span>
                                            <div class="font-mono text-sm font-semibold text-slate-800 dark:text-slate-200 truncate mt-0.5">
                                                {jwt.iss.clone().unwrap_or_else(|| "N/A".to_string())}
                                            </div>
                                        </div>

                                        // Expiration
                                        {jwt.exp.map(|val| view! {
                                            <div class="space-y-1">
                                                <span class="text-xs text-slate-400 dark:text-slate-500">"Expiration Time (exp)"</span>
                                                <div class="text-xs font-semibold text-slate-800 dark:text-slate-200 mt-0.5">
                                                    {format_epoch_seconds(val)}
                                                </div>
                                            </div>
                                        })}

                                        // Issued At
                                        {jwt.iat.map(|val| view! {
                                            <div class="space-y-1">
                                                <span class="text-xs text-slate-400 dark:text-slate-500">"Issued At (iat)"</span>
                                                <div class="text-xs font-semibold text-slate-800 dark:text-slate-200 mt-0.5">
                                                    {format_epoch_seconds(val)}
                                                </div>
                                            </div>
                                        })}

                                        // Audience
                                        <div class="space-y-1">
                                            <span class="text-xs text-slate-400 dark:text-slate-500">"Audience (aud)"</span>
                                            <div class="font-mono text-sm font-semibold text-slate-800 dark:text-slate-200 truncate mt-0.5">
                                                {jwt.aud.clone().unwrap_or_else(|| "N/A".to_string())}
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // Header & Payload Grid
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    // Header Box
                                    <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 flex flex-col space-y-4">
                                        <div class="flex items-center justify-between">
                                            <span class="text-xs font-bold uppercase tracking-wider text-rose-500">"Header (Algorithm & Type)"</span>
                                            
                                            <button
                                                on:click=handle_copy_header
                                                class=move || {
                                                    let is_copied = copied_header.get();
                                                    format!(
                                                        "flex items-center space-x-1.5 px-3 py-1 font-semibold rounded-md text-xs transition duration-200 {}",
                                                        if is_copied {
                                                            "bg-emerald-100 text-emerald-800 dark:bg-emerald-950/40 dark:text-emerald-400"
                                                        } else {
                                                            "bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 dark:hover:bg-slate-700 text-slate-700 dark:text-slate-300"
                                                        }
                                                    )
                                                }
                                            >
                                                {move || if copied_header.get() { "Copied!" } else { "Copy" }}
                                            </button>
                                        </div>
                                        
                                        <pre class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm text-rose-700 dark:text-rose-400 overflow-x-auto custom-scrollbar whitespace-pre-wrap break-all min-h-[16rem]">
                                            {jwt.header_json.clone()}
                                        </pre>
                                    </div>

                                    // Payload Box
                                    <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 flex flex-col space-y-4">
                                        <div class="flex items-center justify-between">
                                            <span class="text-xs font-bold uppercase tracking-wider text-indigo-500">"Payload (Data & Claims)"</span>
                                            
                                            <button
                                                on:click=handle_copy_payload
                                                class=move || {
                                                    let is_copied = copied_payload.get();
                                                    format!(
                                                        "flex items-center space-x-1.5 px-3 py-1 font-semibold rounded-md text-xs transition duration-200 {}",
                                                        if is_copied {
                                                            "bg-emerald-100 text-emerald-800 dark:bg-emerald-950/40 dark:text-emerald-400"
                                                        } else {
                                                            "bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 dark:hover:bg-slate-700 text-slate-700 dark:text-slate-300"
                                                        }
                                                    )
                                                }
                                            >
                                                {move || if copied_payload.get() { "Copied!" } else { "Copy" }}
                                            </button>
                                        </div>
                                        
                                        <pre class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm text-indigo-700 dark:text-indigo-400 overflow-x-auto custom-scrollbar whitespace-pre-wrap break-all min-h-[16rem]">
                                            {jwt.payload_json.clone()}
                                        </pre>
                                    </div>
                                </div>

                                // Signature Box
                                <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-4">
                                    <span class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400 block">"Signature"</span>
                                    <div class="p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-xs space-y-2 text-slate-600 dark:text-slate-400 break-all">
                                        <div>
                                            <span class="font-bold text-slate-800 dark:text-slate-300">"HMACSHA256("</span>
                                        </div>
                                        <div class="pl-4">
                                            <span>"base64UrlEncode(header) + \".\" + base64UrlEncode(payload),"</span>
                                        </div>
                                        <div class="pl-4 font-bold text-indigo-500">
                                            <span>"YOUR_256_BIT_SECRET"</span>
                                        </div>
                                        <div>
                                            <span class="font-bold text-slate-800 dark:text-slate-300">")"</span>
                                        </div>
                                        
                                        <div class="border-t border-slate-200 dark:border-slate-800/80 pt-3 mt-3 space-y-1 text-[11px]">
                                            <div>
                                                <span class="font-semibold text-slate-700 dark:text-slate-400">"Hex: "</span>
                                                <span class="text-slate-500 dark:text-slate-500 font-mono">{jwt.signature_hex.clone()}</span>
                                            </div>
                                            <div>
                                                <span class="font-semibold text-slate-700 dark:text-slate-400">"Base64URL: "</span>
                                                <span class="text-slate-500 dark:text-slate-500 font-mono">{jwt.signature_base64.clone()}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                }
            })}
        </div>
    }
}
