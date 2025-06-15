<script lang="ts">
  import type { GHUBApp } from '$lib/types';

  interface GameCardProps {
    game: GHUBApp;
    tabindex?: number;
  }

  const { game, tabindex }: GameCardProps = $props();

  const handleCardClick = () => {
    console.log('Game selected:', game.name);
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleCardClick();
    }
  };

  const handleImageError = (event: Event) => {
    const img = event.target as HTMLImageElement;
    img.src = '/favicon.png';
  };
</script>

<div
  class="cursor-pointer group"
  onclick={handleCardClick}
  onkeydown={handleKeyDown}
  tabindex={tabindex}
  role="button"
  aria-label="Select {game.name}"
>
    <div class="relative aspect-[3/4] overflow-hidden rounded-lg transition-transform duration-300 group-hover:scale-105">
        <img
            src={game.poster_url}
            alt="{game.name} poster"
            class="w-full h-full object-cover rounded-lg"
            onerror={handleImageError}
            loading="lazy"
        />
    </div>
    <div class="mt-2">
        <h3 class="text-white font-medium text-sm leading-tight truncate" title={game.name}>
            {game.name}
        </h3>
    </div>
</div>