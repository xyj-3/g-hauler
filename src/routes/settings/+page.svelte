<script lang="ts">
  import Setting from '$lib/components/Setting.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let autoStart = $state(false);
  let savingAutoStart = $state(false);

  function handleAutoStartLoaded({ value }: { value: boolean }) {
    autoStart = value;
  }

  async function handleAutoStartToggle({ checked }: { checked: boolean }) {
    if (savingAutoStart) return;
    savingAutoStart = true;

    try {
      await invoke('store_set_key', { key: 'autostart', value: checked });
      await invoke(checked ? 'enable_auto_start' : 'disable_auto_start');
      autoStart = checked;
    } catch (err) {
      console.error('Failed to update autostart setting:', err);
    } finally {
      savingAutoStart = false;
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
      disabled={savingAutoStart}
      onLoaded={handleAutoStartLoaded}
      onChange={handleAutoStartToggle}
    />
  </div>
</main>
