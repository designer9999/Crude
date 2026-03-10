<!--
  Main transfer view — per-contact or fallback mode
  Assembles: UnifiedSendArea + QuickReceive + CodeDisplay + ActivityLog + LogPanel
-->
<script lang="ts">
  import { getAppState } from "$lib/state/app-state.svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import Card from "$lib/ui/Card.svelte";
  import Button from "$lib/ui/Button.svelte";
  import UnifiedSendArea from "./UnifiedSendArea.svelte";
  import QuickReceive from "./QuickReceive.svelte";
  import ActivityLog from "./ActivityLog.svelte";
  import CodeDisplay from "../send/CodeDisplay.svelte";
  import LogPanel from "../LogPanel.svelte";
  import ContactAvatar from "../contacts/ContactAvatar.svelte";

  interface Props {
    onsnackbar?: (msg: string) => void;
    onaddcontact?: () => void;
  }

  let { onsnackbar, onaddcontact }: Props = $props();

  const app = getAppState();
  const contact = $derived(app.activeContact);
</script>

<div class="flex flex-col gap-4">

  {#if !app.contacts.length}
    <!-- No contacts — onboarding -->
    <Card variant="filled">
      <div class="flex flex-col items-center text-center gap-3 py-4">
        <span class="text-primary"><Icon name="group_add" size={48} /></span>
        <div>
          <div class="text-lg font-medium text-on-surface">Welcome to Croc Transfer</div>
          <div class="text-sm text-on-surface-variant mt-1">
            Add a contact with a shared code phrase to start transferring files with one click.
          </div>
        </div>
        <Button onclick={onaddcontact}>
          <Icon name="person_add" size={18} />
          Add Contact
        </Button>
      </div>
    </Card>
  {/if}

  {#if contact}
    <!-- Contact header — minimal inline -->
    <div class="flex items-center gap-2 px-1">
      <ContactAvatar name={contact.name} color={contact.color} size="sm" />
      <span class="text-sm font-medium text-on-surface truncate flex-1">{contact.name}</span>
      {#if app.lanConnected}
        <span class="flex items-center gap-1 text-[11px] text-primary">
          <Icon name="bolt" size={11} />
          LAN — {app.lanPeerIp}
        </span>
      {:else}
        <span class="text-[11px] text-on-surface-variant">Searching LAN...</span>
      {/if}
    </div>
  {/if}

  <!-- Unified send -->
  <UnifiedSendArea contactName={contact?.name} {onsnackbar} />

  <!-- Transfer code display (only for manual/no-contact sends) -->
  {#if app.transferCode && !contact}
    <CodeDisplay code={app.transferCode} oncopied={() => onsnackbar?.("Code copied!")} />
  {/if}

  <!-- Quick receive -->
  <QuickReceive contactName={contact?.name} {onsnackbar} />

  <!-- Activity log -->
  <ActivityLog />

  <!-- Raw croc log (collapsible) -->
  <LogPanel />

</div>
