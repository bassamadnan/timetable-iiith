use leptos::prelude::*;
use crate::model::Course;

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
) -> impl IntoView {
    let courses = move || {
        selected_courses.get()
            .into_iter()
            .filter(|c| c.day == day && c.slot == time_slot)
            .collect::<Vec<_>>()
    };

    let courses_for_conflict = courses.clone();
    let has_conflict = move || courses_for_conflict().len() > 1;
    
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

    view! {
        <div class=move || format!(
            "relative min-h-[100px] p-2 border-2 border-black transition-all duration-200 flex flex-col gap-1 \
            {}",
            if has_conflict_for_class() {
                "bg-[#FF6B6B]" 
            } else if is_cell_hovered_for_class() {
                "bg-black scale-105 z-10 shadow-[8px_8px_0px_0px_rgba(0,0,0,0.5)]" // Special hover style
            } else if is_occupied() {
                "bg-[#A5B4FC] hover:shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:-translate-y-1 hover:-translate-x-1 cursor-pointer"
            } else {
                "bg-white hover:bg-gray-50"
            }
        )>
            <For
                each=courses
                key=|c| c.name.clone()
                children=move |course| {
                    let course_name = course.name.clone();
                    let c_for_click = course.name.clone();
                    
                    let is_this_hovered = move || hovered_course.get().as_deref() == Some(course_name.as_str());
                    let is_pending_deletion = move || pending_deletion.get().as_deref() == Some(c_for_click.as_str());
                    let is_pending_deletion_for_show = is_pending_deletion.clone();
                    let click_name = course.name.clone();

                    view! {
                        <div 
                            class=move || format!(
                                "text-xs font-bold border border-black p-1 shadow-sm leading-tight break-words transition-colors cursor-pointer select-none {}",
                                if is_pending_deletion() {
                                    "bg-[#FF6B6B] text-white animate-pulse"
                                } else if is_this_hovered() { 
                                    "bg-[#FEF08A] text-black" 
                                } else { 
                                    "bg-white text-black hover:bg-red-100" 
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
                            {course.name}
                            <Show when=is_pending_deletion_for_show>
                                <span class="block text-[8px] uppercase pt-1">"Tap to del"</span>
                            </Show>
                        </div>
                    }
                }
            />
            <Show when=is_cell_hovered>
                <div class="absolute -bottom-3 left-1/2 -translate-x-1/2 bg-[#FEF08A] text-black text-[10px] font-black px-2 py-1 border-2 border-black shadow-md uppercase tracking-wide whitespace-nowrap z-20">
                    "HERE!"
                </div>
            </Show>
            <Show when=has_conflict>
                <div class="absolute -top-3 -right-3 bg-black text-white text-[10px] font-black px-2 py-1 border-2 border-white shadow-md uppercase tracking-wide">
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
) -> impl IntoView {
    view! {
        <div class="overflow-x-auto pb-4">
            <div class="min-w-[800px]">
                // Grid Container
                <div class="grid grid-cols-[100px_repeat(6,1fr)] gap-4 bg-white">
                    
                    // Header Row (Timeslots)
                    <div class="h-12 border-b-4 border-r-4 border-black bg-[#F3F4F6]"></div> // Empty corner cell
                    {TIMESLOTS.iter().map(|(code, time)| {
                        view! {
                            <div class="flex flex-col items-center justify-center p-2 bg-black text-white border-b-4 border-black">
                                <span class="font-black text-lg">{*code}</span>
                                <span class="text-[10px] font-bold uppercase text-gray-300">{*time}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}

                    // Days Rows
                    {DAYS.iter().map(|day| {
                        view! {
                            <div class="contents group">
                                // Day Label
                                <div class="flex items-center justify-center p-4 bg-black text-white font-black uppercase text-sm tracking-wider border-r-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,0.2)]">
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
