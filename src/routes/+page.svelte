<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import GameCard from '$lib/components/GameCard.svelte';
  import type { GHUBApp, WebSocketMessage } from '$lib/types';
  import { ws } from '$lib/services/websocket';
  import { homePageLoaded } from '$lib/stores/appState';
  import { ghub } from '$lib/stores';

  let loading = $state(true);
  let error = $state<string | null>(null);
  let loadingTimeoutId: ReturnType<typeof setTimeout> | null = null;
  let unlistenWebSocketMessage: UnlistenFn | null = null;

  // Maximum time to show loading before showing error (10 seconds)
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

      // Send GET /applications message - response will be handled by the store
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
    // No need to manually update - the store will handle updates via WebSocket events
    console.log('[library] Game updated:', updatedGame.name);
  };

  // Track if we've received a response from the server
  let hasReceivedResponse = $state(false);

  // React to applications data arriving
  $effect(() => {
    const apps = ghub.asGHUBApps;

    // If we have apps, clear loading state
    if (apps.length > 0 && loading) {
      console.log('[library] Applications loaded from store:', apps.length);
      loading = false;
      hasReceivedResponse = true;
      homePageLoaded.set(true);

      // Clear the loading timeout since we successfully loaded
      if (loadingTimeoutId) {
        clearTimeout(loadingTimeoutId);
        loadingTimeoutId = null;
      }
    }
  });

  // React to connection state changes
  $effect(() => {
    if (ghub.connected && !hasReceivedResponse) {
      console.log('[library] WebSocket connected, loading applications');
      loadApplications();
    }
  });

  onMount(async () => {
    // Listen for WebSocket messages to detect responses
    unlistenWebSocketMessage = await listen('websocket-message', (event) => {
      try {
        const message: WebSocketMessage = typeof event.payload === 'string'
          ? JSON.parse(event.payload)
          : event.payload;

        // Check if this is the applications response
        if (message.path === '/applications') {
          console.log('[library] Received /applications response');
          hasReceivedResponse = true;
          loading = false;
          homePageLoaded.set(true);

          // Clear the loading timeout since we got a response
          if (loadingTimeoutId) {
            clearTimeout(loadingTimeoutId);
            loadingTimeoutId = null;
          }
        }
      } catch (error) {
        console.error('[library] Failed to parse WebSocket message:', error);
      }
    });

    // Check if WebSocket is already connected and we have no apps yet
    // This handles the case where the component remounts (e.g., hot reload)
    // but the WebSocket store persists with its connection state
    if (ghub.connected && ghub.asGHUBApps.length === 0) {
      console.log('[library] onMount: WebSocket already connected, loading applications');
      loadApplications();
    } else if (ghub.asGHUBApps.length > 0) {
      // If we already have apps in the store (from hot reload), don't show loading
      console.log('[library] onMount: Applications already in store, skipping load');
      loading = false;
      hasReceivedResponse = true;
      homePageLoaded.set(true);
    }
  });

  onDestroy(() => {
    // Clear any pending timeout
    if (loadingTimeoutId) {
      clearTimeout(loadingTimeoutId);
    }

    // Clean up WebSocket message listener
    if (unlistenWebSocketMessage) {
      unlistenWebSocketMessage();
    }

    // Don't reset homePageLoaded during hot reload - it causes splash screen to show unnecessarily
    // Only reset when actually navigating away (not during hot module reload)
    if (!import.meta.hot) {
      homePageLoaded.set(false);
    }
  });
</script>

<main class="w-full text-white min-h-full px-6 py-4">
  <h1 class="text-xl font-dm-sans mb-1">Library</h1>

  <div class="mb-3"></div>

  <div class="max-w-[2000px] mx-auto">
    <!-- Debug info -->
    {console.log('[library] Rendering - loading:', loading, 'error:', error, 'applications.length:', ghub.asGHUBApps.length)}

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
    {:else if ghub.asGHUBApps.length > 0 || loading}
      <!-- Responsive grid -->
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8 gap-5 pb-4">
        {#if loading}
          {#each Array(24) as _, index}
            <GameCard loading={true} tabindex={index} />
          {/each}
        {:else}
          {#each ghub.asGHUBApps as game, index}
            <GameCard {game} tabindex={index} ongameUpdated={handleGameUpdated} />
          {/each}
        {/if}
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
