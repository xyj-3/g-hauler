<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import SettingRow from '$lib/components/Setting.svelte';

  type Setting = any;
  type SettingItemState = {
    key: string;
    user_value: any;
    effective_value: any;
    in_sync: boolean;
    capable: boolean;
    error?: string | null;
  };

  let loading = $state(true);
  let registry = $state<Setting[]>([]);
  let items = $state<SettingItemState[]>([]);

  let byKey: Map<string, SettingItemState> = $derived.by(() => {
    const m = new Map<string, SettingItemState>();
    for (const it of items) m.set(it.key, it);
    return m;
  });

  $effect(() => {
    (async () => {
      try {
        const [reg, st] = await Promise.all([
          invoke<Setting[]>('settings_get_registry'),
          invoke<SettingItemState[]>('settings_get_state')
        ]);
        registry = reg;
        items = st;
      } catch (e) {
        console.error('Failed to load settings', e);
      } finally {
        loading = false;
      }
    })();
  });

  function patchState(next: SettingItemState[]) {
    items = next;
  }
</script>

<main class="w-full min-h-full px-6 py-4 text-white">
  <h1 class="text-xl font-dm-sans mb-1">Settings</h1>

  {#if loading}
    <p>Loading settings...</p>
  {:else if registry.length === 0}
    <p class="text-gray-300">No settings available.</p>
  {:else}
    <div class="space-y-3">
      {#each registry as item (item.key)}
        <SettingRow item={item} itemState={byKey.get(item.key)} onStatePatched={patchState} />
      {/each}
    </div>
  {/if}
</main>
