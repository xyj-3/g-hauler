<script lang="ts">
  import { Gamepad2, Settings } from 'lucide-svelte';
  import { page } from '$app/stores';
  $: currentPath = $page.url.pathname;
</script>

<div class="w-12 flex flex-col items-center justify-center relative h-full">
  <div class="flex flex-col items-center gap-4 py-8 relative">
    <!-- Vertical line positioned to the right of icons -->
    <div class="absolute -right-2 top-0 bottom-0 w-px bg-gray-700"></div>
    <a href="/" class="w-8 h-8 flex items-center justify-center relative group">
      <Gamepad2 class={`w-7 h-7 cursor-pointer ${currentPath === '/' ? 'text-teal-400' : 'text-gray-400 hover:text-teal-400 transition-colors duration-200'}`} />
      <!-- Hover label -->
      <div class="absolute left-full top-0 h-8 w-16 bg-gray-950 text-white text-sm flex items-center justify-center ml-3 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10 before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]">
        Library
      </div>
    </a>
    <!-- Settings icon -->
    <a href="/settings" class="w-8 h-8 flex items-center justify-center relative group">
      <Settings class={`w-6 h-6 cursor-pointer ${currentPath === '/settings' ? 'text-teal-400' : 'text-gray-400 hover:text-teal-400 transition-colors duration-200'}`} />
      <!-- Hover label -->
      <div class="absolute left-full top-0 h-8 w-16 bg-gray-950 text-white text-sm flex items-center justify-center ml-3 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10 before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]">
        Settings
      </div>
    </a>
  </div>
</div>
