use crate::models::appdatamodel::{AppDataModel, AppDataModelState};
use crate::models::jokemodel::JokeModel;
use crate::models::jokesmodel::{FavFilter, JokesModel, JokesModelState};

use tauri::State;

#[tauri::command]
pub fn next_joke(app_data_model_state: State<AppDataModelState>, jokes_model_state: State<JokesModelState>) {
    let mut app_data_model = app_data_model_state.0.lock().unwrap();
    let jokes_model = jokes_model_state.0.lock().unwrap();
    let rand_index = jokes_model.get_random_joke();
    app_data_model.current_joke_index = rand_index;
    app_data_model.joke_history.push(rand_index);
}

#[tauri::command]
pub fn previous_joke(app_data_model_state: State<AppDataModelState>, jokes_model_state: State<JokesModelState>) {
    let mut app_data_model = app_data_model_state.0.lock().unwrap();
    let mut jokes_model = jokes_model_state.0.lock().unwrap();

    if app_data_model.joke_history.len() > 1 {
        app_data_model.joke_history.pop();
    }

    match app_data_model.joke_history.len() {
        0 => {},
        _ => {
            app_data_model.current_joke_index = app_data_model.joke_history[app_data_model.joke_history.len() - 1];
        }
    };
}

#[tauri::command]
pub fn get_current_joke_id(app_data_model_state: State<AppDataModelState>, jokes_model_state: State<JokesModelState>) -> usize {
    let app_data_model = app_data_model_state.0.lock().unwrap();
    let jokes_model = jokes_model_state.0.lock().unwrap();
    jokes_model.jokes[app_data_model.current_joke_index].get_id()
}

#[tauri::command]
pub fn get_current_joke_title(app_data_model_state: State<AppDataModelState>, jokes_model_state: State<JokesModelState>) -> String {
    let mut app_data_model = app_data_model_state.0.lock().unwrap();
    let mut jokes_model = jokes_model_state.0.lock().unwrap();
    jokes_model.jokes[app_data_model.current_joke_index].get_title().into()
}

#[tauri::command]
pub fn get_current_joke_body(app_data_model_state: State<AppDataModelState>, jokes_model_state: State<JokesModelState>) -> String {
    let mut app_data_model = app_data_model_state.0.lock().unwrap();
    let mut jokes_model = jokes_model_state.0.lock().unwrap();
    jokes_model.jokes[app_data_model.current_joke_index].get_body().into()
}

#[tauri::command]
pub fn get_current_joke_is_favourited(app_data_model_state: State<AppDataModelState>, jokes_model_state: State<JokesModelState>) -> bool {
    let mut app_data_model = app_data_model_state.0.lock().unwrap();
    let mut jokes_model = jokes_model_state.0.lock().unwrap();
    jokes_model.jokes[app_data_model.current_joke_index].get_is_favourited().into()
}

#[tauri::command]
pub fn set_current_joke_is_favourited(app_data_model_state: State<AppDataModelState>, jokes_model_state: State<JokesModelState>, val: bool) {
    let mut app_data_model = app_data_model_state.0.lock().unwrap();
    let mut jokes_model = jokes_model_state.0.lock().unwrap();
    jokes_model.jokes[app_data_model.current_joke_index].set_is_favourited(val);
}

#[tauri::command]
pub fn get_joke_is_favourited(jokes_model_state: State<JokesModelState>, joke_id: usize) -> bool {
    let mut jokes_model = jokes_model_state.0.lock().unwrap();
    jokes_model.jokes[joke_id].get_is_favourited().into()
}

#[tauri::command]
pub fn set_joke_is_favourited(jokes_model_state: State<JokesModelState>, joke_id: usize, val: bool) {
    let mut jokes_model = jokes_model_state.0.lock().unwrap();
    jokes_model.jokes[joke_id].set_is_favourited(val);
}

#[tauri::command]
pub fn get_filtered_jokes(jokes_model_state: State<JokesModelState>, fav_filter: String, tag_filters: Vec<String>) -> Vec<JokeModel> {
    let fav_filter_val: FavFilter;

    match fav_filter.as_str() {
        "all" => {
            fav_filter_val = FavFilter::All;
        },
        "favourited" => {
            fav_filter_val = FavFilter::Favourited;
        },
        "non-favourited" => {
            fav_filter_val = FavFilter::NonFavourited;
        },
        _ => {
            fav_filter_val = FavFilter::All;
        }
    }

    jokes_model_state.0.lock().unwrap().get_filtered_jokes(fav_filter_val, tag_filters)
}

#[tauri::command]
pub fn get_joke(jokes_model_state: State<JokesModelState>, joke_id: usize) -> JokeModel {
    // TODO: For performance reasons, avoid cloning/copying/duplicating data
    let jokes_model = jokes_model_state.0.lock().unwrap();

    // Avoid the error caused by filtering the jokes model
    if joke_id >= jokes_model.jokes.len() {
        return JokeModel::new(
            0,
            "".to_string(),
            "".to_string(),
            vec![],
            false
        );
    }

    jokes_model.jokes[joke_id].clone()
}

#[tauri::command]
pub fn get_jokes(jokes_model_state: State<JokesModelState>) -> Vec<JokeModel> {
    jokes_model_state.0.lock().unwrap().get_jokes()
}

#[tauri::command]
pub fn get_is_first_joke(app_data_model_state: State<AppDataModelState>) -> bool {
    app_data_model_state.0.lock().unwrap().get_is_first_joke()
}
