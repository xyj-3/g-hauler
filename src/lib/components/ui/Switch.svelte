<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let label: string = '';
  export let checked: boolean = false;
  export let fontSize: number = 16;

  const dispatch = createEventDispatcher();
  let switchId = `switch-${Math.floor(Math.random() * 1_000_000)}`;

  function toggle() {
    checked = !checked;
    dispatch('change', { checked });
  }
</script>

<div class="w-full flex items-center justify-between">
  <div class="flex-1 min-w-0">
    <label
      for={switchId}
      class="text-white block truncate"
      style="font-family: var(--font-dm-sans); font-size: {fontSize}px;"
    >
      {label}
    </label>
  </div>

  <button
    id={switchId}
    role="switch"
    aria-checked={checked}
    aria-labelledby={switchId}
    on:click={toggle}
    class={`w-[44px] h-[24px] rounded-full border-none flex items-center 
            p-[4px] transition-colors duration-300 focus:outline-none shrink-0 mr-4`}
    class:bg-[var(--color-teal)]={checked}
    class:bg-gray-600={!checked}
  >
    <span
      class="w-[16px] h-[16px] rounded-full bg-white transition-transform duration-300"
      style="transform: translateX({checked ? '20px' : '0'})"
    ></span>
  </button>
</div>
