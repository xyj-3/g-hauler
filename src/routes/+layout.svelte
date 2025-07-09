<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import SplashScreen from '$components/SplashScreen.svelte';
  import TitleBar from '$components/TitleBar.svelte';
  import '../app.css';
  import { invoke } from '@tauri-apps/api/core';
  import GHubDataLocModal from '$components/GHubDataLocModal.svelte';
  import { Gamepad2 } from 'lucide-svelte';

  type PathValidationResult = {
    data_path_exists: boolean;
    applications_json_exists: boolean;
    current_json_exists: boolean;
    version_json_exists: boolean;
    build_id: string | null;
    images_dir_exists: boolean;
  };

  let { children } = $props();
  let showSplash = $state(true);
  let validationResult = $state<PathValidationResult | null>(null);
  let dataPath = $state<string | null>(null);
  let showDataModal = $state(false);
  function openDataModal() { showDataModal = true; }
  function closeDataModal() { showDataModal = false; }

  async function fetchDataPath() {
    const value = await invoke<string | null>('store_get_key', { key: 'lghub_data_path' });
    dataPath = value;
  }

  onMount(() => {
    let splashDone = false;
    let validationDone = false;

    const hideSplashIfReady = () => {
      if (splashDone && validationDone) {
        showSplash = false;
      }
    };

    setTimeout(() => {
      splashDone = true;
      hideSplashIfReady();
    }, 1000);

    invoke<PathValidationResult>('validate_paths').then((result) => {
      validationResult = result;
      validationDone = true;
      if (result.data_path_exists) {
        fetchDataPath();
      } else {
        dataPath = null;
      }
      hideSplashIfReady();
    });
  });
</script>

<div class="w-full h-screen relative bg-background flex flex-col font-sans">
  <TitleBar />
  {#if showSplash}
    <div transition:fade={{ duration: 300 }} class="flex-1 overflow-hidden relative">
      <SplashScreen />
    </div>
  {:else}
    <div transition:fade={{ duration: 300 }} class="flex-1 flex flex-col overflow-hidden">
      <div class="flex-1 flex overflow-hidden">
        <!-- Sidebar Area -->
        <div class="w-12 bg-gray-900 flex flex-col items-center justify-start pt-4">
          <div class="w-8 h-8 flex items-center justify-center">
            <Gamepad2 class="w-7 h-7 text-teal-400" />
          </div>
        </div>
        
        <!-- Main Content Area -->
        <div class="flex-1 flex flex-col overflow-hidden">
          <div class="flex-1 overflow-y-auto relative">
            {@render children?.()}
          </div>
        </div>
      </div>
      <!-- Bottom Bar -->
      <div class="bg-color-background px-4 py-2 text-xs text-white font-light flex justify-between items-center">
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
        <!-- <div class="text-neutral-400">
          {#if validationResult?.build_id}
            Build: {validationResult.build_id}
          {/if}
        </div> -->
      </div>
      <GHubDataLocModal open={showDataModal} onClose={closeDataModal} on:pathChange={fetchDataPath} />
    </div>
  {/if}
</div>
