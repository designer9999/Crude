<!--
  Log output panel — collapsible, shows raw croc output.
  Collapsed by default to keep the UI clean.
-->
<script lang="ts">
  import Card from "$lib/ui/Card.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import { getAppState } from "$lib/state/app-state.svelte";

  const app = getAppState();

  let open = $state(false);
  let logEl: HTMLDivElement | undefined = $state();

  $effect(() => {
    if (app.logs.length && logEl) {
      logEl.scrollTop = logEl.scrollHeight;
    }
  });

  // Auto-open when a new error appears
  $effect(() => {
    const last = app.logs.at(-1);
    if (last?.level === "error") open = true;
  });

  async function copyLog() {
    const text = app.logs.map(e => `${e.time} [${e.level}] ${e.text}`).join("\n");
    await navigator.clipboard.writeText(text);
  }
</script>

<Card variant="outlined">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="flex items-center gap-2 text-on-surface text-sm font-medium cursor-pointer"
    onclick={() => open = !open}
  >
    <span class="text-primary"><Icon name="terminal" size={20} /></span>
    Log
    {#if app.logs.length > 0}
      <span class="text-xs font-normal text-on-surface-variant">({app.logs.length})</span>
    {/if}
    <span class="flex-1"></span>
    {#if open && app.logs.length > 0}
      <button
        class="text-xs text-on-surface-variant cursor-pointer hover:text-on-surface
               bg-transparent border-none p-0"
        onclick={(e) => { e.stopPropagation(); copyLog(); }}
      >Copy</button>
      <button
        class="text-xs text-on-surface-variant cursor-pointer hover:text-on-surface
               bg-transparent border-none p-0"
        onclick={(e) => { e.stopPropagation(); app.clearLogs(); }}
      >Clear</button>
    {/if}
    <span class="text-on-surface-variant section-arrow" class:section-arrow-open={open}>
      <Icon name="expand_more" size={20} />
    </span>
  </div>

  {#if open}
    <div
      bind:this={logEl}
      class="bg-surface-container-lowest rounded-md p-3 mt-3
             font-mono text-xs leading-relaxed select-text
             overflow-y-auto max-h-48 min-h-16 section-enter"
    >
      {#each app.logs as entry (entry.time + entry.text)}
        <div
          class="log-line
                 {entry.level === 'error' ? 'text-error'
                   : entry.level === 'warn' ? 'text-tertiary'
                   : entry.level === 'success' ? 'text-primary'
                   : 'text-on-surface-variant'}"
        >
          <span class="text-outline">{entry.time}</span>
          {entry.text}
        </div>
      {/each}

      {#if app.logs.length === 0}
        <div class="text-outline text-center py-4">No log entries yet</div>
      {/if}
    </div>
  {/if}
</Card>

<style>
  .log-line {
    animation: fade-in var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects) both;
  }
  @keyframes fade-in {
    from { opacity: 0; }
    to   { opacity: 1; }
  }
  .section-arrow {
    transition: transform var(--md-spring-fast-spatial-dur) var(--md-spring-fast-spatial);
  }
  .section-arrow-open {
    transform: rotate(180deg);
  }
  .section-enter {
    animation: section-slide var(--md-spring-fast-spatial-dur) var(--md-spring-fast-spatial) both;
  }
  @keyframes section-slide {
    from { opacity: 0; transform: translateY(-8px); }
    to   { opacity: 1; transform: translateY(0); }
  }
</style>
