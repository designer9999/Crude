<!--
  File list — shows selected files with icon, name, size, and remove button.
-->
<script lang="ts">
  import Icon from "$lib/ui/Icon.svelte";
  import { getAppState } from "$lib/state/app-state.svelte";

  const app = getAppState();

  function iconForType(type: string): string {
    if (type === "folder") return "folder";
    if ([".jpg", ".jpeg", ".png", ".gif", ".svg", ".webp", ".bmp"].includes(type)) return "image";
    if ([".mp4", ".avi", ".mkv", ".mov", ".webm"].includes(type)) return "movie";
    if ([".mp3", ".wav", ".flac", ".ogg", ".aac"].includes(type)) return "audio_file";
    if ([".zip", ".rar", ".7z", ".tar", ".gz"].includes(type)) return "folder_zip";
    if ([".pdf"].includes(type)) return "picture_as_pdf";
    if ([".doc", ".docx", ".txt", ".rtf", ".md"].includes(type)) return "description";
    if ([".xls", ".xlsx", ".csv"].includes(type)) return "table_chart";
    return "draft";
  }
</script>

<div class="flex flex-col gap-1 max-h-48 overflow-y-auto">
  {#each app.files as file (file.path)}
    <div class="file-item group flex items-center gap-3 px-3 py-2.5 rounded-sm relative">
      <!-- State layer -->
      <div
        class="absolute inset-0 rounded-sm bg-on-surface opacity-0 state-layer"
        style="transition: opacity var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
      ></div>

      <!-- Icon -->
      <span class="relative z-10 text-primary shrink-0">
        <Icon name={file.info ? iconForType(file.info.type) : "draft"} size={22} />
      </span>

      <!-- Name + meta -->
      <div class="relative z-10 flex-1 min-w-0">
        <div class="text-sm text-on-surface truncate">
          {file.info?.name ?? file.path.split(/[\\/]/).pop()}
        </div>
        <div class="text-xs text-on-surface-variant">
          {#if file.info}
            {file.info.size}
            {#if file.info.type === "folder" && file.info.count != null}
              &middot; {file.info.count} {file.info.count === 1 ? "file" : "files"}
            {/if}
          {/if}
        </div>
      </div>

      <!-- Remove (hidden during active transfer) -->
      {#if !app.transferActive}
        <button
          class="relative z-10 shrink-0 w-7 h-7 rounded-full flex items-center justify-center
                 text-on-surface-variant cursor-pointer opacity-0 group-hover:opacity-100
                 transition-opacity"
          onclick={() => app.removeFile(file.path)}
          title="Remove"
        >
          <Icon name="close" size={16} />
        </button>
      {/if}
    </div>
  {/each}
</div>

<style>
  .file-item:hover .state-layer {
    opacity: 0.04;
  }
</style>
