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

  async function setFavFilter(favFilter: string) {
    activeTagId = favFilter;

    await invoke("set_fav_filter", {
      favFilter: activeTagId
    });

    invalidateAll();
  }

  async function handleTagClick(tagId: string) {
    setFavFilter(tagId);
  }

  onMount(async () => {
    if ($page.url.searchParams.get('favFilter') === "favourited") {
      setFavFilter("favourited");
    } else {
      setFavFilter("all");
    };
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
    <a class="btn bg-blue text-black text-dec-none text-body browse-home-button" href="/">
      <svg width="24" height="24" fill="currentColor" viewBox="0 0 16 16">
        <path d="M8.707 1.5a1 1 0 0 0-1.414 0L.646 8.146a.5.5 0 0 0 .708.708L2 8.207V13.5A1.5 1.5 0 0 0 3.5 15h9a1.5 1.5 0 0 0 1.5-1.5V8.207l.646.647a.5.5 0 0 0 .708-.708L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293L8.707 1.5ZM13 7.207V13.5a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5V7.207l5-5 5 5Z"/>
      </svg>
      Home
    </a>
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
        <Favourite jokeId={ id } isFavourited={ is_favourited }></Favourite>
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
