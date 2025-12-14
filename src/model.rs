use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub enum FilterMode {
    Day(String),
    Slot(String),
    Intersection(String, String), // Day, Slot
}
use std::collections::HashMap;

pub type DaySchedule = HashMap<String, Vec<String>>;
pub type TimetableData = HashMap<String, DaySchedule>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Course {
    pub name: String,
    pub day: String,
    pub slot: String,
}

pub trait TimetableExt {
    fn flatten_courses(&self) -> Vec<Course>;
}

impl TimetableExt for TimetableData {
    fn flatten_courses(&self) -> Vec<Course> {
        let mut courses = Vec::new();
        for (day, slots) in self {
            for (slot, course_names) in slots {
                for name in course_names {
                    courses.push(Course {
                        name: name.clone(),
                        day: day.clone(),
                        slot: slot.clone(),
                    });
                }
            }
        }
        courses
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data_s26() {
        let json_data = include_str!("../data/data_s26.json");
        let parsed: TimetableData = serde_json::from_str(json_data).expect("Failed to parse data_s26.json");
        
        assert!(parsed.contains_key("Monday"));
        assert!(parsed.contains_key("Tuesday"));
        assert!(parsed.contains_key("Wednesday"));
        
        // Basic validation
        let monday = parsed.get("Monday").unwrap();
        assert!(monday.contains_key("T1"));
    }
}
