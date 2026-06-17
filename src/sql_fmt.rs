use leptos::prelude::*;
use sqlformat::{format, FormatOptions, Indent, QueryParams};
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[component]
pub fn SqlFormatter() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (indent_style, set_indent_style) = signal("spaces".to_string()); // "spaces", "tabs"
    let (indent_width, set_indent_width) = signal(4);
    let (keyword_case, set_keyword_case) = signal("uppercase".to_string()); // "uppercase", "preserve"
    let (copied, set_copied) = signal(false);

    let conversion_result = Memo::new(move |_| {
        let sql = input.get();
        if sql.trim().is_empty() {
            return String::new();
        }

        let indent = match indent_style.get().as_str() {
            "tabs" => Indent::Tabs,
            _ => Indent::Spaces(indent_width.get() as u8),
        };

        let uppercase = keyword_case.get() == "uppercase";

        let mut options = FormatOptions::default();
        options.indent = indent;
        options.uppercase = Some(uppercase);
        options.lines_between_queries = 2;

        format(&sql, &QueryParams::None, &options)
    });

    let output_text = Memo::new(move |_| conversion_result.get());

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
                    set_timeout(
                        move || {
                            set_copied.set(false);
                        },
                        Duration::from_millis(1500),
                    );
                }
            });
        }
    };

    let handle_clear = move |_| {
        set_input.set(String::new());
    };

    let load_sample = move |_| {
        let sample = "SELECT users.id, users.name, orders.amount, orders.created_at FROM users INNER JOIN orders ON users.id = orders.user_id WHERE users.active = true AND orders.created_at > '2026-01-01' ORDER BY orders.amount DESC LIMIT 50;";
        set_input.set(sample.to_string());
    };

    view! {
        <div class="space-y-6 animate-fade-in">
            // Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"SQL Formatter"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">"Format and pretty-print raw SQL queries into clean, readable structures instantly."</p>
            </div>

            // Card Panel
            <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-6">
                // Formatting Options
                <div class="flex flex-wrap items-center gap-6">
                    // Indent style
                    <div class="flex flex-col space-y-1.5">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Indent Style"</label>
                        <select
                            on:change=move |ev| set_indent_style.set(event_target_value(&ev))
                            prop:value=indent_style
                            class="px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm outline-none"
                        >
                            <option value="spaces">"Spaces"</option>
                            <option value="tabs">"Tabs"</option>
                        </select>
                    </div>

                    // Indent width
                    {move || if indent_style.get() == "spaces" {
                        Some(view! {
                            <div class="flex flex-col space-y-1.5 animate-fade-in">
                                <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Indent Size"</label>
                                <select
                                    on:change=move |ev| {
                                        if let Ok(w) = event_target_value(&ev).parse::<i32>() {
                                            set_indent_width.set(w);
                                        }
                                    }
                                    prop:value=move || indent_width.get().to_string()
                                    class="px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm outline-none"
                                >
                                    <option value="2">"2 Spaces"</option>
                                    <option value="4">"4 Spaces"</option>
                                    <option value="8">"8 Spaces"</option>
                                </select>
                            </div>
                        })
                    } else {
                        None
                    }}

                    // Keyword case
                    <div class="flex flex-col space-y-1.5">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"SQL Keywords"</label>
                        <select
                            on:change=move |ev| set_keyword_case.set(event_target_value(&ev))
                            prop:value=keyword_case
                            class="px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm outline-none"
                        >
                            <option value="uppercase">"UPPERCASE"</option>
                            <option value="preserve">"Preserve Case"</option>
                        </select>
                    </div>

                    <div class="flex items-end h-full pt-5 ml-auto">
                        <button
                            on:click=load_sample
                            class="px-4 py-2 border border-dashed border-slate-200 dark:border-slate-800 hover:border-indigo-500 dark:hover:border-indigo-500 text-indigo-600 dark:text-indigo-400 font-semibold rounded-lg text-sm transition duration-200"
                        >
                            "Load Sample SQL"
                        </button>
                    </div>
                </div>

                // Inputs
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"SQL Input"</label>
                        <textarea
                            rows="20"
                            placeholder="Paste SQL query here..."
                            prop:value=input
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                            class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-xs focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                        />
                    </div>

                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Formatted Output"</label>
                        <textarea
                            rows="20"
                            readonly=true
                            placeholder="Formatted SQL will appear here..."
                            prop:value=output_text
                            class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-xs focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                        />
                    </div>
                </div>

                // Actions
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
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
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
