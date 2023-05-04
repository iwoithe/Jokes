<script lang="ts">
  import type { PageData } from './$types'
  export let data: PageData;

  import Select from 'svelte-select';
  
  import Favourite from "$lib/Favourite.svelte";
  import Tag from '$lib/Tag.svelte';
  import TagGroup from '$lib/TagGroup.svelte';
  import { invalidateAll } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';

  let activeTagId = "all";

  async function handleTagClick(tagId: string) {
    activeTagId = tagId;

    await invoke("set_fav_filter", {
      favFilter: activeTagId
    });

    invalidateAll();
  }

  onMount(async () => {
    if ($page.url.searchParams.get('favFilter') === "favourited") {
      activeTagId = "favourited";
    } else {
      activeTagId = "all";
    };

    await invoke("set_fav_filter", {
      favFilter: activeTagId
    });

    invalidateAll();
  });

  async function updateFilter(e) {
    let tags = [];
    
    if (e.detail !== null) {
      e.detail.forEach(tag => {
        tags.push(tag.value);
      });
    }

    await invoke("set_tag_filters", {
      tagFilters: tags
    });

    invalidateAll();
  }
</script>

<div class="wrapper browse-content">
  <div class="browse-title-row">
    <a class="btn bg-blue text-black text-dec-none text-body browse-home-button" href="/">Home</a>
    <h1 class="text-title text-align-center">Browse</h1>
  </div>

  <TagGroup>
    <Tag id="all" text="All" isActive={activeTagId === "all"} on:click={function() { handleTagClick("all"); } } />
    <Tag id="favourited" text="Favourited" isActive={activeTagId === "favourited"} on:click={function() { handleTagClick("favourited"); } } />
    <Tag id="non-favourited" text="Non-favourited" isActive={activeTagId === "non-favourited"} on:click={function() { handleTagClick("non-favourited"); } } />
  </TagGroup>

  <!-- TODO: Style -->
  <Select class="tag-filter-input" closeListOnChange={false} items={data.tags} multiple={true} placeholder="Add tag filters" on:input={updateFilter} />

  <div class="browse-jokes-view">
    {#each data.jokes as { body, id, is_favourited, tags, title } }
      <div class="browse-joke">
        <Favourite jokeId={ id }></Favourite>
        <div class="browse-joke-content">
            <p class="text-body-bold text-align-left">{@html title}</p>
            {#if body !== ""}
              <p class="text-body text-align-left">{@html body}</p>
            {/if}
          </div>
      </div>
    {/each}
  </div>

</div>
