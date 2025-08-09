<script lang="ts">
  import Switch from './ui/Switch.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let {
    label = '',
    key = '',
    checked = false,
    disabled = false,
    onChange = () => {},
    onLoaded = () => {}
  } = $props();

  let isLoading = $state(true);
  const id = `${key}-setting`;

  $effect(() => {
    invoke<boolean>('store_get_key', { key })
      .then((value) => onLoaded({ value: value ?? false }))
      .catch((err) => console.error(`Failed to load setting "${key}" from store:`, err))
      .finally(() => isLoading = false);
  });
</script>

{#if isLoading}
  <p>Loading {label}...</p>
{:else}
  <div class="w-full flex items-center justify-between">
    <label for={id} class="text-white font-dm-sans text-base truncate">
      {label}
    </label>
    <Switch name={key} {checked} {disabled} {onChange} />
  </div>
{/if}
