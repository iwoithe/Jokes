use crate::models::jokesmodel::JokesModelState;

use tauri::State;

#[tauri::command]
pub fn get_used_tags(jokes_model_state: State<JokesModelState>) -> Vec<String> {
    jokes_model_state.0.lock().unwrap().get_used_tags()
}
