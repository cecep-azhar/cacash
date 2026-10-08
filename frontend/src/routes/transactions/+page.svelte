<script lang="ts">
  import { cashStore } from "$lib/stores/cashStore.svelte";
  import TransactionView from "$lib/components/TransactionView.svelte";
</script>

<TransactionView
  transactions={cashStore.transactions}
  onRefresh={() => cashStore.refreshAll()}
  onQuickAdd={() => (cashStore.showQuickAddModal = true)}
/>
