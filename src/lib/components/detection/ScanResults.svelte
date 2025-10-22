<script lang="ts">
  import type { GameScanResult, DetectedGame } from '$lib/types';
  import ScanResultCard from './ScanResultCard.svelte';

  interface Props {
    scanResults: GameScanResult | null;
    selectedGames: Set<string>;
    isScanning: boolean;
    showLoadingUI: boolean;
    onImport?: () => void;
    onToggleGame?: (gameId: string) => void;
    onSelectAll?: () => void;
    onDeselectAll?: () => void;
  }

  let {
    scanResults,
    selectedGames,
    isScanning,
    showLoadingUI,
    onImport,
    onToggleGame,
    onSelectAll,
    onDeselectAll
  }: Props = $props();

  let selectedPlatformTab = $state<string>('all');
  let searchQuery = $state<string>('');

  // Define platform order
  const platformOrder = [
    'Steam',
    'Epic Games',
    'GOG Galaxy',
    'EA App',
    'Ubisoft Connect',
    'Riot Games',
    'Windows Registry',
    'macOS Apps'
  ];

  // Helper function to get platforms that have games in order
  function getPlatformsWithGames(results: GameScanResult): string[] {
    return platformOrder.filter(platform =>
      results.gamesByPlatform[platform]?.length > 0
    );
  }

  // Helper function to sort games alphabetically by name
  function sortGamesByName(games: DetectedGame[]): DetectedGame[] {
    return [...games].sort((a, b) =>
      a.name.localeCompare(b.name, undefined, { sensitivity: 'base' })
    );
  }

  // Filter games by search query
  function filterGamesBySearch(games: DetectedGame[]): DetectedGame[] {
    if (!searchQuery.trim()) return games;
    const query = searchQuery.toLowerCase();
    return games.filter(game => game.name.toLowerCase().includes(query));
  }

  // Get platforms that have games
  let platformsWithGames = $derived(() => {
    if (!scanResults) return [];
    return getPlatformsWithGames(scanResults);
  });

  // Get filtered games based on selected tab and search query
  let filteredGames = $derived((): [string, DetectedGame[]][] => {
    if (!scanResults) return [];
    const results = scanResults;

    if (selectedPlatformTab === 'all') {
      // Use the games list from backend, which already contains all games
      const sortedAndFiltered = filterGamesBySearch(sortGamesByName(results.games));

      // Return as a single group
      return sortedAndFiltered.length > 0 ? [['all', sortedAndFiltered]] : [];
    }

    const games = results.gamesByPlatform[selectedPlatformTab] || [];
    return [[selectedPlatformTab, filterGamesBySearch(sortGamesByName(games))]];
  });

  function handleToggleGame(gameId: string) {
    if (onToggleGame) {
      onToggleGame(gameId);
    }
  }

  function handleSelectAll() {
    if (onSelectAll) {
      onSelectAll();
    }
  }

  function handleDeselectAll() {
    if (onDeselectAll) {
      onDeselectAll();
    }
  }

  function handleImport() {
    if (onImport) {
      onImport();
    }
  }
</script>

