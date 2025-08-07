<script lang="ts">
  import Switch from './ui/Switch.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let { label = '', key = '', checked = false, onChange = () => {} } = $props();

  let isLoading = $state(true);

  $effect(() => {
    invoke<boolean>('store_get_key', { key })
      .then(value => {
        if (value !== null && value !== undefined) checked = value;
      })
      .catch(err => console.error(`Failed to load setting "${key}" from store:`, err))
      .finally(() => isLoading = false);
  });

  const id = `${key}-setting`;
</script>

{#if isLoading}
  <p>Loading {label}...</p>
{:else}
  <div class="w-full flex items-center justify-between">
    <label for={id} class="text-white font-dm-sans text-base truncate">
      {label}
    </label>
    <Switch name={key} {checked} {onChange} />
  </div>
{/if}
