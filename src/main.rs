mod model;
mod components;
mod config;

use leptos::prelude::*;
use crate::model::{Course, TimetableData, TimetableExt, FilterMode, CourseDuration};
use crate::components::{
    search::Search,
    timetable::Timetable,
    modal::CourseModal,
};
use console_log;
use log::Level;

use crate::config::{DEFAULT_SEMESTER, AVAILABLE_SEMESTERS};
use gloo_timers::callback::Timeout;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(Level::Debug);
    
    // Embed All Data Sets
    // Construct a map of "S26" -> (Raw JSON, Parsed Data lazy loaded? No, eager parse is fine for 4 small files)
    // Actually, to keep it simple and compile-time safe, we'll parse on demand when switching? 
    // Or just parse all 4 at start? They are small.
    
    // We'll use a match expression in a helper function to get the data
    let get_data = |semester: &str| -> TimetableData {
        let json = match semester {
            "S26" => include_str!("../data/data_s26.json"),
            "M25" => include_str!("../data/data_m25.json"),
            "S25" => include_str!("../data/data_s25.json"),
            "M24" => include_str!("../data/data_m24.json"),
            _ => include_str!("../data/data_s26.json"), // Fallback
        };
        serde_json::from_str(json).unwrap()
    };

    mount_to_body(move || {
        // App State
        // 1. Parse URL State
        let window = web_sys::window().unwrap();
        let location = window.location();
        let search = location.search().unwrap_or_default();
        let params = web_sys::UrlSearchParams::new_with_str(&search)
            .ok()
            .unwrap_or_else(|| web_sys::UrlSearchParams::new().unwrap());
        
        let url_sem = params.get("s");
        let url_courses = params.get("c");

        // Determine Initial Semester
        let initial_sem = url_sem.unwrap_or_else(|| DEFAULT_SEMESTER.to_string());
        
        let (current_semester, set_current_semester) = signal(initial_sem.clone());
        let (show_semester_modal, set_show_semester_modal) = signal(false);
        let (share_status, set_share_status) = signal("SELECTED COURSES".to_string());

        // Derived State for Data
        let current_data = Memo::new(move |_| {
            let sem = current_semester.get();
            get_data(&sem).flatten_courses()
        });
        
        // Determine Initial Selected Courses
        let initial_selected = if let Some(c_str) = url_courses {
            let all = current_data.get_untracked();
            c_str.split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .filter_map(|idx| all.get(idx).cloned())
                .collect()
        } else {
            Vec::new()
        };
        
        // Main Course State
        let (selected_courses, set_selected_courses) = signal(initial_selected);
        let (hovered_course, set_hovered_course) = signal(Option::<String>::None);
        let (pending_deletion, set_pending_deletion) = signal(Option::<String>::None);
        let (active_filter, set_active_filter) = signal(Option::<FilterMode>::None);

        // Effect to clear selection when semester changes (only if not initial load?)
        // We need to distinguish between user switch and initial load.
        // Actually, initial load sets the signal. The effect tracks changes.
        // But effect runs once immediately. We should check if value changed?
        // standard Leptos Effect runs immediately.
        // We can use `create_effect` with a tracker that ignores first run?
        // Or just store "is_initialized" ref.
        let is_init = StoredValue::new(true);
        
        Effect::new(move |_| {
            let sem = current_semester.get();
            if !is_init.get_value() {
                set_selected_courses.set(Vec::new()); // Clear if user switched manually
                set_pending_deletion.set(None);
            } else {
                is_init.set_value(false);
            }
        });

        // Share Logic
        let on_share = move |_| {
            let sem = current_semester.get_untracked();
            let selected = selected_courses.get_untracked();
            let all = current_data.get_untracked();
            
            // Map selected courses to indices in 'all'
            // We assume 'all' is stable (sorted by name/id internally in flatten_courses?)
            // flatten_courses structure: iterate days, slots, sort? 
            // We should ensure flatten_courses is deterministic. It iterates Vecs. If JSON order is stable, it's fine.
            // But to be safe, we might match by Name? Index is shorter.
            // Let's rely on index for now, assuming static data.
            
            let indices: Vec<String> = selected.iter()
                .filter_map(|c| all.iter().position(|x| x.name == c.name).map(|i| i.to_string()))
                .collect();
            
            let courses_param = indices.join(",");
            
            // Construct URL using current href base (preserves path like /timetable-iiith/)
            let href = window.location().href().unwrap_or_default();
            let base_url = href.split('?').next().unwrap_or_default();
            // Remove trailing slash if present to avoid double slash with query? 
            // Query usually starts with ?, so base ending with / is fine. e.g. /app/?s=...
            // If base is /app, we want /app?s=...
            
            let url = format!("{}?s={}&c={}", base_url, sem, courses_param);
            
            let navigator = window.navigator();
            // web-sys types navigator.clipboard() as Clipboard (not Option)
            let clipboard = navigator.clipboard();
            let _ = clipboard.write_text(&url);
            set_share_status.set("COPIED TO CLIPBOARD!".to_string());
            Timeout::new(2000, move || set_share_status.set("SELECTED".to_string())).forget();
        };

        // Global Conflict Detection
        let conflicts = move || {
            let selected = selected_courses.get();
            let mut conflict_list = Vec::new();
            for i in 0..selected.len() {
                for j in (i + 1)..selected.len() {
                    let c1 = &selected[i];
                    let c2 = &selected[j];
                    if c1.day == c2.day && c1.slot == c2.slot {
                        let is_compatible = matches!(
                            (&c1.duration, &c2.duration),
                            (CourseDuration::H1, CourseDuration::H2) | (CourseDuration::H2, CourseDuration::H1)
                        );
                        
                        if !is_compatible {
                            conflict_list.push((c1.clone(), c2.clone()));
                        }
                    }
                }
            }
            conflict_list
        };

        // Easter Egg
        let (show_easter_egg, set_show_easter_egg) = signal(false);
        // Theme Easter Egg
        let (theme, set_theme) = signal("default".to_string());

        view! {
            <div 
                class="min-h-screen bg-[var(--bg-main)] p-4 md:p-8 font-mono relative text-[var(--text-main)] selection:bg-[var(--text-main)] selection:text-[var(--bg-main)]"
                data-theme=move || theme.get()
                on:click=move |ev| {
                    set_pending_deletion.set(None);
                    // Strict check: Only trigger if clicking directly on the background div
                    if let Some(target) = ev.target() {
                        if let Some(current_target) = ev.current_target() {
                            if target == current_target {
                                set_show_easter_egg.set(true);
                                // Auto-hide after duration
                                Timeout::new(
                                    crate::config::EASTER_EGG_DURATION_MS.try_into().unwrap_or(3000), 
                                    move || set_show_easter_egg.set(false)
                                ).forget();
                            }
                        }
                    }
                }
            >
                <Show when=move || show_easter_egg.get()>
                    <div class="fixed bottom-4 left-4 z-50 animate-in fade-in slide-in-from-bottom-4 duration-300 pointer-events-none">
                        <div class="bg-[var(--accent-2)] border-4 border-[var(--border-main)] p-4 shadow-[4px_4px_0px_0px_var(--shadow-main)] rotate-[-2deg]">
                            <p class="font-black uppercase text-lg text-black">
                                "you are a smart one, aren't you!"
                            </p>
                        </div>
                    </div>
                </Show>
                
                // Semester Selection Modal
                <CourseModal 
                    all_courses=current_data 
                    selected_courses=selected_courses
                    set_selected_courses=set_selected_courses
                    active_filter=active_filter
                    set_active_filter=set_active_filter
                />
                
                // Semester Selection Modal
                <Show when=move || show_semester_modal.get()>
                    <div class="fixed inset-0 z-[100] flex items_center justify-center p-4">
                        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm" on:click=move |_| set_show_semester_modal.set(false)></div>
                        <div class="relative bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] p-8 max-w-md w-full animate-in zoom-in-95 duration-200">
                            <h2 class="text-3xl font-black uppercase mb-6 text-center">"Select Semester"</h2>
                            <div class="grid grid-cols-2 gap-4">
                                {AVAILABLE_SEMESTERS.iter().map(|(label, _)| {
                                    let label_str = label.to_string();
                                    let label_for_active = label_str.clone();
                                    let label_for_click = label_str.clone();
                                    
                                    let is_active = move || current_semester.get() == label_for_active;
                                    view! {
                                        <button
                                            class=move || format!(
                                                "p-4 border-4 border-[var(--border-main)] font-black text-xl uppercase transition-all active:translate-y-1 active:shadow-none {}",
                                                if is_active() {
                                                    "bg-[var(--text-main)] text-[var(--bg-card)] shadow-none translate-y-1"
                                                } else {
                                                    "bg-[var(--bg-card)] hover:bg-[var(--accent-1)] shadow-[4px_4px_0px_0px_var(--shadow-main)] hover:-translate-y-1 hover:shadow-[6px_6px_0px_0px_var(--shadow-main)]"
                                                }
                                            )
                                            on:click=move |_| {
                                                set_current_semester.set(label_for_click.clone());
                                                set_show_semester_modal.set(false);
                                            }
                                        >
                                            {label_str}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    </div>
                </Show>

                <div class="max-w-7xl mx-auto flex flex-col gap-8">
                    // Header
                    <div 
                        class="bg-[var(--accent-1)] border-4 border-[var(--border-main)] p-4 shadow-[8px_8px_0px_0px_var(--shadow-main)] flex items-center justify-center cursor-pointer hover:brightness-110 transition-all group relative"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            set_show_semester_modal.set(true);
                        }
                    >
                        <h1 class="text-4xl md:text-6xl font-black uppercase tracking-tighter flex items-center gap-2">
                            "Timetable" 
                            <span class="text-[var(--bg-card)] text-stroke-black px-2 relative">
                                {"-"} {move || current_semester.get()}
                            </span>
                        </h1>
                        
                        // Tooltip hint
                        <span class="absolute -bottom-3 left-1/2 -translate-x-1/2 text-[10px] text-[var(--text-main)] bg-[var(--bg-card)] border-2 border-[var(--border-main)] px-2 py-0.5 font-bold uppercase whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10 shadow-[2px_2px_0px_0px_var(--shadow-main)]">
                            "CLICK TO CHANGE"
                        </span>
                    </div>

                    // Search Section
                    <div 
                        class="bg-[var(--bg-card)] border-4 border-[var(--border-main)] p-6 shadow-[8px_8px_0px_0px_var(--shadow-main)] cursor-pointer"
                        on:click=move |ev| {
                            // Strict check: Only trigger if clicking directly on the container (padding area)
                            if let Some(target) = ev.target() {
                                if let Some(current_target) = ev.current_target() {
                                    if target == current_target {
                                        set_theme.update(|t| *t = if t == "default" { "retro".to_string() } else { "default".to_string() });
                                    }
                                }
                            }
                        }
                    >
                        <Search 
                            all_courses=current_data 
                            selected_courses=selected_courses
                            set_selected=set_selected_courses 
                        />
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
                        // Main Timetable
                        <div class="lg:col-span-3 bg-[var(--bg-card)] border-4 border-[var(--border-main)] p-6 shadow-[8px_8px_0px_0px_var(--shadow-main)] overflow-hidden">
                            <Timetable 
                                selected_courses=selected_courses
                                set_selected_courses=set_selected_courses
                                hovered_course=hovered_course
                                pending_deletion=pending_deletion
                                set_pending_deletion=set_pending_deletion
                                set_active_filter=set_active_filter
                            />
                        </div>

                        // Sidebar: Selected & Conflicts
                        <div class="flex flex-col gap-8">
                            // Conflicts Panel
                            <Show when=move || !conflicts().is_empty()>
                                <div class="bg-[var(--accent-danger)] border-4 border-[var(--border-main)] p-6 shadow-[8px_8px_0px_0px_var(--shadow-main)] animate-in slide-in-from-right">
                                    <h3 class="text-2xl font-black uppercase mb-4 flex items-center gap-2">
                                        <span class="text-4xl">"!"</span> "CONFLICTS"
                                    </h3>
                                    <div class="space-y-4">
                                        <For
                                            each=conflicts
                                            key=|c| format!("{}{}", c.0.name, c.1.name)
                                            children=move |(c1, c2)| {
                                                view! {
                                                    <div class="bg-[var(--bg-card)] border-2 border-[var(--border-main)] p-3 font-bold text-sm">
                                                        <div class="text-[var(--accent-danger)]">{c1.day} " - " {c1.slot}</div>
                                                        <div class="border-t border-[var(--border-main)] my-1"></div>
                                                        <div>{c1.name}</div>
                                                        <div class="text-center font-black">"VS"</div>
                                                        <div>{c2.name}</div>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            </Show>

                            // Selected Courses Panel
                            <div class="bg-[var(--accent-2)] border-4 border-[var(--border-main)] p-6 shadow-[8px_8px_0px_0px_var(--shadow-main)]">
                                <div 
                                    class="flex items-center justify-between mb-4 cursor-pointer group select-none"
                                    on:click=on_share
                                >
                                    <h3 class="text-2xl font-black uppercase text-black">"Selected"</h3>
                                    <div 
                                        class=move || format!(
                                            "border-2 border-[var(--border-main)] px-3 py-1 text-sm font-bold uppercase transition-all shadow-[4px_4px_0px_0px_var(--shadow-main)] hover:shadow-none hover:translate-x-1 hover:translate-y-1 active:translate-x-1 active:translate-y-1 opacity-0 group-hover:opacity-100 text-black {}",
                                            if share_status.get().contains("COPIED") {
                                                "bg-[var(--accent-3)] opacity-100" // Stay visible if copied
                                            } else {
                                                "bg-[var(--bg-card)]"
                                            }
                                        )
                                    >
                                        {move || if share_status.get().contains("COPIED") {
                                            "COPIED! ✓"
                                        } else {
                                            "GET LINK 🔗"
                                        }}
                                    </div>
                                </div>
                                <div class="flex flex-col gap-3">
                                    <For
                                        each=move || selected_courses.get()
                                        key=|c| c.name.clone()
                                        children=move |course| {
                                            let c = course.clone();
                                            let c_for_click = c.clone();
                                            let c_for_hover = c.clone();
                                            let c_name_check = c.name.clone();
                                            
                                            let is_pending_deletion = move || {
                                                pending_deletion.get().as_deref() == Some(c_name_check.as_str())
                                            };

                                            let is_pending_deletion_for_show = is_pending_deletion.clone();

                                            view! {
                                                <div 
                                                    class=move || format!(
                                                        "border-2 border-[var(--border-main)] p-3 shadow-[4px_4px_0px_0px_var(--shadow-main)] flex justify-between items-center group cursor-pointer transition-all {}",
                                                        if is_pending_deletion() {
                                                            "bg-[var(--accent-danger)] text-white hover:bg-black"
                                                        } else {
                                                            "bg-[var(--bg-card)] hover:translate-x-1 hover:translate-y-1 hover:shadow-none bg-[var(--bg-card)] text-[var(--text-main)]"
                                                        }
                                                    )
                                                    on:mouseenter=move |_| set_hovered_course.set(Some(c_for_hover.name.clone()))
                                                    on:mouseleave=move |_| set_hovered_course.set(None)
                                                    on:click=move |ev| {
                                                        ev.stop_propagation();
                                                        let name = c_for_click.name.clone();
                                                        let current_pending = pending_deletion.get();
                                                        
                                                        if current_pending.as_deref() == Some(name.as_str()) {
                                                            set_selected_courses.update(|v| v.retain(|x| x.name != name));
                                                            set_pending_deletion.set(None);
                                                        } else {
                                                            set_pending_deletion.set(Some(name));
                                                        }
                                                    }
                                                >
                                                    <span class="font-bold text-sm pr-2">{c.name}</span>
                                                    <Show when=is_pending_deletion_for_show>
                                                        <span class="bg-black text-white text-[10px] font-bold px-2 py-0.5 uppercase">"Delete?"</span>
                                                    </Show>
                                                </div>
                                            }
                                        }
                                    />
                                    <Show when=move || selected_courses.get().is_empty()>
                                        <div 
                                            class=move || format!(
                                                "text-center py-4 font-bold {}",
                                                if theme.get() == "retro" {
                                                    "text-[#FF0000] font-black uppercase text-xl animate-pulse tracking-widest"
                                                } else {
                                                    "text-gray-500 italic"
                                                }
                                            )
                                        >
                                            {move || if theme.get() == "retro" {
                                                "WELCOME TO THE DARK SIDE"
                                            } else {
                                                "No courses selected."
                                            }}
                                        </div>
                                    </Show>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    })
}
