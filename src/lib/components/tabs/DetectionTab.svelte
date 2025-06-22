<script lang="ts">
  import type { GHUBApp } from '$lib/types';

  interface DetectionTabProps {
    game: GHUBApp;
  }

  const { game }: DetectionTabProps = $props();
</script>

<div class="space-y-6">
  <h3 class="text-lg font-medium text-white mb-4">Detection Methods</h3>
  
  {#if game.detection.length === 0}
    <p class="text-gray-400 italic">No detection methods configured</p>
  {:else}
    <div class="space-y-3">
      {#each game.detection as detection, index}
        <div class="bg-gray-700 rounded-lg p-4">
          <div class="text-white font-medium mb-2">Detection Method #{index + 1}</div>
          <pre class="whitespace-pre-wrap bg-gray-800 p-2 rounded text-xs overflow-x-auto text-gray-300">{JSON.stringify(detection, null, 2)}</pre>
        </div>
      {/each}
    </div>
  {/if}
  
  <div class="bg-yellow-900 border-l-4 border-yellow-500 p-4">
    <p class="text-sm text-yellow-200">
      <strong>Note:</strong> Detection methods are complex structures that define how the game is found on the system. 
      Editing these requires technical knowledge of the detection systems.
    </p>
  </div>
</div>
