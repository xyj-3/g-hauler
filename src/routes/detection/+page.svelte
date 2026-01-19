<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { type as osType } from '@tauri-apps/plugin-os';
  import type { GameScanResult, ScanOptions } from '$lib/types';
  import CustomScanPanel from '$lib/components/detection/CustomScanPanel.svelte';
  import ScanResults from '$lib/components/detection/ScanResults.svelte';
  import { detectionStore } from '$lib/stores';

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

  // Local state (resets on component unmount)
  let isScanning = $state(false);
  let showLoadingUI = $state(false);
  let errorMessage = $state<string | null>(null);
  let showCustomScan = $state(false);

  // Persistent state (survives navigation)
  let scanResults = $derived(detectionStore.scanResults);
  let selectedGames = $derived(detectionStore.selectedGames);

  async function handleScanForGames() {
    isScanning = true;
    errorMessage = null;
    detectionStore.clearSelectedGames();
    showCustomScan = false; // Close custom scan panel when starting scan

    // Only show loading UI if scan takes longer than 300ms
    const loadingTimeout = setTimeout(() => {
      if (isScanning) {
        showLoadingUI = true;
        // Clear results only when showing loading UI
        detectionStore.setScanResults(null);
      }
    }, 300);

    try {
      const result = await invoke<GameScanResult>('scan_installed_games', {
        options: scanOptions
      });
      detectionStore.setScanResults(result);
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
      console.error('Scan failed:', error);
      detectionStore.setScanResults(null);
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
      detectionStore.removeSelectedGame(gameId);
    } else {
      detectionStore.addSelectedGame(gameId);
    }
  }

  function handleImportGames() {
    // TODO: Implement game import logic
    console.log('Importing games:', Array.from(selectedGames));
  }

  function handleSelectAll() {
    if (!scanResults) return;
    const allGameIds = new Set(scanResults.games.map(game => game.id));
    detectionStore.setSelectedGames(allGameIds);
  }

  function handleDeselectAll() {
    detectionStore.clearSelectedGames();
  }
</script>

<main class="w-full h-full flex flex-col px-6 py-4 text-white overflow-hidden">
  <h1 class="text-xl font-dm-sans mb-1">Fix Game Detection</h1>

  <p class="text-xs text-gray-400 mb-3">
    Scan your system to find locally installed games that G HUB is not detecting and add profiles for them.
  </p>

  <!-- Top Action Bar -->
  <div class="mb-3 space-y-2 flex-shrink-0">
    <!-- Scan Button -->
    <div class="inline-flex gap-2">
      <button
        class="px-4 py-2 bg-btn hover:bg-btn-hover disabled:bg-btn disabled:opacity-50 text-white rounded-lg font-medium transition-colors shadow-lg hover:shadow-xl flex items-center gap-2 min-w-[140px] justify-center active:scale-[0.98]"
        onclick={handleScanForGames}
        disabled={showLoadingUI}
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
        {showLoadingUI ? 'Scanning...' : 'Scan For Games'}
      </button>

      <!-- Gear Toggle -->
      <button
        class="px-2.5 py-2 bg-gray-700 hover:bg-gray-600 disabled:bg-gray-700 disabled:opacity-50 text-white rounded-lg transition-colors shadow-lg hover:shadow-xl {showCustomScan ? 'bg-btn hover:bg-btn-hover' : ''} active:scale-[0.98]"
        onclick={toggleCustomScan}
        disabled={showLoadingUI}
        aria-label="Custom scan options"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </button>
    </div>

    <!-- Custom Scan Configuration -->
    {#if showCustomScan}
      <CustomScanPanel {scanOptions} {isWindows} {isMacOS} />
    {/if}
  </div>

  <!-- Error Message -->
  {#if errorMessage}
    <div class="border border-red-500/50 rounded-lg p-3 bg-red-900/20 mb-2 flex-shrink-0">
      <p class="text-red-400 text-sm">
        <strong>Error:</strong> {errorMessage}
      </p>
    </div>
  {/if}

  <!-- Results Panel -->
  <ScanResults
    {scanResults}
    {selectedGames}
    {isScanning}
    {showLoadingUI}
    onImport={handleImportGames}
    onToggleGame={toggleGameSelection}
    onSelectAll={handleSelectAll}
    onDeselectAll={handleDeselectAll}
  />
</main>
