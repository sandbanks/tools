use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone, PartialEq)]
struct Tool {
    name: &'static str,
    desc: &'static str,
    path: &'static str,
    category: &'static str,
    icon_d: &'static str,
    is_active: bool,
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());

    // Master list of tools
    let get_tools = move || {
        vec![
        Tool {
            name: "Base64 Codec",
            desc: "Encode and decode plain text or URL-safe Base64 strings instantly.",
            path: "base64",
            category: "Encoders & Decoders",
            icon_d: "M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4",
            is_active: true,
        },
        Tool {
            name: "JSON Formatter",
            desc: "Format, prettify, and minify your JSON data with custom indentation styles.",
            path: "json",
            category: "Formatters & Beautifiers",
            icon_d: "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4",
            is_active: true,
        },
        Tool {
            name: "URL Codec",
            desc: "Encode and decode URL parameters safely to prevent character conflicts.",
            path: "url",
            category: "Encoders & Decoders",
            icon_d: "M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1",
            is_active: true,
        },
        Tool {
            name: "UUID Generator",
            desc: "Generate cryptographically secure Universally Unique Identifiers (UUIDv4 and UUIDv7) in bulk.",
            path: "uuid",
            category: "Generators",
            icon_d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
            is_active: true,
        },
        Tool {
            name: "JWT Decoder",
            desc: "Decode and inspect JSON Web Token (JWT) headers, payloads, and signatures locally.",
            path: "jwt",
            category: "Encoders & Decoders",
            icon_d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z",
            is_active: true,
        },
        Tool {
            name: "Mermaid Diagrams",
            desc: "Create flowcharts, sequence diagrams, and class diagrams using simple Markdown-like text.",
            path: "mermaid",
            category: "Generators",
            icon_d: "M7 12l3-3 3 3M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z",
            is_active: true,
        },
        Tool {
            name: "JSON to Go Struct",
            desc: "Convert JSON objects into fully typed, nested Go structs instantly.",
            path: "json-to-go",
            category: "Converters",
            icon_d: "M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5",
            is_active: true,
        },
        Tool {
            name: "JSON to Rust Struct",
            desc: "Convert JSON objects into fully typed, nested Rust struct definitions.",
            path: "json-to-rust",
            category: "Converters",
            icon_d: "M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5",
            is_active: true,
        },
        Tool {
            name: "QR Code Generator",
            desc: "Generate customized QR codes with colors, gradients, logos, and custom eyes.",
            path: "qr-generator",
            category: "Generators",
            icon_d: "M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 013.75 9.375v-4.5zM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 01-1.125-1.125v-4.5zM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0113.5 9.375v-4.5zM15 21a3 3 0 100-6 3 3 0 000 6zM21 15a3 3 0 11-6 0 3 3 0 016 0zM21 21a3 3 0 11-6 0 3 3 0 016 0z",
            is_active: true,
        },
        Tool {
            name: "SQL Formatter",
            desc: "Format and pretty-print raw SQL queries into clean, readable structures instantly.",
            path: "sql-format",
            category: "Formatters & Beautifiers",
            icon_d: "M4 6h16M4 12h16M4 18h7",
            is_active: true,
        },
        Tool {
            name: "YAML ↔ JSON",
            desc: "Convert configuration files between YAML syntax and JSON format seamlessly.",
            path: "yaml-json",
            category: "Converters",
            icon_d: "M7.5 21L3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5",
            is_active: true,
        },
        Tool {
            name: "CSV ↔ JSON",
            desc: "Convert tabular data between CSV format and structured JSON arrays.",
            path: "csv-json",
            category: "Converters",
            icon_d: "M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25A2.25 2.25 0 0113.5 8.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z",
            is_active: true,
        },
        Tool {
            name: "Timestamp Converter",
            desc: "Convert Unix epoch timestamps to UTC, Local, and ISO dates and vice versa.",
            path: "timestamp",
            category: "Converters",
            icon_d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z",
            is_active: true,
        },
        ]
    };

    // Filter tools based on query
    let filtered_tools = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        get_tools()
            .into_iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query)
                    || t.desc.to_lowercase().contains(&query)
                    || t.category.to_lowercase().contains(&query)
            })
            .collect::<Vec<Tool>>()
    });

    // Extract unique categories that have matching tools
    let active_categories = Memo::new(move |_| {
        let preferred_order = vec![
            "Encoders & Decoders",
            "Formatters & Beautifiers",
            "Converters",
            "Generators",
        ];
        let mut cats = Vec::new();
        let tools = filtered_tools.get();
        for cat in preferred_order {
            if tools.iter().any(|t| t.category == cat) {
                cats.push(cat);
            }
        }
        // Catch-all for any other categories
        for tool in &tools {
            if !cats.contains(&tool.category) {
                cats.push(tool.category);
            }
        }
        cats
    });

    view! {
        <div class="space-y-8 animate-fade-in">
            // Welcome Jumbotron / Header
            <div class="relative overflow-hidden rounded-2xl bg-linear-to-r from-indigo-600 to-violet-600 p-8 md:p-10 shadow-lg text-white">
                <div class="relative z-10 max-w-2xl space-y-3">
                    <div class="flex items-center space-x-2">
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-white/20 backdrop-blur-md">
                            "🚀 WebAssembly Toolbox"
                        </span>
                        <a
                            href="https://github.com/sandbanks/tools"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="p-1.5 rounded-md bg-white/20 backdrop-blur-md hover:bg-white/30 transition-colors"
                            title="GitHub Repository"
                        >
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                        </a>
                        <a
                            href="https://tangled.org/richard.tngl.sh/tools"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="p-1.5 rounded-md bg-white/20 backdrop-blur-md hover:bg-white/30 transition-colors"
                            title="Tangled Repository"
                        >
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                            </svg>
                        </a>
                    </div>
                    <h1 class="text-3xl md:text-4xl font-extrabold tracking-tight">
                        "DevTools Dashboard"
                    </h1>
                    <p class="text-indigo-100 text-sm md:text-base font-medium leading-relaxed">
                        "A collection of secure, browser-based, client-side tools running in high-performance WebAssembly. No data leaves your machine."
                    </p>
                </div>
                // Decorative backdrop gradients
                <div class="absolute -right-10 -bottom-10 w-64 h-64 bg-white/10 rounded-full blur-3xl pointer-events-none" />
                <div class="absolute -left-10 -top-10 w-48 h-48 bg-indigo-500/30 rounded-full blur-3xl pointer-events-none" />
            </div>

            // Search Bar Widget
            <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                    <svg class="h-5 h-5 text-slate-400" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                    </svg>
                </div>
                <input
                    type="text"
                    placeholder="Search tools by name, description, or category..."
                    prop:value=search_query
                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    class="w-full pl-11 pr-4 py-3.5 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl text-slate-900 dark:text-white shadow-xs focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 outline-none transition-all duration-200"
                />
            </div>

            // Grouped Category Cards
            {move || {
                let categories = active_categories.get();
                let tools_list = filtered_tools.get();

                if categories.is_empty() {
                    view! {
                        <div class="flex flex-col items-center justify-center py-16 text-center space-y-3">
                            <svg class="w-12 h-12 text-slate-400" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                            </svg>
                            <h3 class="text-lg font-bold text-slate-700 dark:text-slate-300">"No tools found matching your query"</h3>
                            <p class="text-sm text-slate-500 dark:text-slate-400">"Try searching with keywords like 'base64', 'json', or 'format'."</p>
                        </div>
                    }.into_any()
                } else {
                    categories.into_iter().map(move |category| {
                        let category_tools: Vec<&Tool> = tools_list.iter().filter(|t| t.category == category).collect();

                        let (cat_icon, cat_badge_class) = match category {
                            "Encoders & Decoders" => (
                                view! {
                                    <svg class="w-4.5 h-4.5 text-indigo-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
                                    </svg>
                                }.into_any(),
                                "bg-indigo-50 dark:bg-indigo-950/40 text-indigo-700 dark:text-indigo-400 border-indigo-200/50 dark:border-indigo-800/40"
                            ),
                            "Formatters & Beautifiers" => (
                                view! {
                                    <svg class="w-4.5 h-4.5 text-violet-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                                    </svg>
                                }.into_any(),
                                "bg-violet-50 dark:bg-violet-950/40 text-violet-700 dark:text-violet-400 border-violet-200/50 dark:border-violet-800/40"
                            ),
                            "Converters" => (
                                view! {
                                    <svg class="w-4.5 h-4.5 text-blue-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                    </svg>
                                }.into_any(),
                                "bg-blue-50 dark:bg-blue-950/40 text-blue-700 dark:text-blue-400 border-blue-200/50 dark:border-blue-800/40"
                            ),
                            _ => (
                                view! {
                                    <svg class="w-4.5 h-4.5 text-emerald-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904L9 21l8.982-11.795m-9 0L14 3L5.018 14.805" />
                                    </svg>
                                }.into_any(),
                                "bg-emerald-50 dark:bg-emerald-950/40 text-emerald-700 dark:text-emerald-400 border-emerald-200/50 dark:border-emerald-800/40"
                            ),
                        };

                        view! {
                            <section class="p-6 rounded-2xl bg-white dark:bg-slate-900 border border-slate-200/60 dark:border-slate-800/80 shadow-xs space-y-5">
                                <div class="flex items-center justify-between border-b border-slate-100 dark:border-slate-800/60 pb-3">
                                    <div class="flex items-center space-x-2.5">
                                        {cat_icon}
                                        <h2 class="text-sm font-bold tracking-wide text-slate-800 dark:text-slate-200 uppercase">
                                            {category}
                                        </h2>
                                    </div>
                                    <span class=format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold border {}", cat_badge_class)>
                                        {format!(
                                            "{} {}",
                                            category_tools.len(),
                                            if category_tools.len() == 1 { "tool" } else { "tools" }
                                        )}
                                    </span>
                                </div>

                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                    {category_tools.into_iter().map(move |tool| {
                                        let is_active = tool.is_active;
                                        let path = tool.path;

                                        let card_content = view! {
                                            <div class=move || {
                                                format!(
                                                    "group relative flex items-start space-x-4 p-4 rounded-xl border transition-all duration-200 bg-slate-50 dark:bg-slate-950/40 border-slate-200/50 dark:border-slate-800/40 shadow-xs h-full {}",
                                                    if is_active {
                                                        "hover:scale-[1.01] hover:border-indigo-500/50 hover:bg-white dark:hover:bg-slate-900 hover:shadow-md hover:shadow-indigo-500/5 cursor-pointer"
                                                    } else {
                                                        "opacity-60 cursor-not-allowed select-none"
                                                    }
                                                )
                                            }>
                                                // Icon Wrapper
                                                <div class="flex-shrink-0 p-2.5 rounded-lg bg-white dark:bg-slate-900 text-indigo-600 dark:text-indigo-400 group-hover:scale-105 border border-slate-200/40 dark:border-slate-800/40 transition-transform duration-200">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d=tool.icon_d />
                                                    </svg>
                                                </div>

                                                // Info
                                                <div class="flex-1 space-y-1 pr-6">
                                                    <h3 class="font-bold text-sm text-slate-850 dark:text-slate-200 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">
                                                        {tool.name}
                                                    </h3>
                                                    <p class="text-xs text-slate-500 dark:text-slate-400 leading-relaxed">
                                                        {tool.desc}
                                                    </p>
                                                </div>

                                                // Badges
                                                <div class="absolute top-4 right-4">
                                                    {if is_active {
                                                        None
                                                    } else {
                                                        Some(view! {
                                                            <span class="inline-flex items-center px-2 py-0.5 rounded text-[9px] font-bold bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 border border-slate-200 dark:border-slate-700/50">
                                                                "Soon"
                                                            </span>
                                                        }.into_any())
                                                    }}
                                                </div>
                                            </div>
                                        };

                                        view! {
                                            {if is_active {
                                                view! {
                                                    <A href=path attr:class="block no-underline h-full">
                                                        {card_content}
                                                    </A>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="block h-full">
                                                        {card_content}
                                                    </div>
                                                }.into_any()
                                            }}
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>
                        }
                    }).collect::<Vec<_>>().into_any()
                }
            }}
        </div>
    }
}
