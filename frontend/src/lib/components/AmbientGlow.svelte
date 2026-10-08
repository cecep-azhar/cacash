<script lang="ts">
  interface Props {
    accent?: string;
  }

  let { accent = "#10b981" }: Props = $props();
</script>

<div
  class="pointer-events-none absolute -inset-[2px] rounded-none rounded-tl-2xl md:rounded-none md:rounded-tl-xl z-0 overflow-visible transition-opacity duration-300 opacity-70 will-change-[filter,opacity]"
  style:box-shadow="0 0 32px 6px {accent}22, -2px -2px 48px 8px {accent}18"
  aria-hidden="true"
></div>
