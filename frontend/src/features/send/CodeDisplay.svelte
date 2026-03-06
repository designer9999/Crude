<!--
  Transfer code display with copy button.
  Uses primaryContainer (M3 key standout fills).
-->
<script lang="ts">
  import Icon from "$lib/ui/Icon.svelte";

  interface Props {
    code: string;
    oncopied?: () => void;
  }

  let { code, oncopied }: Props = $props();

  async function copy() {
    await navigator.clipboard.writeText(code);
    oncopied?.();
  }
</script>

<div class="flex items-center gap-3 px-5 py-4 bg-primary-container rounded-lg code-enter">
  <Icon name="key" size={24} />
  <span class="flex-1 font-mono text-lg font-semibold tracking-wider text-on-primary-container select-text">
    {code}
  </span>
  <button
    class="group relative w-10 h-10 rounded-full bg-black/15 flex items-center justify-center
           cursor-pointer text-on-primary-container hover:bg-black/25 transition-colors"
    onclick={copy}
    title="Copy code"
  >
    <Icon name="content_copy" size={18} />
  </button>
</div>

<style>
  .code-enter {
    animation: slide-in var(--md-spring-default-spatial-dur) var(--md-spring-default-spatial) both;
  }
  @keyframes slide-in {
    from { opacity: 0; transform: translateY(-8px); }
    to   { opacity: 1; transform: translateY(0); }
  }
</style>
