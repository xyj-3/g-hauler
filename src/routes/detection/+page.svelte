<script lang="ts">
  function handleScanForGames() {
    // TODO: Implement game scanning logic
    console.log('Scanning for games...');
  }
</script>

<main class="w-full min-h-full px-6 py-6 text-white">
  <h1 class="text-2xl font-dm-sans mb-6">Fix Game Detection</h1>

  <div class="space-y-6">
    <p class="text-gray-300">
      Scan your system to find locally installed games that G HUB is not detecting and add profiles for them.
    </p>

    <div class="border border-gray-700 rounded-lg p-6 bg-gray-800/50">
      <div class="flex items-center justify-between">
        <div class="space-y-1">
          <h2 class="text-lg font-medium">Scan for Installed Games</h2>
          <p class="text-sm text-gray-400">
            Search your system for games that aren't currently detected by G HUB
          </p>
        </div>
        <button
          class="px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors shadow-lg hover:shadow-xl"
          onclick={() => handleScanForGames()}
        >
          Scan for Games
        </button>
      </div>
    </div>
  </div>
</main>