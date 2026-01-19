<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Switch from './ui/Switch.svelte';

  // Types (keep in sync with your backend payloads)
  type SelectOption = { value: string; label: string; description?: string };
  type SettingType =
    | { type: 'toggle' }
    | { type: 'text'; placeholder?: string; validation?: { pattern?: string; min_length?: number; max_length?: number } }
    | { type: 'number'; min?: number; max?: number; step?: number; unit?: string }
    | { type: 'select'; options: SelectOption[] }
    | { type: 'path'; directory: boolean; extensions?: string[] }
    | { type: 'color' }
    | { type: 'keybind' };

  type Setting = {
    key: string;
    label: string;
    description?: string;
    category: string;
    default_value: any;
    setting_type: SettingType;
    requires_restart: boolean;
    system_managed: boolean;
  };

  type SettingItemState = {
    key: string;
    user_value: any;
    effective_value: any;
    in_sync: boolean;
    capable: boolean;
    error?: string | null;
  };

  let {
    item,
    itemState,
    onStatePatched = (_: SettingItemState[]) => {}
  } = $props<{
    item: Setting;
    itemState: SettingItemState | undefined;
    onStatePatched?: (items: SettingItemState[]) => void;
  }>();

  let saving = $state(false);
  let textValue = $state('');

  function currentToggle(): boolean {
    // Backend guarantees booleans for toggle user_value/default_value
    const v = itemState?.user_value ?? item.default_value;
    return !!v;
  }

  function currentText(): string {
    const v = itemState?.user_value ?? item.default_value;
    return String(v || '');
  }

  $effect(() => {
    if (item.setting_type.type === 'text') {
      textValue = currentText();
    }
  });

  async function applyValue(value: any) {
    if (saving) return;
    saving = true;
    try {
      const next = await invoke<SettingItemState[]>('settings_set_and_apply', {
        key: item.key,
        value
      });
      onStatePatched(next); // replace full state in parent
    } catch (e) {
      console.error(`Failed to set "${item.key}"`, e);
    } finally {
      saving = false;
    }
  }
</script>

<div class="w-full">
  <div class="flex items-center justify-between gap-4">
    <div class="min-w-0">
      <label for={`${item.key}-setting`} class="text-white font-dm-sans text-base truncate">
        {item.label}
      </label>
      {#if item.description}
        <div class="text-xs text-gray-300 mt-1 truncate">{item.description}</div>
      {/if}
      {#if itemState && !itemState.in_sync}
        <div class="text-[10px] text-yellow-400 mt-1">Not in sync with system</div>
      {/if}
      {#if itemState && itemState.error}
        <div class="text-[10px] text-red-400 mt-1">{itemState.error}</div>
      {/if}
    </div>

    {#if item.setting_type.type === 'toggle'}
      <Switch
        name={item.key}
        checked={currentToggle()}
        disabled={saving || (itemState && !itemState.capable)}
        onChange={({ checked }) => applyValue(checked)}
      />
    {:else if item.setting_type.type === 'text'}
      <input
        type="text"
        id={`${item.key}-setting`}
        class="px-3 py-2 bg-[#1a1a1a] border border-gray-600 rounded-md text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-[300px]"
        placeholder={item.setting_type.placeholder || ''}
        bind:value={textValue}
        disabled={saving || (itemState && !itemState.capable)}
        onblur={() => {
          if (textValue !== currentText()) {
            applyValue(textValue);
          }
        }}
        onkeydown={(e) => {
          if (e.key === 'Enter' && textValue !== currentText()) {
            applyValue(textValue);
          }
        }}
      />
    {:else}
      <div class="text-sm text-gray-400">Unsupported type: {item.setting_type.type}</div>
    {/if}
  </div>
  <div class="h-4"></div>
</div>
