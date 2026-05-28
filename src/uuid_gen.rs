use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use std::time::Duration;

// Generates a batch of UUIDs
fn generate_batch(q: usize, version: u8, up: bool, flat: bool) -> String {
    let mut results = Vec::with_capacity(q);
    for _ in 0..q {
        let mut uuid = if version == 7 {
            uuid::Uuid::now_v7().to_string()
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        if flat {
            uuid = uuid.replace("-", "");
        }
        if up {
            uuid = uuid.to_uppercase();
        }
        results.push(uuid);
    }
    results.join("\n")
}

#[component]
pub fn UuidGen() -> impl IntoView {
    let (uuid_version, set_uuid_version) = signal(7); // 7 for v7, 4 for v4
    let (quantity, set_quantity) = signal(5);
    let (uppercase, set_uppercase) = signal(false);
    let (no_hyphens, set_no_hyphens) = signal(false);
    let (copied, set_copied) = signal(false);

    let initial_value = generate_batch(5, 7, false, false);
    let (output, set_output) = signal(initial_value);

    // Action to regenerate
    let handle_generate = move |_| {
        let val = generate_batch(quantity.get(), uuid_version.get(), uppercase.get(), no_hyphens.get());
        set_output.set(val);
    };

    // Clipboard copy
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

    // Clear handler
    let handle_clear = move |_| {
        set_output.set(String::new());
    };

    view! {
        <div class="space-y-6">
            // Tool Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"UUID Generator"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">"Generate cryptographically secure Universally Unique Identifiers (UUID v4) in bulk."</p>
            </div>

            // Main Control Card
            <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-6">
                // Options and Configuration row
                <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
                    // Left options: Version & Quantity
                    <div class="flex flex-wrap items-center gap-4">
                        // Version selector
                        <div class="flex items-center space-x-3">
                            <span class="text-sm font-semibold text-slate-700 dark:text-slate-300">"Version:"</span>
                            <select
                                on:change=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<u8>() {
                                        set_uuid_version.set(v);
                                    }
                                }
                                prop:value=uuid_version
                                class="bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-900 dark:text-white rounded-lg px-3 py-1.5 text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none transition"
                            >
                                <option value="7">"UUIDv7 (Time-Ordered)"</option>
                                <option value="4">"UUIDv4 (Random)"</option>
                            </select>
                        </div>

                        // Quantity selector
                        <div class="flex items-center space-x-3">
                            <span class="text-sm font-semibold text-slate-700 dark:text-slate-300">"Quantity:"</span>
                            <select
                                on:change=move |ev| {
                                    if let Ok(q) = event_target_value(&ev).parse::<usize>() {
                                        set_quantity.set(q);
                                    }
                                }
                                prop:value=quantity
                                class="bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-900 dark:text-white rounded-lg px-3 py-1.5 text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none transition"
                            >
                                <option value="1">"1 UUID"</option>
                                <option value="5">"5 UUIDs"</option>
                                <option value="10">"10 UUIDs"</option>
                                <option value="50">"50 UUIDs"</option>
                                <option value="100">"100 UUIDs"</option>
                            </select>
                        </div>
                    </div>

                    // Configuration Options (Checkboxes)
                    <div class="flex flex-wrap items-center gap-6">
                        <label class="flex items-center space-x-2.5 cursor-pointer select-none">
                            <input
                                type="checkbox"
                                prop:checked=uppercase
                                on:change=move |ev| set_uppercase.set(event_target_checked(&ev))
                                class="w-4 h-4 rounded-md border-slate-300 dark:border-slate-700 text-indigo-600 focus:ring-indigo-500 focus:ring-offset-2 dark:bg-slate-800"
                            />
                            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Uppercase"</span>
                        </label>
                        <label class="flex items-center space-x-2.5 cursor-pointer select-none">
                            <input
                                type="checkbox"
                                prop:checked=no_hyphens
                                on:change=move |ev| set_no_hyphens.set(event_target_checked(&ev))
                                class="w-4 h-4 rounded-md border-slate-300 dark:border-slate-700 text-indigo-600 focus:ring-indigo-500 focus:ring-offset-2 dark:bg-slate-800"
                            />
                            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Remove hyphens"</span>
                        </label>
                    </div>
                </div>

                // Large Result Textarea
                <div class="flex flex-col space-y-2">
                    <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Generated UUIDs"</label>
                    <textarea
                        rows="10"
                        readonly=true
                        placeholder="Click Generate to create UUIDs..."
                        prop:value=output
                        class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                    />
                </div>

                // Action Buttons Row
                <div class="flex items-center justify-between border-t border-slate-200 dark:border-slate-800 pt-5">
                    // Left Actions: Generate, Clear
                    <div class="flex items-center space-x-3">
                        <button
                            on:click=handle_generate
                            class="px-5 py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-lg text-sm shadow-xs transition duration-200"
                        >
                            "Generate"
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
