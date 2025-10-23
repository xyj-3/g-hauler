<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import GameCardWebSocket from '$lib/components/GameCardWebSocket.svelte';
  import GameCardSkeleton from '$lib/components/GameCardSkeleton.svelte';
  import type { GHUBApp, ApplicationsResponse } from '$lib/types';
  import { ws } from '$lib/services/websocket';
  import { homePageLoaded } from '$lib/stores/appState';

  let applications = $state<GHUBApp[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let unsubscribeMessage: UnlistenFn | null = null;
  let unsubscribeConnected: UnlistenFn | null = null;
  let loadingTimeoutId: ReturnType<typeof setTimeout> | null = null;

  // Number of skeleton cards to show while loading
  const SKELETON_COUNT = 24;
  // Maximum time to show skeleton before showing error (10 seconds)
  const LOADING_TIMEOUT_MS = 10000;

  const loadApplications = async () => {
    console.log('[library] loadApplications() called');
    try {
      // Check if WebSocket is connected
      const isConnected = await ws.isConnected();
      console.log('[library] WebSocket connected?', isConnected);

      if (!isConnected) {
        console.log('[library] WebSocket not connected yet, will retry when connected');
        return;
      }

      loading = true;
      error = null;
      console.log('[library] Setting loading to true, clearing error');

      // Clear any existing timeout
      if (loadingTimeoutId) {
        clearTimeout(loadingTimeoutId);
      }

      // Set timeout for loading failure
      loadingTimeoutId = setTimeout(() => {
        if (loading) {
          loading = false;
          error = 'Loading timeout: Unable to fetch applications. The WebSocket server may not be responding.';
          homePageLoaded.set(true);
        }
      }, LOADING_TIMEOUT_MS);

      // Send GET /applications message using helper method
      await ws.getApplications();
      console.log('[library] GET /applications message sent');
    } catch (err) {
      console.error('Failed to load applications:', err);
      error = err instanceof Error ? err.message : 'Failed to load applications';
      loading = false;
      if (loadingTimeoutId) {
        clearTimeout(loadingTimeoutId);
      }
    }
  };

  const handleGameUpdated = (updatedGame: GHUBApp) => {
    const index = applications.findIndex(app => app.applicationId === updatedGame.applicationId);
    if (index !== -1) {
      applications[index] = updatedGame;
      applications = [...applications]; // Trigger reactivity
    }
  };

  onMount(async () => {
    // Listen for WebSocket connected event
    unsubscribeConnected = await listen('websocket-connected', () => {
      console.log('[library] WebSocket connected event received, loading applications');
      loadApplications();
    });

    // Listen for WebSocket messages
    unsubscribeMessage = await listen('websocket-message', (event) => {
      try {
        console.log('[library] WebSocket event received:', event);

        const message: ApplicationsResponse = typeof event.payload === 'string'
          ? JSON.parse(event.payload)
          : event.payload;

        console.log('[library] Parsed message:', message);
        console.log('[library] Message path:', message.path);
        console.log('[library] Payload type:', typeof message.payload);
        console.log('[library] Has applications?:', !!message.payload?.applications);

        // Check if this is a response to /applications
        if (message.path === '/applications') {
          console.log('[library] Path matches /applications');

          if (message.payload?.applications) {
            console.log('[library] Applications array found, length:', message.payload.applications.length);

            // Map the WebSocket response to GHUBApp format
            applications = message.payload.applications.map((app) => ({
              applicationId: app.applicationId || app.databaseId || '',
              categoryColors: app.categoryColors || [],
              commands: (app.commands || []).map((cmd) => ({
                category: cmd.category || '',
                keystroke: [], // WebSocket response doesn't include keystroke data
                name: cmd.name || ''
              })),
              detection: [], // WebSocket response doesn't include detection
              name: app.name,
              posterTitlePosition: app.posterTitlePosition || '0',
              posterUrl: app.posterUrl || '',
              version: app.version || 1
            }));

            loading = false;
            homePageLoaded.set(true); // Signal that home page has loaded

            // Clear the loading timeout since we successfully loaded
            if (loadingTimeoutId) {
              clearTimeout(loadingTimeoutId);
              loadingTimeoutId = null;
            }

            console.log('[library] Successfully loaded', applications.length, 'applications');
          } else {
            console.warn('[library] Path matches but no applications in payload');
            loading = false;
            homePageLoaded.set(true); // Still signal loaded even if empty

            // Clear the loading timeout
            if (loadingTimeoutId) {
              clearTimeout(loadingTimeoutId);
              loadingTimeoutId = null;
            }
          }
        } else {
          console.log('[library] Path does not match /applications, ignoring');
        }
      } catch (err) {
        console.error('[library] Failed to process WebSocket message:', err);
        error = err instanceof Error ? err.message : 'Failed to process response';
        loading = false;
        homePageLoaded.set(true); // Signal loaded even on error

        // Clear the loading timeout
        if (loadingTimeoutId) {
          clearTimeout(loadingTimeoutId);
          loadingTimeoutId = null;
        }
      }
    });

    // Try to load applications immediately if already connected
    loadApplications();
  });

  onDestroy(() => {
    if (unsubscribeMessage) {
      unsubscribeMessage();
    }
    if (unsubscribeConnected) {
      unsubscribeConnected();
    }
    // Clear any pending timeout
    if (loadingTimeoutId) {
      clearTimeout(loadingTimeoutId);
    }
    // Reset the store when leaving the page
    homePageLoaded.set(false);
  });
</script>

<main class="w-full text-white min-h-full px-6 py-6">
  <div class="max-w-[2000px] mx-auto">
    <!-- Debug info -->
    {console.log('[library] Rendering - loading:', loading, 'error:', error, 'applications.length:', applications.length)}

    {#if error}
      <div class="text-center text-red-400 h-screen flex flex-col items-center justify-center">
        <svg class="w-16 h-16 mb-4 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p class="text-xl font-semibold mb-2">Error loading applications</p>
        <p class="text-sm text-gray-400 mb-6">{error}</p>
        <button
          onclick={loadApplications}
          class="px-6 py-3 bg-btn hover:bg-btn-hover rounded-lg transition-colors font-medium shadow-lg hover:shadow-xl"
        >
          Retry
        </button>
      </div>
    {:else if loading}
      <!-- Skeleton loading state -->
      <div class="mb-6">
        <div class="h-8 bg-gray-700 rounded w-48 mb-2 animate-pulse"></div>
        <div class="h-4 bg-gray-700 rounded w-32 animate-pulse"></div>
      </div>

      <!-- Skeleton grid -->
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8 gap-5 pb-4">
        {#each Array(SKELETON_COUNT) as _, index}
          <GameCardSkeleton />
        {/each}
      </div>
    {:else if applications.length > 0}
      <div class="mb-6">
        <h1 class="text-2xl font-dm-sans font-bold mb-1">Game Library (WebSocket)</h1>
        <p class="text-sm text-gray-400">{applications.length} {applications.length === 1 ? 'profile' : 'profiles'}</p>
      </div>

      <!-- Responsive grid with improved spacing -->
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8 gap-5 pb-4">
        {#each applications as game, index}
          <GameCardWebSocket {game} tabindex={index} ongameUpdated={handleGameUpdated} />
        {/each}
      </div>
    {:else}
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
