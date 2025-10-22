<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import GameCard from '$lib/components/GameCard.svelte';
  import type { GHUBApp } from '$lib/types';

  let applications = $state<GHUBApp[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  const loadApplications = async () => {
    try {
      loading = true;
      error = null;
      const apps = await invoke<GHUBApp[]>('get_applications');
      applications = apps;
    } catch (err) {
      console.error('Failed to load applications:', err);
      error = err instanceof Error ? err.message : 'Failed to load applications';
    } finally {
      loading = false;
    }
  };
  const handleGameUpdated = (updatedGame: GHUBApp) => {
    const index = applications.findIndex(app => app.applicationId === updatedGame.applicationId);
    if (index !== -1) {
      applications[index] = updatedGame;
      applications = [...applications]; // Trigger reactivity
    }
  };

  onMount(() => {
    loadApplications();
  });
</script>

<main class="w-full text-white min-h-full px-6 py-6">
  <div class="max-w-[2000px] mx-auto">
    {#if error}
      <div class="text-center text-red-400 h-screen flex flex-col items-center justify-center">
        <svg class="w-16 h-16 mb-4 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p class="text-xl font-semibold mb-2">Error loading applications</p>
        <p class="text-sm text-gray-400 mb-6">{error}</p>
        <button
          onclick={loadApplications}
          class="px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors font-medium shadow-lg hover:shadow-xl"
        >
          Retry
        </button>
      </div>
    {:else if applications.length > 0}
      <div class="mb-6">
        <h1 class="text-2xl font-dm-sans font-bold mb-1">Game Library</h1>
        <p class="text-sm text-gray-400">{applications.length} {applications.length === 1 ? 'profile' : 'profiles'}</p>
      </div>

      <!-- Responsive grid with improved spacing -->
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8 gap-5 pb-4">
        {#each applications as game, index}
          <GameCard {game} tabindex={index} ongameUpdated={handleGameUpdated} />
        {/each}
      </div>
    {:else if !loading}
      <div class="h-screen flex flex-col items-center justify-center text-gray-400">
        <svg class="w-20 h-20 mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
        </svg>
        <p class="text-lg mb-2">No game profiles found</p>
        <p class="text-sm text-center">Add game profiles to get started</p>
      </div>
    {/if}
  </div>
</main>
