use leptos::prelude::*;
use qrcodegen::{QrCode, QrCodeEcc};
use wasm_bindgen::JsCast;
use web_sys::window;

#[component]
pub fn QrGenerator() -> impl IntoView {
    let (url, set_url) = signal("https://github.com".to_string());
    let (body_style, set_body_style) = signal("square".to_string());
    let (eye_frame_style, set_eye_frame_style) = signal("square".to_string());
    let (eye_ball_style, set_eye_ball_style) = signal("square".to_string());

    // Colors
    let (bg_color, set_bg_color) = signal("#ffffff".to_string());
    let (fg_type, set_fg_type) = signal("solid".to_string());
    let (fg_color, set_fg_color) = signal("#000000".to_string());
    let (grad_start, set_grad_start) = signal("#312e81".to_string());
    let (grad_end, set_grad_end) = signal("#4f46e5".to_string());
    let (grad_type, set_grad_type) = signal("linear".to_string());
    let (grad_angle, set_grad_angle) = signal(45);

    let (custom_eye_color, set_custom_eye_color) = signal(false);
    let (eye_color, set_eye_color) = signal("#1d4ed8".to_string()); // default custom eye color: blue-700

    // Logo
    let (logo_type, set_logo_type) = signal("none".to_string());
    let (logo_preset, set_logo_preset) = signal("🦀".to_string());
    let (logo_upload, set_logo_upload) = signal(String::new());

    // Generate QrCode from URL
    let qr_code_result = Memo::new(move |_| {
        let text = url.get();
        if text.trim().is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        // We use High error correction so that overlaying logos doesn't break scans
        match QrCode::encode_text(&text, QrCodeEcc::High) {
            Ok(qr) => Ok(qr),
            Err(e) => Err(format!("Failed to encode: {}", e)),
        }
    });

    let on_file_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let reader = web_sys::FileReader::new().unwrap();
                let reader_c = reader.clone();

                let onload =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                        if let Ok(result) = reader_c.result() {
                            if let Some(data_url) = result.as_string() {
                                set_logo_upload.set(data_url);
                                set_logo_type.set("upload".to_string());
                            }
                        }
                    })
                        as Box<dyn FnMut(web_sys::Event)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                let _ = reader.read_as_data_url(&file);
                onload.forget();
            }
        }
    };

    let trigger_download = move |format: &'static str| {
        if let Some(win) = window() {
            // Retrieve JS function window.downloadQrCode
            let val =
                js_sys::Reflect::get(&win, &wasm_bindgen::JsValue::from_str("downloadQrCode"))
                    .unwrap();
            if !val.is_undefined() {
                let func: js_sys::Function = val.unchecked_into();
                let svg_id = wasm_bindgen::JsValue::from_str("qr-svg-preview");
                let filename = wasm_bindgen::JsValue::from_str("qr-code");
                let fmt = wasm_bindgen::JsValue::from_str(format);
                let _ = func.call3(&wasm_bindgen::JsValue::UNDEFINED, &svg_id, &filename, &fmt);
            }
        }
    };

    view! {
        <div class="space-y-6 animate-fade-in">
            // Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"QR Code Generator"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">"Create customized, high-quality QR codes with custom colors, gradients, styles, and logos."</p>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                // Left Panel: Configuration (Spans 2 columns)
                <div class="lg:col-span-2 bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-6">

                    // 1. Text/URL Input (Required)
                    <div class="space-y-2">
                        <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"URL / Text (Required)"</label>
                        <input
                            type="text"
                            placeholder="Enter URL to encode (e.g. https://example.com)..."
                            prop:value=url
                            on:input=move |ev| set_url.set(event_target_value(&ev))
                            class="w-full px-4 py-2.5 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm text-slate-900 dark:text-white focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none transition-all"
                        />
                    </div>

                    // 2. Custom Shapes/Designs
                    <div class="border-t border-slate-100 dark:border-slate-800/60 pt-5 space-y-4">
                        <h3 class="text-sm font-bold text-slate-850 dark:text-slate-200">"Design & Styles"</h3>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            // Body shape
                            <div class="flex flex-col space-y-1.5">
                                <label class="text-xs font-semibold text-slate-500 dark:text-slate-400">"Body Module Style"</label>
                                <select
                                    on:change=move |ev| set_body_style.set(event_target_value(&ev))
                                    prop:value=body_style
                                    class="px-3 py-2 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none"
                                >
                                    <option value="square">"Classic (Squares)"</option>
                                    <option value="dot">"Dots (Circles)"</option>
                                    <option value="rounded">"Rounded Squares"</option>
                                </select>
                            </div>

                            // Eye Frame Shape
                            <div class="flex flex-col space-y-1.5">
                                <label class="text-xs font-semibold text-slate-500 dark:text-slate-400">"Eye Outer Frame"</label>
                                <select
                                    on:change=move |ev| set_eye_frame_style.set(event_target_value(&ev))
                                    prop:value=eye_frame_style
                                    class="px-3 py-2 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none"
                                >
                                    <option value="square">"Square Frame"</option>
                                    <option value="rounded">"Rounded Frame"</option>
                                    <option value="circle">"Circle Frame"</option>
                                </select>
                            </div>

                            // Eye Ball Shape
                            <div class="flex flex-col space-y-1.5">
                                <label class="text-xs font-semibold text-slate-500 dark:text-slate-400">"Eye Inner Ball"</label>
                                <select
                                    on:change=move |ev| set_eye_ball_style.set(event_target_value(&ev))
                                    prop:value=eye_ball_style
                                    class="px-3 py-2 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none"
                                >
                                    <option value="square">"Square Ball"</option>
                                    <option value="rounded">"Rounded Ball"</option>
                                    <option value="circle">"Circle Ball"</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    // 3. Color Customization
                    <div class="border-t border-slate-100 dark:border-slate-800/60 pt-5 space-y-4">
                        <h3 class="text-sm font-bold text-slate-850 dark:text-slate-200">"Colors & Gradients"</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            // Left: Background & Solid/Gradient Choice
                            <div class="space-y-4">
                                <div class="flex flex-col space-y-1.5">
                                    <label class="text-xs font-semibold text-slate-500 dark:text-slate-400">"Background Color"</label>
                                    <div class="flex items-center space-x-2">
                                        <input
                                            type="color"
                                            prop:value=bg_color
                                            on:input=move |ev| set_bg_color.set(event_target_value(&ev))
                                            class="w-8 h-8 rounded-md cursor-pointer border border-slate-200 dark:border-slate-800 bg-transparent"
                                        />
                                        <input
                                            type="text"
                                            prop:value=bg_color
                                            on:input=move |ev| set_bg_color.set(event_target_value(&ev))
                                            class="flex-1 px-3 py-1 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm font-mono focus:ring-1 focus:ring-indigo-500 outline-none"
                                        />
                                    </div>
                                </div>

                                <div class="flex flex-col space-y-1.5">
                                    <label class="text-xs font-semibold text-slate-500 dark:text-slate-400">"Foreground Style"</label>
                                    <div class="inline-flex p-1 bg-slate-100 dark:bg-slate-800/80 rounded-lg w-fit">
                                        <button
                                            on:click=move |_| set_fg_type.set("solid".to_string())
                                            class=move || {
                                                let active = fg_type.get() == "solid";
                                                format!(
                                                    "px-3 py-1 text-xs font-bold rounded-md transition-all {}",
                                                    if active {
                                                        "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                                    } else {
                                                        "text-slate-600 dark:text-slate-400 hover:text-slate-900"
                                                    }
                                                )
                                            }
                                        >
                                            "Solid"
                                        </button>
                                        <button
                                            on:click=move |_| set_fg_type.set("gradient".to_string())
                                            class=move || {
                                                let active = fg_type.get() == "gradient";
                                                format!(
                                                    "px-3 py-1 text-xs font-bold rounded-md transition-all {}",
                                                    if active {
                                                        "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                                    } else {
                                                        "text-slate-600 dark:text-slate-400 hover:text-slate-900"
                                                    }
                                                )
                                            }
                                        >
                                            "Gradient"
                                        </button>
                                    </div>
                                </div>
                            </div>

                            // Right: Solid/Gradient detailed controls
                            <div class="space-y-4 bg-slate-50 dark:bg-slate-950 p-4 rounded-xl border border-slate-200/50 dark:border-slate-800/60">
                                {move || if fg_type.get() == "solid" {
                                    view! {
                                        <div class="flex flex-col space-y-1.5 animate-fade-in">
                                            <label class="text-xs font-semibold text-slate-500 dark:text-slate-400">"Foreground Color"</label>
                                            <div class="flex items-center space-x-2">
                                                <input
                                                    type="color"
                                                    prop:value=fg_color
                                                    on:input=move |ev| set_fg_color.set(event_target_value(&ev))
                                                    class="w-8 h-8 rounded-md cursor-pointer border border-slate-200 dark:border-slate-800 bg-transparent"
                                                />
                                                <input
                                                    type="text"
                                                    prop:value=fg_color
                                                    on:input=move |ev| set_fg_color.set(event_target_value(&ev))
                                                    class="flex-1 px-3 py-1 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg text-sm font-mono focus:ring-1 focus:ring-indigo-500 outline-none"
                                                />
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-3 animate-fade-in">
                                            // Gradient type
                                            <div class="flex items-center justify-between">
                                                <label class="text-xs font-semibold text-slate-500 dark:text-slate-400">"Gradient Mode"</label>
                                                <select
                                                    on:change=move |ev| set_grad_type.set(event_target_value(&ev))
                                                    prop:value=grad_type
                                                    class="px-2 py-0.5 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-md text-xs outline-none"
                                                >
                                                    <option value="linear">"Linear"</option>
                                                    <option value="radial">"Radial"</option>
                                                </select>
                                            </div>

                                            // Start / End color pickers
                                            <div class="grid grid-cols-2 gap-3">
                                                <div class="flex flex-col space-y-1">
                                                    <span class="text-[10px] font-bold text-slate-400 uppercase">"Start"</span>
                                                    <div class="flex items-center space-x-1.5">
                                                        <input
                                                            type="color"
                                                            prop:value=grad_start
                                                            on:input=move |ev| set_grad_start.set(event_target_value(&ev))
                                                            class="w-6 h-6 rounded-md cursor-pointer"
                                                        />
                                                        <input
                                                            type="text"
                                                            prop:value=grad_start
                                                            on:input=move |ev| set_grad_start.set(event_target_value(&ev))
                                                            class="w-full px-2 py-0.5 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-md text-xs font-mono"
                                                        />
                                                    </div>
                                                </div>
                                                <div class="flex flex-col space-y-1">
                                                    <span class="text-[10px] font-bold text-slate-400 uppercase">"End"</span>
                                                    <div class="flex items-center space-x-1.5">
                                                        <input
                                                            type="color"
                                                            prop:value=grad_end
                                                            on:input=move |ev| set_grad_end.set(event_target_value(&ev))
                                                            class="w-6 h-6 rounded-md cursor-pointer"
                                                        />
                                                        <input
                                                            type="text"
                                                            prop:value=grad_end
                                                            on:input=move |ev| set_grad_end.set(event_target_value(&ev))
                                                            class="w-full px-2 py-0.5 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-md text-xs font-mono"
                                                        />
                                                    </div>
                                                </div>
                                            </div>

                                            // Direction Angle slider (Only show for linear gradient)
                                            {move || if grad_type.get() == "linear" {
                                                Some(view! {
                                                    <div class="flex flex-col space-y-1 mt-1 animate-fade-in">
                                                        <div class="flex items-center justify-between text-xs text-slate-500">
                                                            <span>"Angle"</span>
                                                            <span>{move || format!("{}°", grad_angle.get())}</span>
                                                        </div>
                                                        <input
                                                            type="range"
                                                            min="0"
                                                            max="360"
                                                            prop:value=grad_angle
                                                            on:input=move |ev| {
                                                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                                    set_grad_angle.set(val);
                                                                }
                                                            }
                                                            class="w-full accent-indigo-500 cursor-pointer h-1.5 bg-slate-200 dark:bg-slate-800 rounded-lg appearance-none"
                                                        />
                                                    </div>
                                                })
                                            } else {
                                                None
                                            }}
                                        </div>
                                    }.into_any()
                                }}
                            </div>
                        </div>

                        // Eyeball & Frame custom color picker toggle
                        <div class="pt-4 border-t border-slate-100 dark:border-slate-800/40">
                            <div class="flex items-start md:items-center justify-between flex-col md:flex-row gap-4">
                                <label class="flex items-center space-x-2.5 cursor-pointer select-none">
                                    <input
                                        type="checkbox"
                                        prop:checked=custom_eye_color
                                        on:change=move |ev| set_custom_eye_color.set(event_target_checked(&ev))
                                        class="w-4 h-4 rounded-md border-slate-300 dark:border-slate-700 text-indigo-600 focus:ring-indigo-500 focus:ring-offset-2 dark:bg-slate-800"
                                    />
                                    <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Customize Eye Color (Finder patterns)"</span>
                                </label>

                                {move || if custom_eye_color.get() {
                                    Some(view! {
                                        <div class="flex items-center space-x-2 animate-fade-in w-full md:w-auto">
                                            <input
                                                type="color"
                                                prop:value=eye_color
                                                on:input=move |ev| set_eye_color.set(event_target_value(&ev))
                                                class="w-8 h-8 rounded-md cursor-pointer border border-slate-200 dark:border-slate-800 bg-transparent"
                                            />
                                            <input
                                                type="text"
                                                prop:value=eye_color
                                                on:input=move |ev| set_eye_color.set(event_target_value(&ev))
                                                class="w-32 px-3 py-1 bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg text-sm font-mono focus:ring-1 focus:ring-indigo-500 outline-none"
                                            />
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                            </div>
                        </div>
                    </div>

                    // 4. Logo Customization
                    <div class="border-t border-slate-100 dark:border-slate-800/60 pt-5 space-y-4">
                        <h3 class="text-sm font-bold text-slate-850 dark:text-slate-200">"Logo / Central Image"</h3>
                        <div class="flex flex-col space-y-3">
                            // Select Logo Type
                            <div class="inline-flex p-1 bg-slate-100 dark:bg-slate-800/80 rounded-lg w-fit">
                                <button
                                    on:click=move |_| set_logo_type.set("none".to_string())
                                    class=move || {
                                        let active = logo_type.get() == "none";
                                        format!(
                                            "px-4 py-1.5 text-xs font-bold rounded-md transition-all {}",
                                            if active {
                                                "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                            } else {
                                                "text-slate-600 dark:text-slate-400 hover:text-slate-900"
                                            }
                                        )
                                    }
                                >
                                    "No Logo"
                                </button>
                                <button
                                    on:click=move |_| set_logo_type.set("preset".to_string())
                                    class=move || {
                                        let active = logo_type.get() == "preset";
                                        format!(
                                            "px-4 py-1.5 text-xs font-bold rounded-md transition-all {}",
                                            if active {
                                                "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                            } else {
                                                "text-slate-600 dark:text-slate-400 hover:text-slate-900"
                                            }
                                        )
                                    }
                                >
                                    "Preset Emoji"
                                </button>
                                <button
                                    on:click=move |_| set_logo_type.set("upload".to_string())
                                    class=move || {
                                        let active = logo_type.get() == "upload";
                                        format!(
                                            "px-4 py-1.5 text-xs font-bold rounded-md transition-all {}",
                                            if active {
                                                "bg-white dark:bg-slate-700 text-indigo-600 dark:text-white shadow-xs"
                                            } else {
                                                "text-slate-600 dark:text-slate-400 hover:text-slate-900"
                                            }
                                        )
                                    }
                                >
                                    "Upload Image"
                                </button>
                            </div>

                            // Inner settings
                            {move || match logo_type.get().as_str() {
                                "preset" => {
                                    let presets = vec!["🦀", "🐹", "⚛️", "🚀", "💻", "🌐", "🔑", "📞", "✉️", "❤️", "⚡", "🔥"];
                                    Some(view! {
                                        <div class="p-4 bg-slate-50 dark:bg-slate-950 rounded-xl border border-slate-200/50 dark:border-slate-800/60 space-y-2 animate-fade-in">
                                            <span class="text-xs font-semibold text-slate-500">"Select a preset emoji:"</span>
                                            <div class="flex flex-wrap gap-2 pt-1">
                                                {presets.into_iter().map(|emoji| {
                                                    let is_selected = logo_preset.get() == emoji;
                                                    view! {
                                                        <button
                                                            on:click=move |_| set_logo_preset.set(emoji.to_string())
                                                            class=move || {
                                                                format!(
                                                                    "w-10 h-10 text-xl flex items-center justify-center rounded-lg border transition-all {}",
                                                                    if is_selected {
                                                                        "bg-indigo-50 dark:bg-indigo-950/40 border-indigo-500 scale-105"
                                                                    } else {
                                                                        "bg-white dark:bg-slate-900 border-slate-200 dark:border-slate-800 hover:bg-slate-100 dark:hover:bg-slate-800"
                                                                    }
                                                                )
                                                            }
                                                        >
                                                            {emoji}
                                                        </button>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    }.into_any())
                                }
                                "upload" => {
                                    Some(view! {
                                        <div class="p-4 bg-slate-50 dark:bg-slate-950 rounded-xl border border-slate-200/50 dark:border-slate-800/60 space-y-3 animate-fade-in">
                                            <span class="text-xs font-semibold text-slate-500">"Upload custom logo image (rendered inside the QR center):"</span>
                                            <div class="flex items-center space-x-4">
                                                <input
                                                    type="file"
                                                    accept="image/*"
                                                    on:change=on_file_change
                                                    class="block w-full text-xs text-slate-500 dark:text-slate-400 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-xs file:font-semibold file:bg-indigo-50 file:text-indigo-700 dark:file:bg-indigo-950/40 dark:file:text-indigo-400 file:cursor-pointer"
                                                />
                                            </div>
                                            {move || if !logo_upload.get().is_empty() {
                                                Some(view! {
                                                    <div class="flex items-center space-x-2 pt-1 animate-fade-in">
                                                        <span class="text-[10px] text-emerald-600 dark:text-emerald-400 font-bold">"✓ Image loaded successfully"</span>
                                                        <button
                                                            on:click=move |_| set_logo_upload.set(String::new())
                                                            class="text-[10px] text-red-500 hover:underline font-bold"
                                                        >
                                                            "Remove"
                                                        </button>
                                                    </div>
                                                })
                                            } else {
                                                None
                                            }}
                                        </div>
                                    }.into_any())
                                }
                                _ => None
                            }}
                        </div>
                    </div>
                </div>

                // Right Panel: Preview & Downloads (Spans 1 column)
                <div class="flex flex-col space-y-6">
                    <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 space-y-6 flex-1 flex flex-col justify-between">
                        <div class="space-y-4">
                            <label class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400 block">"QR Code Preview"</label>

                            // SVG Preview container
                            <div class="p-4 rounded-xl border border-slate-100 dark:border-slate-800/80 bg-slate-50 dark:bg-slate-950/50 flex items-center justify-center flex-1 min-h-[300px]">
                                {move || match qr_code_result.get() {
                                    Ok(qr) => {
                                        let size = qr.size();
                                        let padding = 3;
                                        let grid_size = size + 2 * padding;
                                        let module_size = 500.0 / grid_size as f64;

                                        let fill_color = if fg_type.get() == "gradient" {
                                            "url(#qr-fg-grad)".to_string()
                                        } else {
                                            fg_color.get()
                                        };

                                        let body_shape = body_style.get();
                                        let has_logo = logo_type.get() != "none";

                                        // Body Modules
                                        let mut body_elements = Vec::new();
                                        for y in 0..size {
                                            for x in 0..size {
                                                if !qr.get_module(x, y) {
                                                    continue;
                                                }

                                                // Skip Finder Patterns (Eyes)
                                                if (x < 7 && y < 7) || (x >= size - 7 && y < 7) || (x < 7 && y >= size - 7) {
                                                    continue;
                                                }

                                                // Skip center if logo is active
                                                if has_logo {
                                                    let mid = size / 2;
                                                    let radius = 2; // skip a 5x5 block in center
                                                    if x >= mid - radius && x <= mid + radius && y >= mid - radius && y <= mid + radius {
                                                        continue;
                                                    }
                                                }

                                                let mx = (x + padding) as f64 * module_size;
                                                let my = (y + padding) as f64 * module_size;

                                                let element = match body_shape.as_str() {
                                                    "dot" => {
                                                        view! {
                                                            <circle cx=mx + module_size / 2.0 cy=my + module_size / 2.0 r=module_size * 0.4 fill=fill_color.clone() />
                                                        }.into_any()
                                                    }
                                                    "rounded" => {
                                                        view! {
                                                            <rect x=mx + 0.5 y=my + 0.5 width=module_size - 1.0 height=module_size - 1.0 rx=module_size * 0.3 ry=module_size * 0.3 fill=fill_color.clone() />
                                                        }.into_any()
                                                    }
                                                    _ => { // "square"
                                                        view! {
                                                            <rect x=mx y=my width=module_size height=module_size fill=fill_color.clone() />
                                                        }.into_any()
                                                    }
                                                };
                                                body_elements.push(element);
                                            }
                                        }

                                        // Eye configuration
                                        let eye_fill = if custom_eye_color.get() {
                                            eye_color.get()
                                        } else if fg_type.get() == "gradient" {
                                            "url(#qr-fg-grad)".to_string()
                                        } else {
                                            fg_color.get()
                                        };

                                        let frame_shape = eye_frame_style.get();
                                        let ball_shape = eye_ball_style.get();
                                        let finders = vec![(0, 0), (size - 7, 0), (0, size - 7)];

                                        let mut eye_elements = Vec::new();
                                        for (fx, fy) in finders {
                                            let mx = (fx + padding) as f64 * module_size;
                                            let my = (fy + padding) as f64 * module_size;

                                            // Draw Outer Frame
                                            let frame_el = match frame_shape.as_str() {
                                                "rounded" => {
                                                    view! {
                                                        <rect x=mx + module_size * 0.5 y=my + module_size * 0.5
                                                              width=module_size * 6.0 height=module_size * 6.0
                                                              rx=module_size * 1.5 ry=module_size * 1.5
                                                              stroke=eye_fill.clone() stroke-width=module_size fill="none" />
                                                    }.into_any()
                                                }
                                                "circle" => {
                                                    view! {
                                                        <circle cx=mx + module_size * 3.5 cy=my + module_size * 3.5
                                                                r=module_size * 3.0 stroke=eye_fill.clone() stroke-width=module_size fill="none" />
                                                    }.into_any()
                                                }
                                                _ => { // "square"
                                                    view! {
                                                        <rect x=mx + module_size * 0.5 y=my + module_size * 0.5
                                                              width=module_size * 6.0 height=module_size * 6.0
                                                              stroke=eye_fill.clone() stroke-width=module_size fill="none" />
                                                    }.into_any()
                                                }
                                            };
                                            eye_elements.push(frame_el);

                                            // Draw Inner Ball
                                            let ball_el = match ball_shape.as_str() {
                                                "rounded" => {
                                                    view! {
                                                        <rect x=mx + module_size * 2.0 y=my + module_size * 2.0
                                                              width=module_size * 3.0 height=module_size * 3.0
                                                              rx=module_size * 0.8 ry=module_size * 0.8 fill=eye_fill.clone() />
                                                    }.into_any()
                                                }
                                                "circle" => {
                                                    view! {
                                                        <circle cx=mx + module_size * 3.5 cy=my + module_size * 3.5
                                                                r=module_size * 1.5 fill=eye_fill.clone() />
                                                    }.into_any()
                                                }
                                                _ => { // "square"
                                                    view! {
                                                        <rect x=mx + module_size * 2.0 y=my + module_size * 2.0
                                                              width=module_size * 3.0 height=module_size * 3.0 fill=eye_fill.clone() />
                                                    }.into_any()
                                                }
                                            };
                                            eye_elements.push(ball_el);
                                        }

                                        // Logo elements
                                        let logo_size = 90.0;
                                        let cutout_size = logo_size + 14.0;
                                        let has_logo_type = logo_type.get();

                                        let logo_content = if has_logo_type != "none" {
                                            let logo_el = match has_logo_type.as_str() {
                                                "preset" => {
                                                    let emoji = logo_preset.get();
                                                    view! {
                                                        <text x="250" y="258" font-size="64" font-family="Outfit, sans-serif" text-anchor="middle" dominant-baseline="middle">
                                                            {emoji}
                                                        </text>
                                                    }.into_any()
                                                }
                                                "upload" => {
                                                    let data_url = logo_upload.get();
                                                    if data_url.is_empty() {
                                                        view! { <g /> }.into_any()
                                                    } else {
                                                        view! {
                                                            <image href=data_url x=250.0 - logo_size / 2.0 y=250.0 - logo_size / 2.0 width=logo_size height=logo_size />
                                                        }.into_any()
                                                    }
                                                }
                                                _ => view! { <g /> }.into_any()
                                            };

                                            Some(view! {
                                                <rect x=250.0 - cutout_size / 2.0 y=250.0 - cutout_size / 2.0
                                                      width=cutout_size height=cutout_size rx=16 ry=16 fill=bg_color.get() />
                                                {logo_el}
                                            })
                                        } else {
                                            None
                                        };

                                        // Compile linear gradient properties
                                        let angle_rad = (grad_angle.get() as f64).to_radians();
                                        let x1 = 50.0 - angle_rad.cos() * 50.0;
                                        let y1 = 50.0 - angle_rad.sin() * 50.0;
                                        let x2 = 50.0 + angle_rad.cos() * 50.0;
                                        let y2 = 50.0 + angle_rad.sin() * 50.0;

                                        view! {
                                            <svg id="qr-svg-preview" viewBox="0 0 500 500" class="w-64 h-64 mx-auto select-none">
                                                <defs>
                                                    {if fg_type.get() == "gradient" {
                                                        if grad_type.get() == "linear" {
                                                            Some(view! {
                                                                <linearGradient id="qr-fg-grad" x1=format!("{}%", x1) y1=format!("{}%", y1) x2=format!("{}%", x2) y2=format!("{}%", y2)>
                                                                    <stop offset="0%" stop-color=grad_start.get() />
                                                                    <stop offset="100%" stop-color=grad_end.get() />
                                                                </linearGradient>
                                                            }.into_any())
                                                        } else {
                                                            Some(view! {
                                                                <radialGradient id="qr-fg-grad" cx="50%" cy="50%" r="50%" fx="50%" fy="50%">
                                                                    <stop offset="0%" stop-color=grad_start.get() />
                                                                    <stop offset="100%" stop-color=grad_end.get() />
                                                                </radialGradient>
                                                            }.into_any())
                                                        }
                                                    } else {
                                                        None
                                                    }}
                                                </defs>

                                                // Background rect
                                                <rect width="500" height="500" fill=bg_color.get() />

                                                // Modules
                                                {body_elements}

                                                // Eyes
                                                {eye_elements}

                                                // Logo overlay
                                                {logo_content}
                                            </svg>
                                        }.into_any()
                                    }
                                    Err(err) => {
                                        view! {
                                            <div class="text-center p-4 text-rose-500">
                                                <svg class="w-10 h-10 mx-auto text-rose-500 mb-2" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                                </svg>
                                                <span class="text-xs font-semibold">{err}</span>
                                            </div>
                                        }.into_any()
                                    }
                                }}
                            </div>
                        </div>

                        // Downloads
                        <div class="space-y-3 pt-6 border-t border-slate-100 dark:border-slate-800/60">
                            <span class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Export"</span>
                            <div class="grid grid-cols-2 gap-3">
                                <button
                                    on:click=move |_| trigger_download("png")
                                    disabled=move || qr_code_result.get().is_err()
                                    class="flex items-center justify-center space-x-2 px-4 py-2.5 font-bold rounded-lg text-sm bg-indigo-600 hover:bg-indigo-700 text-white shadow-xs hover:shadow-indigo-500/10 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                    </svg>
                                    <span>"Download PNG"</span>
                                </button>
                                <button
                                    on:click=move |_| trigger_download("svg")
                                    disabled=move || qr_code_result.get().is_err()
                                    class="flex items-center justify-center space-x-2 px-4 py-2.5 font-bold rounded-lg text-sm border border-slate-200 dark:border-slate-800 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                    </svg>
                                    <span>"Download SVG"</span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
