<script lang="ts">
  import type { GHUBApp, Command } from '$lib/types';
  import GeneralTab from './tabs/GeneralTab.svelte';
  import CommandsTab from './tabs/CommandsTab.svelte';
  import DetectionTab from './tabs/DetectionTab.svelte';

  interface GameEditModalProps {
    game: GHUBApp;
    isOpen: boolean;
    onclose?: () => void;
    onsave?: (game: GHUBApp) => Promise<void> | void;
  }

  const { game, isOpen, onclose, onsave }: GameEditModalProps = $props();

  // Custom deep clone function for GHUBApp to avoid structuredClone issues
  const cloneGame = (gameObj: GHUBApp | null | undefined): GHUBApp => {
    if (!gameObj) {
      return {
        application_id: '',
        category_colors: [],
        commands: [],
        detection: [],
        name: '',
        poster_title_position: '',
        poster_url: '',
        version: 0
      };
    }
    
    return {
      application_id: gameObj.application_id || '',
      category_colors: (gameObj.category_colors || []).map(cc => ({ ...cc })),
      commands: (gameObj.commands || []).map(cmd => ({
        category: cmd.category || '',
        keystroke: [...(cmd.keystroke || [])],
        name: cmd.name || ''
      })),
      detection: (gameObj.detection || []).map(det => {
        if (typeof det === 'object' && det !== null) {
          return { ...det };
        }
        return det;
      }),
      name: gameObj.name || '',
      poster_title_position: gameObj.poster_title_position || '',
      poster_url: gameObj.poster_url || '',
      version: gameObj.version || 0
    };
  };
  let editedGame = $state<GHUBApp>(cloneGame(game));
  let isSaving = $state(false);
  let activeTab = $state<'general' | 'commands' | 'detection'>('commands');
  // Update editedGame when game prop changes
  $effect(() => {
    editedGame = cloneGame(game);
  });

  const handleClose = () => {
    if (!isSaving) {
      onclose?.();
    }
  };
  const handleSave = async () => {
    if (isSaving) return;
    
    try {
      isSaving = true;
      await onsave?.(editedGame);
      // Close modal only on successful save
      handleClose();
    } catch (error) {
      console.error('Save failed:', error);
      // Keep modal open on error so user can try again
      // You could add error UI here if needed
    } finally {
      isSaving = false;
    }
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key === 'Escape') {
      handleClose();
    }
  };

  const handleBackdropClick = (event: MouseEvent | KeyboardEvent) => {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  };
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if isOpen}
  <div 
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
    style="top: 2rem;"
    onclick={handleBackdropClick}
    onkeydown={handleBackdropClick}
    role="button"
    tabindex="0"
    aria-label="Close modal by clicking outside"
  >
    <div class="bg-gray-800 rounded-lg w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col"
         role="dialog"
         aria-modal="true"
         aria-labelledby="modal-title">
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b border-gray-700">
        <h2 id="modal-title" class="text-xl font-semibold text-white">
          Edit Game - {game.name}
        </h2>
        <button
          onclick={handleClose}
          class="text-gray-400 hover:text-white transition-colors"
          aria-label="Close modal"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      <!-- Tabs -->
      <div class="border-b border-gray-700">
        <div class="flex space-x-8 px-6">
          <button
            onclick={() => activeTab = 'general'}
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors {activeTab === 'general' ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
          >
            General
          </button>
          <button
            onclick={() => activeTab = 'commands'}
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors {activeTab === 'commands' ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
          >
            Commands
          </button>
          <button
            onclick={() => activeTab = 'detection'}
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors {activeTab === 'detection' ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
          >
            Detection
          </button>
        </div>
      </div>
      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6">
        {#if activeTab === 'general'}
          <GeneralTab game={editedGame} />
        {:else if activeTab === 'commands'}
          <CommandsTab game={editedGame} />
        {:else if activeTab === 'detection'}
          <DetectionTab game={editedGame} />
        {/if}
      </div>
      <!-- Footer -->
      <div class="flex justify-end space-x-3 p-6 border-t border-gray-700">
        <button
          onclick={handleClose}
          disabled={isSaving}
          class="px-4 py-2 border border-gray-600 text-gray-300 hover:text-white hover:border-gray-500 disabled:opacity-50 disabled:cursor-not-allowed rounded transition-colors"
        >
          Cancel
        </button>
        <button
          onclick={handleSave}
          disabled={isSaving}
          class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-500 disabled:cursor-not-allowed text-white rounded transition-colors flex items-center space-x-2"
        >
          {#if isSaving}
            <div class="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full"></div>
            <span>Saving...</span>
          {:else}
            <span>Save Changes</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
