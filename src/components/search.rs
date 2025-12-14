use leptos::prelude::*;
use crate::model::{Course, CourseDuration};
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn Search(
    #[prop(into)] all_courses: Signal<Vec<Course>>,
    #[prop(into)] selected_courses: Signal<Vec<Course>>,
    #[prop(into)] set_selected: WriteSignal<Vec<Course>>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (is_focused, set_is_focused) = signal(false);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    // Global Keydown Listener
    let _ = window_event_listener(leptos::ev::keydown, move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        
        // Ignore if modifier keys are pressed or if target is already an input/textarea
        if ev.ctrl_key() || ev.alt_key() || ev.meta_key() {
            return;
        }

        if let Some(target) = ev.target() {
            if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                if el.tag_name() == "INPUT" || el.tag_name() == "TEXTAREA" {
                    return;
                }
            }
        }

        // Check if key is a single printable character (a-z, 0-9)
        if key.len() == 1 {
            if let Some(input) = input_ref.get() {
                input.focus().unwrap_or_default();
                // We don't need to manually append because browser specific behavior might handle it 
                // if we focus immediately, but usually focusing eats the key unless we are careful.
                // A better UX for "type to search" is to focus AND append the key if it wasn't typed into the input itself.
                // However, focusing mid-event often drops the key. 
                // Let's explicitly append it.
                // Actually, if we just focus, the keydown might trigger input? No, the keydown happened on body.
                // The `keypress` or `input` event would follow.
                
                // Let's just focus and append the char to the query signal.
                set_query.update(|q| q.push_str(&key));
                set_is_focused.set(true);
            }
        }
    });

    let filtered_courses = move || {
        let q = query.get().to_lowercase();
        if q.is_empty() {
            return vec![];
        }
        
        let all = all_courses.get();
        let selected = selected_courses.get();
        
        all.into_iter()
            .filter(|c| {
                let name_match = c.name.to_lowercase().contains(&q);
                let already_selected = selected.contains(c);
                name_match && !already_selected
            })
            // Detect conflicts
            .map(|c| {
                let conflict = selected.iter().find(|s| {
                    if s.day != c.day || s.slot != c.slot { return false; }
                    // Check if they are incompatible
                    !matches!(
                        (&s.duration, &c.duration),
                        (CourseDuration::H1, CourseDuration::H2) | (CourseDuration::H2, CourseDuration::H1)
                    )
                });
                let conflict_name = conflict.map(|s| s.name.clone());
                (c, conflict_name)
            })
            .take(10) // Limit results
            .collect::<Vec<_>>()
    };

    let on_select = move |course: Course| {
        set_selected.update(|s| {
            // Remove colliding courses ONLY if they actually conflict
            s.retain(|c| {
                if c.day != course.day || c.slot != course.slot { return true; }
                 matches!(
                    (&c.duration, &course.duration),
                    (CourseDuration::H1, CourseDuration::H2) | (CourseDuration::H2, CourseDuration::H1)
                )
            });
            s.push(course);
        });
        set_query.set(String::new());
        set_is_focused.set(false);
    };

    view! {
        <div class="relative w-full z-50">
            <div class="relative group">
                <input
                    node_ref=input_ref
                    type="text"
                    class="w-full px-6 py-4 text-xl font-bold bg-white border-4 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] focus:shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] focus:translate-x-[-2px] focus:translate-y-[-2px] placeholder-gray-500 outline-none transition-all duration-200 uppercase"
                    placeholder="SEARCH COURSES..."
                    prop:value=query
                    on:input=move |ev| {
                        set_query.set(event_target_value(&ev));
                        set_is_focused.set(true);
                    }
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            ev.prevent_default();
                            let results = filtered_courses();
                            if let Some((course, _)) = results.first() {
                                on_select(course.clone());
                            }
                        }
                    }
                    on:focus=move |_| set_is_focused.set(true)
                    on:blur=move |_| {
                        set_timeout(move || set_is_focused.set(false), std::time::Duration::from_millis(200));
                    }
                />
                <div class="absolute right-6 top-1/2 -translate-y-1/2 text-black pointer-events-none">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="3" stroke="currentColor" class="w-8 h-8">
                        <path stroke-linecap="square" stroke-linejoin="miter" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
                    </svg>
                </div>
            </div>

            <Show when=move || is_focused.get() && !filtered_courses().is_empty()>
                <div class="absolute w-full mt-4 bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] z-50">
                    <ul class="max-h-80 overflow-y-auto">
                        <For
                            each=filtered_courses
                            key=|t| t.0.name.clone()
                            children=move |(course, conflict)| {
                                let c_clone = course.clone();
                                let has_conflict = conflict.is_some();
                                let conflict_name = conflict.clone().unwrap_or_default();
                                let duration_badge = match course.duration {
                                    CourseDuration::Full => None,
                                    CourseDuration::H1 => Some("H1"),
                                    CourseDuration::H2 => Some("H2"),
                                };

                                view! {
                                    <li 
                                        class=if has_conflict {
                                            "px-6 py-4 border-b-2 border-black last:border-b-0 bg-[#FF6B6B] hover:bg-red-400 cursor-pointer flex flex-col group transition-colors"
                                        } else {
                                            "px-6 py-4 border-b-2 border-black last:border-b-0 hover:bg-[#A5B4FC] cursor-pointer transition-colors flex flex-col group"
                                        }
                                        on:mousedown=move |ev| {
                                            ev.prevent_default();
                                            on_select(c_clone.clone());
                                        }
                                    >
                                        <div class="flex justify-between items-center w-full">
                                            <span class="font-black text-lg text-black group-hover:translate-x-2 transition-transform duration-200">
                                                {course.name}
                                            </span>
                                            <div class="flex gap-2">
                                                <Show when=move || has_conflict>
                                                    <span class="text-xs font-bold uppercase bg-black text-white px-2 py-1 flex items-center gap-1">
                                                        "REPLACES: " {conflict_name.clone()}
                                                    </span>
                                                </Show>
                                                <Show when=move || duration_badge.is_some()>
                                                    <span class="text-xs font-bold uppercase bg-black text-white px-2 py-1">
                                                        {duration_badge.unwrap()}
                                                    </span>
                                                </Show>
                                            </div>
                                        </div>
                                        <div class="flex gap-2 mt-1">
                                            <span class="text-xs font-bold bg-black text-white px-2 py-0.5">
                                                {course.day}
                                            </span>
                                            <span class="text-xs font-bold bg-black text-white px-2 py-0.5">
                                                {course.slot}
                                            </span>
                                        </div>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </div>
            </Show>
        </div>
    }
}
