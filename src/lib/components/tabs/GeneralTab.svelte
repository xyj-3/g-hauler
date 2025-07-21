<script lang="ts">
  import type { GHUBApp } from '$lib/types';

  interface GeneralTabProps {
    game: GHUBApp;
  }

  const { game }: GeneralTabProps = $props();
</script>

<div class="space-y-6">
  <div>
    <label for="game-name" class="block text-sm font-medium text-gray-300 mb-2">
      Application ID
    </label>
    <input
      id="app-id"
      type="text"
      bind:value={game.applicationId}
      readonly
      class="w-full px-3 py-2 bg-gray-600 border border-gray-500 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
  </div>

  <div>
    <label for="game-name" class="block text-sm font-medium text-gray-300 mb-2">
      Game Name
    </label>
    <input
      id="game-name"
      type="text"
      bind:value={game.name}
      placeholder="Enter game name"
      class="w-full px-3 py-2 bg-gray-600 border border-gray-500 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
  </div>

  <div>
    <label for="poster-url" class="block text-sm font-medium text-gray-300 mb-2">
      Poster URL
    </label>
    <input
      id="poster-url"
      type="text"
      bind:value={game.posterUrl}
      placeholder="Enter poster URL"
      class="w-full px-3 py-2 bg-gray-600 border border-gray-500 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
  </div>

  <div>
    <label for="poster-title-position" class="block text-sm font-medium text-gray-300 mb-2">
      Poster Title Position
    </label>
    <input
      id="poster-title-position"
      type="text"
      bind:value={game.posterTitlePosition}
      placeholder="Enter poster title position"
      class="w-full px-3 py-2 bg-gray-600 border border-gray-500 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
  </div>

  <div>
    <label for="version" class="block text-sm font-medium text-gray-300 mb-2">
      Version
    </label>
    <input
      id="version"
      type="number"
      bind:value={game.version}
      placeholder="Enter version number"
      class="w-full px-3 py-2 bg-gray-600 border border-gray-500 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
  </div>
</div>
 