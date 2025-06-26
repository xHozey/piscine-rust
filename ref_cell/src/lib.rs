mod messenger;
use messenger::*;
use std::collections::HashMap;
pub struct Worker {
    track_value: messenger::Tracker,
    mapped_messages: HashMap<String, String>,
    all_messages: Vec<String>
}

impl Worker {
    pub fn new() -> Self {
        Self { track_value: (), mapped_messages: (), all_messages: () }
    }
}