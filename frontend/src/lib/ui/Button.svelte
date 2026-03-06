<!--
  M3 Button — filled, tonal, outlined, error
  Height: 40dp | Shape: Full (pill) | Label Large typography
  Interaction states: hover 8%, focus 10%, press 10% of content color
  Disabled: container 12% alpha, content 38% opacity (independent)
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    variant?: "filled" | "tonal" | "outlined" | "error";
    disabled?: boolean;
    full?: boolean;
    onclick?: () => void;
    children: Snippet;
  }

  let { variant = "filled", disabled = false, full = false, onclick, children }: Props = $props();
</script>

<button
  class="group relative inline-flex items-center justify-center gap-2 h-10 px-4
         rounded-full text-sm font-medium tracking-[0.1px] cursor-pointer select-none overflow-hidden
         disabled:cursor-not-allowed disabled:pointer-events-none
         {variant === 'filled'
           ? 'bg-primary text-on-primary shadow-level0 hover:shadow-level1'
           : variant === 'tonal'
             ? 'bg-secondary-container text-on-secondary-container shadow-level0 hover:shadow-level1'
             : variant === 'error'
               ? 'bg-error text-on-error shadow-level0 hover:shadow-level1'
               : 'bg-transparent text-on-surface-variant border border-outline-variant hover:shadow-level1'}"
  class:w-full={full}
  class:btn-disabled-container={disabled && variant !== 'outlined'}
  style="transition: box-shadow var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
  {disabled}
  {onclick}
>
  <!-- State layer -->
  <div
    class="absolute inset-0 opacity-0
           group-hover:opacity-8 group-focus-visible:opacity-10 group-active:opacity-10
           {variant === 'filled'
             ? 'bg-on-primary'
             : variant === 'tonal'
               ? 'bg-on-secondary-container'
               : variant === 'error'
                 ? 'bg-on-error'
                 : 'bg-on-surface-variant'}"
    style="transition: opacity var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
  ></div>

  <!-- Content — disabled at 38% opacity independently from container -->
  <span class="relative z-10 inline-flex items-center gap-2" class:opacity-38={disabled}>
    {@render children()}
  </span>
</button>

<style>
  .btn-disabled-container {
    background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 12%, transparent) !important;
    color: inherit;
    box-shadow: none !important;
    border-color: transparent !important;
  }
</style>
