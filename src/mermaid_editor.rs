use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn renderMermaid(id: &str, code: &str, is_dark: bool) -> Result<JsValue, JsValue>;
}

fn parse_render_result(val: JsValue) -> Result<String, String> {
    let success = js_sys::Reflect::get(&val, &JsValue::from_str("success"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if success {
        let svg = js_sys::Reflect::get(&val, &JsValue::from_str("svg"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "Failed to extract SVG".to_string());
        Ok(svg)
    } else {
        let error = js_sys::Reflect::get(&val, &JsValue::from_str("error"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "Unknown rendering error".to_string());
        Err(error)
    }
}

// Templates helper
fn get_template(name: &str) -> &'static str {
    match name {
        "Flowchart" => {
            "\
graph TD
    A[Start] --> B{Is it working?}
    B -- Yes --> C[Great!]
    B -- No --> D[Debug it]
    D --> B
"
        }
        "Sequence" => {
            "\
sequenceDiagram
    Alice->>Bob: Hello Bob, how are you?
    alt is sick
        Bob-->>Alice: Not so good :(
    else is well
        Bob-->>Alice: Feeling great!
    end
    Bob->>Alice: How about you?
    Alice-->>Bob: Good!
"
        }
        "Class" => {
            "\
classDiagram
    Animal <|-- Duck
    Animal <|-- Fish
    Animal <|-- Zebra
    Animal : +int age
    Animal : +String gender
    Animal: +isMammal()
    Animal: +mate()
    class Duck{
        +String beakColor
        +swim()
        +quack()
    }
    class Fish{
        -int sizeInFeet
        -canEat()
    }
"
        }
        "State" => {
            "\
stateDiagram-v2
    [*] --> Still
    Still --> [*]
    Still --> Moving
    Moving --> Still
    Moving --> Crash
    Crash --> [*]
"
        }
        "Gantt" => {
            "\
gantt
    title A Gantt Chart
    dateFormat YYYY-MM-DD
    section Section
    A task           :a1, 2026-05-20, 30d
    Another task     :after a1, 20d
    section Another
    Task in Another  :2026-05-28, 12d
    another task     :24d
"
        }
        _ => "graph TD\n    A --> B",
    }
}

#[component]
pub fn MermaidEditor() -> impl IntoView {
    // Consume theme context if provided, default to true (dark)
    let is_dark_theme = use_context::<ReadSignal<bool>>().unwrap_or_else(|| signal(true).0);

    let default_code = get_template("Flowchart").to_string();
    let (code, set_code) = signal(default_code);
    let (is_live, set_is_live) = signal(true);

    // Rendering states
    let (rendered_svg, set_rendered_svg) = signal(String::new());
    let (error_msg, set_error_msg) = signal(None::<String>);
    let (rendering, set_rendering) = signal(false);

    // Trigger render manually or reactively
    let (trigger_counter, set_trigger_counter) = signal(0);

    let handle_render = move || {
        let code_val = code.get();
        let dark = is_dark_theme.get();

        if code_val.trim().is_empty() {
            set_rendered_svg.set(String::new());
            set_error_msg.set(None);
            return;
        }

        set_rendering.set(true);
        spawn_local(async move {
            // Unique ID to avoid clashes
            let id = "mermaid-svg-render-temp";
            match renderMermaid(id, &code_val, dark).await {
                Ok(val) => {
                    set_rendering.set(false);
                    match parse_render_result(val) {
                        Ok(svg) => {
                            set_rendered_svg.set(svg);
                            set_error_msg.set(None);
                        }
                        Err(err) => {
                            set_error_msg.set(Some(err));
                        }
                    }
                }
                Err(err) => {
                    set_rendering.set(false);
                    let err_str = err.as_string().unwrap_or_else(|| format!("{:?}", err));
                    set_error_msg.set(Some(err_str));
                }
            }
        });
    };

    // Effect for live-rendering and theme changes
    Effect::new(move |_| {
        // Track dependencies
        let _ = code.get();
        let _ = is_dark_theme.get();
        let _ = trigger_counter.get();

        if is_live.get() {
            handle_render();
        }
    });

    let handle_manual_render = move |_| {
        handle_render();
    };

    let handle_template_change = move |ev| {
        let name = event_target_value(&ev);
        let tpl = get_template(&name).to_string();
        set_code.set(tpl);
        if !is_live.get() {
            set_trigger_counter.update(|c| *c += 1);
        }
    };

    let handle_export = move |_| {
        let svg_content = rendered_svg.get();
        if svg_content.is_empty() {
            return;
        }

        if let Some(win) = window() {
            if let Some(doc) = win.document() {
                if let Ok(Some(el)) = doc
                    .create_element("a")
                    .map(|v| v.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    let encoded = js_sys::encode_uri_component(&svg_content);
                    let href = format!("data:image/svg+xml;utf8,{}", encoded);
                    let _ = el.set_attribute("href", &href);
                    let _ = el.set_attribute("download", "diagram.svg");
                    el.click();
                }
            }
        }
    };

    let handle_clear = move |_| {
        set_code.set(String::new());
        set_rendered_svg.set(String::new());
        set_error_msg.set(None);
    };

    view! {
        <div class="space-y-6">
            // Tool Header
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">"Mermaid Diagram Editor"</h1>
                <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">
                    "Write code using Mermaid syntax to generate Flowcharts, Sequence Diagrams, and more client-side."
                </p>
            </div>

            // Editor / Preview split panel
            <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 items-stretch">
                // Left Panel: Code Input & Config
                <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 flex flex-col space-y-4">
                    <div class="flex items-center justify-between">
                        <span class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Mermaid Syntax"</span>

                        // Select Template dropdown
                        <select
                            on:change=handle_template_change
                            class="bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-900 dark:text-white rounded-lg px-2.5 py-1 text-xs outline-none"
                        >
                            <option value="Flowchart">"Preset: Flowchart"</option>
                            <option value="Sequence">"Preset: Sequence Diagram"</option>
                            <option value="Class">"Preset: Class Diagram"</option>
                            <option value="State">"Preset: State Diagram"</option>
                            <option value="Gantt">"Preset: Gantt Chart"</option>
                        </select>
                    </div>

                    <textarea
                        rows="18"
                        prop:value=code
                        on:input=move |ev| set_code.set(event_target_value(&ev))
                        class="w-full p-4 rounded-lg bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 font-mono text-sm focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none resize-y transition-all custom-scrollbar flex-1"
                    />

                    // Controls Row
                    <div class="flex flex-wrap items-center justify-between gap-4 pt-3 border-t border-slate-200 dark:border-slate-800">
                        <div class="flex items-center space-x-4">
                            // Live mode checkbox
                            <label class="flex items-center space-x-2.5 cursor-pointer select-none">
                                <input
                                    type="checkbox"
                                    prop:checked=is_live
                                    on:change=move |ev| set_is_live.set(event_target_checked(&ev))
                                    class="w-4 h-4 rounded-md border-slate-300 dark:border-slate-700 text-indigo-600 focus:ring-indigo-500 dark:bg-slate-800"
                                />
                                <span class="text-xs font-medium text-slate-700 dark:text-slate-300">"Live render"</span>
                            </label>

                            // Manual render button
                            {move || (!is_live.get()).then(|| view! {
                                <button
                                    on:click=handle_manual_render
                                    class="px-3.5 py-1.5 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-lg text-xs shadow-xs transition duration-200"
                                >
                                    "Render Now"
                                </button>
                            })}
                        </div>

                        <div class="flex items-center space-x-2">
                            <button
                                on:click=handle_clear
                                class="px-3 py-1.5 border border-slate-200 dark:border-slate-800 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 font-semibold rounded-lg text-xs transition duration-200"
                            >
                                "Clear"
                            </button>
                        </div>
                    </div>
                </div>

                // Right Panel: Visualizer Viewport
                <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs p-6 flex flex-col min-h-[30rem]">
                    <div class="flex items-center justify-between mb-4">
                        <span class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">"Rendered Diagram"</span>

                        <div class="flex items-center space-x-2">
                            {move || rendering.get().then(|| view! {
                                <span class="inline-flex items-center space-x-2 text-xs text-indigo-500 dark:text-indigo-400 font-semibold">
                                    <svg class="animate-spin h-3.5 w-3.5" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
                                    </svg>
                                    <span>"Rendering..."</span>
                                </span>
                            })}

                            <button
                                on:click=handle_export
                                disabled=move || rendered_svg.get().is_empty()
                                class=move || {
                                    let disabled = rendered_svg.get().is_empty();
                                    format!(
                                        "px-3.5 py-1.5 font-semibold rounded-lg text-xs transition duration-200 {}",
                                        if disabled {
                                            "bg-slate-100 dark:bg-slate-800 text-slate-400 dark:text-slate-600 cursor-not-allowed"
                                        } else {
                                            "bg-indigo-50 dark:bg-slate-800 hover:bg-indigo-100 dark:hover:bg-slate-700 text-indigo-600 dark:text-indigo-400 border border-indigo-200/30"
                                        }
                                    )
                                }
                            >
                                "Export SVG"
                            </button>
                        </div>
                    </div>

                    // Diagram Display / Error Viewport Container
                    <div class="flex-1 flex flex-col justify-center items-center rounded-lg bg-slate-50 dark:bg-slate-950/40 border border-slate-200 dark:border-slate-800/80 p-6 overflow-auto custom-scrollbar">
                        {move || {
                            if let Some(err) = error_msg.get() {
                                view! {
                                    <div class="max-w-md w-full flex items-start space-x-3 p-4 bg-rose-50 dark:bg-rose-950/20 border border-rose-200 dark:border-rose-800/40 rounded-lg text-rose-800 dark:text-rose-400">
                                        <svg class="w-5 h-5 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                        </svg>
                                        <div class="text-xs font-mono space-y-1 overflow-x-auto">
                                            <div class="font-bold">"Syntax Error:"</div>
                                            <div class="whitespace-pre-wrap">{err}</div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                let svg = rendered_svg.get();
                                if svg.is_empty() {
                                    view! {
                                        <div class="text-center text-slate-400 dark:text-slate-500 py-24 space-y-2">
                                            <svg class="w-12 h-12 mx-auto stroke-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                                            </svg>
                                            <p class="text-sm font-medium">"Type Mermaid code to see visual preview..."</p>
                                        </div>
                                    }.into_any()
                                } else {
                                    // Inject SVG raw markup
                                    view! {
                                        <div
                                            class="w-full h-full flex justify-center items-center select-all"
                                            inner_html=svg
                                        />
                                    }.into_any()
                                }
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
