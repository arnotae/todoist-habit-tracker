use crate::{get_day_string_begin, get_day_string_end};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Event {
    pub event_name: String,
    // user_id: u64,
    pub event_data: EventData,
}

#[derive(Deserialize)]
pub struct EventData {
    // day_order: i32,
    // added_by_uid: u64,
    // assigned_by_uid: u64,
    // labels: Vec<String>,
    // sync_id: Option<u64>,
    // in_history: i8,
    // has_notifications: i8,
    // parent_id: Option<u64>,
    // checked: i8,
    // date_added: String,
    pub id: u64,
    pub content: String,
    // user_id: u64,
    // due: Option<String>,
    // children: Option<Vec<u64>>,
    // priority: u8,
    // child_order: u8,
    // is_deleted: i8,
    // responsible_uid: Option<String>,
    // project_id: u64,
    // collapsed: i8,
}

#[derive(Serialize)]
pub struct EventUpdate {
    pub content: String,
}

impl Event {
    /// Check if day string begin is in content
    pub fn has_day_string(&self) -> bool {
        self.event_data.content.contains(&get_day_string_begin())
    }

    /// Extract number of day and add one
    pub fn add_one_day_to_content(&self) -> String {
        // Return content if there is no
        if !self.has_day_string() {
            return self.event_data.content.to_owned();
        }

        // Get start (Add len of day_string_begin + 1 for space)
        let day_string_begin = get_day_string_begin();
        let start = match self.event_data.content.find(&day_string_begin) {
            Some(i) => i,
            None => {
                println!("Weird behaviour for start '{}' finding '{}'", self.event_data.content, day_string_begin);
                return self.event_data.content.to_owned();
            },
        };
        let start = start + day_string_begin.len() + 1;

        // Get end
        let day_string_end = get_day_string_end();
        let end = match self.event_data.content.find(&day_string_end) {
            Some(i) => i,
            None => {
                println!("Weird behaviour for end '{}' finding '{}'", self.event_data.content, day_string_begin);
                return self.event_data.content.to_owned();
            },
        };

        // Increment day
        let day = match self.event_data.content[start..end]
            .to_owned()
            .parse::<u32>()
        {
            Ok(d) => d,
            Err(_) => {
                println!("Weird behaviour for day '{}' slicing [{}..{}]", self.event_data.content, start, end);
                return self.event_data.content.to_owned();
            },
        };
        let day = day + 1;

        format!("{}{}{}", self.event_data.content[0..start].to_owned(), day, day_string_end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn content_has_string_mono_number() {
        dotenv().ok();
        let event = Event {
            event_name: "item:completed".to_string(),
            event_data: EventData {
                id: 42,
                content: "enjoy rust [day 5]".to_string(),
            },
        };
        assert_eq!(event.add_one_day_to_content(), "enjoy rust [day 6]");
    }

    #[test]
    fn content_has_string_multi_number() {
        dotenv().ok();
        let event = Event {
            event_name: "item:completed".to_string(),
            event_data: EventData {
                id: 42,
                content: "enjoy rust [day 42]".to_string(),
            },
        };
        assert_eq!(event.add_one_day_to_content(), "enjoy rust [day 43]");
    }

    #[test]
    fn content_dont_have_string() {
        dotenv().ok();
        let event = Event {
            event_name: "item:completed".to_string(),
            event_data: EventData {
                id: 42,
                content: "just enjoy rust".to_string(),
            },
        };
        assert_eq!(event.add_one_day_to_content(), "just enjoy rust");
    }
}
