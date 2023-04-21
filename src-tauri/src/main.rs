#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
use models::{
    AppDataModel,
    AppDataModelState,
    get_current_joke_id,
    get_current_joke_title,
    get_current_joke_body,
    get_current_joke_is_favourited,
    set_current_joke_is_favourited,
    get_joke_is_favourited,
    set_joke_is_favourited,
    get_filtered_jokes,
    get_fav_filter,
    set_fav_filter,
    get_tag_filters,
    set_tag_filters,
    get_used_tags,
    get_joke,
    get_jokes,
    next_joke,
    previous_joke,
    JokeModel,
    JokesModel,
    JokesModelState
};

fn main() {
    tauri::Builder::default()
        .manage(AppDataModelState(Default::default()))
        .manage({
            let mut jms: JokesModelState = JokesModelState(Default::default());
            jms.0.lock().unwrap().load();
            jms
        })
        .invoke_handler(tauri::generate_handler![
            get_current_joke_id,
            get_current_joke_title,
            get_current_joke_body,
            get_filtered_jokes,
            get_used_tags,
            get_fav_filter,
            set_fav_filter,
            get_tag_filters,
            set_tag_filters,
            get_joke,
            get_jokes,
            get_current_joke_is_favourited,
            set_current_joke_is_favourited,
            get_joke_is_favourited,
            set_joke_is_favourited,
            next_joke,
            previous_joke
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
