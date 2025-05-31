<script lang="ts">
  import { open as openDialog } from '@tauri-apps/plugin-dialog';
  import { onMount, createEventDispatcher } from 'svelte';
  const STORE_KEY_DATA_PATH = 'lghub_data_path';
  import Modal from './Popup.svelte';
  import { invoke } from '@tauri-apps/api/core';

  const dispatch = createEventDispatcher();
  export let open: boolean;
  export let onClose: () => void;
  let lghubPath: string = '';
  let validation: any = null;
  let validating = false;

  onMount(async () => {
    try {
      let pathValue = await invoke('store_get_key', { key: STORE_KEY_DATA_PATH });
	  if (typeof pathValue === 'string') {
		lghubPath = pathValue;
	  }
      await validate();
    } catch (e) {
      console.error('Error retrieving stored path:', e);
    }
  });

  async function validate() {
    validating = true;
    try {
      await invoke('store_set_key', { key: STORE_KEY_DATA_PATH, value: lghubPath });
      validation = await invoke('validate_paths');
	  dispatch('pathChange');
    } catch (e) {
      validation = null;
    }
    validating = false;
  }

  async function handleBrowse() {
    const selected = await openDialog({
      directory: true,
      multiple: false,
      title: 'Select G HUB Folder'
    });
    if (typeof selected === 'string') {
      lghubPath = selected;
      await validate();
    }
  }
</script>

<Modal {open} {onClose}>
  <div class="flex flex-col gap-8">
    <div class="text-xl font-semibold text-white">G HUB Data Location</div>
    <div class="text-base text-neutral-200">Please select the G HUB data folder.</div>
    <div class="flex gap-2 items-center">
      <input
        class="flex-1 px-2 py-1 rounded bg-neutral-800 text-white border border-neutral-600"
        type="text"
        bind:value={lghubPath}
        placeholder="Enter or select G HUB path"
      />
      <button
        class="px-3 py-1 rounded bg-blue-600 text-white hover:bg-blue-700"
        type="button"
        on:click={handleBrowse}
      >Browse</button>
    </div>
    {#if validation}
      <div class="text-sm text-white bg-neutral-900 rounded p-2 mt-2">
        <div>Data Path Exists: {validation.data_path_exists ? '✅' : '❌'}</div>
        <div>applications.json: {validation.applications_json_exists ? '✅' : '❌'}</div>
        <div>current.json: {validation.current_json_exists ? '✅' : '❌'}</div>
        <div>version.json: {validation.version_json_exists ? '✅' : '❌'}</div>
        <div>buildId: {validation.build_id ?? 'None'}</div>
        <div>images Directory: {validation.images_dir_exists ? '✅' : '❌'}</div>
      </div>
    {/if}
  </div>
</Modal>
