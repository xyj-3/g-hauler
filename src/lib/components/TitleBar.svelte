<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  
  const appWindow = getCurrentWindow();
  let isMenuOpen = $state(false);

  function minimize() {
    appWindow.minimize();
  }
  function maximize() {
    appWindow.toggleMaximize();
  }
  function close() {
    appWindow.close();
  }

  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
  }

  function closeMenu() {
    isMenuOpen = false;
  }

  function navigateToGames() {
    goto('/');
    closeMenu();
  }

  // Handle keyboard navigation
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && isMenuOpen) {
      closeMenu();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div data-tauri-drag-region class="flex items-center justify-center w-full h-8 bg-gray-900 relative">
  <!-- Hamburger Menu Button -->
  <div class="absolute left-0 top-0 h-full flex items-center">
    <button 
      aria-label="Open menu" 
      class="w-10 h-full flex items-center justify-center hover:bg-white/10 transition" 
      onclick={toggleMenu}
    >
      <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" viewBox="0 0 24 24">
        <line x1="3" y1="6" x2="21" y2="6" />
        <line x1="3" y1="12" x2="21" y2="12" />
        <line x1="3" y1="18" x2="21" y2="18" />
      </svg>
    </button>
  </div>

  <h1 data-tauri-drag-region class="text-sm font-medium text-teal-400 cursor-default font-dm-sans">G Hauler</h1>
  
  <div class="absolute right-0 top-0 h-full flex items-center gap-1">
    <button aria-label="Minimize" class="w-10 h-full flex items-center justify-center hover:bg-white/10 transition" onclick={minimize}>
      <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" viewBox="0 0 24 24"><line x1="6" y1="12" x2="18" y2="12" /></svg>
    </button>
    <button aria-label="Maximize" class="w-10 h-full flex items-center justify-center hover:bg-white/10 transition" onclick={maximize}>
      <svg class="w-3.5 h-3.5 text-white" fill="none" stroke="currentColor" stroke-width="2" rx="2" viewBox="0 0 24 24"><rect x="5" y="5" width="14" height="14" rx="3" /></svg>
    </button>
    <button aria-label="Close" class="w-10 h-full flex items-center justify-center hover:bg-red-600/80 transition" onclick={close}>
      <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="6" y1="6" x2="18" y2="18" /><line x1="6" y1="18" x2="18" y2="6" /></svg>
    </button>
  </div>
</div>

<!-- Menu Overlay -->
{#if isMenuOpen}
  <!-- Background overlay -->
  <div 
    class="fixed inset-0 bg-black/50 z-40" 
    role="button"
    tabindex="0"
    onclick={closeMenu}
    onkeydown={(e) => e.key === 'Escape' && closeMenu()}
  ></div>
  
  <!-- Menu panel -->
  <div class="fixed top-8 left-0 bg-gray-900 border border-gray-700 shadow-xl z-50 w-40 rounded-br-lg">
    <div class="p-2">
      <nav class="space-y-1">
        <button 
          class="w-full text-left px-2 py-1 text-sm text-white hover:bg-gray-800 rounded transition-colors flex items-center gap-2"
          onclick={navigateToGames}
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
            <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
            <line x="8" y1="21" x2="16" y2="21"/>
            <line x1="12" y1="17" x2="12" y2="21"/>
          </svg>
          Games
        </button>
      </nav>
    </div>
  </div>
{/if}
