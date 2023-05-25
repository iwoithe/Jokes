use serde::{Serialize, Deserialize};

use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppDataModel {
    pub current_joke_index: usize,
    pub joke_history: Vec<usize>,
    pub fav_filter: String,
    pub tag_filters: Vec<String>
}

impl AppDataModel {
    pub fn new() -> Self {
        Self {
            current_joke_index: 0,
            joke_history: vec![0],
            fav_filter: String::from("all"),
            tag_filters: Vec::new()
        }
    }

    pub fn load(&self) {
        todo!();
    }

    pub fn save(&self) {
        todo!();
    }

    pub fn get_tag_filters(&self) -> Vec<String> {
        self.tag_filters.clone()
    }

    pub fn get_is_first_joke(&self) -> bool {
        self.joke_history.len() == 1
    }
}

pub struct AppDataModelState(pub Mutex<AppDataModel>);
