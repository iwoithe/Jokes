<script lang="ts">
  import type { PageData } from './$types'
  export let data: PageData;

  import Select from 'svelte-select';
  
  import Favourite from "$lib/Favourite.svelte";
  import Tag from '$lib/Tag.svelte';
  import { invalidateAll } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/tauri';

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

  <!-- TODO: All/Favourited/Non-favourited -->
  <div class="fav-filter-container">
    <!-- TODO: Tag group (only allow one tag to be selected, not multiple) -->
    <Tag text="All" isActive={true} />
    <Tag text="Favourited" isActive={false} />
    <Tag text="Non-favourited" isActive={false} />
  </div>

  <!-- TODO: Style -->
  <Select class="tag-filter-input" closeListOnChange={false} items={data.tags} multiple={true} placeholder="Add tag filters" on:input={updateFilter} />

  <div class="browse-jokes-view">
    {#each data.jokes as { body, id, is_favourited, tags, title } }
      <div class="browse-joke">
        <!-- TODO: Add black separator line after each joke -->
        <Favourite jokeId={ id }></Favourite>
        <div class="browse-joke-content">
            <p class="text-body-bold text-align-left">{title}</p>
            <p class="text-body text-align-left">{body}</p>
          </div>
      </div>
    {/each}
  </div>

</div>
