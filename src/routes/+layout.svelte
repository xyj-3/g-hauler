<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import SplashScreen from '$components/SplashScreen.svelte';
  import TitleBar from '$components/layout/TitleBar.svelte';
  import Sidebar from '$components/layout/Sidebar.svelte';
  import BottomBar from '$components/layout/BottomBar.svelte';
  import { invoke } from '@tauri-apps/api/core';

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

<div class="w-full h-screen relative bg-background flex flex-col font-sans text-white">
  {#if showSplash}
    <div transition:fade={{ duration: 300 }} class="flex-1 overflow-hidden relative">
      <SplashScreen />
    </div>
  {:else}
    <div transition:fade={{ duration: 300 }} class="flex-1 flex flex-col overflow-hidden">
      <TitleBar />
      <div class="flex-1 flex overflow-hidden">
        <!-- Sidebar Area -->
        <Sidebar />
        
        <!-- Main Content Area -->
        <div class="flex-1 flex flex-col overflow-hidden">
          <div class="flex-1 overflow-y-auto relative">
            {@render children?.()}
          </div>
        </div>
      </div>
      <!-- Bottom Bar -->
      <BottomBar
        {dataPath}
        {validationResult}
        {showDataModal}
        {openDataModal}
        {closeDataModal}
        {fetchDataPath}
      />
    </div>
  {/if}
</div>
