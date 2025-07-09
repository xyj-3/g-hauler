<script lang="ts">
  import { Gamepad2, Settings } from 'lucide-svelte';
</script>

<div class="group w-12 hover:w-36 transition-all duration-200 bg-gray-900 flex flex-col items-center hover:items-start justify-center relative h-full overflow-hidden">
  <div class="flex flex-col items-center group-hover:items-start gap-4 py-8 relative w-full">
    <!-- Vertical line positioned to the right of icons, only as tall as the icon group -->
    <div class="absolute -right-2 top-8 bottom-8 w-px bg-gray-700"></div>
    <a class="w-full flex items-center gap-3 pl-2 py-1 cursor-pointer group/link hover:bg-gray-800 rounded transition-colors duration-150">
      <span class="w-8 h-8 flex items-center justify-center">
        <Gamepad2 class="w-7 h-7 text-teal-400" />
      </span>
      <span class="text-white text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-200 whitespace-nowrap">Library</span>
    </a>
    <!-- Settings icon -->
    <a class="w-full flex items-center gap-3 pl-2 py-1 cursor-pointer group/link hover:bg-gray-800 rounded transition-colors duration-150">
      <span class="w-8 h-8 flex items-center justify-center">
        <Settings class="w-6 h-6 text-gray-400 group-hover/link:text-teal-400 transition-colors duration-200" />
      </span>
      <span class="text-white text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-200 whitespace-nowrap">Settings</span>
    </a>
  </div>
</div>
