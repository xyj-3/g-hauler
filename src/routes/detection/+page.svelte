<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { type as osType } from '@tauri-apps/plugin-os';
  import type { GameScanResult, ScanOptions, DetectedGame } from '$lib/types';

  // Detect the current OS
  const currentOS = osType();
  const isWindows = currentOS === 'windows';
  const isMacOS = currentOS === 'macos';

  let scanOptions = $state<ScanOptions>({
    scanSteam: true,
    scanEpicGames: true,
    scanUplay: true,
    scanGogGalaxy: true,
    scanRiotGames: true,
    scanWinRegistry: false,
    scanOsxBundle: false,
    scanEaApp: true
  });

  // State for scanning and results
  let isScanning = $state(false);
  let showLoadingUI = $state(false);
  let scanResults = $state<GameScanResult | null>(null);
  let selectedGames = $state<Set<string>>(new Set());
  let errorMessage = $state<string | null>(null);
  let selectedPlatformTab = $state<string>('all');
  let searchQuery = $state<string>('');
  let showCustomScan = $state(false);

  async function handleScanForGames() {
    isScanning = true;
    errorMessage = null;
    selectedGames = new Set();

    // Only show loading UI if scan takes longer than 300ms
    const loadingTimeout = setTimeout(() => {
      if (isScanning) {
        showLoadingUI = true;
        // Clear results only when showing loading UI
        scanResults = null;
      }
    }, 300);

    try {
      const result = await invoke<GameScanResult>('scan_installed_games', {
        options: scanOptions
      });
      scanResults = result;
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
      console.error('Scan failed:', error);
      scanResults = null;
    } finally {
      clearTimeout(loadingTimeout);
      isScanning = false;
      showLoadingUI = false;
    }
  }

  function toggleCustomScan() {
    showCustomScan = !showCustomScan;
  }

  function toggleGameSelection(gameId: string) {
    if (selectedGames.has(gameId)) {
      selectedGames.delete(gameId);
    } else {
      selectedGames.add(gameId);
    }
    selectedGames = selectedGames; // Trigger reactivity
  }

  function getPlatformName(game: DetectedGame): string {
    const platform = game.platform;
    if ('steam' in platform) return 'Steam';
    if ('epicGames' in platform) return 'Epic Games';
    if ('winRegistry' in platform) return 'Windows Registry';
    if ('uplay' in platform) return 'Ubisoft Connect';
    if ('gogGalaxy' in platform) return 'GOG Galaxy';
    if ('riotGames' in platform) return 'Riot Games';
    if ('osxBundle' in platform) return 'macOS App';
    if ('eaApp' in platform) return 'EA App';
    return '';
  }

  function getPlatformIdentifier(game: DetectedGame): string {
    const platform = game.platform;
    if ('steam' in platform) return `App ID: ${platform.steam.appId}`;
    if ('epicGames' in platform) return `App: ${platform.epicGames.appName}`;
    if ('winRegistry' in platform) return `Key: ${platform.winRegistry.registryKey}`;
    if ('uplay' in platform) return `App ID: ${platform.uplay.appId}`;
    if ('gogGalaxy' in platform) return `Product ID: ${platform.gogGalaxy.productId}`;
    if ('riotGames' in platform) return `App: ${platform.riotGames.appName}`;
    if ('osxBundle' in platform) return `Bundle ID: ${platform.osxBundle.bundleId}`;
    if ('eaApp' in platform) return `Game ID: ${platform.eaApp.gameId}`;
    return '';
  }

  function getPlatformColor(platformName: string): string {
    const colors: Record<string, string> = {
      'Steam': 'bg-blue-600/20 text-blue-400',
      'Epic Games': 'bg-yellow-600/20 text-yellow-400',
      'Ubisoft Connect': 'bg-indigo-600/20 text-indigo-400',
      'GOG Galaxy': 'bg-purple-600/20 text-purple-400',
      'Riot Games': 'bg-red-600/20 text-red-400',
      'Windows Registry': 'bg-cyan-600/20 text-cyan-400',
      'macOS App': 'bg-slate-600/20 text-slate-300',
      'EA App': 'bg-orange-600/20 text-orange-400'
    };
    return colors[platformName] || 'bg-gray-600/20 text-gray-400';
  }

  function handleImportGames() {
    // TODO: Implement game import logic
    console.log('Importing games:', Array.from(selectedGames));
  }

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

  // Get platforms that have games
  let platformsWithGames = $derived(() => {
    if (!scanResults) return [];
    return getPlatformsWithGames(scanResults);
  });

  // Filter games by search query
  function filterGamesBySearch(games: DetectedGame[]): DetectedGame[] {
    if (!searchQuery.trim()) return games;
    const query = searchQuery.toLowerCase();
    return games.filter(game => game.name.toLowerCase().includes(query));
  }

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
</script>

