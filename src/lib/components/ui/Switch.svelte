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
  class="w-11 h-6 rounded-full border-none flex items-center p-1 transition-colors duration-300 focus:outline-none shrink-0"
  class:bg-blue-500={checked}
  class:bg-gray-600={!checked}
>
  <span
    class="w-4 h-4 rounded-full bg-white transition-transform duration-300 transform"
    class:translate-x-5={checked}
    class:translate-x-0={!checked}
  ></span>
</button>