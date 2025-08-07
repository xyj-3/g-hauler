<script lang="ts">
  import Setting from '$lib/components/Setting.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let autoStart = $state(false);

  async function handleAutoStartToggle({ checked }: { checked: boolean }) {
    try {
      await invoke('store_set_key', { key: 'autostart', value: checked });

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

    <Setting
      label="Launch application on startup"
      key="autostart"
      checked={autoStart}
      onChange={handleAutoStartToggle}
    />
  </div>
</main>