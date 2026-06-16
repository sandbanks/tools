use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use web_sys::window;

use crate::base64::Base64Tool;
use crate::csv_json::CsvJsonConverter;
use crate::dashboard::Dashboard;
use crate::json_fmt::JsonTool;
use crate::json_to_go::JsonToGo;
use crate::json_to_rust::JsonToRust;
use crate::jwt_decoder::JwtDecoder;
use crate::mermaid_editor::MermaidEditor;
use crate::qr_generator::QrGenerator;
use crate::sql_fmt::SqlFormatter;
use crate::timestamp_conv::TimestampConv;
use crate::url_codec::UrlCodec;
use crate::uuid_gen::UuidGen;
use crate::yaml_json::YamlJsonConverter;

// Helper to set the HTML dark/light class
fn set_theme_class(dark: bool) {
    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if let Some(html) = doc.document_element() {
                let class_list = html.class_list();
                if dark {
                    let _ = class_list.add_1("dark");
                } else {
                    let _ = class_list.remove_1("dark");
                }
            }
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Detect system or stored dark preference
    let initial_dark = if let Some(win) = window() {
        if let Some(storage) = win.local_storage().ok().flatten() {
            if let Some(stored) = storage.get_item("theme").ok().flatten() {
                stored == "dark"
            } else {
                win.match_media("(prefers-color-scheme: dark)")
                    .ok()
                    .flatten()
                    .map(|m| m.matches())
                    .unwrap_or(true)
            }
        } else {
            true
        }
    } else {
        true
    };

    let (is_dark, set_is_dark) = signal(initial_dark);
    provide_context(is_dark);
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

    // Apply dark class and persist theme state
    Effect::new(move |_| {
        let dark = is_dark.get();
        set_theme_class(dark);
        if let Some(win) = window() {
            if let Some(storage) = win.local_storage().ok().flatten() {
                let _ = storage.set_item("theme", if dark { "dark" } else { "light" });
            }
        }
    });

    view! {
        <Router>
            <div class="flex flex-col md:flex-row min-h-dvh bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 font-sans transition-colors duration-300">

                // Mobile Top Bar
                <header class="md:hidden flex items-center justify-between px-6 py-4 bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 z-30">
                    <div class="flex items-center space-x-4">
                        <A href="" attr:class="flex items-center space-x-2.5 no-underline">
                            <svg class="w-6 h-6 text-indigo-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                            </svg>
                            <span class="font-bold text-lg tracking-tight bg-gradient-to-r from-indigo-500 to-violet-500 bg-clip-text text-transparent">"DevTools"</span>
                        </A>

                        <div class="flex items-center space-x-1">
                            <a
                                href="https://github.com/sandbanks/tools"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="p-1.5 rounded-md bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
                            >
                                <svg class="w-3.5 h-3.5 text-slate-600 dark:text-slate-300" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                </svg>
                            </a>
                            <button
                                on:click=move |_| set_is_dark.update(|dark| *dark = !*dark)
                                class="p-1.5 rounded-md bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
                            >
                                {move || if is_dark.get() {
                                    view! {
                                        <svg class="w-3.5 h-3.5 text-amber-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364-6.364l-.707.707M6.343 17.657l-.707.707m0-12.728l.707.707m12.728 12.728l.707.707M12 8a4 4 0 100 8 4 4 0 000-8z" />
                                        </svg>
                                    }.into_any()
                                } else {
                                    view! {
                                        <svg class="w-3.5 h-3.5 text-indigo-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                        </svg>
                                    }.into_any()
                                }}
                            </button>
                        </div>
                    </div>

                    <button
                        on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                        class="p-2 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 focus:outline-none"
                    >
                        <span class="sr-only">"Toggle Menu"</span>
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            {move || if mobile_menu_open.get() {
                                view! { <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" /> }
                            } else {
                                view! { <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" /> }
                            }}
                        </svg>
                    </button>
                </header>

                // Mobile Sidebar Drawer Backdrop
                {move || if mobile_menu_open.get() {
                    Some(view! {
                        <div
                            on:click=move |_| set_mobile_menu_open.set(false)
                            class="md:hidden fixed inset-0 bg-slate-900/40 dark:bg-black/60 backdrop-blur-xs z-35 transition-opacity"
                        />
                    })
                } else {
                    None
                }}

                // Navigation Sidebar (Desktop + Mobile overlay Drawer)
                <aside class=move || {
                    let base_classes = "fixed md:sticky top-0 left-0 h-dvh w-72 md:w-64 bg-white dark:bg-slate-900 border-r border-slate-200 dark:border-slate-800 z-40 flex flex-col justify-start py-6 px-4 transition-transform duration-300 md:translate-x-0";
                    if mobile_menu_open.get() {
                        format!("{} translate-x-0", base_classes)
                    } else {
                        format!("{} -translate-x-full", base_classes)
                    }
                }>
                    // Logo & Theme Toggle
                    <div class="hidden md:flex items-center justify-between px-3 mb-8 flex-shrink-0 w-full">
                        <A href="" attr:class="flex items-center space-x-2.5 no-underline">
                            <svg class="w-7 h-7 text-indigo-500" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                            </svg>
                            <span class="font-bold text-xl tracking-tight bg-gradient-to-r from-indigo-500 to-violet-500 bg-clip-text text-transparent">"DevTools"</span>
                        </A>

                        <div class="flex items-center space-x-1">
                            <a
                                href="https://github.com/sandbanks/tools"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="p-1.5 rounded-md bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
                            >
                                <svg class="w-4 h-4 text-slate-600 dark:text-slate-300" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                </svg>
                            </a>
                            <button
                                on:click=move |_| set_is_dark.update(|dark| *dark = !*dark)
                                class="p-1.5 rounded-md bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
                            >
                                {move || if is_dark.get() {
                                    view! {
                                        <svg class="w-4 h-4 text-amber-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364-6.364l-.707.707M6.343 17.657l-.707.707m0-12.728l.707.707m12.728 12.728l.707.707M12 8a4 4 0 100 8 4 4 0 000-8z" />
                                        </svg>
                                    }.into_any()
                                } else {
                                    view! {
                                        <svg class="w-4 h-4 text-indigo-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                        </svg>
                                    }.into_any()
                                }}
                            </button>
                        </div>
                    </div>

                    // Navigation Scroll Container
                    <div class="flex-1 overflow-y-auto custom-scrollbar pr-1 -mr-2 my-2">
                        <nav class="space-y-4">
                            <div>
                                <A
                                    href=""
                                    on:click=move |_| set_mobile_menu_open.set(false)
                                    attr:class="nav-link flex items-center space-x-3 px-3 py-2 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                >
                                    <svg class="w-4.5 h-4.5 opacity-80" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                                    </svg>
                                    <span class="font-semibold text-sm">"Dashboard"</span>
                                </A>
                            </div>

                            // Category: Encoders & Decoders
                            <div class="space-y-1">
                                <div class="px-3 text-[10px] font-bold uppercase tracking-wider text-slate-400 dark:text-slate-500 flex items-center justify-between">
                                    <span>"Encoders & Decoders"</span>
                                    <span class="w-1 h-1 rounded-full bg-indigo-500/60"></span>
                                </div>
                                <div class="space-y-0.5">
                                    <A
                                        href="base64"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
                                        </svg>
                                        <span class="font-medium text-xs">"Base64 Codec"</span>
                                    </A>
                                    <A
                                        href="url"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
                                        </svg>
                                        <span class="font-medium text-xs">"URL Codec"</span>
                                    </A>
                                    <A
                                        href="jwt"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                        </svg>
                                        <span class="font-medium text-xs">"JWT Decoder"</span>
                                    </A>
                                </div>
                            </div>

                            // Category: Formatters & Beautifiers
                            <div class="space-y-1">
                                <div class="px-3 text-[10px] font-bold uppercase tracking-wider text-slate-400 dark:text-slate-500 flex items-center justify-between">
                                    <span>"Formatters & Beautifiers"</span>
                                    <span class="w-1 h-1 rounded-full bg-violet-500/60"></span>
                                </div>
                                <div class="space-y-0.5">
                                    <A
                                        href="json"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                                        </svg>
                                        <span class="font-medium text-xs">"JSON Formatter"</span>
                                    </A>
                                    <A
                                        href="sql-format"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 0v3.75C20.25 16.153 16.556 18 12 18s-8.25-1.847-8.25-4.125v-3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125" />
                                        </svg>
                                        <span class="font-medium text-xs">"SQL Formatter"</span>
                                    </A>
                                </div>
                            </div>

                            // Category: Converters
                            <div class="space-y-1">
                                <div class="px-3 text-[10px] font-bold uppercase tracking-wider text-slate-400 dark:text-slate-500 flex items-center justify-between">
                                    <span>"Converters"</span>
                                    <span class="w-1 h-1 rounded-full bg-blue-500/60"></span>
                                </div>
                                <div class="space-y-0.5">
                                    <A
                                        href="json-to-go"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
                                        </svg>
                                        <span class="font-medium text-xs">"JSON to Go"</span>
                                    </A>
                                    <A
                                        href="json-to-rust"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
                                        </svg>
                                        <span class="font-medium text-xs">"JSON to Rust"</span>
                                    </A>
                                    <A
                                        href="yaml-json"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                        </svg>
                                        <span class="font-medium text-xs">"YAML ↔ JSON"</span>
                                    </A>
                                    <A
                                        href="csv-json"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
                                        </svg>
                                        <span class="font-medium text-xs">"CSV ↔ JSON"</span>
                                    </A>
                                    <A
                                        href="timestamp"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                        <span class="font-medium text-xs">"Timestamp Converter"</span>
                                    </A>
                                </div>
                            </div>

                            // Category: Generators
                            <div class="space-y-1">
                                <div class="px-3 text-[10px] font-bold uppercase tracking-wider text-slate-400 dark:text-slate-500 flex items-center justify-between">
                                    <span>"Generators"</span>
                                    <span class="w-1 h-1 rounded-full bg-emerald-500/60"></span>
                                </div>
                                <div class="space-y-0.5">
                                    <A
                                        href="uuid"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                                        </svg>
                                        <span class="font-medium text-xs">"UUID Generator"</span>
                                    </A>
                                    <A
                                        href="mermaid"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M7 12l3-3 3 3M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" />
                                        </svg>
                                        <span class="font-medium text-xs">"Mermaid Diagrams"</span>
                                    </A>

                                    <A
                                        href="qr-generator"
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        attr:class="nav-link flex items-center space-x-3 px-3 py-1.5 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-all duration-200"
                                    >
                                        <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 013.75 9.375v-4.5zM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 01-1.125-1.125v-4.5zM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0113.5 9.375v-4.5zM15 21a3 3 0 100-6 3 3 0 000 6zM21 15a3 3 0 11-6 0 3 3 0 016 0zM21 21a3 3 0 11-6 0 3 3 0 016 0z" />
                                        </svg>
                                        <span class="font-medium text-xs">"QR Generator"</span>
                                    </A>
                                </div>
                            </div>
                        </nav>
                    </div>

                </aside>

                // Main Content View Area
                <main class="flex-1 overflow-y-auto custom-scrollbar min-h-[calc(100vh-4rem)] md:h-dvh p-6 lg:p-10">
                    <div class="max-w-6xl mx-auto h-full flex flex-col">
                        <Routes fallback=|| view! {
                            <div class="flex flex-col items-center justify-center h-96 space-y-4">
                                <svg class="w-16 h-16 text-red-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                </svg>
                                <h3 class="text-xl font-bold">"Route Not Found"</h3>
                                <p class="text-slate-500 dark:text-slate-400">"The requested tool could not be located."</p>
                                <A href="" attr:class="px-4 py-2 bg-indigo-500 text-white rounded-lg hover:bg-indigo-600 transition duration-200">"Go to Dashboard"</A>
                            </div>
                        }>
                            <Route path=path!("") view=Dashboard />
                            <Route path=path!("base64") view=Base64Tool />
                            <Route path=path!("json") view=JsonTool />
                            <Route path=path!("sql-format") view=SqlFormatter />
                            <Route path=path!("url") view=UrlCodec />
                            <Route path=path!("uuid") view=UuidGen />
                            <Route path=path!("jwt") view=JwtDecoder />
                            <Route path=path!("mermaid") view=MermaidEditor />
                            <Route path=path!("timestamp") view=TimestampConv />
                            <Route path=path!("json-to-go") view=JsonToGo />
                            <Route path=path!("json-to-rust") view=JsonToRust />
                            <Route path=path!("yaml-json") view=YamlJsonConverter />
                            <Route path=path!("csv-json") view=CsvJsonConverter />
                            <Route path=path!("qr-generator") view=QrGenerator />
                        </Routes>
                    </div>
                </main>

            </div>
        </Router>
    }
}
