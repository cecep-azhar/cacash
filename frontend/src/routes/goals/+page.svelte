<script lang="ts">
  import { cashStore } from "$lib/stores/cashStore.svelte";
  import GoalsView from "$lib/components/GoalsView.svelte";
</script>

<GoalsView
  goals={cashStore.goals}
  onRefresh={() => cashStore.refreshAll()}
/>
