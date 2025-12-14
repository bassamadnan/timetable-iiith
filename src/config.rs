// Animation Configuration
pub const TYPING_SPEED_MS: u32 = 25; // milliseconds per character
pub const PAUSE_MS: u32 = 2000;      // milliseconds to wait before deleting

// Semester Configuration
pub const DEFAULT_SEMESTER: &str = "S26";

// Available Data Sets (Label, Filename key)
// We will use this to determine which file to load via a macro in main.rs
pub const AVAILABLE_SEMESTERS: &[(&str, &str)] = &[
    ("S26", "data_s26"),
    ("M25", "data_m25"),
    ("S25", "data_s25"),
    ("M24", "data_m24"),
];

// Animation Constants
pub const ANIMATION_TYPING_SPEED_MS: u64 = 50;
pub const ANIMATION_PAUSE_MS: u64 = 3000;
pub const EASTER_EGG_DURATION_MS: u64 = 3000;

pub fn get_facts() -> Vec<&'static str> {
    vec![
        "TRY PRESSING ANY KEY!!",
        "DID YOU KNOW? EVERY RECTANGLE IS CLICKABLE!!",
        "TRY TO CLICK ON RANDOM OBJECTS, YOU NEVER KNOW!!",
    ]
}
