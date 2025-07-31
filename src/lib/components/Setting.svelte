<script lang="ts">
  import Switch from './ui/Switch.svelte';

  let { label = '', name = '', checked = false, onChange = () => {} } = $props();
</script>

<div class="w-full flex items-center justify-between">
  <label
    for={`${name}-setting`}
    class="text-white font-dm-sans text-base truncate"
  >
    {label}
  </label>

  <Switch {name} {checked} {onChange} />
</div>
