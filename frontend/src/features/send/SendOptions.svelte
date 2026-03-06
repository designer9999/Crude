<!--
  Collapsible send options panel.
  Uses M3 Switch, TextField, Select components.
  All options persist to localStorage.
-->
<script lang="ts">
  import Icon from "$lib/ui/Icon.svelte";
  import Switch from "$lib/ui/Switch.svelte";
  import TextField from "$lib/ui/TextField.svelte";
  import Select from "$lib/ui/Select.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Card from "$lib/ui/Card.svelte";
  import { getAppState } from "$lib/state/app-state.svelte";
  import { pickSaveFolder } from "$lib/api/bridge";

  const app = getAppState();
  let open = $state(false);

  const curveOptions = [
    { value: "",        label: "P-256 (default)" },
    { value: "p384",    label: "P-384" },
    { value: "p521",    label: "P-521 (strongest)" },
    { value: "siec",    label: "SIEC" },
    { value: "ed25519", label: "Ed25519" },
  ];

  const hashOptions = [
    { value: "",        label: "xxhash (default, fastest)" },
    { value: "imohash", label: "imohash (fast for large files)" },
    { value: "md5",     label: "MD5" },
  ];
</script>

<Card variant="outlined">
  <!-- Toggle header -->
  <button
    class="w-full flex items-center gap-2 text-on-surface text-sm font-medium
           cursor-pointer border-none bg-transparent p-0 -my-0.5"
    onclick={() => open = !open}
  >
    <span class="text-primary"><Icon name="tune" size={20} /></span>
    Send options
    <span class="flex-1"></span>
    <span
      class="text-on-surface-variant arrow"
      class:arrow-open={open}
    >
      <Icon name="expand_more" size={20} />
    </span>
  </button>

  <!-- Collapsible content -->
  {#if open}
    <div class="flex flex-col gap-5 mt-4 opts-enter" style="--tf-bg: var(--color-surface);">

      <!-- Quick access folder -->
      <div>
        <p class="text-xs font-medium text-on-surface-variant mb-2">Quick access folder</p>
        {#if app.sendOptions.sourceFolder}
          <div class="flex items-center gap-2 h-10 px-3 bg-surface-container-lowest border border-outline-variant rounded-xs">
            <span class="text-tertiary"><Icon name="folder_open" size={16} /></span>
            <span class="flex-1 text-xs text-on-surface font-mono truncate">{app.sendOptions.sourceFolder}</span>
            <button
              class="w-7 h-7 inline-flex items-center justify-center rounded-full
                     text-on-surface-variant hover:text-error cursor-pointer bg-transparent border-none"
              style="transition: color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
              onclick={() => app.updateSendOption("sourceFolder", undefined)}
            >
              <Icon name="close" size={16} />
            </button>
          </div>
        {:else}
          <Button variant="outlined" full onclick={async () => {
            const f = await pickSaveFolder();
            if (f) app.updateSendOption("sourceFolder", f);
          }}>
            <Icon name="create_new_folder" size={18} />
            Set quick access folder
          </Button>
        {/if}
        <p class="text-xs text-on-surface-variant mt-1">Set a default folder to quickly browse files from. Saved across sessions.</p>
      </div>

      <!-- Custom code -->
      <TextField
        label="Custom code phrase"
        placeholder="e.g. my-secret-code"
        supportingText="6+ characters. Leave empty for random code. Saved for reuse."
        mono
        value={app.sendOptions.code ?? ""}
        oninput={(e) => app.updateSendOption("code", (e.target as HTMLInputElement).value || undefined)}
      />

      <!-- Encryption curve -->
      <Select
        label="Encryption curve"
        options={curveOptions}
        value={app.sendOptions.curve ?? ""}
        onchange={(v) => app.updateSendOption("curve", v || undefined)}
      />

      <!-- Hash algorithm -->
      <Select
        label="Hash algorithm"
        options={hashOptions}
        value={app.sendOptions.hash ?? ""}
        onchange={(v) => app.updateSendOption("hash", v || undefined)}
      />

      <!-- Exclude patterns -->
      <TextField
        label="Exclude patterns"
        placeholder="e.g. node_modules,.git,.venv"
        supportingText="Comma-separated. Skips files/folders containing these strings."
        mono
        value={app.sendOptions.exclude ?? ""}
        oninput={(e) => app.updateSendOption("exclude", (e.target as HTMLInputElement).value || undefined)}
      />

      <!-- Speed limit -->
      <TextField
        label="Upload speed limit"
        placeholder="e.g. 500k or 2M"
        supportingText="Leave empty for unlimited. Use k (KB/s) or M (MB/s)."
        mono
        value={app.sendOptions.throttleUpload ?? ""}
        oninput={(e) => app.updateSendOption("throttleUpload", (e.target as HTMLInputElement).value || undefined)}
      />

      <!-- Toggle switches -->
      <div class="flex flex-col gap-1">
        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-on-surface">Zip folders before sending</div>
            <div class="text-xs text-on-surface-variant">Compresses folder into zip before transfer</div>
          </div>
          <Switch
            checked={app.sendOptions.zip ?? false}
            onchange={(v) => app.updateSendOption("zip", v || undefined)}
          />
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-on-surface">Respect .gitignore</div>
            <div class="text-xs text-on-surface-variant">Skip files listed in .gitignore</div>
          </div>
          <Switch
            checked={app.sendOptions.git ?? false}
            onchange={(v) => app.updateSendOption("git", v || undefined)}
          />
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-on-surface">Disable compression</div>
            <div class="text-xs text-on-surface-variant">Faster for pre-compressed files (zips, videos)</div>
          </div>
          <Switch
            checked={app.sendOptions.noCompress ?? false}
            onchange={(v) => app.updateSendOption("noCompress", v || undefined)}
          />
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-on-surface">Local network only</div>
            <div class="text-xs text-on-surface-variant">Skip relay, transfer over LAN only (faster)</div>
          </div>
          <Switch
            checked={app.sendOptions.local ?? false}
            onchange={(v) => app.updateSendOption("local", v || undefined)}
          />
        </div>
      </div>

    </div>
  {/if}
</Card>

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
