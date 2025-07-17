<script lang="ts">
  import GHubDirSelectModal from '$components/modal/GHubDirSelectModal.svelte';
  import InfoModal from '$components/modal/InfoModal.svelte';
  import { Info } from 'lucide-svelte';
  import { siGithub, siDiscord } from 'simple-icons';
  let { dataPath = null, validationResult = null, showDataModal = false, openDataModal, closeDataModal, fetchDataPath } = $props();
  
  // Info modal state
  let showInfoModal = $state(false);
  function openInfoModal() { showInfoModal = true; }
  function closeInfoModal() { showInfoModal = false; }
</script>

<div class="bg-color-background px-4 h-8 text-xs text-white font-light flex justify-between items-center">
  <div class="flex items-center space-x-4">
    {#if dataPath && validationResult && validationResult.data_path_exists && validationResult.applications_json_exists && validationResult.current_json_exists && validationResult.version_json_exists && validationResult.build_id && validationResult.images_dir_exists}
      <button type="button" class="cursor-pointer bg-transparent border-0 p-0 m-0 text-left hover:text-blue-300 transition-colors" onclick={openDataModal}>
        G HUB data location: {dataPath}
      </button>
    {:else}
      <button type="button" class="cursor-pointer hover:text-blue-300 transition-colors bg-transparent border-0 p-0 m-0 text-left" onclick={openDataModal}>
        G HUB data location: Click to Select
      </button>
    {/if}
  </div>
  <!-- Icons on the right -->
  <div class="flex items-center space-x-2">
    <button type="button" class="p-1 cursor-pointer hover:text-blue-300 transition-colors" title="Info" onclick={openInfoModal}>
      <Info class="w-4 h-4" />
    </button>
    <button type="button" class="p-1 cursor-pointer hover:text-blue-300 transition-colors" title="GitHub" aria-label="GitHub">
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor">
        <path d={siGithub.path} />
      </svg>
    </button>
    <button type="button" class="p-1 cursor-pointer hover:text-blue-300 transition-colors" title="Discord" aria-label="Discord">
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor">
        <path d={siDiscord.path} />
      </svg>
    </button>
  </div>
  <GHubDirSelectModal open={showDataModal} onClose={closeDataModal} on:pathChange={fetchDataPath} />
  <InfoModal open={showInfoModal} onClose={closeInfoModal} />
</div>
