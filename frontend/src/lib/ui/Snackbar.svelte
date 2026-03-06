<!--
  M3 Snackbar — inverseSurface, auto-dismiss 4s
-->
<script lang="ts">
  interface Props {
    message?: string;
    visible?: boolean;
    ondismiss?: () => void;
  }

  let { message = "", visible = $bindable(false), ondismiss }: Props = $props();

  let exiting = $state(false);

  $effect(() => {
    if (visible) {
      const timer = setTimeout(() => { exiting = true; }, 4000);
      return () => clearTimeout(timer);
    }
  });

  function handleAnimationEnd() {
    if (exiting) {
      exiting = false;
      visible = false;
      ondismiss?.();
    }
  }
</script>

{#if visible}
  <div
    class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50
           flex items-center gap-2 px-6 py-3.5 rounded-sm
           bg-inverse-surface text-inverse-on-surface
           shadow-level3 text-sm w-fit max-w-[90%]"
    class:snackbar-enter={!exiting}
    class:snackbar-exit={exiting}
    onanimationend={handleAnimationEnd}
    role="status"
    aria-live="polite"
  >
    {message}
  </div>
{/if}

<style>
  .snackbar-enter {
    animation: snackbar-in var(--md-dur-short4) var(--md-easing-emphasized-decel) both;
  }
  .snackbar-exit {
    animation: snackbar-out var(--md-dur-short3) var(--md-easing-emphasized-accel) forwards;
  }
  @keyframes snackbar-in {
    from { transform: translateX(-50%) translateY(100%) scaleY(0.8); opacity: 0; }
    to   { transform: translateX(-50%) translateY(0) scaleY(1); opacity: 1; }
  }
  @keyframes snackbar-out {
    from { transform: translateX(-50%) translateY(0); opacity: 1; }
    to   { transform: translateX(-50%) translateY(100%); opacity: 0; }
  }
</style>
