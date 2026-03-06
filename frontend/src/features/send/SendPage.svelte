<!--
  Send tab — file/text mode, source folder, options, send action
  Layout: scrollable content area + sticky send button at bottom
-->
<script lang="ts">
  import Card from "$lib/ui/Card.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import { getAppState } from "$lib/state/app-state.svelte";
  import { pickFiles, pickFolder, sendFiles, sendText, getFileInfo } from "$lib/api/bridge";
  import DropZone from "./DropZone.svelte";
  import FileList from "./FileList.svelte";
  import CodeDisplay from "./CodeDisplay.svelte";
  import SendOptions from "./SendOptions.svelte";
  import LogPanel from "../LogPanel.svelte";
  import TextField from "$lib/ui/TextField.svelte";

  interface Props {
    onsnackbar?: (msg: string) => void;
  }

  let { onsnackbar }: Props = $props();

  const app = getAppState();

  let sendMode = $state<"files" | "text">("files");
  let textContent = $state("");

  async function addPaths(paths: string[]) {
    for (const path of paths) {
      const info = await getFileInfo(path);
      app.addFile(path, info);
    }
  }

  async function handlePickFiles() {
    const result = await pickFiles();
    if (result && result.length > 0) await addPaths(result);
  }

  async function handlePickFolder() {
    const result = await pickFolder();
    if (result && result.length > 0) await addPaths(result);
  }

  async function handleSend() {
    if (app.transferActive) return;

    if (sendMode === "text") {
      if (!textContent.trim()) {
        onsnackbar?.("Enter some text to send");
        return;
      }
      await sendText(textContent.trim(), app.sendOptions);
    } else {
      if (!app.hasFiles) {
        onsnackbar?.("Select files or folders first");
        return;
      }
      await sendFiles(app.filePaths, app.sendOptions);
    }
  }

  async function handleBrowseFromSource() {
    const result = await pickFiles();
    if (result && result.length > 0) await addPaths(result);
  }

  let canSend = $derived(
    !app.transferActive && (sendMode === "files" ? app.hasFiles : !!textContent.trim())
  );
</script>

<div class="flex flex-col gap-4">

  <!-- Mode toggle -->
  <div class="flex bg-surface-container-highest rounded-full p-1 gap-1">
    <button
      class="flex-1 flex items-center justify-center gap-1.5 py-2 rounded-full text-sm font-medium
             cursor-pointer border-none mode-btn
             {sendMode === 'files'
               ? 'bg-primary text-on-primary'
               : 'bg-transparent text-on-surface-variant'}"
      onclick={() => sendMode = "files"}
    >
      <Icon name="folder_open" size={16} />
      Files
    </button>
    <button
      class="flex-1 flex items-center justify-center gap-1.5 py-2 rounded-full text-sm font-medium
             cursor-pointer border-none mode-btn
             {sendMode === 'text'
               ? 'bg-primary text-on-primary'
               : 'bg-transparent text-on-surface-variant'}"
      onclick={() => sendMode = "text"}
    >
      <Icon name="notes" size={16} />
      Text
    </button>
  </div>

  {#if sendMode === "files"}
    <!-- File selection -->
    <Card>
      <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-4">
        <span class="text-primary"><Icon name="folder_open" size={20} /></span>
        Select files or folders
      </div>

      <DropZone onadd={addPaths} />

      <div class="flex gap-2 mt-3">
        <Button variant="tonal" full onclick={handlePickFiles}>
          <Icon name="description" size={18} />
          Browse Files
        </Button>
        <Button variant="tonal" full onclick={handlePickFolder}>
          <Icon name="folder" size={18} />
          Browse Folder
        </Button>
      </div>

      <!-- Quick access shortcut inside the same card -->
      {#if app.sendOptions.sourceFolder}
        <div class="mt-3 pt-3 border-t border-outline-variant">
          <div class="flex items-center gap-2 mb-2">
            <span class="text-tertiary"><Icon name="bookmark" size={16} /></span>
            <span class="text-xs text-on-surface-variant font-mono truncate flex-1">{app.sendOptions.sourceFolder}</span>
          </div>
          <Button variant="tonal" full onclick={handleBrowseFromSource}>
            <Icon name="file_open" size={18} />
            Browse from saved folder
          </Button>
        </div>
      {/if}
    </Card>

    <!-- File list -->
    {#if app.hasFiles}
      <Card>
        <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
          <span class="text-primary"><Icon name="inventory_2" size={20} /></span>
          {app.files.length} {app.files.length === 1 ? "item" : "items"} selected
          <span class="flex-1"></span>
          <button
            class="text-xs text-on-surface-variant cursor-pointer hover:text-error px-2 py-1 rounded-sm
                   bg-transparent border-none"
            style="transition: color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
            onclick={() => app.clearFiles()}
          >
            Clear all
          </button>
        </div>
        <FileList />
      </Card>
    {/if}
  {:else}
    <!-- Text input -->
    <Card>
      <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-4">
        <span class="text-primary"><Icon name="notes" size={20} /></span>
        Send text or URL
      </div>
      <div style="--tf-bg: var(--md-sys-color-surface-container-highest);">
        <TextField
          label="Content"
          placeholder="Type or paste text, URLs, commands..."
          multiline
          rows={5}
          mono
          bind:value={textContent}
        />
      </div>
    </Card>
  {/if}

  <!-- Transfer code -->
  {#if app.transferCode}
    <CodeDisplay code={app.transferCode} oncopied={() => onsnackbar?.("Code copied!")} />
  {/if}

  <!-- SEND BUTTON — always visible, prominent -->
  <Button
    full
    onclick={handleSend}
    disabled={!canSend}
  >
    <Icon name="send" size={18} />
    {app.transferActive ? "Sending..." : sendMode === "text" ? "Send Text" : "Send"}
  </Button>

  <!-- Options (collapsed by default, below the send button) -->
  <SendOptions />

  <!-- Log -->
  <LogPanel />
</div>

<style>
  .mode-btn {
    transition: background-color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects),
                color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);
  }
</style>
