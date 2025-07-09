<script lang="ts">
  export let open: boolean;
  export let onClose: () => void;
  export let showCloseButton: boolean = true;
  export let maxWidth: string = 'max-w-lg';

  function handleBackgroundClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }
</script>

{#if open}
  <div
    class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
    role="dialog"
    aria-modal="true"
    tabindex="0"
    on:click={handleBackgroundClick}
    on:keydown={(e) => { if (e.key === 'Escape') onClose(); }}
  >
    <div class="relative bg-background rounded-xl shadow-xl p-8 {maxWidth} w-full">
      <div class="w-full h-full">
        {#if showCloseButton}
          <button type="button" aria-label="Close" class="absolute top-3 right-3 text-neutral-400 hover:text-white transition" on:click={onClose}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        {/if}
        <slot />
      </div>
    </div>
  </div>
{/if}
