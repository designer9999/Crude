<!--
  File content preview modal — shows text file contents with metadata.
-->
<script lang="ts">
  import Icon from "$lib/ui/Icon.svelte";
  import { showInExplorer } from "$lib/api/bridge";
  import type { FilePreview } from "$lib/api/bridge";

  interface Props {
    preview: FilePreview | null;
    path: string;
    loading: boolean;
    onclose: () => void;
  }

  let { preview, path, loading, onclose }: Props = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="file-preview-overlay" onclick={onclose}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="file-preview-modal" onclick={(e) => e.stopPropagation()}>
    <div class="file-preview-header">
      <div class="file-preview-title">
        <span class="file-preview-name">{preview?.name ?? '...'}</span>
        {#if preview}
          <span class="file-preview-meta">
            {preview.size} &bull; {preview.line_count} lines
            {#if preview.truncated} &bull; Truncated{/if}
          </span>
        {/if}
      </div>
      <div class="file-preview-actions">
        <button class="overlay-btn" onclick={() => showInExplorer(path)} title="Open in folder">
          <Icon name="folder_open" size={18} />
        </button>
        <button class="overlay-btn" onclick={onclose}>
          <Icon name="close" size={20} />
        </button>
      </div>
    </div>
    <div class="file-preview-body">
      {#if loading}
        <div class="file-preview-loading">Loading preview...</div>
      {:else if preview?.content}
        <pre class="file-preview-code">{preview.content}</pre>
      {:else}
        <div class="file-preview-loading">No preview available for this file type.</div>
      {/if}
    </div>
  </div>
</div>

<style>
  .file-preview-overlay {
    position: fixed;
    inset: 0;
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.7);
    animation: overlay-in var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects) both;
    cursor: pointer;
  }
  @keyframes overlay-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  .file-preview-modal {
    display: flex;
    flex-direction: column;
    width: 90%;
    max-width: 700px;
    max-height: 80%;
    border-radius: 16px;
    background: var(--md-sys-color-surface-container);
    color: var(--md-sys-color-on-surface);
    box-shadow: 0 8px 32px rgba(0,0,0,0.4);
    cursor: default;
    overflow: hidden;
    animation: dialog-scale-in var(--md-spring-fast-spatial-dur) var(--md-spring-fast-spatial) both;
  }
  @keyframes dialog-scale-in {
    from { transform: scale(0.95); opacity: 0; }
    to   { transform: scale(1); opacity: 1; }
  }
  .file-preview-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    padding: 16px 16px 12px;
    border-bottom: 1px solid color-mix(in srgb, var(--md-sys-color-outline) 20%, transparent);
    flex-shrink: 0;
  }
  .file-preview-title {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }
  .file-preview-name {
    font-size: 16px;
    font-weight: 600;
    word-break: break-word;
  }
  .file-preview-meta {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    opacity: 0.7;
  }
  .file-preview-actions {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
    margin-left: 12px;
  }
  .overlay-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 50%;
    background: rgba(255,255,255,0.15);
    color: var(--md-sys-color-on-surface);
    cursor: pointer;
    flex-shrink: 0;
    transition: background var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);
  }
  .overlay-btn:hover {
    background: rgba(255,255,255,0.25);
  }
  .file-preview-body {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    scrollbar-width: thin;
  }
  .file-preview-code {
    margin: 0;
    padding: 16px;
    font-family: var(--md-sys-typescale-body-small-font, "Roboto Mono", monospace);
    font-size: 13px;
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-word;
    background: color-mix(in srgb, var(--md-sys-color-on-surface) 4%, transparent);
    border-radius: 0 0 16px 16px;
  }
  .file-preview-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 40px 20px;
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
    opacity: 0.6;
  }
</style>
