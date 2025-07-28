<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Switch from '$components/ui/Switch.svelte';

  let autoStart = false;
  let isLoading = true;

  onMount(async () => {
    try {
      autoStart = await invoke<boolean>('is_auto_start_enabled');
    } catch (error) {
      console.error('Failed to fetch autostart state:', error);
    } finally {
      isLoading = false;
    }
  });

  async function handleAutoStartToggle(event: CustomEvent<{ checked: boolean }>) {
    const enabled = event.detail.checked;
    try {
      if (enabled) {
        await invoke('enable_auto_start');
      } else {
        await invoke('disable_auto_start');
      }
    } catch (error) {
      console.error('Failed to update autostart state:', error);
    }
  }
</script>

<main class="w-full min-h-full px-6 py-6 text-white">
  <div class="w-full">
    <h1 class="text-2xl font-semibold mb-6">Settings</h1>

    {#if isLoading}
      <p>Loading startup settings...</p>
    {:else}
      <Switch
        label="Launch application on startup"
        checked={autoStart}
        on:change={handleAutoStartToggle}
      />
    {/if}
  </div>
</main>
