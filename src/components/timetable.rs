use leptos::prelude::*;
use web_sys;
use crate::model::{Course, FilterMode, CourseDuration};

const TIMESLOTS: &[(&str, &str)] = &[
    ("T1", "8:30 - 9:55"),
    ("T2", "10:05 - 11:30"),
    ("T3", "11:40 - 1:05"),
    ("T4", "2:00 - 3:25"),
    ("T5", "3:35 - 5:00"),
    ("T6", "5:10 - 6:40"),
];

const DAYS: &[&str] = &["Monday", "Tuesday", "Wednesday"];

#[component]
fn TimetableCell(
    #[prop(into)] day: String,
    #[prop(into)] time_slot: String,
    selected_courses: Signal<Vec<Course>>,
    set_selected_courses: WriteSignal<Vec<Course>>,
    hovered_course: Signal<Option<String>>,
    pending_deletion: Signal<Option<String>>,
    set_pending_deletion: WriteSignal<Option<String>>,
    set_active_filter: WriteSignal<Option<FilterMode>>,
) -> impl IntoView {
    let day_clone = day.clone();
    let slot_clone = time_slot.clone();
    let courses = move || {
        selected_courses.get()
            .into_iter()
            .filter(|c| c.day == day_clone && c.slot == slot_clone)
            .collect::<Vec<_>>()
    };

    let courses_for_conflict = courses.clone();
    let has_conflict = move || {
        let cs = courses_for_conflict();
        for i in 0..cs.len() {
            for j in (i + 1)..cs.len() {
                let compatible = matches!(
                    (&cs[i].duration, &cs[j].duration),
                    (CourseDuration::H1, CourseDuration::H2) | (CourseDuration::H2, CourseDuration::H1)
                );
                if !compatible {
                    return true;
                }
            }
        }
        false
    };
    
    let courses_for_occupied = courses.clone();
    let is_occupied = move || !courses_for_occupied().is_empty();

    let courses_for_hover = courses.clone();
    let is_cell_hovered = move || {
        if let Some(hovered) = hovered_course.get() {
            courses_for_hover().iter().any(|c| c.name == hovered)
        } else {
            false
        }
    };

    let has_conflict_for_class = has_conflict.clone();
    let is_cell_hovered_for_class = is_cell_hovered.clone();



    // Click handler for the container (empty space)
    let d = day.clone();
    let s = time_slot.clone();
    let on_container_click = move |ev: web_sys::MouseEvent| {
        // Only trigger if we aren't clicking strictly on a course (though stop_propagation in the child handles most of this)
        set_active_filter.set(Some(FilterMode::Intersection(d.clone(), s.clone())));
    };

    view! {
        <div 
            class=move || format!(
                "relative min-h-[100px] p-2 border-2 border-black transition-all duration-200 flex flex-col gap-1 \
                {}",
                if has_conflict_for_class() {
                    "bg-[var(--accent-danger)]" 
                } else if is_cell_hovered_for_class() {
                    "bg-[var(--text-main)] scale-105 z-10 shadow-[8px_8px_0px_0px_rgba(0,0,0,0.5)]" // Special hover style
                } else if is_occupied() {
                    "bg-[var(--accent-1)] hover:shadow-[4px_4px_0px_0px_var(--shadow-main)] hover:-translate-y-1 hover:-translate-x-1 cursor-pointer"
                } else {
                    "bg-[var(--bg-card)] hover:brightness-95 cursor-pointer hover:shadow-[inner_0_0_10px_rgba(0,0,0,0.1)]"
                }
            )
                on:click=on_container_click
        >
            <div class="w-full h-full flex flex-col relative">
                <For
                    each=move || {
                        let mut sorted = courses(); 
                        sorted.sort_by_key(|c| match c.duration {
                            CourseDuration::H1 => 1,
                            CourseDuration::Full => 2,
                            CourseDuration::H2 => 3,
                        });
                        sorted
                    }
                    key=|c| c.name.clone()
                    children=move |course| {
                        let course_name = course.name.clone();
                        let c_for_click = course.name.clone();
                        
                        let is_this_hovered = move || hovered_course.get().as_deref() == Some(course_name.as_str());
                        let is_pending_deletion = move || pending_deletion.get().as_deref() == Some(c_for_click.as_str());
                        let is_pending_deletion_for_show = is_pending_deletion.clone();
                        let click_name = course.name.clone();
                        
                        let (duration_badge, height_class) = match course.duration {
                            CourseDuration::Full => (None, "h-full"),
                            CourseDuration::H1 => (Some("H1"), "h-[49%] mb-auto"),
                            CourseDuration::H2 => (Some("H2"), "h-[49%] mt-auto"),
                        };

                        view! {
                            <div 
                                class=move || format!(
                                    "w-full flex-none relative group/item transition-all duration-200 cursor-pointer select-none overflow-hidden flex flex-col justify-center px-1 py-1 {} {}",
                                    height_class,
                                    if is_pending_deletion() {
                                        "bg-red-50 border-2 border-red-500"
                                    } else if is_this_hovered() { 
                                        "bg-yellow-50 border-2 border-[var(--border-main)]" 
                                    } else { 
                                        "bg-[var(--bg-card)] hover:bg-red-50 border-2 border-[var(--border-main)]"
                                    }
                                )
                                on:click=move |ev| {
                                    ev.stop_propagation();
                                    let name = click_name.clone();
                                    let current_pending = pending_deletion.get();

                                    if current_pending.as_deref() == Some(name.as_str()) {
                                        set_selected_courses.update(|v| v.retain(|x| x.name != name));
                                        set_pending_deletion.set(None);
                                    } else {
                                        set_pending_deletion.set(Some(name));
                                    }
                                }
                            >
                                // Badge & Text Container
                                <div class="flex flex-col relative z-10 h-full justify-center">
                                    <Show when=move || duration_badge.is_some()>
                                        <div class="absolute top-0 right-0 bg-[var(--text-main)] text-[var(--bg-card)] text-[8px] font-bold px-1 py-0.5 pointer-events-none">
                                            {duration_badge.unwrap()}
                                        </div>
                                    </Show>
                                    <span class="line-clamp-3 text-[10px] font-bold text-[var(--text-main)] leading-tight pr-1">
                                        {course.name.clone()}
                                    </span>
                                </div>

                                // Deletion Overlay
                                <Show when=is_pending_deletion_for_show>
                                    <div class="absolute inset-0 flex items-center justify-center bg-red-100/90 backdrop-blur-[1px] z-30 transition-opacity">
                                        <span class="text-[10px] font-bold uppercase text-red-600 tracking-wider bg-white px-2 py-1 border-2 border-red-500 shadow-sm">
                                            "Delete?"
                                        </span>
                                    </div>
                                </Show>
                            </div>
                        }
                    }
                />
            </div>
            <Show when=is_cell_hovered>
                <div class="absolute -bottom-3 left-1/2 -translate-x-1/2 bg-[var(--accent-2)] text-black text-[10px] font-black px-2 py-1 border-2 border-[var(--border-main)] shadow-md uppercase tracking-wide whitespace-nowrap z-20">
                    "HERE!"
                </div>
            </Show>
            <Show when=has_conflict>
                <div class="absolute -top-3 -right-3 bg-[var(--text-main)] text-[var(--bg-card)] text-[10px] font-black px-2 py-1 border-2 border-[var(--bg-card)] shadow-md uppercase tracking-wide">
                    "CONFLICT"
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn Timetable(
    #[prop(into)] selected_courses: Signal<Vec<Course>>,
    #[prop(into)] set_selected_courses: WriteSignal<Vec<Course>>,
    #[prop(into)] hovered_course: Signal<Option<String>>,
    #[prop(into)] pending_deletion: Signal<Option<String>>,
    #[prop(into)] set_pending_deletion: WriteSignal<Option<String>>,
    set_active_filter: WriteSignal<Option<FilterMode>>,
) -> impl IntoView {
    view! {
        <div class="overflow-x-auto pb-4">
            <div class="min-w-[800px]">
                // Grid Container
                <div class="grid grid-cols-[100px_repeat(6,1fr)] gap-4 bg-[var(--bg-card)]">
                    
                    // Header Row (Timeslots)
                    // Header Row (Timeslots)
                    <a 
                        href="https://github.com/sambuaneesh/timetable-iiith" 
                        target="_blank"
                        class="h-full flex items-center justify-center bg-[var(--text-main)] text-[var(--bg-card)] hover:opacity-80 transition-colors border-b-4 border-r-4 border-[var(--border-main)]"
                        title="View Source on GitHub"
                    >
                        // GitHub Icon
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                        </svg>
                    </a>
                    {TIMESLOTS.iter().map(|(code, time)| {
                        let c = code.to_string();
                        view! {
                            <div 
                                class="flex flex-col items-center justify-center p-2 bg-[var(--text-main)] text-[var(--bg-card)] border-b-4 border-[var(--border-main)] cursor-pointer hover:opacity-80 transition-colors"
                                on:click=move |_| set_active_filter.set(Some(FilterMode::Slot(c.clone())))
                            >
                                <span class="font-black text-lg">{*code}</span>
                                <span class="text-[10px] font-bold uppercase text-gray-300">{*time}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}

                    // Days Rows
                    {DAYS.iter().map(|day| {
                        let d = day.to_string();
                        view! {
                            <div class="contents group">
                                // Day Label
                                <div 
                                    class="flex items-center justify-center p-4 bg-[var(--text-main)] text-[var(--bg-card)] font-black uppercase text-sm tracking-wider border-r-4 border-[var(--border-main)] shadow-[4px_4px_0px_0px_rgba(0,0,0,0.2)] cursor-pointer hover:opacity-80 transition-colors"
                                    on:click=move |_| set_active_filter.set(Some(FilterMode::Day(d.clone())))
                                >
                                    {*day}
                                </div>

                                // Slots
                                {TIMESLOTS.iter().map(|(slot_code, _)| {
                                    view! {
                                        <TimetableCell 
                                            day=day.to_string() 
                                            time_slot=slot_code.to_string() 
                                            selected_courses=selected_courses
                                            set_selected_courses=set_selected_courses
                                            hovered_course=hovered_course
                                            pending_deletion=pending_deletion
                                            set_pending_deletion=set_pending_deletion
                                            set_active_filter=set_active_filter
                                        />
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
