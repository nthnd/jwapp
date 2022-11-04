use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct EntryData {
    pub time: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct AppState {
    pub first_time: bool,
    pub entry_groups: RcSignal<HashMap<String, RcSignal<Vec<EntryData>>>>,
}
impl AppState {
    pub fn insert_with_date(&self, date: String, entry: EntryData) {
        self.entry_groups
            .modify()
            .entry(date)
            .or_insert_with(|| create_rc_signal(Vec::new()))
            .modify()
            .insert(0, entry);
    }
}
