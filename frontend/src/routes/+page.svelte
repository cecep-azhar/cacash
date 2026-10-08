<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  onMount(() => {
    goto("/dashboard", { replaceState: true });
  });
</script>

<div class="flex-1 flex items-center justify-center text-xs text-[#9CA3AF]">
  <span>Mengalihkan ke Dashboard...</span>
</div>
