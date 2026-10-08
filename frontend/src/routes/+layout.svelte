<script lang="ts">
  import "../app.css";
  import type { Snippet } from "svelte";

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="h-screen w-screen flex flex-col bg-[#0A0A0C] text-[#EDEDED] overflow-hidden select-none font-sans">
  {#if children}
    {@render children()}
  {/if}
</div>
