<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { GameScanResult, ScanOptions, DetectedGame } from '$lib/types';

  let scanOptions = $state<ScanOptions>({
    scanSteam: true,
    scanEpicGames: true,
    scanUplay: true,
    scanGogGalaxy: true,
    scanRiotGames: true,
    scanWinRegistry: true,
    scanHumbleApp: true,
    scanOsxBundle: true
  });

  // State for scanning and results
  let isScanning = $state(false);
  let scanResults = $state<GameScanResult | null>(null);
  let selectedGames = $state<Set<string>>(new Set());
  let errorMessage = $state<string | null>(null);
  let selectedPlatformTab = $state<string>('all');

  async function handleScanForGames() {
    isScanning = true;
    errorMessage = null;
    scanResults = null;
    selectedGames = new Set();

    try {
      const result = await invoke<GameScanResult>('scan_installed_games', {
        options: scanOptions
      });
      scanResults = result;
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
      console.error('Scan failed:', error);
    } finally {
      isScanning = false;
    }
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
    if ('humbleApp' in platform) return 'Humble App';
    if ('riotGames' in platform) return 'Riot Games';
    if ('osxBundle' in platform) return 'macOS App';
    return 'Unknown';
  }

  function getPlatformIdentifier(game: DetectedGame): string {
    const platform = game.platform;
    if ('steam' in platform) return `App ID: ${platform.steam.appId}`;
    if ('epicGames' in platform) return `App: ${platform.epicGames.appName}`;
    if ('winRegistry' in platform) return `Key: ${platform.winRegistry.registryKey}`;
    if ('uplay' in platform) return `App ID: ${platform.uplay.appId}`;
    if ('gogGalaxy' in platform) return `Product ID: ${platform.gogGalaxy.productId}`;
    if ('humbleApp' in platform) return `Game ID: ${platform.humbleApp.gameId}`;
    if ('riotGames' in platform) return `App: ${platform.riotGames.appName}`;
    if ('osxBundle' in platform) return `Bundle ID: ${platform.osxBundle.bundleId}`;
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
      'Humble App': 'bg-orange-600/20 text-orange-400',
      'macOS App': 'bg-slate-600/20 text-slate-300'
    };
    return colors[platformName] || 'bg-gray-600/20 text-gray-400';
  }

  function handleImportGames() {
    // TODO: Implement game import logic
    console.log('Importing games:', Array.from(selectedGames));
  }

  // Get platforms that have games
  let platformsWithGames = $derived(() => {
    if (!scanResults) return [];
    return Object.entries(scanResults.gamesByPlatform)
      .filter(([_, games]) => games.length > 0)
      .map(([platform]) => platform);
  });

  // Get filtered games based on selected tab
  let filteredGames = $derived(() => {
    if (!scanResults) return [];
    if (selectedPlatformTab === 'all') {
      return Object.entries(scanResults.gamesByPlatform);
    }
    return [[selectedPlatformTab, scanResults.gamesByPlatform[selectedPlatformTab] || []]];
  });
</script>

<main class="w-full min-h-full px-6 py-6 text-white">
  <h1 class="text-2xl font-dm-sans mb-2">Fix Game Detection</h1>

  <p class="text-sm text-gray-400 mb-4">
    Scan your system to find locally installed games that G HUB is not detecting and add profiles for them.
  </p>

  <!-- Split View Layout -->
  <div class="flex gap-6 h-[calc(100vh-170px)]">
    <!-- Left Panel: Settings -->
    <div class="w-64 flex-shrink-0 flex flex-col gap-4">
      <!-- Platform Selection -->
      <div class="border border-gray-700 rounded-lg bg-gray-800/50 p-4">
        <h2 class="text-base font-medium mb-3">Platforms</h2>

        <div class="space-y-1">
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
              bind:checked={scanOptions.scanUplay}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">Ubisoft Connect</span>
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
              bind:checked={scanOptions.scanRiotGames}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">Riot Games</span>
          </label>

          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanWinRegistry}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">Windows Registry</span>
          </label>

          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanHumbleApp}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">Humble App</span>
          </label>

          <label class="flex items-center space-x-2 p-1.5 rounded hover:bg-gray-700/30 transition-colors cursor-pointer">
            <input
              type="checkbox"
              bind:checked={scanOptions.scanOsxBundle}
              class="w-4 h-4 rounded border-gray-600 bg-gray-700 text-blue-600 focus:ring-0 cursor-pointer"
            />
            <span class="text-sm">macOS Apps</span>
          </label>
        </div>
      </div>

      <!-- Scan Button -->
      <button
        class="w-full px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg font-medium transition-colors shadow-lg hover:shadow-xl"
        onclick={handleScanForGames}
        disabled={isScanning}
      >
        {isScanning ? 'Scanning...' : 'Scan for Games'}
      </button>

      <!-- Error Message -->
      {#if errorMessage}
        <div class="border border-red-500/50 rounded-lg p-4 bg-red-900/20">
          <p class="text-red-400 text-sm">
            <strong>Error:</strong> {errorMessage}
          </p>
        </div>
      {/if}
    </div>

    <!-- Right Panel: Results -->
    <div class="flex-1 border border-gray-700 rounded-lg bg-gray-800/50 overflow-hidden flex flex-col">
      {#if isScanning}
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
          <div class="border-b border-gray-700 flex-shrink-0 px-6">
            <div class="flex gap-1 overflow-x-auto">
              <button
                class="px-4 py-3 text-sm font-medium transition-colors border-b-2 whitespace-nowrap {selectedPlatformTab === 'all' ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
                onclick={() => selectedPlatformTab = 'all'}
              >
                All ({scanResults.totalCount})
              </button>
              {#each platformsWithGames() as platform}
                {@const count = scanResults.gamesByPlatform[platform]?.length || 0}
                <button
                  class="px-4 py-3 text-sm font-medium transition-colors border-b-2 whitespace-nowrap {selectedPlatformTab === platform ? 'border-blue-500 text-blue-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
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
            {#if selectedPlatformTab === 'all'}
              <!-- All platforms view - flat list with platform badges -->
              <div class="p-4 space-y-1.5">
                {#each filteredGames() as [platformName, games]}
                  {#each games as game}
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
                          <span class="px-2 py-0.5 {getPlatformColor(platformName)} rounded text-xs font-medium whitespace-nowrap">
                            {platformName}
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
                {#each filteredGames() as [platformName, games]}
                  {#if games.length > 0}
                    <div class="p-4">
                      <div class="space-y-1.5">
                        {#each games as game}
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
          <p class="text-sm text-center">Select platforms and click "Scan for Games" to get started.</p>
        </div>
      {/if}
    </div>
  </div>
</main>
