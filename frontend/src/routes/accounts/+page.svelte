<script lang="ts">
  import { cashStore } from "$lib/stores/cashStore.svelte";
  import AccountsView from "$lib/components/AccountsView.svelte";
</script>

<AccountsView
  accounts={cashStore.accounts}
  activeMember={cashStore.activeMember}
  onRefresh={() => cashStore.refreshAll()}
/>
