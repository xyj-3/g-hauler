<script lang="ts">
  let { checked = false, name = '', onChange = () => {} } = $props();
  const id = `${name}-setting`;

  function toggle() {
    onChange({ checked: !checked });
  }
</script>

<button
  id={id}
  name={name}
  role="switch"
  aria-checked={checked}
  aria-labelledby={id}
  onclick={toggle}
  class={`w-[44px] h-[24px] rounded-full border-none flex items-center 
          p-[4px] transition-colors duration-300 focus:outline-none shrink-0`}
  class:bg-blue-500={checked}
  class:bg-gray-600={!checked}
>
  <span
    class="w-[16px] h-[16px] rounded-full bg-white transition-transform duration-300"
    style="transform: translateX({checked ? '20px' : '0'})"
  ></span>
</button>
