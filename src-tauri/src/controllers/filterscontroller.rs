use crate::models::appdatamodel::AppDataModelState;

use tauri::State;

#[tauri::command]
pub fn get_fav_filter(app_data_model_state: State<AppDataModelState>) -> String {
    app_data_model_state.0.lock().unwrap().fav_filter.to_owned()
}

#[tauri::command]
pub fn set_fav_filter(app_data_model_state: State<AppDataModelState>, fav_filter: String) {
    app_data_model_state.0.lock().unwrap().fav_filter = fav_filter;
}

#[tauri::command]
pub fn get_tag_filters(app_data_model_state: State<AppDataModelState>) -> Vec<String> {
    app_data_model_state.0.lock().unwrap().get_tag_filters()
}

#[tauri::command]
pub fn set_tag_filters(app_data_model_state: State<AppDataModelState>, tag_filters: Vec<String>) {
    app_data_model_state.0.lock().unwrap().tag_filters = tag_filters;
}
