<script lang="ts">
  import { Gamepad2, Truck, Settings, Wrench } from 'lucide-svelte';
  import { page } from '$app/state';
  const currentPath = $derived(page.url.pathname);
</script>

<div class="w-14 flex flex-col items-center justify-center relative h-full">
  <div class="flex flex-col items-center gap-6 h-full w-full justify-center relative">
    <!-- Game Library -->
    <a href="/" class="w-8 h-8 flex items-center justify-center relative group">
      <Gamepad2 class={`w-7 h-7 cursor-pointer stroke-[1.45] ${currentPath === '/' ? 'text-teal' : 'text-gray-400 hover:text-teal transition-colors duration-200'}`} />
      <!-- Hover label -->
      <div class="absolute left-full top-0 h-8 w-16 bg-gray-950 text-white text-sm flex items-center justify-center ml-3 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10 before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]">
        Library
      </div>
    </a>
    <!-- Game Detection Patching -->
    <a href="/detection" class="w-8 h-8 flex items-center justify-center relative group">
      <Wrench class={`w-6.5 h-6.5 cursor-pointer stroke-[1.5] ${currentPath === '/detection' ? 'text-teal' : 'text-gray-400 hover:text-teal transition-colors duration-200'}`} />
      <!-- Hover label -->
      <div class="absolute left-full top-0 h-8 w-44 bg-gray-950 text-white text-sm flex items-center justify-center ml-3 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10 before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]">
        Game Detection Patching
      </div>
    </a>
    <!-- Add Games -->
     <a href="/add_games" class="w-8 h-8 flex items-center justify-center relative group">
      <Truck class={`w-6.5 h-6.5 cursor-pointer stroke-[1.5] ${currentPath === '/add_games' ? 'text-teal' : 'text-gray-400 hover:text-teal transition-colors duration-200'}`} />
      <!-- Hover label -->
      <div class="absolute left-full top-0 h-8 w-38 bg-gray-950 text-white text-sm flex items-center justify-center ml-3 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10 before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]">
        Add Games to G HUB
      </div>
    </a>
    <!-- Settings -->
    <a href="/settings" class="w-8 h-8 flex items-center justify-center relative group">
      <Settings class={`w-6 h-6 cursor-pointer stroke-[1.65] ${currentPath === '/settings' ? 'text-teal' : 'text-gray-400 hover:text-teal-400 transition-colors duration-200'}`} />
      <!-- Hover label -->
      <div class="absolute left-full top-0 h-8 w-17 bg-gray-950 text-white text-sm flex items-center justify-center ml-3 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10 before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]">
        Settings
      </div>
    </a>
    <!-- Vertical divider -->
    <div class="absolute right-0 top-1/2 -translate-y-1/2 h-56 w-px bg-gray-700"></div>
  </div>
</div>
