<script lang="ts">
  import Setting from '$lib/components/Setting.svelte';
</script>

<main class="w-full min-h-full px-6 py-6 text-white">
  <h1 class="text-2xl font-dm-sans mb-6">Settings</h1>

  <div class="space-y-4">
    <Setting
      label="Launch application on startup"
      key="autostart"
      onCommand="enable_auto_start"
      offCommand="disable_auto_start"
    />
  </div>
</main>
