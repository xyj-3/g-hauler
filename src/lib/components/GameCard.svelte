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
    aria-label="Select {game.name}"
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
      {/if}
      <!-- Very subtle overlay -->
      <div class="absolute inset-0 bg-gradient-to-t from-black/40 via-black/0 to-black/0 opacity-0 hover:opacity-100 transition-opacity duration-200"></div>
    </div>

    <div class="mt-2.5 px-1">
      <h3 class="text-white font-semibold text-sm leading-tight truncate font-dm-sans hover:text-blue-300 transition-colors" title={game.name}>
        {game.name}
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
