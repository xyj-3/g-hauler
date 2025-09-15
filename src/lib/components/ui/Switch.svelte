<script lang="ts">
  type ChangePayload = { checked: boolean };
  const noop = (_: ChangePayload) => {};

  let {
    name,
    checked,
    disabled = false,
    onChange = noop
  } = $props<{
    name: string;
    checked: boolean;
    disabled?: boolean;
    onChange?: (p: ChangePayload) => void;
  }>();

  const id = `${name}-setting`;

  function toggle() {
    if (!disabled) onChange({ checked: !checked });
  }
</script>

<button
  id={id}
  name={name}
  role="switch"
  aria-checked={checked}
  aria-labelledby={id}
  aria-disabled={disabled}
  disabled={disabled}
  onclick={toggle}
  class="w-11 h-6 rounded-full border-none flex items-center p-1 transition-colors duration-300 focus:outline-none shrink-0
         data-[on=true]:bg-blue-500 data-[on=false]:bg-gray-600 disabled:opacity-60"
  data-on={checked}
>
  <span
    class="w-4 h-4 rounded-full bg-white transition-transform duration-300"
    class:translate-x-5={checked}
    class:translate-x-0={!checked}
  ></span>
</button>
