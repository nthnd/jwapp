use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct EntryData {
    pub id: Uuid,
    pub time: NaiveTime,
    pub tags: RcSignal<String>,
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
    pub fn delete_entry(&self, id: Uuid) {
        self.entry_groups
            .get()
            .iter()
            .for_each(|x| x.1.modify().retain(|y| y.id != id));
    }
    pub fn get_entry_data(&self, id: Uuid) -> Option<(String, String)> {
        let mut entry = None;
        for g in self.entry_groups.get().values() {
            for e in g.get().iter() {
                if e.id == id {
                    entry = Some(((*e.value.get()).clone(), (*e.tags.get()).clone()));
                    break;
                }
            }
        }
        entry
    }
    pub fn set_entry_data(&self, id: Uuid, value: String, tags: String) {
        for g in self.entry_groups.get().values() {
            for e in g.get().iter() {
                if e.id == id {
                    e.value.set(value.clone());
                    e.tags.set(tags.clone());
                }
            }
        }
    }
}
