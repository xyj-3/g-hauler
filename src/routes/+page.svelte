<script lang="ts">
  import { onMount } from 'svelte';
  import GameCard from '$lib/components/GameCard.svelte';
  import type { GHUBApp } from '$lib/types';
  import { ghub } from '$lib/stores';
  import { homePageLoaded } from '$lib/stores/appState';

  const handleGameUpdated = (updatedGame: GHUBApp) => {
    console.log('[Library] Game updated:', updatedGame.name);
  };

  onMount(() => {
    const cleanup = $effect.root(() => {
      $effect(() => {
        if (ghub.isReady) {
          homePageLoaded.set(true);
        }
      });
    });

    return () => {
      cleanup();
      if (!import.meta.hot) {
        homePageLoaded.set(false);
      }
    };
  });
</script>

<main class="w-full text-white min-h-full px-6 py-4">
  <h1 class="text-xl font-dm-sans mb-1">Library</h1>

  <div class="mb-3"></div>

  <div class="max-w-[2000px] mx-auto">
    {#if ghub.status === 'error'}
      <div class="text-center text-red-400 h-screen flex flex-col items-center justify-center">
        <svg class="w-16 h-16 mb-4 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p class="text-xl font-semibold mb-2">Connection Error</p>
        <p class="text-sm text-gray-400 mb-6">{ghub.error || 'Unable to connect to G HUB'}</p>
        <p class="text-xs text-gray-500">Make sure Logitech G HUB is running</p>
      </div>
    {:else if ghub.isLoading}
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8 gap-5 pb-4">
        {#each Array(24) as _, index}
          <GameCard loading={true} tabindex={index} />
        {/each}
      </div>
    {:else if ghub.isReady && ghub.asGHUBApps.length > 0}
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8 gap-5 pb-4">
        {#each ghub.asGHUBApps as game, index}
          <GameCard {game} tabindex={index} ongameUpdated={handleGameUpdated} />
        {/each}
      </div>
    {:else}
      <div class="h-screen flex flex-col items-center justify-center text-gray-400">
        <svg class="w-20 h-20 mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
        </svg>
        <p class="text-lg mb-2">No game profiles found</p>
        <p class="text-sm text-center">Add game profiles in G HUB to get started</p>
      </div>
    {/if}
  </div>
</main>
