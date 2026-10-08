<script lang="ts">
  import { cashStore } from "$lib/stores/cashStore.svelte";
  import BudgetView from "$lib/components/BudgetView.svelte";
</script>

<BudgetView
  envelopes={cashStore.envelopes}
  onRefresh={() => cashStore.refreshAll()}
/>
