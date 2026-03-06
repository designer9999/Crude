<!--
  Quick receive — one-click receive or auto-receive toggle
  When auto-receive is on, files are accepted automatically.
-->
<script lang="ts">
  import Card from "$lib/ui/Card.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import Switch from "$lib/ui/Switch.svelte";
  import TextField from "$lib/ui/TextField.svelte";
  import { getAppState } from "$lib/state/app-state.svelte";
  import { receiveFile } from "$lib/api/bridge";

  interface Props {
    contactName?: string;
    onsnackbar?: (msg: string) => void;
  }

  let { contactName, onsnackbar }: Props = $props();

  const app = getAppState();

  let manualCode = $state("");

  const contact = $derived(app.activeContact);
  const hasContact = $derived(!!contact);

  const isAutoReceiving = $derived(
    app.autoReceiveActive && app.autoReceiveContactId === app.activeContactId
  );

  function toggleAutoReceive(on: boolean) {
    if (!contact) return;
    app.updateContact(contact.id, { autoReceive: on });
  }

  async function handleReceive() {
    if (app.transferActive) return;

    const code = contact?.code ?? manualCode.trim();
    if (!code) {
      onsnackbar?.("Enter a code to receive");
      return;
    }

    if (contact) app.touchContact(contact.id);
    await receiveFile(code, app.effectiveReceiveOptions);
  }
</script>

<Card variant="outlined">
  <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
    <span class="text-primary"><Icon name="download" size={20} /></span>
    {contactName ? `Receive from ${contactName}` : "Receive"}
  </div>

  {#if hasContact}
    <!-- Auto-receive toggle — directly visible, not hidden in settings -->
    <div class="flex items-center gap-3 mb-3">
      <Switch
        checked={contact?.autoReceive ?? false}
        onchange={toggleAutoReceive}
      />
      <div class="flex-1 min-w-0">
        <div class="text-sm text-on-surface">Auto-receive</div>
        <div class="text-xs text-on-surface-variant">Accept incoming files automatically</div>
      </div>
      {#if isAutoReceiving}
        <div class="flex items-center gap-1.5 text-xs text-tertiary">
          <div class="w-2 h-2 rounded-full bg-tertiary animate-pulse"></div>
          Listening
        </div>
      {/if}
    </div>

    {#if isAutoReceiving && app.localIp && app.localIp !== "unknown"}
      <div class="mb-3 px-3 py-2 rounded-sm bg-surface-container-highest text-xs">
        <div class="text-on-surface-variant mb-1">Other PC must set relay to:</div>
        <div class="font-mono text-on-surface select-all">{app.localIp}:9009</div>
        <div class="text-on-surface-variant mt-1">Set this in the contact's relay field on the sending PC</div>
      </div>
    {/if}
  {/if}

  {#if !isAutoReceiving}
    {#if !hasContact}
      <div class="mb-3" style="--tf-bg: var(--md-sys-color-surface-container-highest);">
        <TextField
          label="Transfer code"
          placeholder="Enter the code from sender"
          mono
          bind:value={manualCode}
        />
      </div>
    {/if}

    <Button
      full
      variant="tonal"
      onclick={handleReceive}
      disabled={app.transferActive || (!hasContact && !manualCode.trim())}
    >
      <Icon name="download" size={18} />
      {app.transferActive && app.transferMode === "receive" ? "Receiving..." : "Receive"}
    </Button>
  {/if}

  {#if app.receiveOptions.outFolder}
    <div class="mt-2 flex items-center gap-1.5 text-xs text-on-surface-variant">
      <Icon name="folder" size={14} />
      <span class="truncate">Saving to: {app.receiveOptions.outFolder}</span>
    </div>
  {/if}
</Card>
