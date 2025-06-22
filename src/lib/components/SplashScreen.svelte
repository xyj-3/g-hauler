<script lang="ts">
  import { onMount } from 'svelte';
  
  let logoOpacity = $state(0);
  let logoScale = $state(0.9);
  
  let logoStyle = $derived(`opacity: ${logoOpacity}; transform: scale(${logoScale})`);
  
  onMount(() => {
    setTimeout(() => {
      logoOpacity = 1;
      logoScale = 1;
    }, 100);
  });
</script>

<div class="absolute inset-0 flex justify-center items-center bg-[#1a1a1a] z-[9999]">
  <div class="flex flex-col justify-center items-center gap-5">
    <div class="text-2xl font-bold text-white mb-2.5 tracking-wider">G-Hauler</div>
    <img 
      src="/tauri.svg" 
      alt="Logo" 
      class="w-[120px] h-[120px] transition-all duration-500 ease-out" 
      style={logoStyle}
    />
  </div>
</div>
