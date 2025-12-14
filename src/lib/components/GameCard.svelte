<script lang="ts">
  import type { GHUBApp } from '$lib/types';
  import { onMount } from 'svelte';
  import GameEditModal from '$components/modal/GameEditModal.svelte';

  interface GameCardProps {
    game?: GHUBApp;
    loading?: boolean;
    tabindex?: number;
    ongameUpdated?: (game: GHUBApp) => void;
  }

  const { game, loading = false, tabindex, ongameUpdated }: GameCardProps = $props();

  // Show skeleton when explicitly loading or when no game data
  const isLoading = $derived(loading || !game);

  // Map special profile names
  const displayName = $derived(
    game?.name === 'APPLICATION_NAME_DESKTOP' ? 'Desktop' : game?.name
  );

  let cardElement: HTMLButtonElement;
  let isVisible = $state(false);
  let imageLoaded = $state(false);
  let observer: IntersectionObserver;
  let showEditModal = $state(false);

  const handleCardClick = () => {
    console.log('Game selected:', game.name);
    showEditModal = true;
    // Remove focus from the button to prevent persistent highlighting
    if (cardElement) {
      cardElement.blur();
    }
  };

  const handleModalClose = () => {
    showEditModal = false;
  };

  const handleGameSave = async (updatedGame: GHUBApp) => {
    try {
      // For WebSocket route, we might need different save logic
      // For now, just update the frontend state
      ongameUpdated?.(updatedGame);
      showEditModal = false;
      console.log('Game updated in WebSocket mode');
    } catch (error) {
      console.error('Failed to update application:', error);
      // Rethrow the error so the modal can handle it
      throw error;
    }
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleCardClick();
    }
  };

  const handleImageLoad = () => {
    imageLoaded = true;
  };

  const handleImageError = () => {
    imageLoaded = false;
  };

  onMount(() => {
    if (!cardElement) return;

    observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            isVisible = true;
            observer.unobserve(entry.target);
          }
        });
      },
      {
        rootMargin: '300px',
        threshold: 0,
      }
    );

    observer.observe(cardElement);

    return () => {
      if (observer && cardElement) {
        observer.unobserve(cardElement);
      }
    };
  });
</script>

{#if isLoading}
  <!-- Loading skeleton state -->
  <div role="status" class="cursor-default animate-pulse">
    <!-- Card skeleton with same border and shadow as loaded state -->
    <div class="relative aspect-[3/4] overflow-hidden rounded-lg border-2 border-gray-700/50 shadow-md">
      <div class="absolute inset-0 bg-gray-700"></div>
    </div>

    <!-- Title skeleton -->
    <div class="mt-2.5 px-1 flex justify-center">
      <div class="h-4 bg-gray-700 rounded w-3/4"></div>
    </div>

    <span class="sr-only">Loading...</span>
  </div>
{:else}
  <!-- Loaded state with interactive card -->
  <button
    bind:this={cardElement}
    class="cursor-pointer group transition-all duration-200 focus:outline-none rounded-lg active:scale-[0.98]"
    onclick={handleCardClick}
    onkeydown={handleKeyDown}
    tabindex={tabindex}
    aria-label="Select {displayName}"
  >
    <!-- Minimal border highlight -->
    <div class="relative aspect-[3/4] overflow-hidden rounded-lg transition-all duration-200 border-2 border-gray-700/50 hover:border-blue-400/60 shadow-md hover:shadow-lg">
      <div class="absolute inset-0 bg-gradient-to-br from-gray-700 to-gray-800"></div>
      {#if isVisible && game.posterUrl}
        <img
          src={game.posterUrl}
          alt="{game.name} poster"
          class="absolute inset-0 w-full h-full object-cover transition-opacity duration-300"
          class:opacity-0={!imageLoaded}
          onload={handleImageLoad}
          onerror={handleImageError}
          loading="lazy"
        />
      {:else if game.name === 'APPLICATION_NAME_DESKTOP'}
        <!-- Desktop SVG icon for APPLICATION_NAME_DESKTOP profile -->
        <div class="absolute inset-0 flex items-center justify-center p-8">
          <svg class="w-full h-full" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
            <!-- Monitor outer frame -->
            <rect x="10" y="15" width="80" height="50" rx="3" fill="#374151" stroke="#4B5563" stroke-width="2"/>
            <!-- Monitor screen -->
            <rect x="14" y="19" width="72" height="42" rx="1.5" fill="#1F2937"/>
            <!-- Screen content - window icon -->
            <rect x="22" y="27" width="20" height="16" rx="1" fill="#3B82F6" opacity="0.6"/>
            <rect x="44" y="27" width="20" height="16" rx="1" fill="#60A5FA" opacity="0.5"/>
            <rect x="22" y="45" width="42" height="12" rx="1" fill="#93C5FD" opacity="0.4"/>
            <!-- Monitor stand neck -->
            <rect x="47" y="65" width="6" height="12" rx="1" fill="#374151"/>
            <!-- Monitor stand base -->
            <ellipse cx="50" cy="82" rx="18" ry="3.5" fill="#374151"/>
            <path d="M 32 82 Q 32 78 50 78 Q 68 78 68 82" fill="#4B5563"/>
            <!-- Shine effect on screen -->
            <rect x="16" y="21" width="30" height="20" rx="1" fill="url(#shine)" opacity="0.15"/>
            <defs>
              <linearGradient id="shine" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#ffffff;stop-opacity:1" />
                <stop offset="100%" style="stop-color:#ffffff;stop-opacity:0" />
              </linearGradient>
            </defs>
          </svg>
        </div>
      {/if}
      <!-- Very subtle overlay -->
      <div class="absolute inset-0 bg-gradient-to-t from-black/40 via-black/0 to-black/0 opacity-0 hover:opacity-100 transition-opacity duration-200"></div>
    </div>

    <div class="mt-2.5 px-1">
      <h3 class="text-white font-semibold text-sm leading-tight truncate font-dm-sans hover:text-blue-300 transition-colors" title={displayName}>
        {displayName}
      </h3>
    </div>
  </button>
{/if}

{#if showEditModal && game}
  <GameEditModal
    {game}
    isOpen={showEditModal}
    onclose={handleModalClose}
    onsave={handleGameSave}
  />
{/if}
