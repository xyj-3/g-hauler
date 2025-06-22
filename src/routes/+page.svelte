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
    const index = applications.findIndex(app => app.application_id === updatedGame.application_id);
    if (index !== -1) {
      applications[index] = updatedGame;
      applications = [...applications]; // Trigger reactivity
    }
  };

  onMount(() => {
    loadApplications();
  });
</script>

<main class="w-full bg-gray-900 text-white min-h-full p-8">
  <div class="max-w-7xl mx-auto">
    {#if loading}
      <div class="text-center h-screen flex flex-col items-center justify-center">
        <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-white mx-auto mb-4"></div>
        <p class="text-lg">Loading applications...</p>
      </div>
    {:else if error}
      <div class="text-center text-red-400 h-screen flex flex-col items-center justify-center">
        <p class="text-lg mb-4">Error loading applications</p>
        <p class="text-sm mb-4">{error}</p>
        <button 
          onclick={loadApplications}
          class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
        >
          Retry
        </button>
      </div>
    {:else if applications.length > 0}
      <!-- <div class="mb-6">
        <h1 class="text-3xl font-bold mb-2">G Hauler</h1>
        <p class="text-gray-400">{applications.length} {applications.length === 1 ? 'game' : 'games'} found</p>
      </div> -->
        <!-- Responsive grid: 2 cols on small screens, 3 on medium, 4 on large, 5 on xl, 6 on 2xl -->      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-7 gap-6 pb-8">
        {#each applications as game, index}
          <GameCard {game} tabindex={index} ongameUpdated={handleGameUpdated} />
        {/each}
      </div>
    {:else}
      <div class="text-center text-gray-400 h-screen flex flex-col items-center justify-center">
        <p class="text-lg">No applications found</p>
        <p class="text-sm mt-2">Make sure G HUB data is properly configured</p>
      </div>
    {/if}
  </div>
</main>
