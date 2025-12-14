use leptos::prelude::*;
use crate::model::{Course, FilterMode, CourseDuration};

#[component]
pub fn CourseModal(
    #[prop(into)] all_courses: Signal<Vec<Course>>,
    #[prop(into)] selected_courses: Signal<Vec<Course>>,
    #[prop(into)] set_selected_courses: WriteSignal<Vec<Course>>,
    #[prop(into)] active_filter: Signal<Option<FilterMode>>,
    #[prop(into)] set_active_filter: WriteSignal<Option<FilterMode>>,
) -> impl IntoView {
    let filtered_courses = move || {
        let filter = match active_filter.get() {
            Some(f) => f,
            None => return vec![],
        };
        
        let all = all_courses.get();
        let selected = selected_courses.get();

        all.into_iter()
            .filter(|c| {
                let matches_filter = match &filter {
                    FilterMode::Day(d) => &c.day == d,
                    FilterMode::Slot(s) => &c.slot == s,
                    FilterMode::Intersection(d, s) => &c.day == d && &c.slot == s,
                };
                let already_selected = selected.contains(c);
                matches_filter && !already_selected
            })
            .collect::<Vec<_>>()
    };

    let title = move || {
        match active_filter.get() {
            Some(FilterMode::Day(d)) => format!("COURSES ON {}", d),
            Some(FilterMode::Slot(s)) => format!("COURSES IN {}", s),
            Some(FilterMode::Intersection(d, s)) => format!("{} - {}", d, s),
            None => String::new(),
        }
    };

    view! {
        <Show when=move || active_filter.get().is_some()>
            <div 
                class="fixed inset-0 z-[100] flex items-center justify-center bg-black/50 backdrop-blur-sm animate-in fade-in"
                on:click=move |_| set_active_filter.set(None)
            >
                <div 
                    class="bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] w-full max-w-2xl max-h-[80vh] flex flex-col m-4"
                    on:click=move |ev| ev.stop_propagation()
                >
                    // Header
                    <div class="bg-[#A5B4FC] p-4 border-b-4 border-black flex justify-between items-center">
                        <h2 class="text-2xl font-black uppercase tracking-tight">{title}</h2>
                        <button 
                            class="bg-black text-white w-8 h-8 flex items-center justify-center font-bold hover:bg-red-500 transition-colors"
                            on:click=move |_| set_active_filter.set(None)
                        >
                            "X"
                        </button>
                    </div>

                    // List
                    <div class="overflow-y-auto p-4 flex-1">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                            <For
                                each=filtered_courses
                                key=|c| c.name.clone()
                                children=move |course| {
                                    let c_clone = course.clone();
                                    
                                    let duration_badge = match course.duration {
                                        CourseDuration::Full => None,
                                        CourseDuration::H1 => Some("H1"),
                                        CourseDuration::H2 => Some("H2"),
                                    };

                                    view! {
                                        <div 
                                            class="bg-white border-2 border-black p-3 hover:bg-[#FEF08A] hover:translate-x-1 hover:translate-y-1 hover:shadow-none shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] transition-all cursor-pointer flex flex-col gap-1 group"
                                            on:click=move |_| {
                                                set_selected_courses.update(|v| v.push(c_clone.clone())); 
                                                set_active_filter.set(None);
                                            }
                                        >
                                            <div class="font-black text-lg group-hover:underline">{course.name}</div>
                                            <div class="flex gap-2 text-xs font-bold items-center">
                                                <span class="bg-black text-white px-2 py-0.5">{course.day}</span>
                                                <span class="bg-black text-white px-2 py-0.5">{course.slot}</span>
                                                <Show when=move || duration_badge.is_some()>
                                                    <span class="bg-black text-white px-2 py-0.5">{duration_badge.unwrap()}</span>
                                                </Show>
                                            </div>
                                        </div>
                                    }
                                }
                            />
                            <Show when=move || filtered_courses().is_empty()>
                                <div class="col-span-full text-center py-8 font-bold text-gray-400 uppercase text-xl">
                                    "No available courses found."
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
