mod appdatamodel;
mod jokemodel;
mod jokesmodel;

pub use appdatamodel::{
    AppDataModel,
    AppDataModelState,
    next_joke,
    previous_joke,
    get_filtered_jokes,
    get_used_tags,
    get_fav_filter,
    set_fav_filter,
    get_tag_filters,
    set_tag_filters,
    get_jokes,
    get_current_joke_id,
    get_current_joke_title,
    get_current_joke_body,
    get_current_joke_is_favourited,
    set_current_joke_is_favourited,
    get_joke_is_favourited,
    set_joke_is_favourited,
    get_joke
};
pub use jokemodel::JokeModel;
pub use jokesmodel::{
    JokesModel,
    JokesModelState
};
