<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import SplashScreen from '$components/SplashScreen.svelte';
  import TitleBar from '$components/TitleBar.svelte';
  import '../app.css';
  import { invoke } from '@tauri-apps/api/core';

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
      {@render children?.()}
    </div>
  {/if}
</div>
