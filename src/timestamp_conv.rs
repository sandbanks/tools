use leptos::prelude::*;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

// Helper to get formatted date components from a js_sys::Date
fn format_date_to_utc(date: &js_sys::Date) -> String {
    String::from(date.to_utc_string())
}

fn format_date_to_iso(date: &js_sys::Date) -> String {
    String::from(date.to_iso_string())
}

fn format_date_to_local(date: &js_sys::Date) -> String {
    String::from(date.to_locale_string("default", &wasm_bindgen::JsValue::undefined()))
}

// Relative time calculation
fn format_relative_time(sec: i64) -> String {
    let now_sec = (js_sys::Date::now() / 1000.0) as i64;
    let diff = now_sec - sec;

    if diff == 0 {
        return "now".to_string();
    }

    let is_past = diff > 0;
    let abs_diff = diff.abs() as u64;

    let time_unit = if abs_diff < 60 {
        format!("{} seconds", abs_diff)
    } else if abs_diff < 3600 {
        format!("{} minutes", abs_diff / 60)
    } else if abs_diff < 86400 {
        format!("{} hours", abs_diff / 3600)
    } else {
        format!("{} days", abs_diff / 86400)
    };

    if is_past {
        format!("{} ago", time_unit)
    } else {
        format!("in {}", time_unit)
    }
}

// Recursive function to tick the current clock every second
fn tick_clock(set_time: WriteSignal<u64>) {
    set_time.set((js_sys::Date::now() / 1000.0) as u64);
    set_timeout(
        move || {
            tick_clock(set_time);
        },
        Duration::from_secs(1),
    );
}

