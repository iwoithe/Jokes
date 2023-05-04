<script lang="ts">
  import { invalidateAll } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";

  export let isActive = false;
  export let text = "";
  export let id: String;

  async function toggle() {
    isActive = !isActive;

    await invoke("set_fav_filter", {
      favFilter: id
    });

    invalidateAll();
  }
</script>

<div class="tag" class:tag-active={isActive} role="presentation" on:click>
  <p class="text-small text-align-center">{text}</p>
</div>
