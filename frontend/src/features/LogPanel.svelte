<!--
  Log output panel — shows transfer log entries.
  Background: surfaceContainerLowest (deepest layer)
-->
<script lang="ts">
  import Card from "$lib/ui/Card.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import { getAppState } from "$lib/state/app-state.svelte";

  const app = getAppState();
  let logEl: HTMLDivElement | undefined = $state();

  $effect(() => {
    if (app.logs.length && logEl) {
      logEl.scrollTop = logEl.scrollHeight;
    }
  });

  async function copyLog() {
    const text = app.logs.map(e => `${e.time} [${e.level}] ${e.text}`).join("\n");
    await navigator.clipboard.writeText(text);
  }
</script>

<Card>
  <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-2">
    <span class="text-primary"><Icon name="terminal" size={20} /></span>
    Log
    <span class="flex-1"></span>
    {#if app.logs.length > 0}
      <Button variant="outlined" onclick={copyLog}>
        <Icon name="content_copy" size={16} />
        Copy
      </Button>
      <Button variant="outlined" onclick={() => app.clearLogs()}>
        Clear
      </Button>
    {/if}
  </div>

  <div
    bind:this={logEl}
    class="bg-surface-container-lowest rounded-md p-3
           font-mono text-xs leading-relaxed select-text
           overflow-y-auto max-h-48 min-h-24"
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
</Card>

<style>
  .log-line {
    animation: fade-in var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects) both;
  }
  @keyframes fade-in {
    from { opacity: 0; }
    to   { opacity: 1; }
  }
</style>