#[component]
pub fn TimestampConv() -> impl IntoView {
    // Current live ticking timestamp
    let (live_epoch, set_live_epoch) = signal((js_sys::Date::now() / 1000.0) as u64);
    let (copied_live, set_copied_live) = signal(false);

    Effect::new(move |_| {
        tick_clock(set_live_epoch);
    });

    // Copy live timestamp to clipboard
    let handle_copy_live = move |_| {
        let val = live_epoch.get().to_string();
        if let Some(win) = window() {
            let promise = win.navigator().clipboard().write_text(&val);
            spawn_local(async move {
                if wasm_bindgen_futures::JsFuture::from(promise).await.is_ok() {
                    set_copied_live.set(true);
                    set_timeout(
                        move || set_copied_live.set(false),
                        Duration::from_millis(1500),
                    );
                }
            });
        }
    };

    // --- State for Timestamp -> Date Converter ---
    let (ts_input, set_ts_input) = signal(String::new());

    // Auto-prepopulate input with live epoch initially
    Effect::new(move |_| {
        if ts_input.get().is_empty() {
            // Take the starting time
            set_ts_input.set(live_epoch.get().to_string());
        }
    });

    // Reactive computation of conversions
    let ts_conversion = Memo::new(move |_| {
        let input_str = ts_input.get();
        let trimmed = input_str.trim();
        if trimmed.is_empty() {
            return Err("Input is empty".to_string());
        }

        match trimmed.parse::<i64>() {
            Ok(num) => {
                // Heuristic: if value is larger than 10,000,000,000, assume milliseconds
                let (sec, ms) = if num > 10_000_000_000 {
                    (num / 1000, num % 1000)
                } else {
                    (num, 0)
                };

                let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(
                    (num * if num > 10_000_000_000 { 1 } else { 1000 }) as f64,
                ));

                // If it is NaN
                if date.get_time().is_nan() {
                    return Err("Invalid Unix timestamp value".to_string());
                }

                Ok((sec, ms, date))
            }
            Err(_) => Err("Invalid number. Please enter a valid integer timestamp.".to_string()),
        }
    });

    // --- State for Date -> Timestamp Converter ---
    let (date_input, set_date_input) = signal(String::new());

    // Auto-prepopulate date input with current ISO string
    Effect::new(move |_| {
        if date_input.get().is_empty() {
            let date = js_sys::Date::new_0();
            set_date_input.set(format_date_to_iso(&date));
        }
    });

    let date_conversion = Memo::new(move |_| {
        let input_str = date_input.get();
        let trimmed = input_str.trim();
        if trimmed.is_empty() {
            return Err("Input date string is empty".to_string());
        }

        let parsed_ms = js_sys::Date::parse(trimmed);
        if parsed_ms.is_nan() {
            return Err("Could not parse date string. Try ISO 8601 (e.g. YYYY-MM-DDTHH:mm:ssZ) or simple date strings.".to_string());
        }

        let ms = parsed_ms as i64;
        let sec = ms / 1000;

        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(parsed_ms));
        Ok((sec, ms, date))
    });

    view! {
        <div class="space-y-6">
            // Tool Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"Timestamp Converter"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">
                    "Convert Unix epoch timestamps (seconds/milliseconds) to human-readable date strings and back."
                </p>
            </div>

            // 1. Live Ticking Widget
            <div class="bg-gradient-to-r from-indigo-500/10 via-violet-500/10 to-transparent dark:from-indigo-500/20 dark:to-transparent rounded-xl border border-indigo-500/20 dark:border-indigo-500/30 p-5 flex items-center justify-between">
                <div class="space-y-1">
                    <span class="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400">"Current Unix Epoch Time"</span>
                    <div class="font-mono text-2xl md:text-3xl font-extrabold text-slate-950 dark:text-white tracking-wider">
                        {move || live_epoch.get()}
                    </div>
                </div>

                <button
                    on:click=handle_copy_live
                    class=move || {
                        let is_copied = copied_live.get();
                        format!(
                            "flex items-center space-x-1.5 px-4 py-2 font-semibold rounded-lg text-sm shadow-xs transition duration-200 {}",
                            if is_copied {
                                "bg-emerald-600 text-white"
                            } else {
                                "bg-indigo-600 hover:bg-indigo-700 text-white"
                            }
                        )
                    }
                >
                    {move || if copied_live.get() { "Copied!" } else { "Copy Epoch" }}
                </button>
            </div>

            // 2. Converters Row
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 items-start">
                // Left Card: Timestamp -> Date
                <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-5">
                    <h3 class="text-sm font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Timestamp to Date"</h3>

                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-semibold text-slate-700 dark:text-slate-300">"Unix Epoch Timestamp (sec or ms)"</label>
                        <input
                            type="text"
                            prop:value=ts_input
                            on:input=move |ev| set_ts_input.set(event_target_value(&ev))
                            placeholder="Enter epoch timestamp..."
                            class="w-full px-4 py-2.5 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm rounded-lg outline-none focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 transition"
                        />
                    </div>

                    // Conversion Outcomes
                    <div class="border-t border-slate-100 dark:border-slate-800 pt-4 space-y-3.5">
                        {move || match ts_conversion.get() {
                            Err(err) => view! {
                                <div class="text-xs text-rose-500 font-semibold">{err}</div>
                            }.into_any(),
                            Ok((sec, _ms, date)) => view! {
                                <div class="space-y-3 text-sm">
                                    <div class="grid grid-cols-3 gap-2 py-1.5 border-b border-slate-50 dark:border-slate-800/60">
                                        <span class="text-xs text-slate-400 dark:text-slate-500 font-medium">"ISO 8601"</span>
                                        <span class="col-span-2 font-mono text-xs text-slate-800 dark:text-slate-200 break-all select-all">{format_date_to_iso(&date)}</span>
                                    </div>
                                    <div class="grid grid-cols-3 gap-2 py-1.5 border-b border-slate-50 dark:border-slate-800/60">
                                        <span class="text-xs text-slate-400 dark:text-slate-500 font-medium">"UTC String"</span>
                                        <span class="col-span-2 font-mono text-xs text-slate-800 dark:text-slate-200 break-all select-all">{format_date_to_utc(&date)}</span>
                                    </div>
                                    <div class="grid grid-cols-3 gap-2 py-1.5 border-b border-slate-50 dark:border-slate-800/60">
                                        <span class="text-xs text-slate-400 dark:text-slate-500 font-medium">"Local String"</span>
                                        <span class="col-span-2 font-mono text-xs text-slate-800 dark:text-slate-200 break-all select-all">{format_date_to_local(&date)}</span>
                                    </div>
                                    <div class="grid grid-cols-3 gap-2 py-1.5">
                                        <span class="text-xs text-slate-400 dark:text-slate-500 font-medium">"Relative Time"</span>
                                        <span class="col-span-2 text-xs font-semibold text-indigo-600 dark:text-indigo-400">{format_relative_time(sec)}</span>
                                    </div>
                                </div>
                            }.into_any()
                        }}
                    </div>
                </div>

                // Right Card: Date -> Timestamp
                <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-5">
                    <h3 class="text-sm font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Date to Timestamp"</h3>

                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-semibold text-slate-700 dark:text-slate-300">"Readable Date String (ISO, UTC, or Local)"</label>
                        <input
                            type="text"
                            prop:value=date_input
                            on:input=move |ev| set_date_input.set(event_target_value(&ev))
                            placeholder="Enter date string..."
                            class="w-full px-4 py-2.5 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm rounded-lg outline-none focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 transition"
                        />
                    </div>

                    // Conversion Outcomes
                    <div class="border-t border-slate-100 dark:border-slate-800 pt-4 space-y-3.5">
                        {move || match date_conversion.get() {
                            Err(err) => view! {
                                <div class="text-xs text-rose-500 font-semibold">{err}</div>
                            }.into_any(),
                            Ok((sec, ms, date)) => view! {
                                <div class="space-y-3 text-sm">
                                    <div class="grid grid-cols-3 gap-2 py-1.5 border-b border-slate-50 dark:border-slate-800/60">
                                        <span class="text-xs text-slate-400 dark:text-slate-500 font-medium">"Seconds"</span>
                                        <span class="col-span-2 font-mono text-xs text-slate-800 dark:text-slate-200 select-all font-semibold">{sec}</span>
                                    </div>
                                    <div class="grid grid-cols-3 gap-2 py-1.5 border-b border-slate-50 dark:border-slate-800/60">
                                        <span class="text-xs text-slate-400 dark:text-slate-500 font-medium">"Milliseconds"</span>
                                        <span class="col-span-2 font-mono text-xs text-slate-800 dark:text-slate-200 select-all font-semibold">{ms}</span>
                                    </div>
                                    <div class="grid grid-cols-3 gap-2 py-1.5">
                                        <span class="text-xs text-slate-400 dark:text-slate-500 font-medium">"Parsed UTC"</span>
                                        <span class="col-span-2 font-mono text-xs text-slate-500 dark:text-slate-400 break-all">{format_date_to_utc(&date)}</span>
                                    </div>
                                </div>
                            }.into_any()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
