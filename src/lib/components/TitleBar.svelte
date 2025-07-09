<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  
  const appWindow = getCurrentWindow();

  function minimize() {
    appWindow.minimize();
  }
  function maximize() {
    appWindow.toggleMaximize();
  }
  function close() {
    appWindow.close();
  }
</script>

<div data-tauri-drag-region class="flex items-center justify-center w-full h-8 bg-gray-900 relative">
  <h1 data-tauri-drag-region class="text-sm font-medium text-teal cursor-default font-dm-sans">G Hauler</h1>
  
  <div class="absolute right-0 top-0 h-full flex items-center gap-1">
    <button aria-label="Minimize" class="w-10 h-full flex items-center justify-center hover:bg-white/10 transition" onclick={minimize}>
      <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" viewBox="0 0 24 24"><line x1="6" y1="12" x2="18" y2="12" /></svg>
    </button>
    <button aria-label="Maximize" class="w-10 h-full flex items-center justify-center hover:bg-white/10 transition" onclick={maximize}>
      <svg class="w-3.5 h-3.5 text-white" fill="none" stroke="currentColor" stroke-width="2" rx="2" viewBox="0 0 24 24"><rect x="5" y="5" width="14" height="14" rx="3" /></svg>
    </button>
    <button aria-label="Close" class="w-10 h-full flex items-center justify-center hover:bg-red-600/80 transition" onclick={close}>
      <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="6" y1="6" x2="18" y2="18" /><line x1="6" y1="18" x2="18" y2="6" /></svg>
    </button>
  </div>
</div>
