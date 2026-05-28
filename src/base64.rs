use leptos::prelude::*;
use base64::Engine;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use std::time::Duration;

#[component]
pub fn Base64Tool() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (is_decode, set_is_decode) = signal(false);
    let (url_safe, set_url_safe) = signal(false);
    let (no_pad, set_no_pad) = signal(false);
    let (copied, set_copied) = signal(false);

    // Reactive Memo to compute conversion
    let conversion_result = Memo::new(move |_| {
        let text = input.get();
        if text.is_empty() {
            return Ok(String::new());
        }

        if is_decode.get() {
            // Select the appropriate engine based on toggles
            let engine = match (url_safe.get(), no_pad.get()) {
                (true, true) => &base64::prelude::BASE64_URL_SAFE_NO_PAD,
                (true, false) => &base64::prelude::BASE64_URL_SAFE,
                (false, true) => &base64::prelude::BASE64_STANDARD_NO_PAD,
                (false, false) => &base64::prelude::BASE64_STANDARD,
            };

            // Remove whitespace/newlines from input
            let cleaned: String = text.chars().filter(|c| !c.is_whitespace()).collect();

            match engine.decode(cleaned) {
                Ok(bytes) => {
                    match String::from_utf8(bytes) {
                        Ok(s) => Ok(s),
                        Err(_) => Err("Invalid UTF-8: Decoded binary data is not a valid text string.".to_string())
                    }
                }
                Err(e) => Err(format!("Invalid Base64 format: {}", e))
            }
        } else {
            let engine = match (url_safe.get(), no_pad.get()) {
                (true, true) => &base64::prelude::BASE64_URL_SAFE_NO_PAD,
                (true, false) => &base64::prelude::BASE64_URL_SAFE,
                (false, true) => &base64::prelude::BASE64_STANDARD_NO_PAD,
                (false, false) => &base64::prelude::BASE64_STANDARD,
            };
            Ok(engine.encode(text.as_bytes()))
        }
    });

    // Derive output and error from conversion result
    let output_text = Memo::new(move |_| {
        match conversion_result.get() {
            Ok(val) => val,
            Err(_) => String::new(),
        }
    });

    let error_msg = Memo::new(move |_| {
        match conversion_result.get() {
            Err(e) => Some(e),
            Ok(_) => None,
        }
    });

    // Clipboard copy handler
    let handle_copy = move |_| {
        let text = output_text.get();
        if text.is_empty() {
            return;
        }

        if let Some(win) = window() {
            let nav = win.navigator();
            let clipboard = nav.clipboard();
            let promise = clipboard.write_text(&text);
            
            spawn_local(async move {
                let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                if result.is_ok() {
                    set_copied.set(true);
                    set_timeout(move || {
                        set_copied.set(false);
                    }, Duration::from_millis(1500));
                }
            });
        }
    };

    // Clear handler
    let handle_clear = move |_| {
        set_input.set(String::new());
    };

    view! {
        <div class="space-y-6">
            // Tool Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"Base64 Codec"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">"Encode and decode plain text or URL-safe Base64 strings instantly."</p>
            </div>

            // Main Control Card
            <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-6">
                // Mode Toggle & Settings Row
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                    // Mode Select (Tabs)
                    <div class="inline-flex p-1 bg-slate-100 dark:bg-slate-800/80 rounded-lg">
                        <button
                            on:click=move |_| set_is_decode.set(false)
                            class=move || {
                                let active = !is_decode.get();
                                format!(
                                    "px-4 py-1.5 text-sm font-semibold rounded-md transition-all duration-200 {}",
                                    if active {
                                        "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                    } else {
                                        "text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200"
                                    }
                                )
                            }
                        >
                            "Encode"
                        </button>
                        <button
                            on:click=move |_| set_is_decode.set(true)
                            class=move || {
                                let active = is_decode.get();
                                format!(
                                    "px-4 py-1.5 text-sm font-semibold rounded-md transition-all duration-200 {}",
                                    if active {
                                        "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                    } else {
                                        "text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200"
                                    }
                                )
                            }
                        >
                            "Decode"
                        </button>
                    </div>

                    // Configuration Options
                    <div class="flex items-center space-x-6">
                        <label class="flex items-center space-x-2.5 cursor-pointer select-none">
                            <input
                                type="checkbox"
                                prop:checked=url_safe
                                on:change=move |ev| set_url_safe.set(event_target_checked(&ev))
                                class="w-4 h-4 rounded-md border-slate-300 dark:border-slate-700 text-indigo-600 focus:ring-indigo-500 focus:ring-offset-2 dark:bg-slate-800"
                            />
                            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"URL Safe"</span>
                        </label>
                        <label class="flex items-center space-x-2.5 cursor-pointer select-none">
                            <input
                                type="checkbox"
                                prop:checked=no_pad
                                on:change=move |ev| set_no_pad.set(event_target_checked(&ev))
                                class="w-4 h-4 rounded-md border-slate-300 dark:border-slate-700 text-indigo-600 focus:ring-indigo-500 focus:ring-offset-2 dark:bg-slate-800"
                            />
                            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"No Padding"</span>
                        </label>
                    </div>
                </div>

                // Inputs & Outputs Grid
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    // Input Column
                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Input"</label>
                        <textarea
                            rows="10"
                            placeholder=move || if is_decode.get() { "Paste Base64 string here to decode..." } else { "Type plain text here to encode..." }
                            prop:value=input
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                            class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                        />
                    </div>

                    // Output Column
                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Output"</label>
                        <textarea
                            rows="10"
                            readonly=true
                            placeholder=move || if is_decode.get() { "Decoded text will appear here..." } else { "Encoded Base64 will appear here..." }
                            prop:value=output_text
                            class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                        />
                    </div>
                </div>

                // Error Message Box (Shows on decode failure)
                {move || error_msg.get().map(|err| {
                    view! {
                        <div class="flex items-start space-x-3 p-4 bg-rose-50 dark:bg-rose-950/20 border border-rose-200 dark:border-rose-800/40 rounded-lg text-rose-800 dark:text-rose-400 transition-all duration-300">
                            <svg class="w-5 h-5 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                            </svg>
                            <div class="text-sm font-medium">
                                {err}
                            </div>
                        </div>
                    }
                })}

                // Action Buttons Row
                <div class="flex items-center justify-between border-t border-slate-200 dark:border-slate-800 pt-5">
                    <button
                        on:click=handle_clear
                        class="px-4 py-2 border border-slate-200 dark:border-slate-800 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 font-semibold rounded-lg text-sm transition duration-200"
                    >
                        "Clear"
                    </button>

                    <button
                        on:click=handle_copy
                        disabled=move || output_text.get().is_empty()
                        class=move || {
                            let disabled = output_text.get().is_empty();
                            let is_copied = copied.get();
                            format!(
                                "flex items-center space-x-2 px-5 py-2 font-semibold rounded-lg text-sm transition-all duration-200 {}",
                                if disabled {
                                    "bg-slate-100 dark:bg-slate-800 text-slate-400 dark:text-slate-600 cursor-not-allowed"
                                } else if is_copied {
                                    "bg-emerald-600 hover:bg-emerald-700 text-white shadow-xs"
                                } else {
                                    "bg-indigo-600 hover:bg-indigo-700 text-white shadow-xs hover:shadow-indigo-500/10"
                                }
                            )
                        }
                    >
                        {move || if copied.get() {
                            view! {
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                </svg>
                                <span>"Copied!"</span>
                            }.into_any()
                        } else {
                            view! {
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 002 2h2a2 2 0 002-2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
                                </svg>
                                <span>"Copy Result"</span>
                            }.into_any()
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}
