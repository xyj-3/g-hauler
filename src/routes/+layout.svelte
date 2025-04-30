<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import SplashScreen from '$components/SplashScreen.svelte';
  import TitleBar from '$components/TitleBar.svelte';
  import '../app.css';

  let { children } = $props();
  let showSplash = $state(true);
  let mainContentReady = $state(false);

  onMount(() => {
    mainContentReady = true;
    const timer = setTimeout(() => {
      showSplash = false;
    }, 1000);
    return () => {
      clearTimeout(timer);
    };
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
