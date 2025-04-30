<script lang="ts">
  import Modal from './Modal.svelte';
  export let open: boolean;
  export let onClose: () => void;
</script>

<Modal {open} {onClose}>
  <div class="flex flex-col gap-8">
    <div class="text-xl font-semibold text-white">G HUB Install Location</div>
    <div class="text-base text-neutral-200">Placeholder text.</div>
  </div>
</Modal>
