<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import SplashScreen from '$components/SplashScreen.svelte';
  import TitleBar from '$components/TitleBar.svelte';
  import '../app.css';
  import { invoke } from '@tauri-apps/api/core';
  import GHubDataLocModal from '$components/GHubDataLocModal.svelte';

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

<div class="w-full h-screen relative bg-neutral-900 flex flex-col">
  <TitleBar />
  {#if showSplash}
    <div transition:fade={{ duration: 300 }} class="flex-1 overflow-hidden relative">
      <SplashScreen />
    </div>
  {:else}
    <div transition:fade={{ duration: 300 }} class="flex-1 overflow-y-auto relative">
      <div class="fixed bottom-3 right-4 z-50 text-xs text-white font-light">
        {#if dataPath && validationResult && validationResult.data_path_exists && validationResult.applications_json_exists && validationResult.current_json_exists && validationResult.version_json_exists && validationResult.build_id && validationResult.images_dir_exists}
          <button type="button" class="cursor-pointer bg-transparent border-0 p-0 m-0 text-left" onclick={openDataModal}>
            G HUB data location: {dataPath}
          </button>
        {:else}
          <button type="button" class="cursor-pointer hover:bg-blue-700/90 transition bg-transparent border-0 p-0 m-0 text-left" onclick={openDataModal}>
            G HUB data location: Click to Select
          </button>
        {/if}
        <GHubDataLocModal open={showDataModal} onClose={closeDataModal} on:pathChange={fetchDataPath} />
      </div>
      {@render children?.()}
    </div>
  {/if}
</div>
