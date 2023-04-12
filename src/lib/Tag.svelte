<script lang="ts">
  export let isActive = false;
  export let text = "";

  function toggle() {
    isActive = !isActive;
  }
</script>

<div class="tag" class:tag-active={isActive} on:click={toggle}>
  <p class="text-small text-align-center">{text}</p>
</div>
