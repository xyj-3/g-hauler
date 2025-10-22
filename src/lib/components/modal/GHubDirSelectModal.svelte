<script lang="ts">
  import { open as openDialog } from '@tauri-apps/plugin-dialog';
  import { onMount, createEventDispatcher } from 'svelte';
  import Modal from '$components/modal/Modal.svelte';
  import { invoke } from '@tauri-apps/api/core';

  const STORE_KEY_DATA_PATH = 'lghub_data_path';
  const dispatch = createEventDispatcher();

  let { open, onClose } = $props();
  let lghubPath = $state('');
  let validation = $state<Validation | null>(null);
  let showDetails = $state(false);

  type Validation = {
    data_path_exists: boolean;
    applications_json_exists: boolean;
    current_json_exists: boolean;
    version_json_exists: boolean;
    build_id: string | null;
    images_dir_exists: boolean;
  };

  async function handleClose() {
    try {
      await invoke('store_set_key', { key: STORE_KEY_DATA_PATH, value: lghubPath });
    } catch (e) {
      console.error('Error saving path on close:', e);
    }
    onClose();
  }

  onMount(async () => {
    try {
      const pathValue = await invoke('store_get_key', { key: STORE_KEY_DATA_PATH });
      if (typeof pathValue === 'string') {
        lghubPath = pathValue;
      }
      await validate();
    } catch (e) {
      console.error('Error retrieving stored path:', e);
    }
  });

  async function validate() {
    try {
      await invoke('store_set_key', { key: STORE_KEY_DATA_PATH, value: lghubPath });
      validation = await invoke('validate_paths');
      dispatch('pathChange');
    } catch (e) {
      validation = null;
    }
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

<Modal {open} onClose={handleClose}>
  <div class="flex flex-col gap-4">
    <div class="text-xl font-semibold text-white">G HUB Data Location</div>
    <div class="text-base text-neutral-200">Please select the G HUB data folder.</div>
    <div>Normally <code>C:\ProgramData\LGHUB</code> on Windows.</div>
    <div class="flex gap-2 items-center">
      <input
        class="flex-1 px-2 py-1 rounded bg-neutral-800 text-white border border-neutral-600"
        type="text"
        bind:value={lghubPath}
        placeholder="Enter or select G HUB path"
      />
      <button
        class="px-3 py-1 rounded bg-btn text-white hover:bg-btn-hover"
        type="button"
        onclick={handleBrowse}
      >Browse</button>
      {#if validation && validation.data_path_exists && validation.applications_json_exists && validation.current_json_exists && validation.version_json_exists && validation.images_dir_exists && validation.build_id !== null}
        <button
          class="text-green-500 flex items-center gap-1"
          onclick={() => showDetails = !showDetails}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
          </svg>
          Verified
        </button>
      {:else if validation !== null}
        <button 
          class="text-red-500 flex items-center gap-1"
          onclick={() => showDetails = !showDetails}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
          Invalid
        </button>
      {/if}
    </div>
    {#if validation && showDetails}
      <div class="text-sm text-white rounded p-2 transition-all duration-200">
        <p>Target files:</p>
        <div>applications.json: {validation.applications_json_exists ? '✅' : '❌'}</div>
        <div>current.json: {validation.current_json_exists ? '✅' : '❌'}</div>
        <div>version.json: {validation.version_json_exists ? '✅' : '❌'}</div>
        <div>build id: {validation.build_id ? `${validation.build_id} ✅` : '❌'}</div>
        <div>images directory: {validation.images_dir_exists ? '✅' : '❌'}</div>
      </div>
    {/if}
  </div>
</Modal>
