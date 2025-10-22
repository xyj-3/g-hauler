<script lang="ts">
  import type { DetectedGame } from '$lib/types';

  interface Props {
    game: DetectedGame;
    isSelected: boolean;
    showPlatformBadge?: boolean;
    onclick?: () => void;
  }

  let { game, isSelected, showPlatformBadge = false, onclick }: Props = $props();

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
      'Ubisoft Connect': 'bg-green-600/20 text-green-400',
      'GOG Galaxy': 'bg-purple-600/20 text-purple-400',
      'Riot Games': 'bg-red-600/20 text-red-400',
      'Windows Registry': 'bg-cyan-600/20 text-cyan-400',
      'macOS App': 'bg-slate-600/20 text-slate-300',
      'EA App': 'bg-orange-600/20 text-orange-400'
    };
    return colors[platformName] || 'bg-gray-600/20 text-gray-400';
  }

  let gamePlatform = $derived(getPlatformName(game));
</script>

<button
  {onclick}
  class="relative p-4 rounded-lg border-2 transition-all duration-200 text-left group {isSelected
    ? 'border-blue-500 bg-blue-500/10 shadow-lg shadow-blue-500/20'
    : 'border-gray-700 bg-gray-800/40 hover:border-gray-600 hover:bg-gray-800/60 hover:shadow-md'}"
>
  <!-- Selection indicator -->
  <div class="absolute top-3 right-3 w-6 h-6 rounded-full border-2 transition-all duration-200 flex items-center justify-center {isSelected
    ? 'border-blue-500 bg-blue-500'
    : 'border-gray-600 bg-gray-800 group-hover:border-gray-500'}">
    {#if isSelected}
      <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
      </svg>
    {/if}
  </div>

  <!-- Game content -->
  <div class="pr-8">
    <div class="flex items-center gap-2 mb-2">
      <h3 class="text-sm font-semibold text-white group-hover:text-blue-300 transition-colors line-clamp-1 flex-1">
        {game.name}
      </h3>
      {#if showPlatformBadge}
        <span class="px-2 py-0.5 {getPlatformColor(gamePlatform)} rounded-md text-xs font-medium whitespace-nowrap flex-shrink-0">
          {gamePlatform}
        </span>
      {/if}
    </div>

    <div class="space-y-1.5">
      <p class="text-xs text-gray-400 font-mono">{getPlatformIdentifier(game)}</p>

      {#if game.installPath}
        <p class="text-xs text-gray-500 truncate" title={game.installPath}>
          <svg class="w-3 h-3 inline-block mr-1 -mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          {game.installPath}
        </p>
      {/if}
    </div>
  </div>
</button>
