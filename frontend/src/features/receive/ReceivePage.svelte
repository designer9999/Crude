<!--
  Receive tab — enter code + save folder + options + receive action
  Save folder persists across app restarts via localStorage.
-->
<script lang="ts">
  import Card from "$lib/ui/Card.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import Switch from "$lib/ui/Switch.svelte";
  import TextField from "$lib/ui/TextField.svelte";
  import { getAppState } from "$lib/state/app-state.svelte";
  import { receiveFile, pickSaveFolder } from "$lib/api/bridge";
  import LogPanel from "../LogPanel.svelte";

  interface Props {
    onsnackbar?: (msg: string) => void;
  }

  let { onsnackbar }: Props = $props();

  const app = getAppState();
  let code = $state("");
  let optionsOpen = $state(false);

  async function handleReceive() {
    if (!code.trim()) {
      onsnackbar?.("Enter the transfer code first");
      return;
    }
    if (app.transferActive) return;
    await receiveFile(code.trim(), app.receiveOptions);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") handleReceive();
  }

  async function handlePickFolder() {
    const folder = await pickSaveFolder();
    if (folder) {
      app.updateReceiveOption("outFolder", folder);
    }
  }

  function clearFolder() {
    app.updateReceiveOption("outFolder", undefined);
  }
</script>

<div class="flex flex-col gap-4">
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-4">
      <span class="text-primary"><Icon name="vpn_key" size={20} /></span>
      Enter transfer code
    </div>

    <div style="--tf-bg: var(--md-sys-color-surface-container-highest);">
      <TextField
        label="Transfer code"
        placeholder="e.g. 1234-banana-apple-cherry"
        mono
        bind:value={code}
        onkeydown={handleKeydown}
      />
    </div>

    <div class="mt-4">
      <Button full onclick={handleReceive} disabled={app.transferActive}>
        <Icon name="download" size={18} />
        Receive
      </Button>
    </div>
  </Card>

  <!-- Save folder -->
  <Card variant="outlined">
    <div class="flex items-center gap-2 text-sm font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="folder" size={20} /></span>
      Save files to
    </div>

    {#if app.receiveOptions.outFolder}
      <div class="flex items-center gap-2 h-12 px-3 bg-surface-container-lowest border border-outline-variant rounded-xs">
        <span class="text-primary"><Icon name="folder_open" size={18} /></span>
        <span class="flex-1 text-sm text-on-surface font-mono truncate">{app.receiveOptions.outFolder}</span>
        <button
          class="w-8 h-8 inline-flex items-center justify-center rounded-full
                 text-on-surface-variant hover:text-error cursor-pointer
                 bg-transparent border-none"
          style="transition: color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
          onclick={clearFolder}
          title="Reset to default"
        >
          <Icon name="close" size={18} />
        </button>
      </div>
      <p class="text-xs text-on-surface-variant mt-2">This folder is saved — files will always go here until you change it.</p>
      <div class="mt-2">
        <Button variant="outlined" full onclick={handlePickFolder}>
          <Icon name="folder_open" size={16} />
          Change folder
        </Button>
      </div>
    {:else}
      <p class="text-xs text-on-surface-variant mb-3">Default: project folder (where the app was launched from).</p>
      <Button variant="tonal" full onclick={handlePickFolder}>
        <Icon name="folder_open" size={18} />
        Choose save folder
      </Button>
    {/if}
  </Card>

  <!-- Options -->
  <Card variant="outlined">
    <button
      class="w-full flex items-center gap-2 text-on-surface text-sm font-medium
             cursor-pointer border-none bg-transparent p-0 -my-0.5"
      onclick={() => optionsOpen = !optionsOpen}
    >
      <span class="text-primary"><Icon name="tune" size={20} /></span>
      Receive options
      <span class="flex-1"></span>
      <span
        class="text-on-surface-variant arrow"
        class:arrow-open={optionsOpen}
      >
        <Icon name="expand_more" size={20} />
      </span>
    </button>

    {#if optionsOpen}
      <div class="flex flex-col gap-1 mt-4 opts-enter">

        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-on-surface">Overwrite existing files</div>
            <div class="text-xs text-on-surface-variant">Replace without prompting if they already exist</div>
          </div>
          <Switch
            checked={app.receiveOptions.overwrite ?? false}
            onchange={(v) => app.updateReceiveOption("overwrite", v || undefined)}
          />
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-on-surface">Disable compression</div>
            <div class="text-xs text-on-surface-variant">Must match sender's setting</div>
          </div>
          <Switch
            checked={app.receiveOptions.noCompress ?? false}
            onchange={(v) => app.updateReceiveOption("noCompress", v || undefined)}
          />
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-on-surface">Local network only</div>
            <div class="text-xs text-on-surface-variant">Receive over LAN only (no relay)</div>
          </div>
          <Switch
            checked={app.receiveOptions.local ?? false}
            onchange={(v) => app.updateReceiveOption("local", v || undefined)}
          />
        </div>

      </div>
    {/if}
  </Card>

  <LogPanel />
</div>

<style>
  .arrow {
    transition: transform var(--md-spring-fast-spatial-dur) var(--md-spring-fast-spatial);
  }
  .arrow-open {
    transform: rotate(180deg);
  }
  .opts-enter {
    animation: opts-slide var(--md-spring-fast-spatial-dur) var(--md-spring-fast-spatial) both;
  }
  @keyframes opts-slide {
    from { opacity: 0; transform: translateY(-8px); }
    to   { opacity: 1; transform: translateY(0); }
  }
</style>