<div class="border border-gray-700 rounded-lg bg-gray-800/50 overflow-hidden flex flex-col flex-1 min-h-0">
  {#if showLoadingUI}
    <!-- Loading State -->
    <div class="flex-1 flex flex-col items-center justify-center space-y-4">
      <div class="w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
      <p class="text-gray-300">Scanning for installed games...</p>
    </div>
  {:else if scanResults}
    <!-- Results Header -->
    <div class="px-4 border-b border-gray-700 flex-shrink-0">
      <div class="flex items-center justify-between h-12">
        <div>
          <h2 class="text-sm font-medium">
            Scan Results
            <span class="text-xs text-gray-400 font-normal ml-2">
              {scanResults.totalCount} {scanResults.totalCount === 1 ? 'game' : 'games'} · {scanResults.scanDurationMs}ms
              {#if scanResults.errors.length > 0}
                · <span class="text-yellow-400">{scanResults.errors.length} {scanResults.errors.length === 1 ? 'warning' : 'warnings'}</span>
              {/if}
            </span>
          </h2>
        </div>
        <div class="h-12 flex items-center justify-end">
          {#if selectedGames.size > 0}
            <button
              class="px-5 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg text-sm font-medium transition-colors shadow-lg hover:shadow-xl"
              onclick={handleImport}
            >
              Import {selectedGames.size} Selected
            </button>
          {/if}
        </div>
      </div>
    </div>

    <!-- Platform Tabs -->
    {#if scanResults.totalCount > 0}
      <div class="border-b border-gray-700 flex-shrink-0 px-4 bg-gray-800/30 pt-1">
        <div class="flex gap-2 overflow-x-auto">
          <button
            class="px-3 py-2 text-sm font-medium transition-colors border-b-3 whitespace-nowrap {selectedPlatformTab === 'all' ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
            onclick={() => selectedPlatformTab = 'all'}
          >
            All ({scanResults.totalCount})
          </button>
          {#each platformsWithGames() as platform}
            {@const count = scanResults.gamesByPlatform[platform]?.length || 0}
            <button
              class="px-3 py-2 text-sm font-medium transition-colors border-b-[3px] whitespace-nowrap {selectedPlatformTab === platform ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
              onclick={() => selectedPlatformTab = platform}
            >
              {platform} ({count})
            </button>
          {/each}
        </div>
      </div>

      <!-- Search Bar -->
      <div class="px-4 py-2 border-b border-gray-700/50 bg-gray-800/20 flex-shrink-0">
        <div class="flex items-center gap-2">
          <div class="relative flex-1">
            <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400 pointer-events-none" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <input
              type="text"
              bind:value={searchQuery}
              placeholder="Search games..."
              class="w-full pl-10 pr-10 py-1.5 bg-gray-700/40 border border-gray-600/50 rounded-lg text-sm text-white placeholder-gray-400 focus:outline-none focus:border-blue-500/70 focus:bg-gray-700/60 focus:ring-1 focus:ring-blue-500/50 transition-colors"
            />
            {#if searchQuery}
              <button
                onclick={() => searchQuery = ''}
                class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-300 transition-colors"
                aria-label="Clear search"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            {/if}
          </div>
          <button
            class="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 text-white rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            onclick={handleSelectAll}
          >
            Select All
          </button>
          <button
            class="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 text-white rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            onclick={handleDeselectAll}
          >
            Deselect All
          </button>
        </div>
      </div>
    {/if}

    <!-- Results Content -->
    <div class="flex-1 overflow-y-auto">
      {#if scanResults.totalCount === 0}
        <div class="p-8 text-center text-gray-400">
          <svg class="w-16 h-16 mx-auto mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <p class="text-lg mb-2">No games found</p>
          <p class="text-sm">Try selecting different scan options or check your game installations.</p>
        </div>
      {:else}

        {#if selectedPlatformTab === 'all'}
          <!-- All platforms view - modern card-based selection -->
          <div class="p-4 grid grid-cols-2 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3">
            {#each filteredGames() as [_, games]: [string, DetectedGame[]]}
              {#each games as game: DetectedGame}
                <ScanResultCard
                  {game}
                  isSelected={selectedGames.has(game.id)}
                  showPlatformBadge={true}
                  onclick={() => handleToggleGame(game.id)}
                />
              {/each}
            {/each}
          </div>
        {:else}
          <!-- Single platform view - modern card-based selection -->
          <div class="p-4 grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-3">
            {#each filteredGames() as [_, games]: [string, DetectedGame[]]}
              {#each games as game: DetectedGame}
                <ScanResultCard
                  {game}
                  isSelected={selectedGames.has(game.id)}
                  showPlatformBadge={false}
                  onclick={() => handleToggleGame(game.id)}
                />
              {/each}
            {/each}
          </div>
        {/if}

        {#if scanResults.errors.length > 0}
          <div class="p-6 border-t border-gray-700 bg-yellow-900/10">
            <h3 class="text-sm font-medium text-yellow-400 mb-2">Scan Warnings</h3>
            <ul class="space-y-1 text-xs text-yellow-300/80">
              {#each scanResults.errors as error}
                <li>• {error}</li>
              {/each}
            </ul>
          </div>
        {/if}
      {/if}
    </div>
  {:else}
    <!-- Empty State -->
    <div class="flex-1 flex flex-col items-center justify-center text-gray-400 p-8">
      <svg class="w-20 h-20 mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
      <p class="text-lg mb-2">No scan results yet</p>
      <p class="text-sm text-center">Click "Scan All Platforms" or choose a custom scan to get started.</p>
    </div>
  {/if}
</div>
