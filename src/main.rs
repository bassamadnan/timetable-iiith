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
        let (current_semester, set_current_semester) = signal(DEFAULT_SEMESTER.to_string());
        let (show_semester_modal, set_show_semester_modal) = signal(false);

        // Derived State for Data
        // When current_semester changes, re-fetch and flatten courses
        let current_data = Memo::new(move |_| {
            let sem = current_semester.get();
            get_data(&sem).flatten_courses()
        });
        
        // Main Course State
        // We sync strict signal to the memo, or just use the memo?
        // Components expect Signal<Vec<Course>>. Memo implements Signal implicitly or via .into().
        // BUT set_selected_courses needs to clear on switch.
        
        let (selected_courses, set_selected_courses) = signal(Vec::<Course>::new());
        let (hovered_course, set_hovered_course) = signal(Option::<String>::None);
        let (pending_deletion, set_pending_deletion) = signal(Option::<String>::None);
        let (active_filter, set_active_filter) = signal(Option::<FilterMode>::None);

        // Effect to clear selection when semester changes
        Effect::new(move |_| {
            current_semester.track(); // Track change
            set_selected_courses.set(Vec::new()); // Clear selection
            set_pending_deletion.set(None);
        });

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

        view! {
            <div 
                class="min-h-screen bg-[#E0E7FF] font-mono p-4 md:p-8 text-black selection:bg-black selection:text-[#E0E7FF]"
                on:click=move |_| set_pending_deletion.set(None)
            >
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
                                                "p-4 border-4 border-black font-black text-xl uppercase transition-all active:translate-y-1 active:shadow-none {}",
                                                if is_active() {
                                                    "bg-black text-white shadow-none translate-y-1"
                                                } else {
                                                    "bg-white hover:bg-[#A5B4FC] shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:-translate-y-1 hover:shadow-[6px_6px_0px_0px_rgba(0,0,0,1)]"
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
                        class="bg-[#A5B4FC] border-4 border-black p-4 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] flex items-center justify-center cursor-pointer hover:bg-[#818CF8] transition-colors group relative"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            set_show_semester_modal.set(true);
                        }
                    >
                        <h1 class="text-4xl md:text-6xl font-black uppercase tracking-tighter flex items-center gap-2">
                            "Timetable" 
                            <span class="text-white text-stroke-black px-2 relative">
                                {"-"} {move || current_semester.get()}
                            </span>
                        </h1>
                        
                        // Tooltip hint
                        <span class="absolute -bottom-3 left-1/2 -translate-x-1/2 text-[10px] text-black bg-white border-2 border-black px-2 py-0.5 font-bold uppercase whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10 shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                            "CLICK TO CHANGE"
                        </span>
                    </div>

                    // Search Section
                    <div 
                        class="bg-white border-4 border-black p-6 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]"
                        on:click=move |ev| ev.stop_propagation()
                    >
                        <Search 
                            all_courses=current_data 
                            selected_courses=selected_courses
                            set_selected=set_selected_courses 
                        />
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
                        // Main Timetable
                        <div class="lg:col-span-3 bg-white border-4 border-black p-6 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] overflow-hidden">
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
                                <div class="bg-[#FF6B6B] border-4 border-black p-6 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] animate-in slide-in-from-right">
                                    <h3 class="text-2xl font-black uppercase mb-4 flex items-center gap-2">
                                        <span class="text-4xl">"!"</span> "CONFLICTS"
                                    </h3>
                                    <div class="space-y-4">
                                        <For
                                            each=conflicts
                                            key=|c| format!("{}{}", c.0.name, c.1.name)
                                            children=move |(c1, c2)| {
                                                view! {
                                                    <div class="bg-white border-2 border-black p-3 font-bold text-sm">
                                                        <div class="text-red-600">{c1.day} " - " {c1.slot}</div>
                                                        <div class="border-t border-black my-1"></div>
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
                            <div class="bg-[#FEF08A] border-4 border-black p-6 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]">
                                <h3 class="text-2xl font-black uppercase mb-4">"Selected"</h3>
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
                                                        "border-2 border-black p-3 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] flex justify-between items-center group cursor-pointer transition-all {}",
                                                        if is_pending_deletion() {
                                                            "bg-[#FF6B6B] text-white hover:bg-red-600"
                                                        } else {
                                                            "bg-white hover:translate-x-1 hover:translate-y-1 hover:shadow-none bg-white"
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
                                        <div class="text-gray-500 italic font-bold text-center py-4">
                                            "No courses selected."
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
