import type { PageLoad } from "./$types";

import { invoke } from "@tauri-apps/api/tauri"

export const load = (async ({ params }) => {
    return {
        jokes: await invoke("get_jokes"),
        currentJoke: {
            id: await invoke("get_current_joke_id"),
            title: await invoke("get_current_joke_title"),
            body: await invoke("get_current_joke_body"),
            isFavourited: await invoke("get_current_joke_is_favourited")
        },
        isFirstJoke: await invoke("get_is_first_joke")
    };
}) satisfies PageLoad;
