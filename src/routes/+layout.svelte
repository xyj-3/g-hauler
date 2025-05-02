<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import SplashScreen from '$components/SplashScreen.svelte';
  import TitleBar from '$components/TitleBar.svelte';
  import '../app.css';
  import { invoke } from '@tauri-apps/api/core';
  import SelectGHubLoc from '$components/SelectGHubLocPopup.svelte';

  type PathValidationResult = {
    install_path_exists: boolean;
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
  let installPath = $state<string | null>(null);
  let showInstallModal = $state(false);
  function openInstallModal() { showInstallModal = true; }
  function closeInstallModal() { showInstallModal = false; }

  async function fetchInstallPath() {
    const value = await invoke<string | null>('store_get_key', { key: 'lghub_install_path' });
    installPath = value;
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
      if (result.install_path_exists) {
        fetchInstallPath();
      } else {
        installPath = null;
      }
      hideSplashIfReady();
    });
  });
</script>

<div class="w-full h-screen overflow-hidden relative bg-neutral-900">
  <TitleBar />
  {#if showSplash}
    <div transition:fade={{ duration: 300 }}>
      <SplashScreen />
    </div>
  {:else}
    <div transition:fade={{ duration: 300 }}>
      <div class="fixed bottom-3 right-4 z-50 text-xs text-white font-light">
        {#if validationResult && validationResult.install_path_exists && installPath}
          <button type="button" class="cursor-pointer bg-transparent border-0 p-0 m-0 text-left" onclick={openInstallModal}>
            G HUB install location: {installPath}
          </button>
        {:else}
          <button type="button" class="cursor-pointer hover:bg-blue-700/90 transition bg-transparent border-0 p-0 m-0 text-left" onclick={openInstallModal}>
            G HUB install location: Click to Select
          </button>
        {/if}
        <SelectGHubLoc open={showInstallModal} onClose={closeInstallModal} />
      </div>
      {@render children?.()}
    </div>
  {/if}
</div>
