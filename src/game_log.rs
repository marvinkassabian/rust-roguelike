use std::cmp::{max, min};
use std::fmt::Display;

pub struct GameLog {
    pub entries: Vec<LogEntry>,
    pub display_index: i32,
}

pub struct LogEntry {
    pub message: String,
    pub count: usize,
}

impl GameLog {
    pub fn new_with_first_log<T: Display>(first_log: T) -> GameLog {
        GameLog {
            entries: vec![LogEntry {
                message: first_log.to_string(),
                count: 1,
            }],
            display_index: 0,
        }
    }

    pub fn add<T: Display>(&mut self, log: T) {
        let new_message = log.to_string();

        match self.entries.first_mut() {
            None => self.add_new_entry(new_message),
            Some(last_log) => {
                if last_log.message == new_message {
                    last_log.count += 1;
                } else {
                    self.add_new_entry(new_message);
                }
            }
        }
    }

    fn add_new_entry(&mut self, message: String) {
        self.entries.insert(0, LogEntry {
            message,
            count: 1,
        });
    }

    pub fn move_index(&mut self, delta: i32) {
        self.display_index = min(
            self.entries.len() as i32 - 1,
            max(
                0,
                self.display_index + delta));
    }
}

impl LogEntry {
    pub fn get_formatted_message(&self) -> String {
        if self.count == 1 {
            self.message.to_string()
        } else {
            format!("{} (x{})", self.message, self.count)
        }
    }
}