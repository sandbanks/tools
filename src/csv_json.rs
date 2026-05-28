use leptos::prelude::*;
use std::collections::HashSet;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            fields.push(field.trim().to_string());
            field.clear();
        } else {
            field.push(c);
        }
    }
    fields.push(field.trim().to_string());
    fields
}

fn escape_csv_field(val: &serde_json::Value) -> String {
    let s = match val {
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.clone(),
        _ => val.to_string(),
    };
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s
    }
}

#[component]
pub fn CsvJsonConverter() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (is_csv_to_json, set_is_csv_to_json) = signal(true);
    let (copied, set_copied) = signal(false);

    let conversion_result = Memo::new(move |_| {
        let text = input.get();
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        if is_csv_to_json.get() {
            // Convert CSV -> JSON
            let mut lines = text.lines().filter(|l| !l.trim().is_empty());
            let header_line = match lines.next() {
                Some(h) => h,
                None => return Err("CSV input is empty".to_string()),
            };

            let headers = parse_csv_line(header_line);
            if headers.is_empty() || (headers.len() == 1 && headers[0].is_empty()) {
                return Err("Failed to parse headers from the first row".to_string());
            }

            let mut json_arr = Vec::new();

            for line in lines {
                let fields = parse_csv_line(line);
                let mut map = serde_json::Map::new();

                for (i, header) in headers.iter().enumerate() {
                    let val_str = if i < fields.len() {
                        fields[i].clone()
                    } else {
                        String::new()
                    };

                    // Try parsing fields as numbers or booleans if they match standard formats
                    let json_val = if val_str.eq_ignore_ascii_case("true") {
                        serde_json::Value::Bool(true)
                    } else if val_str.eq_ignore_ascii_case("false") {
                        serde_json::Value::Bool(false)
                    } else if val_str.is_empty() {
                        serde_json::Value::Null
                    } else if let Ok(num) = val_str.parse::<i64>() {
                        serde_json::Value::Number(num.into())
                    } else if let Ok(f_num) = val_str.parse::<f64>() {
                        if let Some(n) = serde_json::Number::from_f64(f_num) {
                            serde_json::Value::Number(n)
                        } else {
                            serde_json::Value::String(val_str)
                        }
                    } else {
                        // Strip surrounding quotes if the field parsing returned them
                        let stripped = if val_str.starts_with('"')
                            && val_str.ends_with('"')
                            && val_str.len() >= 2
                        {
                            val_str[1..val_str.len() - 1].replace("\"\"", "\"")
                        } else {
                            val_str
                        };
                        serde_json::Value::String(stripped)
                    };

                    map.insert(header.clone(), json_val);
                }
                json_arr.push(serde_json::Value::Object(map));
            }

            match serde_json::to_string_pretty(&serde_json::Value::Array(json_arr)) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("Serialization error: {}", e)),
            }
        } else {
            // Convert JSON -> CSV
            let parsed: serde_json::Value = match serde_json::from_str(&text) {
                Ok(val) => val,
                Err(e) => return Err(format!("Invalid JSON: {}", e)),
            };

            let objects = match &parsed {
                serde_json::Value::Array(arr) => {
                    let mut objs = Vec::new();
                    for item in arr {
                        if let serde_json::Value::Object(obj) = item {
                            objs.push(obj);
                        } else {
                            return Err("JSON Array must contain only objects to represent rows."
                                .to_string());
                        }
                    }
                    objs
                }
                serde_json::Value::Object(obj) => vec![obj],
                _ => {
                    return Err(
                        "JSON input must be an array of objects or a single object.".to_string()
                    )
                }
            };

            if objects.is_empty() {
                return Ok(String::new());
            }

            // Extract unique keys as headers (keep order of appearance)
            let mut headers_set = HashSet::new();
            let mut headers = Vec::new();
            for obj in &objects {
                for key in obj.keys() {
                    if headers_set.insert(key.clone()) {
                        headers.push(key.clone());
                    }
                }
            }

            let mut csv_output = String::new();

            // Write Header
            let escaped_headers: Vec<String> = headers
                .iter()
                .map(|h| format!("\"{}\"", h.replace('"', "\"\"")))
                .collect();
            csv_output.push_str(&escaped_headers.join(","));
            csv_output.push_str("\n");

            // Write Rows
            for obj in objects {
                let mut row_fields = Vec::new();
                for header in &headers {
                    let val = obj.get(header).unwrap_or(&serde_json::Value::Null);
                    row_fields.push(escape_csv_field(val));
                }
                csv_output.push_str(&row_fields.join(","));
                csv_output.push_str("\n");
            }

            Ok(csv_output)
        }
    });

    let output_text = Memo::new(move |_| match conversion_result.get() {
        Ok(val) => val,
        Err(_) => String::new(),
    });

    let error_msg = Memo::new(move |_| match conversion_result.get() {
        Err(e) => Some(e),
        Ok(_) => None,
    });

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
        if is_csv_to_json.get() {
            let sample = r#"id,name,active,price,category
101,Rust Book,true,39.95,Books
102,WASM Widget,false,12.50,Utilities
103,Premium "Bundle",true,99.99,"Electronics, Bundle"
104,Sticker Pack,true,2.99,""
"#;
            set_input.set(sample.to_string());
        } else {
            let sample = r#"[
  {
    "id": 101,
    "name": "Rust Book",
    "active": true,
    "price": 39.95,
    "category": "Books"
  },
  {
    "id": 102,
    "name": "WASM Widget",
    "active": false,
    "price": 12.5,
    "category": "Utilities"
  },
  {
    "id": 103,
    "name": "Premium \"Bundle\"",
    "active": true,
    "price": 99.99,
    "category": "Electronics, Bundle"
  }
]"#;
            set_input.set(sample.to_string());
        }
    };

    view! {
        <div class="space-y-6 animate-fade-in">
            // Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"CSV ↔ JSON Converter"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">"Convert tabular data between CSV format and structured JSON arrays effortlessly."</p>
            </div>

            // Card Panel
            <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-6">
                // Mode Select (Tabs) & Controls
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                    <div class="inline-flex p-1 bg-slate-100 dark:bg-slate-800/80 rounded-lg">
                        <button
                            on:click=move |_| {
                                set_is_csv_to_json.set(true);
                                set_input.set(String::new());
                            }
                            class=move || {
                                let active = is_csv_to_json.get();
                                format!(
                                    "px-4 py-1.5 text-sm font-semibold rounded-md transition-all duration-200 {}",
                                    if active {
                                        "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                    } else {
                                        "text-slate-600 dark:text-slate-400 hover:text-slate-900"
                                    }
                                )
                            }
                        >
                            "CSV to JSON"
                        </button>
                        <button
                            on:click=move |_| {
                                set_is_csv_to_json.set(false);
                                set_input.set(String::new());
                            }
                            class=move || {
                                let active = !is_csv_to_json.get();
                                format!(
                                    "px-4 py-1.5 text-sm font-semibold rounded-md transition-all duration-200 {}",
                                    if active {
                                        "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                    } else {
                                        "text-slate-600 dark:text-slate-400 hover:text-slate-900"
                                    }
                                )
                            }
                        >
                            "JSON to CSV"
                        </button>
                    </div>

                    <button
                        on:click=load_sample
                        class="px-4 py-2 border border-dashed border-slate-200 dark:border-slate-800 hover:border-indigo-500 dark:hover:border-indigo-500 text-indigo-600 dark:text-indigo-400 font-semibold rounded-lg text-sm transition duration-200"
                    >
                        "Load Sample Data"
                    </button>
                </div>

                // Inputs
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">
                            {move || if is_csv_to_json.get() { "CSV Input" } else { "JSON Input" }}
                        </label>
                        <textarea
                            rows="20"
                            placeholder=move || if is_csv_to_json.get() { "Paste CSV here..." } else { "Paste JSON here..." }
                            prop:value=input
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                            class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-xs focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                        />
                    </div>

                    <div class="flex flex-col space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">
                            {move || if is_csv_to_json.get() { "JSON Output" } else { "CSV Output" }}
                        </label>
                        <textarea
                            rows="20"
                            readonly=true
                            placeholder=move || if is_csv_to_json.get() { "JSON will appear here..." } else { "CSV will appear here..." }
                            prop:value=output_text
                            class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-xs focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar"
                        />
                    </div>
                </div>

                // Error Message Box
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
