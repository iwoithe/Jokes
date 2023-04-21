import type { PageLoad } from "./$types";

import { invoke } from "@tauri-apps/api/tauri";

export const load = (async ({ params }) => {
    let favFilter = await invoke("get_fav_filter");
    let tagFilters = await invoke("get_tag_filters");

    let jokes = await invoke("get_filtered_jokes", {
        favFilter: favFilter,
        tagFilters: tagFilters
    });

    return {
        jokes: jokes,
        tags: await invoke("get_used_tags")
    };
}) satisfies PageLoad;
