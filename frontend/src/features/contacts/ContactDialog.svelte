<!--
  Add/Edit contact dialog — name, code phrase, color picker
-->
<script lang="ts">
  import Dialog from "$lib/ui/Dialog.svelte";
  import TextField from "$lib/ui/TextField.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Switch from "$lib/ui/Switch.svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import { getAppState, CONTACT_COLORS, type Contact } from "$lib/state/app-state.svelte";

  interface Props {
    open: boolean;
    editContact?: Contact | null;
    onclose: () => void;
  }

  let { open = $bindable(false), editContact = null, onclose }: Props = $props();

  const app = getAppState();

  let name = $state("");
  let code = $state("");
  let color = $state(0);
  let autoReceive = $state(false);
  let relay = $state("");
  let showDelete = $state(false);

  // Reset form when dialog opens
  $effect(() => {
    if (open) {
      if (editContact) {
        name = editContact.name;
        code = editContact.code;
        color = editContact.color;
        autoReceive = editContact.autoReceive ?? false;
        relay = editContact.options?.relay ?? "";
      } else {
        name = "";
        code = "";
        color = Math.floor(Math.random() * CONTACT_COLORS.length);
        autoReceive = false;
        relay = "";
      }
      showDelete = false;
    }
  });

  const codeAsciiOnly = $derived(/^[\x20-\x7E]*$/.test(code.trim()));
  const isValid = $derived(name.trim().length > 0 && code.trim().length >= 4 && codeAsciiOnly);

  function handleSave() {
    if (!isValid) return;

    const contactOpts: Contact["options"] = {};
    if (relay.trim()) contactOpts.relay = relay.trim();

    if (editContact) {
      app.updateContact(editContact.id, {
        name: name.trim(),
        code: code.trim(),
        color,
        autoReceive,
        options: { ...editContact.options, ...contactOpts },
      });
    } else {
      const contact: Contact = {
        id: crypto.randomUUID(),
        name: name.trim(),
        code: code.trim(),
        color,
        autoReceive,
        lastUsedAt: new Date().toISOString(),
        options: Object.keys(contactOpts).length > 0 ? contactOpts : undefined,
      };
      app.addContact(contact);
      app.setActiveContact(contact.id);
    }

    open = false;
    onclose();
  }

  function handleDelete() {
    if (editContact) {
      app.removeContact(editContact.id);
    }
    open = false;
    onclose();
  }
</script>

<Dialog
  bind:open
  headline={editContact ? "Edit Contact" : "Add Contact"}
  confirmLabel="Save"
  confirmDisabled={!isValid}
  onconfirm={handleSave}
  onclose={onclose}
>
  <div class="flex flex-col gap-4">
    <TextField
      label="Name"
      placeholder="e.g. Alice — Office PC"
      bind:value={name}
    />

    <TextField
      label="Code phrase"
      placeholder="Shared password (min 4 chars)"
      mono
      bind:value={code}
    />

    {#if code.trim().length > 0 && !codeAsciiOnly}
      <div class="text-xs text-error">
        Code must contain only English letters, numbers, and symbols (no special characters like ä, ö, etc.)
      </div>
    {:else}
      <div class="text-xs text-on-surface-variant">
        Both you and your colleague must use the same code phrase.
      </div>
    {/if}

    <TextField
      label="Relay (for LAN auto-receive)"
      placeholder="e.g. 192.168.1.50:9009"
      mono
      bind:value={relay}
    />

    <div class="text-xs text-on-surface-variant">
      Set the other PC's IP:9009 here to enable instant LAN transfers.
      Leave empty to use the public relay (one-shot only).
    </div>

    <!-- Color picker -->
    <div>
      <div class="text-xs text-on-surface-variant mb-2">Color</div>
      <div class="flex gap-2">
        {#each CONTACT_COLORS as bg, i}
          <button
            class="w-7 h-7 rounded-full cursor-pointer border-2 shrink-0"
            aria-label="Color {i + 1}"
            style="
              background-color: {bg};
              border-color: {i === color ? 'var(--md-sys-color-on-surface)' : 'transparent'};
              transition: border-color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);
            "
            onclick={() => color = i}
          ></button>
        {/each}
      </div>
    </div>

    <!-- Auto-receive toggle -->
    <div class="flex items-center justify-between pt-2 border-t border-outline-variant">
      <div>
        <div class="text-sm text-on-surface">Auto-receive</div>
        <div class="text-xs text-on-surface-variant">Always listen for incoming files</div>
      </div>
      <Switch checked={autoReceive} onchange={(v) => autoReceive = v} />
    </div>

    <!-- Delete button (edit mode only) -->
    {#if editContact}
      <div class="pt-2 border-t border-outline-variant">
        {#if !showDelete}
          <button
            class="text-sm text-error cursor-pointer bg-transparent border-none px-0"
            onclick={() => showDelete = true}
          >
            Delete this contact
          </button>
        {:else}
          <div class="flex items-center gap-2">
            <span class="text-sm text-error">Are you sure?</span>
            <Button variant="error" onclick={handleDelete}>
              Delete
            </Button>
            <Button variant="outlined" onclick={() => showDelete = false}>
              No
            </Button>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</Dialog>
