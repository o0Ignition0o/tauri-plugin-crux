<script lang="ts">
  import "bulma/css/bulma.css";
  import { onMount } from "svelte";
  import { update } from "$lib/core";
  import view from "$lib/core";
  import {
    EventVariantIncrement,
    EventVariantDecrement,
    EventVariantStartWatch,
  } from "shared_types/types/shared_types";

  onMount(async () => {
    console.log("mount");
    update(new EventVariantStartWatch());
  });
</script>

<section class="box container has-text-centered m-5">
  <p class="is-size-5">{$view.text}</p>
  <div class="buttons section is-centered">
    <button
      class="button is-primary is-success"
      on:click={() => update(new EventVariantIncrement())}
    >
      Increment
    </button>
    <button
      class="button is-primary is-warning"
      on:click={() => update(new EventVariantDecrement())}
    >
      Decrement
    </button>
  </div>
</section>
