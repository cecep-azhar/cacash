<script lang="ts">
  import { cashStore } from "$lib/stores/cashStore.svelte";
  import InvestmentsView from "$lib/components/InvestmentsView.svelte";
</script>

<InvestmentsView
  investments={cashStore.investments}
  debts={cashStore.debts}
  onRefresh={() => cashStore.refreshAll()}
/>
