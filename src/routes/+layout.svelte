<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import SplashScreen from '$lib/components/SplashScreen.svelte';
  import '../app.css';

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

<div class="w-full h-screen overflow-hidden relative bg-[#1a1a1a]">
  {#if showSplash}
    <div transition:fade={{ duration: 300 }}>
      <SplashScreen />
    </div>
  {:else}
    <div transition:fade={{ duration: 300 }}>
      <slot />
    </div>
  {/if}
</div>
