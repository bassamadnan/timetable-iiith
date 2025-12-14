mod model;
mod components;

use leptos::prelude::*;
use crate::model::{Course, TimetableData, TimetableExt, FilterMode};
use crate::components::{
    search::Search,
    timetable::Timetable,
    modal::CourseModal,
};
use console_log;
use log::Level;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(Level::Debug);
    
    // Load and Parse Data
    let data_str = include_str!("../data/data_s26.json");
    let timetable_data: TimetableData = serde_json::from_str(data_str).unwrap();
    let initial_all_courses = timetable_data.flatten_courses();

    mount_to_body(move || {
        // State
        let (all_courses, _) = signal(initial_all_courses.clone());
        let (selected_courses, set_selected_courses) = signal(Vec::<Course>::new());
        let (hovered_course, set_hovered_course) = signal(Option::<String>::None);
        let (pending_deletion, set_pending_deletion) = signal(Option::<String>::None);
        let (active_filter, set_active_filter) = signal(Option::<FilterMode>::None);

        // Global Conflict Detection
        let conflicts = move || {
            let selected = selected_courses.get();
            let mut conflict_list = Vec::new();
            for i in 0..selected.len() {
                for j in (i + 1)..selected.len() {
                    let c1 = &selected[i];
                    let c2 = &selected[j];
                    if c1.day == c2.day && c1.slot == c2.slot {
                        conflict_list.push((c1.clone(), c2.clone()));
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
                    all_courses=all_courses 
                    selected_courses=selected_courses
                    set_selected_courses=set_selected_courses
                    active_filter=active_filter
                    set_active_filter=set_active_filter
                />
                
                <div class="max-w-7xl mx-auto flex flex-col gap-8">
                    // Header
                    // Header
                    <div class="bg-[#A5B4FC] border-4 border-black p-4 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] flex items-center justify-center">
                        <h1 class="text-4xl md:text-6xl font-black uppercase tracking-tighter">
                            "IIITH" <span class="text-white text-stroke-black">"-S26"</span>
                        </h1>
                    </div>

                    // Search Section
                    <div 
                        class="bg-white border-4 border-black p-6 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]"
                        on:click=move |ev| ev.stop_propagation()
                    >
                        <Search 
                            all_courses=all_courses 
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
                                                    <span class="font-bold text-sm truncate pr-2">{c.name}</span>
                                                    <Show when=is_pending_deletion_for_show>
                                                        <span class="bg-black text-white text-[10px] font-bold px-2 py-0.5 uppercase">"Confirm?"</span>
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
