#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controllers;
use controllers::{
    filterscontroller,
    jokescontroller,
    tagscontroller
};

mod models;
use models::{
    AppDataModel,
    AppDataModelState,
    JokeModel,
    JokesModel,
    JokesModelState
};

fn main() {
    let mut aps: AppDataModelState = AppDataModelState(Default::default());
    let mut jms: JokesModelState = JokesModelState(Default::default());

    jms.0.lock().unwrap().load();

    let rand_index = jms.0.lock().unwrap().get_random_joke();
    aps.0.lock().unwrap().current_joke_index = rand_index;
    aps.0.lock().unwrap().joke_history.push(rand_index);

    tauri::Builder::default()
        .manage(aps)
        .manage(jms)
        .invoke_handler(tauri::generate_handler![
            filterscontroller::get_fav_filter,
            filterscontroller::set_fav_filter,
            filterscontroller::get_tag_filters,
            filterscontroller::set_tag_filters,
            jokescontroller::next_joke,
            jokescontroller::previous_joke,
            jokescontroller::get_current_joke_id,
            jokescontroller::get_current_joke_title,
            jokescontroller::get_current_joke_body,
            jokescontroller::get_joke_is_favourited,
            jokescontroller::set_joke_is_favourited,
            jokescontroller::get_current_joke_is_favourited,
            jokescontroller::set_current_joke_is_favourited,
            jokescontroller::get_filtered_jokes,
            jokescontroller::get_joke,
            jokescontroller::get_jokes,
            jokescontroller::get_is_first_joke,
            tagscontroller::get_used_tags
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
