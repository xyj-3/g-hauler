<script lang="ts">
  import { Gamepad2, Truck, Settings, Wrench } from 'lucide-svelte';
  import { page } from '$app/state';

  interface NavItem {
    path: string;
    label: string;
    icon: any; // Lucide icon component
    iconSize: string;
    iconStroke: string;
    labelWidth: string;
  }

  const navItems: NavItem[] = [
    {
      path: '/',
      label: 'Library',
      icon: Gamepad2,
      iconSize: 'w-7 h-7',
      iconStroke: 'stroke-[1.45]',
      labelWidth: 'w-16'
    },
    {
      path: '/detection',
      label: 'Fix Game Detection',
      icon: Wrench,
      iconSize: 'w-6.5 h-6.5',
      iconStroke: 'stroke-[1.5]',
      labelWidth: 'w-44'
    },
    {
      path: '/add-games',
      label: 'Add Games to G HUB',
      icon: Truck,
      iconSize: 'w-6.5 h-6.5',
      iconStroke: 'stroke-[1.5]',
      labelWidth: 'w-38'
    },
    {
      path: '/settings',
      label: 'Settings',
      icon: Settings,
      iconSize: 'w-6 h-6',
      iconStroke: 'stroke-[1.65]',
      labelWidth: 'w-17'
    }
  ];

  const currentPath = $derived(page.url.pathname);

  const isActive = (path: string) => currentPath === path;

  // Shared classes
  const linkClasses = 'w-8 h-8 flex items-center justify-center relative group';
  const iconBaseClasses = 'cursor-pointer transition-colors duration-200';
  const labelBaseClasses = 'absolute left-full top-0 h-8 bg-gray-950 text-white text-sm flex items-center justify-center ml-3 rounded border border-gray-600 opacity-0 group-hover:opacity-100 transition-all duration-200 ease-in-out transform translate-x-[-10px] group-hover:translate-x-0 pointer-events-none whitespace-nowrap z-10';
  const labelArrowClasses = "before:content-[''] before:absolute before:right-full before:top-1/2 before:transform before:-translate-y-1/2 before:border-[5px] before:border-transparent before:border-r-gray-600 after:content-[''] after:absolute after:right-full after:top-1/2 after:transform after:-translate-y-1/2 after:border-[4px] after:border-transparent after:border-r-gray-950 after:translate-x-[1px]";
</script>

<div class="w-14 flex flex-col items-center justify-center relative h-full">
  <div class="flex flex-col items-center gap-6 h-full w-full justify-center relative">
    {#each navItems as item}
      {@const Icon = item.icon}
      <a href={item.path} class={linkClasses}>
        <Icon
          class={`${item.iconSize} ${iconBaseClasses} ${item.iconStroke} ${isActive(item.path) ? 'text-teal' : 'text-gray-400 hover:text-teal'}`}
        />
        <div class="{labelBaseClasses} {item.labelWidth} {labelArrowClasses}">
          {item.label}
        </div>
      </a>
    {/each}
    <div class="absolute right-0 top-1/2 -translate-y-1/2 h-56 w-px bg-gray-700"></div>
  </div>
</div>
