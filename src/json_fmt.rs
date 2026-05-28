use leptos::prelude::*;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use std::time::Duration;

#[component]
pub fn JsonTool() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (output, set_output) = signal(String::new());
    let (error_msg, set_error_msg) = signal(None::<String>);
    let (indent_style, set_indent_style) = signal("2".to_string()); // "2", "4", "tab"
    let (auto_format, set_auto_format) = signal(false);
    let (copied, set_copied) = signal(false);

    // Track the last parsed value to allow instant re-indentation
    let (parsed_val, set_parsed_val) = signal(None::<serde_json::Value>);

    // Helper for formatting with custom indentation
    let format_json_val = move |val: &serde_json::Value, style: &str| -> Result<String, String> {
        let indent_bytes = match style {
            "2" => b"  ".as_slice(),
            "4" => b"    ".as_slice(),
            "tab" => b"\t".as_slice(),
            _ => b"  ".as_slice(),
        };
        let mut buffer = Vec::new();
        let formatter = PrettyFormatter::with_indent(indent_bytes);
        let mut serializer = Serializer::with_formatter(&mut buffer, formatter);
        val.serialize(&mut serializer)
            .map_err(|e| format!("Serialization error: {}", e))?;
        String::from_utf8(buffer)
            .map_err(|e| format!("UTF-8 encoding error: {}", e))
    };

    // Prettify logic
    let perform_prettify = move |text: &str, style: &str| {
        if text.trim().is_empty() {
            set_output.set(String::new());
            set_error_msg.set(None);
            set_parsed_val.set(None);
            return;
        }

        match serde_json::from_str::<serde_json::Value>(text) {
            Ok(val) => {
                set_error_msg.set(None);
                set_parsed_val.set(Some(val.clone()));
                match format_json_val(&val, style) {
                    Ok(formatted) => set_output.set(formatted),
                    Err(e) => set_error_msg.set(Some(e)),
                }
            }
            Err(e) => {
                set_error_msg.set(Some(format!("Syntax Error: {}", e)));
            }
        }
    };

    // Minify logic
    let perform_minify = move |text: &str| {
        if text.trim().is_empty() {
            set_output.set(String::new());
            set_error_msg.set(None);
            set_parsed_val.set(None);
            return;
        }

        match serde_json::from_str::<serde_json::Value>(text) {
            Ok(val) => {
                set_error_msg.set(None);
                set_parsed_val.set(Some(val.clone()));
                match serde_json::to_string(&val) {
                    Ok(minified) => set_output.set(minified),
                    Err(e) => set_error_msg.set(Some(format!("Minification error: {}", e))),
                }
            }
            Err(e) => {
                set_error_msg.set(Some(format!("Syntax Error: {}", e)));
            }
        }
    };

    // Manual format triggers
    let handle_prettify_btn = move |_| {
        perform_prettify(&input.get(), &indent_style.get());
    };

    // Manual minify trigger
    let handle_minify_btn = move |_| {
        perform_minify(&input.get());
    };

    // Auto-format watch effect
    Effect::new(move |_| {
        let text = input.get();
        let auto = auto_format.get();
        let style = indent_style.get();
        if auto {
            perform_prettify(&text, &style);
        }
    });

    // Indent selection watch effect (instant formatting if already valid)
    Effect::new(move |_| {
        let style = indent_style.get();
        if let Some(val) = parsed_val.get() {
            if let Ok(formatted) = format_json_val(&val, &style) {
                set_output.set(formatted);
            }
        }
    });

    // Clear handler
    let handle_clear = move |_| {
        set_input.set(String::new());
        set_output.set(String::new());
        set_error_msg.set(None);
        set_parsed_val.set(None);
    };

    // Copy to clipboard handler
    let handle_copy = move |_| {
        let text = output.get();
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

    view! {
        <div class="space-y-6">
            // Tool Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"JSON Formatter"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">"Format, prettify, and minify your JSON data with custom indentation styles."</p>
            </div>

            // Main Control Card
            <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-6">
                // Options and Configuration row
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                    // Indentation selection
                    <div class="flex items-center space-x-3">
                        <span class="text-sm font-semibold text-slate-700 dark:text-slate-300">"Indentation:"</span>
                        <select
                            on:change=move |ev| set_indent_style.set(event_target_value(&ev))
                            prop:value=indent_style
                            class="bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-900 dark:text-white rounded-lg px-3 py-1.5 text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none transition"
                        >
                            <option value="2">"2 Spaces"</option>
                            <option value="4">"4 Spaces"</option>
                            <option value="tab">"Tabs"</option>
                        </select>
                    </div>

                    // Configuration Options (Checkbox)
                    <div class="flex items-center">
                        <label class="flex items-center space-x-2.5 cursor-pointer select-none">
                            <input
                                type="checkbox"
                                prop:checked=auto_format
                                on:change=move |ev| set_auto_format.set(event_target_checked(&ev))
                                class="w-4 h-4 rounded-md border-slate-300 dark:border-slate-700 text-indigo-600 focus:ring-indigo-500 focus:ring-offset-2 dark:bg-slate-800"
                            />
                            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Auto-format on type"</span>
                        </label>
                    </div>
                </div>

                // Inputs & Outputs Grid
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    // Input Column
                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Raw JSON Input"</label>
                        <textarea
                            rows="12"
                            placeholder="Paste your raw JSON payload here..."
                            prop:value=input
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                            class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                        />
                    </div>

                    // Output Column
                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Formatted Result"</label>
                        <textarea
                            rows="12"
                            readonly=true
                            placeholder="Formatted JSON will appear here..."
                            prop:value=output
                            class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                        />
                    </div>
                </div>

                // Error Message Display
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
                <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-t border-slate-200 dark:border-slate-800 pt-5">
                    // Left Actions: Format, Minify, Clear
                    <div class="flex flex-wrap items-center gap-3">
                        <button
                            on:click=handle_prettify_btn
                            class="px-4 py-2 bg-slate-900 hover:bg-slate-800 dark:bg-slate-800 dark:hover:bg-slate-700 text-white font-semibold rounded-lg text-sm transition duration-200"
                        >
                            "Prettify"
                        </button>
                        
                        <button
                            on:click=handle_minify_btn
                            class="px-4 py-2 border border-slate-200 dark:border-slate-800 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 font-semibold rounded-lg text-sm transition duration-200"
                        >
                            "Minify"
                        </button>

                        <button
                            on:click=handle_clear
                            class="px-4 py-2 border border-slate-200 dark:border-slate-800 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 font-semibold rounded-lg text-sm transition duration-200"
                        >
                            "Clear"
                        </button>
                    </div>

                    // Right Actions: Copy
                    <button
                        on:click=handle_copy
                        disabled=move || output.get().is_empty()
                        class=move || {
                            let disabled = output.get().is_empty();
                            let is_copied = copied.get();
                            format!(
                                "flex items-center justify-center space-x-2 px-5 py-2 font-semibold rounded-lg text-sm transition-all duration-200 {}",
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
