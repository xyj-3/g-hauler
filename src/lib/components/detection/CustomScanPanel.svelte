<script lang="ts">
  import type { ScanOptions } from '$lib/types';

  interface Props {
    scanOptions: ScanOptions;
    isWindows: boolean;
    isMacOS: boolean;
  }

  let { scanOptions, isWindows, isMacOS }: Props = $props();
</script>

<div class="border border-gray-700/50 rounded-lg bg-gradient-to-br from-gray-800/60 to-gray-800/40 backdrop-blur-sm p-2.5 shadow-lg">
  <h3 class="text-xs font-semibold text-gray-300 uppercase tracking-wide mb-1.5">Scan Platforms</h3>
  <div class="flex flex-wrap gap-1.5">
    <button
      onclick={() => scanOptions.scanSteam = !scanOptions.scanSteam}
      class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors {scanOptions.scanSteam
        ? 'bg-btn hover:bg-btn-hover text-white border border-btn-disabled'
        : 'bg-gray-700/40 hover:bg-gray-700/60 text-gray-300 hover:text-white border border-gray-600/40 hover:border-gray-600'}"
    >
      Steam
    </button>

    <button
      onclick={() => scanOptions.scanEpicGames = !scanOptions.scanEpicGames}
      class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors {scanOptions.scanEpicGames
        ? 'bg-btn hover:bg-btn-hover text-white border border-btn-disabled'
        : 'bg-gray-700/40 hover:bg-gray-700/60 text-gray-300 hover:text-white border border-gray-600/40 hover:border-gray-600'}"
    >
      Epic Games
    </button>

    <button
      onclick={() => scanOptions.scanGogGalaxy = !scanOptions.scanGogGalaxy}
      class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors {scanOptions.scanGogGalaxy
        ? 'bg-btn hover:bg-btn-hover text-white border border-btn-disabled'
        : 'bg-gray-700/40 hover:bg-gray-700/60 text-gray-300 hover:text-white border border-gray-600/40 hover:border-gray-600'}"
    >
      GOG Galaxy
    </button>

    <button
      onclick={() => scanOptions.scanEaApp = !scanOptions.scanEaApp}
      class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors {scanOptions.scanEaApp
        ? 'bg-btn hover:bg-btn-hover text-white border border-btn-disabled'
        : 'bg-gray-700/40 hover:bg-gray-700/60 text-gray-300 hover:text-white border border-gray-600/40 hover:border-gray-600'}"
    >
      EA App
    </button>

    <button
      onclick={() => scanOptions.scanUplay = !scanOptions.scanUplay}
      class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors {scanOptions.scanUplay
        ? 'bg-btn hover:bg-btn-hover text-white border border-btn-disabled'
        : 'bg-gray-700/40 hover:bg-gray-700/60 text-gray-300 hover:text-white border border-gray-600/40 hover:border-gray-600'}"
    >
      Ubisoft Connect
    </button>

    <button
      onclick={() => scanOptions.scanRiotGames = !scanOptions.scanRiotGames}
      class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors {scanOptions.scanRiotGames
        ? 'bg-btn hover:bg-btn-hover text-white border border-btn-disabled'
        : 'bg-gray-700/40 hover:bg-gray-700/60 text-gray-300 hover:text-white border border-gray-600/40 hover:border-gray-600'}"
    >
      Riot Games
    </button>

    {#if isWindows}
      <button
        onclick={() => scanOptions.scanWinRegistry = !scanOptions.scanWinRegistry}
        class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors {scanOptions.scanWinRegistry
          ? 'bg-btn hover:bg-btn-hover text-white border border-btn-disabled'
          : 'bg-gray-700/40 hover:bg-gray-700/60 text-gray-300 hover:text-white border border-gray-600/40 hover:border-gray-600'}"
      >
        Windows Registry
      </button>
    {/if}

    {#if isMacOS}
      <button
        onclick={() => scanOptions.scanOsxBundle = !scanOptions.scanOsxBundle}
        class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors {scanOptions.scanOsxBundle
          ? 'bg-btn hover:bg-btn-hover text-white border border-btn-disabled'
          : 'bg-gray-700/40 hover:bg-gray-700/60 text-gray-300 hover:text-white border border-gray-600/40 hover:border-gray-600'}"
      >
        macOS Apps
      </button>
    {/if}
  </div>
</div>
