use leptos::prelude::*;
use crate::model::{Course, CourseDuration};
use web_sys::wasm_bindgen::JsCast;

// Helper for fuzzy search
fn levenshtein(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();
    
    let mut d = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 { d[i][0] = i; }
    for j in 0..=len2 { d[0][j] = j; }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            d[i][j] = std::cmp::min(
                std::cmp::min(d[i - 1][j] + 1, d[i][j - 1] + 1),
                d[i - 1][j - 1] + cost
            );
        }
    }
    d[len1][len2]
}

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
        if ev.ctrl_key() || ev.alt_key() || ev.meta_key() { return; }

        if let Some(target) = ev.target() {
            if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                if el.tag_name() == "INPUT" || el.tag_name() == "TEXTAREA" { return; }
            }
        }

        if key.len() == 1 {
            if let Some(input) = input_ref.get() {
                input.focus().unwrap_or_default();
                ev.prevent_default();
                set_query.update(|q| q.push_str(&key));
                set_is_focused.set(true);
            }
        }
    });

    let filtered_courses = move || {
        let q = query.get().to_lowercase();
        if q.is_empty() { return vec![]; }
        
        let all = all_courses.get();
        let selected = selected_courses.get();

        // Tokenize query once
        let q_tokens: Vec<&str> = q.split_whitespace().collect();
        
        // Scoring Structure: (Course, Score)
        // Score: Lower is better.
        
        let mut scored: Vec<(Course, usize)> = all.into_iter()
            .filter(|c| !selected.contains(c))
            .filter_map(|c| {
                let name_lower = c.name.to_lowercase();
                let c_tokens: Vec<&str> = name_lower.split_whitespace().collect();
                
                let mut total_score = 0;
                
                // Every query token must match SOMETHING in the course name
                for q_tok in &q_tokens {
                    let mut best_tok_score = 1000; // High default
                    
                    for c_tok in &c_tokens {
                        // 1. Check strict prefix/substring first for speed & accuracy
                        if c_tok.starts_with(q_tok) {
                            best_tok_score = 0;
                            break; // Found perfect prefix match for this word
                        }
                        
                        // 2. Fuzzy Prefix Match
                        // We check prefixes of c_tok that are similar in length to q_tok
                        // e.g. q_tok="qan" (len 3), c_tok="quantum".
                        // Check prefixes of len 2, 3, 4, 5...
                        
                        let min_len = q_tok.len().saturating_sub(1).max(1);
                        let max_len = (q_tok.len() + 2).min(c_tok.len());
                        
                        if min_len <= max_len {
                            for len in min_len..=max_len {
                                let prefix = &c_tok[0..len];
                                let dist = levenshtein(q_tok, prefix);
                                best_tok_score = best_tok_score.min(dist);
                            }
                        } else if c_tok.len() < min_len {
                            // If course word is shorter than query word (minus 1), just compare directly
                             let dist = levenshtein(q_tok, c_tok);
                             best_tok_score = best_tok_score.min(dist);
                        }
                    }
                    
                    // Threshold logic
                    // Short query words (<=3 chars) need tight match (dist <= 1)
                    // Longer words allow more slack
                    let threshold = if q_tok.len() <= 3 { 1 } else { 2 };
                    
                    if best_tok_score > threshold {
                        return None; // This query token failed to match any word
                    }
                    
                    total_score += best_tok_score;
                }
                
                Some((c, total_score))
            })
            .collect();
            
        // Sort by Score (ascending)
        scored.sort_by_key(|(_, score)| *score);
        
        // Take top 10 and map to (Course, ConflictName)
        scored.into_iter()
            .take(10)
            .map(|(c, _)| {
                let conflict = selected.iter().find(|s| {
                    if s.day != c.day || s.slot != c.slot { return false; }
                    !matches!(
                        (&s.duration, &c.duration),
                        (CourseDuration::H1, CourseDuration::H2) | (CourseDuration::H2, CourseDuration::H1)
                    )
                });
                let conflict_name = conflict.map(|s| s.name.clone());
                (c, conflict_name)
            })
            .collect::<Vec<_>>()
    };

    let on_select = move |course: Course| {
        set_selected.update(|s| {
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
