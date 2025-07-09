<script lang="ts">
  import GHubDataLocModal from '$components/GHubDataLocModal.svelte';
  import { Info } from 'lucide-svelte';
  import { siGithub, siDiscord } from 'simple-icons';
  export let dataPath: string | null = null;
  export let validationResult: any = null;
  export let showDataModal: boolean = false;
  export let openDataModal: () => void;
  export let closeDataModal: () => void;
  export let fetchDataPath: () => void;
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
    <button type="button" class="p-1 hover:text-blue-300 transition-colors" title="Info">
      <Info class="w-4 h-4" />
    </button>
    <button type="button" class="p-1 hover:text-blue-300 transition-colors" title="GitHub" aria-label="GitHub">
    <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor">
      <path d={siGithub.path} />
    </svg>
    </button>
    <button type="button" class="p-1 hover:text-blue-300 transition-colors" title="Discord" aria-label="Discord">
    <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor">
      <path d={siDiscord.path} />
    </svg>
    </button>
  </div>
  <GHubDataLocModal open={showDataModal} onClose={closeDataModal} on:pathChange={fetchDataPath} />
</div>
