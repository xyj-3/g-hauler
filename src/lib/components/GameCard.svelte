<script lang="ts">
  import type { GHUBApp } from '$lib/types';
  import { invoke } from '@tauri-apps/api/core';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { onMount } from 'svelte';

  interface GameCardProps {
    game: GHUBApp;
    tabindex?: number;
  }

  const { game, tabindex }: GameCardProps = $props();

  let resolvedPosterUrl = $state(game.poster_url);

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

  const resolvePipelineUrl = async (url: string): Promise<string> => {
    if (!url.startsWith('pipeline://')) {
      return url;
    }
    
    try {
      const relativePath = url.replace('pipeline://', '');
      const pipelinePath = await invoke<string | null>('get_pipeline_path');
      if (!pipelinePath) {
        throw new Error('Pipeline path not found');
      }
      const fullPath = `${pipelinePath}/${relativePath}`.replace(/\\/g, '/');

      const fileData = await readFile(fullPath);
      const blob = new Blob([fileData]);
      const dataUrl = await new Promise<string>((resolve) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result as string);
        reader.readAsDataURL(blob);
      });
      
      return dataUrl;
    } catch (error) {
      console.error('Failed to resolve pipeline image:', error);
    }
    return url;
  };

  onMount(async () => {
    if (game.poster_url.startsWith('pipeline://')) {
      try {
        resolvedPosterUrl = await resolvePipelineUrl(game.poster_url);
        console.log('Resolved poster URL:', resolvedPosterUrl);
      } catch (error) {
        console.error('Error resolving pipeline URL:', error);
      }
    }
  });
</script>

<div
  class="cursor-pointer group transition-transform duration-300 hover:scale-102 w-40"
  onclick={handleCardClick}
  onkeydown={handleKeyDown}
  tabindex={tabindex}
  role="button"
  aria-label="Select {game.name}"
><div class="relative aspect-[3/4] overflow-hidden rounded-md">
        <img
            src={resolvedPosterUrl}
            alt="{game.name} poster"
            class="w-full h-full object-cover"
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