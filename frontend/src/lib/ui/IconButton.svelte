<!--
  M3 Icon Button — 4 types: standard, filled, filledTonal, outlined
  Container: 40×40dp, Shape: Full (circle), Icon: 24dp
  Interactive target: 48dp, State layers: hover 8%, focus 10%, press 10%
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    variant?: "standard" | "filled" | "filledTonal" | "outlined";
    disabled?: boolean;
    onclick?: () => void;
    title?: string;
    "aria-label"?: string;
    children: Snippet;
  }

  let {
    variant = "standard",
    disabled = false,
    onclick,
    title,
    "aria-label": ariaLabel,
    children,
  }: Props = $props();
</script>

<button
  class="group relative inline-flex items-center justify-center w-12 h-12 rounded-full
         cursor-pointer select-none
         disabled:cursor-not-allowed disabled:pointer-events-none"
  {disabled}
  {onclick}
  {title}
  aria-label={ariaLabel ?? title}
>
  <div
    class="relative flex items-center justify-center w-10 h-10 rounded-full overflow-hidden
           {variant === 'filled'
             ? 'bg-primary text-on-primary shadow-level0 group-hover:shadow-level1'
             : variant === 'filledTonal'
               ? 'bg-secondary-container text-on-secondary-container shadow-level0 group-hover:shadow-level1'
               : variant === 'outlined'
                 ? 'text-on-surface-variant border border-outline-variant'
                 : 'text-on-surface-variant'}"
    class:icon-btn-disabled-container={disabled && (variant === 'filled' || variant === 'filledTonal')}
    class:icon-btn-disabled-outlined={disabled && variant === 'outlined'}
    style="transition: box-shadow var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
  >
    <div
      class="absolute inset-0 rounded-full opacity-0
             group-hover:opacity-8 group-focus-visible:opacity-10 group-active:opacity-10
             {variant === 'filled'
               ? 'bg-on-primary'
               : variant === 'filledTonal'
                 ? 'bg-on-secondary-container'
                 : 'bg-on-surface-variant'}"
      style="transition: opacity var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
    ></div>

    <span
      class="relative z-10 inline-flex items-center justify-center w-6 h-6 *:w-6 *:h-6"
      class:opacity-38={disabled}
    >
      {@render children()}
    </span>
  </div>
</button>

<style>
  .icon-btn-disabled-container {
    background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 12%, transparent) !important;
    box-shadow: none !important;
  }
  .icon-btn-disabled-outlined {
    border-color: color-mix(in srgb, var(--md-sys-color-on-surface) 12%, transparent) !important;
  }
</style>
