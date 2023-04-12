use serde::{Serialize, Deserialize};

use super::jokesmodel::JokesModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JokeModel {
    id: usize,
    title: String,
    body: String,
    tags: Vec<String>,
    is_favourited: bool
}

impl JokeModel {
    pub fn new(i: usize, t: String, b: String, t_v: Vec<String>, i_f: bool) -> Self {
        Self {
            id: i,
            title: t,
            body: b,
            tags: t_v,
            is_favourited: i_f
        }
    }

    pub fn save(&self) {
        // let j = serde_json::to_string();
        // println!("{:#?}", j);
        todo!();
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn get_is_favourited(&self) -> bool {
        self.is_favourited
    }

    pub fn set_is_favourited(&mut self, val: bool) {
        *&mut self.is_favourited = val;
    }
}
