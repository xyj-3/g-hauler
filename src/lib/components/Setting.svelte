<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Switch from './ui/Switch.svelte';

  let { label = '', key = '', onCommand = '', offCommand = '' } = $props();

  let checked = $state(false);
  let loading = $state(true);
  let saving  = $state(false);

  const id = `${key}-setting`;

  $effect(() => {
    invoke<boolean>('store_get_key', { key })
      .then(v => { checked = v ?? false; })
      .catch(e => { console.error(`Failed to load setting "${key}":`, e); })
      .finally(() => { loading = false; });
  });

  async function handleToggle({ checked: next }: { checked: boolean }) {
    if (saving) return;
    saving = true;
    try {
      await invoke('store_set_key', { key, value: next });
      const cmd = next ? onCommand : offCommand;
      if (cmd) await invoke(cmd);
      checked = next;
    } catch (e) {
      console.error(`Failed to apply "${key}" setting:`, e);
    } finally {
      saving = false;
    }
  }
</script>

{#if loading}
  <p>Loading {label}...</p>
{:else}
  <div class="w-full flex items-center justify-between">
    <label for={id} class="text-white font-dm-sans text-base truncate">{label}</label>
    <Switch name={key} checked={checked} disabled={saving} onChange={handleToggle} />
  </div>
{/if}
