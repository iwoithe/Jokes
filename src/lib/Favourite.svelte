<script lang="ts">
  import { invalidate, invalidateAll } from '$app/navigation';
  import { page } from '$app/stores';

  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  export let jokeId;

  let isFavourited;
  // $: console.log(jokeId + "'s changed: " + isFavourited);

  onMount(async () => {
    let joke = await invoke("get_joke", {
      jokeId: jokeId
    });

    // console.log("1: " + isFavourited);
    isFavourited = joke.is_favourited;
    // console.log("2: " + isFavourited);
  });

  async function toggle() {
    // TODO: After every mount, it takes 2 clicks to toggle, then acts as normal
    let jokePromise = await invoke("get_joke", {
      jokeId
    });
    // console.log(isFavourited);
    isFavourited = await jokePromise.is_favourited;
    // console.log(isFavourited);
    await invoke("set_joke_is_favourited", {
      jokeId: jokeId,
      val: !isFavourited
    });
  }
</script>

<div>
  <button on:click={toggle} class="btn-blank">
      <svg class="star" class:star-toggled={isFavourited} width="50" height="50" viewBox="0 0 16 16">
        <path d="M3.612 15.443c-.386.198-.824-.149-.746-.592l.83-4.73L.173 6.765c-.329-.314-.158-.888.283-.95l4.898-.696L7.538.792c.197-.39.73-.39.927 0l2.184 4.327 4.898.696c.441.062.612.636.282.95l-3.522 3.356.83 4.73c.078.443-.36.79-.746.592L8 13.187l-4.389 2.256z"/>
      </svg>
  </button>
</div>
