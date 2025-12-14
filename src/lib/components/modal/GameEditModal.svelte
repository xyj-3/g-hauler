<script lang="ts">
  import type { GHUBApp, Command } from '$lib/types';
  import GeneralTab from '$components/tabs/GeneralTab.svelte';
  import CommandsTab from '$components/tabs/CommandsTab.svelte';
  import Modal from '$components/modal/Modal.svelte';

  interface GameEditModalProps {
    game: GHUBApp;
    isOpen: boolean;
    onclose?: () => void;
    onsave?: (game: GHUBApp) => Promise<void> | void;
  }

  const { game, isOpen, onclose, onsave }: GameEditModalProps = $props();

  // Map special profile names
  const displayName = $derived(
    game?.name === 'APPLICATION_NAME_DESKTOP' ? 'Desktop' : game?.name
  );

  // Custom deep clone function for GHUBApp to avoid structuredClone issues
  const cloneGame = (gameObj: GHUBApp | null | undefined): GHUBApp => {
    if (!gameObj) {
      return {
        applicationId: '',
        categoryColors: [],
        commands: [],
        detection: [],
        name: '',
        posterTitlePosition: '',
        posterUrl: '',
        version: 0
      };
    }
    
    return {
      applicationId: gameObj.applicationId || '',
      categoryColors: (gameObj.categoryColors || []).map(cc => ({ ...cc })),
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
      posterTitlePosition: gameObj.posterTitlePosition || '',
      posterUrl: gameObj.posterUrl || '',
      version: gameObj.version || 0
    };
  };
  let editedGame = $state<GHUBApp>(cloneGame(game));
  let isSaving = $state(false);
  let activeTab = $state<'general' | 'commands'>('commands');
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
</script>

<Modal
  open={isOpen}
  onClose={handleClose}
  showCloseButton={false}
  maxWidth="max-w-2xl lg:max-w-4xl xl:max-w-5xl"
>
  <div class="max-h-[90vh] overflow-hidden flex flex-col" style="margin: -2rem;">
    <!-- Header -->
    <div class="flex items-center justify-between p-6 border-b border-gray-700">
      <h2 id="modal-title" class="text-xl font-semibold text-white font-dm-sans">
        Edit Profile - {displayName}
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
          class="py-4 px-1 border-b-2 font-medium text-sm transition-colors font-dm-sans {activeTab === 'general' ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
        >
          General
        </button>
        <button
          onclick={() => activeTab = 'commands'}
          class="py-4 px-1 border-b-2 font-medium text-sm transition-colors font-dm-sans {activeTab === 'commands' ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
        >
          Commands
        </button>
      </div>
    </div>
    <!-- Content -->
    <div class="flex-1 overflow-hidden p-6">
      <div class="h-full">
        {#if activeTab === 'general'}
          <GeneralTab game={editedGame} />
        {:else if activeTab === 'commands'}
          <CommandsTab game={editedGame} />
        {/if}
      </div>
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
        class="px-4 py-2 bg-btn hover:bg-btn-hover disabled:bg-btn-disabled disabled:cursor-not-allowed text-white rounded transition-colors flex items-center space-x-2"
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
</Modal>
