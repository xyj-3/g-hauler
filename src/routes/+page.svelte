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

  onMount(() => {
    loadApplications();
  });

  const firstGame = $derived(applications.length > 0 ? applications[0] : null);
</script>

<main class="w-full bg-gray-900 text-white min-h-screen flex flex-col items-center justify-center p-8">
  <div class="max-w-xs">
    {#if loading}
      <div class="text-center">
        <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-white mx-auto mb-4"></div>
        <p class="text-lg">Loading applications...</p>
      </div>
    {:else if error}
      <div class="text-center text-red-400">
        <p class="text-lg mb-4">Error loading applications</p>
        <p class="text-sm mb-4">{error}</p>
        <button 
          onclick={loadApplications}
          class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
        >
          Retry
        </button>
      </div>
    {:else if firstGame}
      <GameCard game={firstGame} />
    {:else}
      <div class="text-center text-gray-400">
        <p class="text-lg">No applications found</p>
        <p class="text-sm mt-2">Make sure G HUB data is properly configured</p>
      </div>
    {/if}
  </div>
</main>
