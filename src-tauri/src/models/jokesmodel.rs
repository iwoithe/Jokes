use std::fs;

use super::jokemodel::JokeModel;

use serde::{Serialize, Deserialize};
use serde_json::Result;

use std::sync::Mutex;

use rand::Rng;

pub enum FavFilter {
    All,
    Favourited,
    NonFavourited
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JokesModel {
    pub jokes: Vec<JokeModel>
}

impl JokesModel {
    pub fn new() -> Self {
        Self {
            jokes: Vec::new()
        }
    }

    pub fn load(&mut self) -> Result<()> {
        let file_path = "./share/jokes.json";
        let file_content = fs::read_to_string(file_path).expect("[ERROR] File not found");
        let v: JokesModel = serde_json::from_str(&file_content)?;
        *&mut self.jokes = v.jokes;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        todo!();
    }

    pub fn get_jokes(&self) -> Vec<JokeModel> {
        // TODO: Should this method be renamed to get_all_jokes()?
        // TODO: For performance reasons, avoid cloning/copying/duplicating data
        self.jokes.clone()
    }

    pub fn get_filtered_jokes(&self, fav_filter: FavFilter, tag_filters: Vec<String>) -> Vec<JokeModel> {
        // TODO: For performance reasons, avoid cloning/copying/duplicating data
        self.jokes
            .clone()
            .into_iter()
            .filter(|j| {
                match fav_filter {
                    FavFilter::All => true,
                    FavFilter::Favourited => j.get_is_favourited(),
                    FavFilter::NonFavourited => !j.get_is_favourited()
                }
            })
            .filter(|j| {
                for tag in tag_filters.clone() {
                    if !j.get_tags().contains(&tag.to_string()) {
                        return false;
                    }
                }
                
                true
                
            })
            .collect::<Vec<JokeModel>>()
    }

    pub fn get_used_tags(&self) -> Vec<String> {
        let mut used_tags: Vec<String> = Vec::new();

        // TODO: Check performance
        // TODO: For performance reasons, avoid cloning/copying/duplicating data
        for j in self.jokes.clone().into_iter() {
            for tag in j.clone().get_tags() {
                if !used_tags.contains(&tag.to_string()) {
                    used_tags.push(tag);
                }
            }
        }

        used_tags
    }

    pub fn get(&self, index: usize) -> JokeModel {
        todo!();
    }

    pub fn get_random_joke(&self) -> usize {
        // TODO: This method may need to return a mutable
        let rand_int: usize = rand::thread_rng().gen_range(0..=self.jokes.len() - 1);

        rand_int
    }
}

pub struct JokesModelState(pub Mutex<JokesModel>);
