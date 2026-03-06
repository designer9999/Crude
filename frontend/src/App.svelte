<!--
  Root Application Component
  Layout: Top bar + Tabs + Content + Status bar
  Theme applied on mount via M3 Expressive (SchemeExpressive)
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { getThemeState } from "$lib/theme/theme-store.svelte";
  import { applyThemeToDOM } from "$lib/theme/apply-theme";
  import { getAppState } from "$lib/state/app-state.svelte";
  import { getStatus, stopTransfer, checkUpdate, downloadUpdate, launchUpdate } from "$lib/api/bridge";
  import Icon from "$lib/ui/Icon.svelte";
  import Button from "$lib/ui/Button.svelte";
  import ProgressBar from "$lib/ui/ProgressBar.svelte";
  import Snackbar from "$lib/ui/Snackbar.svelte";
  import SendPage from "./features/send/SendPage.svelte";
  import ReceivePage from "./features/receive/ReceivePage.svelte";
  import GuidePage from "./features/guide/GuidePage.svelte";

  const theme = getThemeState();
  const app = getAppState();

  let snackbarMsg = $state("");
  let snackbarVisible = $state(false);

  let appVersion = $state("1.0.0");
  let updateAvailable = $state(false);
  let updateUrl = $state("");
  let updateVersion = $state("");
  let updateDownloading = $state(false);
  let updateProgress = $state(0);
  let updateFilePath = $state("");

  function showSnackbar(msg: string) {
    snackbarMsg = msg;
    snackbarVisible = true;
  }

  $effect(() => {
    applyThemeToDOM(theme.tokens);
  });

  onMount(async () => {
    const status = await getStatus();
    app.crocOk = status.ok;
    app.crocVersion = status.version?.replace("croc version ", "") ?? "not found";
    if (status.app_version) appVersion = status.app_version;

    window.onLog = (level: string, msg: string) => {
      const clean = msg.replace(/\x1b\[[0-9;]*m/g, "").replace(/\r/g, "");
      app.addLog(level as any, clean);
    };

    window.onEvent = (event: string, data: any) => {
      switch (event) {
        case "transfer_start":
          app.transferActive = true;
          app.transferMode = data.mode;
          app.transferCode = null;
          break;

        case "code_ready":
          app.transferCode = data.code;
          showSnackbar("Share this code with the receiver");
          break;

        case "transfer_done":
          if (data.success) {
            app.addLog("success", "Transfer completed successfully!");
            showSnackbar("Transfer complete!");
          } else if (!data.stopped) {
            showSnackbar("Transfer failed");
          }
          app.resetTransfer();
          break;

        case "update_progress":
          updateProgress = data.percent;
          break;

        case "update_ready":
          updateDownloading = false;
          updateFilePath = data.path;
          showSnackbar("Update downloaded! Click to install.");
          break;

        case "update_failed":
          updateDownloading = false;
          showSnackbar("Update download failed");
          break;
      }
    };

    // Check for updates silently on startup
    try {
      const info = await checkUpdate();
      if (info.update_available && info.download_url) {
        updateAvailable = true;
        updateUrl = info.download_url;
        updateVersion = info.latest_version ?? "";
      }
    } catch { /* silent fail */ }
  });

  const tabs = [
    { id: "send" as const, icon: "upload", label: "Send" },
    { id: "receive" as const, icon: "download", label: "Receive" },
    { id: "guide" as const, icon: "menu_book", label: "Guide" },
  ];

  function switchTab(tab: "send" | "receive" | "guide") {
    app.activeTab = tab;
  }

  async function handleStop() {
    await stopTransfer();
  }

  async function handleDownloadUpdate() {
    if (!updateUrl) return;
    updateDownloading = true;
    updateProgress = 0;
    await downloadUpdate(updateUrl);
  }

  async function handleInstallUpdate() {
    if (updateFilePath) {
      await launchUpdate(updateFilePath);
    }
  }
</script>

<div class="flex flex-col h-screen bg-surface text-on-surface overflow-hidden select-none">

  <!-- Top App Bar -->
  <div class="flex items-center gap-3 px-4 py-3 min-h-16">
    <span class="text-primary"><Icon name="swap_horiz" size={28} /></span>
    <h1 class="flex-1 text-[22px] font-normal text-on-surface" style="font-family: var(--font-brand);">Croc Transfer</h1>
    <span
      class="text-[11px] font-medium px-2.5 py-1 rounded-full
             bg-surface-container-high text-on-surface-variant"
    >
      v{appVersion}
    </span>
  </div>

  <!-- Update banner -->
  {#if updateAvailable && !updateDownloading && !updateFilePath}
    <div class="flex items-center gap-2 px-4 py-2 bg-tertiary-container text-on-tertiary-container text-sm">
      <Icon name="system_update" size={18} />
      <span class="flex-1">Update {updateVersion} available</span>
      <button
        class="text-sm font-medium px-3 py-1 rounded-full bg-tertiary text-on-tertiary
               cursor-pointer border-none"
        onclick={handleDownloadUpdate}
      >
        Download
      </button>
    </div>
  {/if}

  {#if updateDownloading}
    <div class="flex items-center gap-2 px-4 py-2 bg-tertiary-container text-on-tertiary-container text-sm">
      <Icon name="downloading" size={18} />
      <span class="flex-1">Downloading update... {updateProgress}%</span>
    </div>
    <div class="h-1 bg-surface-container-highest">
      <div
        class="h-full bg-tertiary"
        style="width: {updateProgress}%; transition: width 200ms ease;"
      ></div>
    </div>
  {/if}

  {#if updateFilePath}
    <div class="flex items-center gap-2 px-4 py-2 bg-primary-container text-on-primary-container text-sm">
      <Icon name="check_circle" size={18} />
      <span class="flex-1">Update ready to install</span>
      <button
        class="text-sm font-medium px-3 py-1 rounded-full bg-primary text-on-primary
               cursor-pointer border-none"
        onclick={handleInstallUpdate}
      >
        Install & restart
      </button>
    </div>
  {/if}

  <!-- Progress -->
  <ProgressBar active={app.transferActive} />

  <!-- Tab Bar -->
  <div class="flex bg-surface-container border-b border-outline-variant">
    {#each tabs as tab (tab.id)}
      <button
        class="tab-btn flex-1 flex items-center justify-center gap-2 py-3.5 px-4
               border-none bg-transparent text-sm font-medium cursor-pointer relative
               {app.activeTab === tab.id ? 'text-primary' : 'text-on-surface-variant'}"
        style="transition: color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
        onclick={() => switchTab(tab.id)}
      >
        <div
          class="absolute inset-0 bg-on-surface opacity-0 hover-layer"
          style="transition: opacity var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
        ></div>
        <span class="relative z-10 inline-flex items-center gap-2">
          <Icon name={tab.icon} size={20} />
          {tab.label}
        </span>
        {#if app.activeTab === tab.id}
          <span class="absolute bottom-0 left-1/2 -translate-x-1/2 w-[60%] h-[3px] bg-primary rounded-t-sm"></span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto p-4">
    {#if app.activeTab === "send"}
      <SendPage onsnackbar={showSnackbar} />
    {:else if app.activeTab === "receive"}
      <ReceivePage onsnackbar={showSnackbar} />
    {:else}
      <GuidePage />
    {/if}
  </div>

  <!-- Stop bar -->
  {#if app.transferActive}
    <div class="px-4 py-2">
      <Button variant="error" full onclick={handleStop}>
        <Icon name="stop" size={18} />
        Stop Transfer
      </Button>
    </div>
  {/if}

  <!-- Status Bar -->
  <div class="flex items-center gap-2 px-4 py-2.5
              bg-surface-container border-t border-outline-variant
              text-xs text-on-surface-variant">
    <div
      class="w-2 h-2 rounded-full
             {app.transferActive ? 'bg-primary animate-pulse'
               : app.crocOk ? 'bg-tertiary'
               : 'bg-error'}"
    ></div>
    <span>
      {app.transferActive
        ? app.transferMode === "send" ? "Sending..." : "Receiving..."
        : app.crocOk ? "Ready" : "croc not found"}
    </span>
    <span class="flex-1"></span>
    <span>croc {app.crocVersion}</span>
  </div>
</div>

<Snackbar message={snackbarMsg} bind:visible={snackbarVisible} />

<style>
  .tab-btn:hover .hover-layer {
    opacity: 0.08;
  }
  .tab-btn:active .hover-layer {
    opacity: 0.1;
  }
</style>
