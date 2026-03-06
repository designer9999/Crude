<!--
  Quick receive — one-click receive for croc transfers.
  LAN direct handles receiving automatically, this is for non-LAN fallback.
-->
<script lang="ts">
  import Card from "$lib/ui/Card.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Icon from "$lib/ui/Icon.svelte";
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
    {#if app.lanConnected}
      <span class="text-xs font-normal text-primary ml-auto">LAN direct active</span>
    {/if}
  </div>

  {#if !hasContact}
    <div class="mb-3">
      <TextField
        label="Transfer code"
        placeholder="Enter the code from sender"
        mono
        bind:value={manualCode}
      />
    </div>
  {/if}

  {#if app.lanConnected}
    <div class="flex items-center gap-2 py-2 text-sm text-primary">
      <Icon name="bolt" size={18} />
      Files arrive automatically — no action needed
    </div>
  {:else}
    <Button
      full
      variant="tonal"
      onclick={handleReceive}
      disabled={app.transferActive || (!hasContact && !manualCode.trim())}
    >
      <Icon name="download" size={18} />
      {app.transferActive && app.transferMode === "receive" ? "Receiving..." : "Receive via croc"}
    </Button>
  {/if}

  {#if app.receiveOptions.outFolder}
    <div class="mt-2 flex items-center gap-1.5 text-xs text-on-surface-variant">
      <Icon name="folder" size={14} />
      <span class="truncate">Saving to: {app.receiveOptions.outFolder}</span>
    </div>
  {/if}
</Card>
