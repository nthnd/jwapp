use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct EntryData {
    pub id: Uuid,
    pub time: NaiveTime,
    pub value: RcSignal<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct AppState {
    pub first_time: bool,
    pub entry_groups: RcSignal<HashMap<NaiveDate, RcSignal<Vec<EntryData>>>>,
}
impl AppState {
    pub fn insert_with_date(&self, date: NaiveDate, entry: EntryData) {
        self.entry_groups
            .modify()
            .entry(date)
            .or_insert_with(|| create_rc_signal(Vec::new()))
            .modify()
            .insert(0, entry);
    }
    pub fn delete_entry(&mut self, id: Uuid) {
        self.entry_groups
            .modify()
            .retain(|_, v| v.get().iter().any(|x| x.id == id));
    }
}
