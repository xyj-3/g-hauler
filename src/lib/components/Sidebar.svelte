<script lang="ts">
  import { Gamepad2, Settings } from 'lucide-svelte';
</script>

<div class="w-12 bg-gray-900 flex flex-col items-center pt-4 pb-1 relative h-full">
  <div class="w-8 h-8 flex items-center justify-center relative group">
    <Gamepad2 class="w-7 h-7 text-teal-400" />
    <!-- Hover label -->
    <div class="absolute left-full top-0 h-8 w-16 bg-gray-950 text-white text-sm flex items-center justify-center ml-2 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10 before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]">
      Library
    </div>
  </div>
  
  <!-- Spacer to push settings to bottom -->
  <div class="flex-1"></div>
  
  <!-- Settings icon at the bottom -->
  <div class="w-8 h-8 flex items-center justify-center relative group">
    <Settings class="w-6 h-6 text-gray-400 hover:text-teal-400 transition-colors duration-200 cursor-pointer" />
    <!-- Hover label -->
    <div class="absolute left-full top-0 h-8 w-16 bg-gray-950 text-white text-sm flex items-center justify-center ml-2 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10 before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]">
      Settings
    </div>
  </div>
</div>