<main class="w-full min-h-full px-6 py-6 text-white">
  <h1 class="text-2xl font-dm-sans mb-2">Fix Game Detection</h1>

  <p class="text-sm text-gray-400 mb-4">
    Scan your system to find locally installed games that G HUB is not detecting and add profiles for them.
  </p>

  <!-- Top Action Bar -->
  <div class="mb-4 space-y-3">
    <!-- Scan Button -->
    <div class="inline-flex gap-2">
      <button
        class="px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-600 disabled:opacity-50 text-white rounded-lg font-medium transition-colors shadow-lg hover:shadow-xl flex items-center gap-2 min-w-[180px] justify-center active:scale-[0.98]"
        onclick={handleScanForGames}
        disabled={showLoadingUI}
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
        {showLoadingUI ? 'Scanning...' : 'Scan For Games'}
      </button>

      <!-- Gear Toggle -->
      <button
        class="px-3 py-3 bg-gray-700 hover:bg-gray-600 disabled:bg-gray-700 disabled:opacity-50 text-white rounded-lg transition-colors shadow-lg hover:shadow-xl {showCustomScan ? 'bg-blue-600 hover:bg-blue-700' : ''} active:scale-[0.98]"
        onclick={toggleCustomScan}
        disabled={showLoadingUI}
        aria-label="Custom scan options"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </button>
    </div>

    <!-- Custom Scan Configuration -->
    {#if showCustomScan}
      <div class="border border-gray-700 rounded-lg bg-gray-800/50 p-4">
        <h3 class="text-sm font-medium mb-3">Select Platforms:</h3>
        <div class="grid grid-cols-3 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-7 gap-2">
          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanSteam}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">Steam</span>
          </label>

          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanEpicGames}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">Epic Games</span>
          </label>

          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanGogGalaxy}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">GOG Galaxy</span>
          </label>

          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanEaApp}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">EA App</span>
          </label>

          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanUplay}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">Ubisoft Connect</span>
          </label>

          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanRiotGames}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">Riot Games</span>
          </label>

          {#if isWindows}
            <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
              <input
                type="checkbox"
                bind:checked={scanOptions.scanWinRegistry}
                class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
              />
              <span class="text-sm">Windows Registry</span>
            </label>
          {/if}

          {#if isMacOS}
            <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
              <input
                type="checkbox"
                bind:checked={scanOptions.scanOsxBundle}
                class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
              />
              <span class="text-sm">macOS Apps</span>
            </label>
          {/if}
        </div>
      </div>
    {/if}
  </div>

  <!-- Error Message -->
  {#if errorMessage}
    <div class="border border-red-500/50 rounded-lg p-4 bg-red-900/20 mb-4">
      <p class="text-red-400 text-sm">
        <strong>Error:</strong> {errorMessage}
      </p>
    </div>
  {/if}

  <!-- Results Panel -->
  <div class="border border-gray-700 rounded-lg bg-gray-800/50 overflow-hidden flex flex-col h-[calc(100vh-260px)]">
      {#if showLoadingUI}
        <!-- Loading State -->
        <div class="flex-1 flex flex-col items-center justify-center space-y-4">
          <div class="w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
          <p class="text-gray-300">Scanning for installed games...</p>
        </div>
      {:else if scanResults}
        <!-- Results Header -->
        <div class="px-6 py-4 border-b border-gray-700 flex-shrink-0">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-base font-medium">
                Scan Results
                <span class="text-sm text-gray-400 font-normal ml-2">
                  {scanResults.totalCount} {scanResults.totalCount === 1 ? 'game' : 'games'} · {scanResults.scanDurationMs}ms
                  {#if scanResults.errors.length > 0}
                    · <span class="text-yellow-400">{scanResults.errors.length} {scanResults.errors.length === 1 ? 'warning' : 'warnings'}</span>
                  {/if}
                </span>
              </h2>
            </div>
            {#if selectedGames.size > 0}
              <button
                class="px-5 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg text-sm font-medium transition-colors shadow-lg hover:shadow-xl"
                onclick={handleImportGames}
              >
                Import {selectedGames.size} Selected
              </button>
            {/if}
          </div>
        </div>

        <!-- Platform Tabs -->
        {#if scanResults.totalCount > 0}
          <div class="border-b border-gray-700 flex-shrink-0 px-6 bg-gray-800/30">
            <div class="flex gap-2 overflow-x-auto pb-1">
              <button
                class="px-4 py-2.5 text-sm font-medium transition-colors border-b-2 whitespace-nowrap {selectedPlatformTab === 'all' ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
                onclick={() => selectedPlatformTab = 'all'}
              >
                All ({scanResults.totalCount})
              </button>
              {#each platformsWithGames() as platform}
                {@const count = scanResults.gamesByPlatform[platform]?.length || 0}
                <button
                  class="px-4 py-2.5 text-sm font-medium transition-colors border-b-2 whitespace-nowrap {selectedPlatformTab === platform ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
                  onclick={() => selectedPlatformTab = platform}
                >
                  {platform} ({count})
                </button>
              {/each}
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
            <!-- Search Bar -->
            <div class="px-6 py-3.5 border-b border-gray-700/50 bg-gray-800/20">
              <div class="relative">
                <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400 pointer-events-none" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
                <input
                  type="text"
                  bind:value={searchQuery}
                  placeholder="Search games..."
                  class="w-full pl-10 pr-10 py-2 bg-gray-700/40 border border-gray-600/50 rounded-lg text-sm text-white placeholder-gray-400 focus:outline-none focus:border-blue-500/70 focus:bg-gray-700/60 focus:ring-1 focus:ring-blue-500/50 transition-colors"
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
            </div>

            {#if selectedPlatformTab === 'all'}
              <!-- All platforms view - flat list with platform badges -->
              <div class="p-4 space-y-1.5">
                {#each filteredGames() as [_, games]: [string, DetectedGame[]]}
                  {#each games as game: DetectedGame}
                    {@const gamePlatform = getPlatformName(game)}
                    <label
                      class="flex items-start space-x-2.5 p-2 rounded hover:bg-gray-700/30 transition-colors cursor-pointer group"
                    >
                      <input
                        type="checkbox"
                        checked={selectedGames.has(game.id)}
                        onchange={() => toggleGameSelection(game.id)}
                        class="mt-0.5 w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
                      />
                      <div class="flex-1 min-w-0">
                        <div class="flex items-center gap-2">
                          <p class="text-sm font-medium group-hover:text-white transition-colors">{game.name}</p>
                          <span class="px-2 py-0.5 {getPlatformColor(gamePlatform)} rounded text-xs font-medium whitespace-nowrap">
                            {gamePlatform}
                          </span>
                        </div>
                        <p class="text-xs text-gray-400 mt-0.5">{getPlatformIdentifier(game)}</p>
                        {#if game.installPath}
                          <p class="text-xs text-gray-500 mt-0.5 truncate" title={game.installPath}>
                            {game.installPath}
                          </p>
                        {/if}
                      </div>
                    </label>
                  {/each}
                {/each}
              </div>
            {:else}
              <!-- Single platform view - grouped by platform -->
              <div class="divide-y divide-gray-700">
                {#each filteredGames() as [_, games]: [string, DetectedGame[]]}
                  {#if games.length > 0}
                    <div class="p-4">
                      <div class="space-y-1.5">
                        {#each games as game: DetectedGame}
                          <label
                            class="flex items-start space-x-2.5 p-2 rounded hover:bg-gray-700/30 transition-colors cursor-pointer group"
                          >
                            <input
                              type="checkbox"
                              checked={selectedGames.has(game.id)}
                              onchange={() => toggleGameSelection(game.id)}
                              class="mt-0.5 w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
                            />
                            <div class="flex-1 min-w-0">
                              <p class="text-sm font-medium group-hover:text-white transition-colors">{game.name}</p>
                              <p class="text-xs text-gray-400 mt-0.5">{getPlatformIdentifier(game)}</p>
                              {#if game.installPath}
                                <p class="text-xs text-gray-500 mt-0.5 truncate" title={game.installPath}>
                                  {game.installPath}
                                </p>
                              {/if}
                            </div>
                          </label>
                        {/each}
                      </div>
                    </div>
                  {/if}
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
</main>
