<script lang="ts">
  import Modal from '$components/modal/Modal.svelte';
  let { open = false, onClose = () => {} } = $props();
</script>

<Modal {open} {onClose} showCloseButton={true} maxWidth="max-w-md">
  <h2 class="text-lg font-semibold mb-2">About G Hauler</h2>
  <p class="mb-4 text-sm text-gray-200">
    G Hauler is a tool for managing and customizing your Logitech G HUB data. For more information, visit the GitHub repository or join our Discord community.
  </p>
</Modal>
