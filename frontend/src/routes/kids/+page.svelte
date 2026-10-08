<script lang="ts">
  import { cashStore } from "$lib/stores/cashStore.svelte";
  import KidsView from "$lib/components/KidsView.svelte";
</script>

<KidsView
  members={cashStore.members}
  activeMember={cashStore.activeMember}
/>
