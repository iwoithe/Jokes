import type { PageLoad } from "./$types";

import { invoke } from "@tauri-apps/api/tauri";

export const load = (async ({ params }) => {
    let tagFilters = await invoke("get_tag_filters");
    // console.log(tagFilters);

    let jokes = await invoke("get_filtered_jokes", {
        favFilter: "all",
        tagFilters: tagFilters
    });
    // console.log(jokes);

    return {
        jokes: jokes,
        tags: await invoke("get_used_tags")
    };
}) satisfies PageLoad;
