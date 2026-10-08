<script lang="ts">
  interface Props {
    size?: number | string;
    className?: string;
  }

  let { size = 22, className = "" }: Props = $props();
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 512 512"
  class="shrink-0 {className}"
  xmlns="http://www.w3.org/2000/svg"
>
  <defs>
    <linearGradient id="cacashLogoGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#34D399" />
      <stop offset="100%" stop-color="#10B981" />
    </linearGradient>
  </defs>

  <g fill="none">
    <!-- Outer faceted diamond polygon -->
    <polygon
      points="256,76 388,152 436,256 388,360 256,436 124,360 76,256 124,152"
      stroke="url(#cacashLogoGrad)"
      stroke-width="32"
      stroke-linejoin="miter"
    />

    <!-- Inner ascending growth chevron -->
    <path
      d="M 160 286 L 256 190 L 352 286"
      stroke="#34D399"
      stroke-width="36"
      stroke-linecap="square"
      stroke-linejoin="miter"
    />

    <!-- Prosperity Spark apex -->
    <path
      d="M 200 230 L 256 174 L 312 230"
      stroke="#A7F3D0"
      stroke-width="20"
      stroke-linecap="square"
      stroke-linejoin="miter"
    />

    <!-- Bottom foundation anchor -->
    <path
      d="M 200 340 L 256 376 L 312 340"
      stroke="#10B981"
      stroke-width="18"
      stroke-linecap="square"
      stroke-linejoin="miter"
    />
  </g>
</svg>
