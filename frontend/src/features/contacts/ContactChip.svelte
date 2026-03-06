<!--
  Contact chip — avatar + name, selected state, click to activate
  Shows a small listening indicator when auto-receive is active
-->
<script lang="ts">
  import type { Contact } from "$lib/state/app-state.svelte";
  import { getAppState } from "$lib/state/app-state.svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import ContactAvatar from "./ContactAvatar.svelte";

  interface Props {
    contact: Contact;
    selected: boolean;
    onclick: () => void;
  }

  let { contact, selected, onclick }: Props = $props();
  const app = getAppState();

  const isListening = $derived(
    app.autoReceiveActive && app.autoReceiveContactId === contact.id
  );
</script>

<button
  class="group relative inline-flex items-center gap-1.5 h-8 pl-1 pr-3 rounded-full
         text-sm font-medium cursor-pointer select-none overflow-hidden shrink-0"
  style="
    background-color: {selected ? 'var(--md-sys-color-secondary-container)' : 'transparent'};
    border: {selected ? '1px solid transparent' : '1px solid var(--md-sys-color-outline-variant)'};
    color: {selected ? 'var(--md-sys-color-on-secondary-container)' : 'var(--md-sys-color-on-surface-variant)'};
    transition:
      background-color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects),
      border-color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects),
      color var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);
  "
  {onclick}
>
  <div
    class="absolute inset-0 opacity-0
           group-hover:opacity-8 group-focus-visible:opacity-10 group-active:opacity-10
           {selected ? 'bg-on-secondary-container' : 'bg-on-surface-variant'}"
    style="transition: opacity var(--md-spring-fast-effects-dur) var(--md-spring-fast-effects);"
  ></div>

  <span class="relative z-10">
    <ContactAvatar name={contact.name} color={contact.color} size="sm" />
  </span>
  <span class="relative z-10 max-w-24 truncate">{contact.name}</span>
  {#if isListening}
    <span class="relative z-10 w-1.5 h-1.5 rounded-full bg-tertiary animate-pulse"></span>
  {:else if selected}
    <span class="relative z-10 text-on-secondary-container opacity-50"
          style="font-size: 14px; line-height: 1;">
      <Icon name="edit" size={14} />
    </span>
  {/if}
</button>
