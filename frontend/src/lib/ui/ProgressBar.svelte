<!--
  M3 Linear progress indicator — indeterminate
-->
<script lang="ts">
  interface Props {
    active?: boolean;
  }

  let { active = false }: Props = $props();
</script>

{#if active}
  <div class="w-full h-1 bg-surface-container-highest overflow-hidden">
    <div class="h-full bg-primary rounded-full progress-fill"></div>
  </div>
{/if}

<style>
  .progress-fill {
    width: 30%;
    animation: indeterminate 1.5s cubic-bezier(0.2, 0, 0, 1) infinite;
  }
  @keyframes indeterminate {
    0%   { transform: translateX(-100%); width: 30%; }
    50%  { width: 50%; }
    100% { transform: translateX(400%); width: 30%; }
  }
</style>
