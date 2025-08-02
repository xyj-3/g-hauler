<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Setting from '$lib/components/Setting.svelte';

  let autoStart = $state(false);
  let isLoading = $state(true);

  $effect(() => {
    invoke<boolean>('store_get_key', { key: 'autostart' })
      .then(enabled => autoStart = enabled ?? false)
      .catch(err => console.error('Failed to load autostart from store:', err))
      .finally(() => isLoading = false);
  });

  async function handleAutoStartToggle({ checked }: { checked: boolean }) {
    try {
      await invoke('store_set_key', {
        key: 'autostart',
        value: checked
      });

      await invoke(checked ? 'enable_auto_start' : 'disable_auto_start');

      autoStart = checked;

    } catch (err) {
      console.error('Failed to update autostart setting:', err);
    }
  }
</script>

<main class="w-full min-h-full px-6 py-6 text-white">
  <div class="w-full">
    <h1 class="text-2xl font-dm-sans mb-6">Settings</h1>

    {#if isLoading}
      <p>Loading startup settings...</p>
    {:else}
      <Setting
        label="Launch application on startup"
        name="autostart"
        checked={autoStart}
        onChange={handleAutoStartToggle}
      />
    {/if}
  </div>
</main>
