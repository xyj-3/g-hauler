<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import SplashScreen from '$components/SplashScreen.svelte';
  import TitleBar from '$components/layout/TitleBar.svelte';
  import Sidebar from '$components/layout/Sidebar.svelte';
  import BottomBar from '$components/layout/BottomBar.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { homePageLoaded } from '$lib/stores/appState';
  import { ghub, userPreferences, developerMode } from '$lib/stores';

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
    let minSplashTimeDone = false;
    let validationDone = false;
    let homeLoadDone = false;
    let maxTimeoutReached = false;

    const hideSplashIfReady = () => {
      // Hide splash when all required tasks are done OR max timeout is reached
      if ((minSplashTimeDone && validationDone && homeLoadDone) || maxTimeoutReached) {
        showSplash = false;
      }
    };

    // Minimum splash screen time (1 second)
    setTimeout(() => {
      minSplashTimeDone = true;
      hideSplashIfReady();
    }, 1000);

    // Maximum splash screen time (3 seconds) - feels natural
    setTimeout(() => {
      maxTimeoutReached = true;
      hideSplashIfReady();
    }, 3000);

    // Path validation
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

    // Subscribe to home page loaded state
    const unsubscribe = homePageLoaded.subscribe((loaded) => {
      homeLoadDone = loaded;
      hideSplashIfReady();
    });

    // Initialize stores - ghub.initialize() now handles connection and loading
    Promise.all([
      ghub.initialize(),
      userPreferences.initialize(),
      developerMode.initialize()
    ])
      .then(() => {
        console.log('[Layout] All stores initialized successfully');
      })
      .catch((error) => {
        console.error('[Layout] Failed to initialize stores:', error);
      });

    // Add keyboard shortcut for devtools (F12)
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'F12') {
        event.preventDefault();
        developerMode.toggleDevTools();
      }
    };
    window.addEventListener('keydown', handleKeyDown);

    // Disable context menu when not in developer mode
    const handleContextMenu = (event: MouseEvent) => {
      if (!developerMode.enabled) {
        event.preventDefault();
      }
    };
    window.addEventListener('contextmenu', handleContextMenu);

    return () => {
      unsubscribe();
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('contextmenu', handleContextMenu);
      ghub.destroy();
    };
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
