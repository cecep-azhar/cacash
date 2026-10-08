<script lang="ts">
  import { cashStore } from "$lib/stores/cashStore.svelte";
  import DashboardHome from "$lib/components/DashboardHome.svelte";
</script>

<DashboardHome
  activeMember={cashStore.activeMember}
  accounts={cashStore.accounts}
  transactions={cashStore.transactions}
  envelopes={cashStore.envelopes}
  onQuickAdd={() => (cashStore.showQuickAddModal = true)}
/>
