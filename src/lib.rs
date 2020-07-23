extern crate dotenv;

use std::env;

pub mod todoist_habit_api;

/// Get day string begin from env
fn get_day_string_begin() -> String {
    env::var("DAY_STRING_BEGIN").expect("DAY_STRING_BEGIN must be set")
}

/// Get day string end from env
fn get_day_string_end() -> String {
    env::var("DAY_STRING_END").expect("DAY_STRING_END must be set")
}
