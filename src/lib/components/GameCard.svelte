<script lang="ts">
  import type { GHUBApp } from '$lib/types';
  import { invoke } from '@tauri-apps/api/core';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { onMount } from 'svelte';
  import GameEditModal from '$components/modal/GameEditModal.svelte';
  
  interface GameCardProps {
    game: GHUBApp;
    tabindex?: number;
    ongameUpdated?: (game: GHUBApp) => void;
  }

  const { game, tabindex, ongameUpdated }: GameCardProps = $props();
  
  let cardElement = $state<HTMLDivElement>();
  let isVisible = $state(false);
  let imageLoaded = $state(false);
  let resolvedPosterUrl = $state<string | null>(null);
  let observer: IntersectionObserver;
  let showEditModal = $state(false);

  const handleCardClick = () => {
    console.log('Game selected:', game.name);
    showEditModal = true;
  };

  const handleModalClose = () => {
    showEditModal = false;
  };
  
  const handleGameSave = async (updatedGame: GHUBApp) => {
    try {
      // Update the application in the backend memory
      await invoke('update_application', { updatedApp: updatedGame });
      console.log('Application updated successfully in backend memory');
      
      // Save to disk
      await invoke('save_applications_to_disk');
      console.log('Applications saved to disk successfully');
      
      // Update the frontend state
      ongameUpdated?.(updatedGame);
      showEditModal = false;
    } catch (error) {
      console.error('Failed to update application:', error);
      // Rethrow the error so the modal can handle it
      throw error;
    }
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleCardClick();
    }
  };

  const handleImageLoad = () => {
    imageLoaded = true;
  };

  const handleImageError = () => {
    imageLoaded = false;
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

  const loadContent = async () => {
    if (game.posterUrl.startsWith('pipeline://')) {
      try {
        resolvedPosterUrl = await resolvePipelineUrl(game.posterUrl);
        } catch (error) {
          console.error('Error resolving pipeline URL:', error);
          resolvedPosterUrl = null;
      }
    }
  };
  
  onMount(() => {
    if (!cardElement) return;
    
    observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          // console.log(entry.intersectionRatio.toFixed(2));
          // console.log(entry.isIntersecting);
          if (entry.isIntersecting) {
            isVisible = true;
            loadContent();
            observer.unobserve(entry.target);
          }
        });
      },
      {
        rootMargin: '300px',
        threshold: 0,
      }
    );
    
    observer.observe(cardElement);
    
    return () => {
      if (observer && cardElement) {
        observer.unobserve(cardElement);
      }
    };
  });
</script>

<button
  bind:this={cardElement}
  class="cursor-pointer group transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-900 rounded-lg"
  onclick={handleCardClick}
  onkeydown={handleKeyDown}
  tabindex={tabindex}
  aria-label="Select {game.name}"
>
  <!-- Minimal border highlight -->
  <div class="relative aspect-[3/4] overflow-hidden rounded-lg transition-all duration-200 border-2 border-gray-700/50 group-hover:border-blue-400/60 shadow-md group-hover:shadow-lg">
    <div class="absolute inset-0 bg-gradient-to-br from-gray-700 to-gray-800"></div>
    {#if isVisible && resolvedPosterUrl}
      <img
        src={resolvedPosterUrl}
        alt="{game.name} poster"
        class="absolute inset-0 w-full h-full object-cover transition-opacity duration-300"
        class:opacity-0={!imageLoaded}
        onload={handleImageLoad}
        onerror={handleImageError}
        loading="lazy"
      />
    {/if}
    <!-- Very subtle overlay -->
    <div class="absolute inset-0 bg-gradient-to-t from-black/40 via-black/0 to-black/0 opacity-0 group-hover:opacity-100 transition-opacity duration-200"></div>
  </div>

  <div class="mt-2.5 px-1">
    <h3 class="text-white font-semibold text-sm leading-tight truncate font-dm-sans group-hover:text-blue-300 transition-colors" title={game.name}>
      {game.name}
    </h3>
  </div>
</button>

{#if showEditModal}
  <GameEditModal 
    {game} 
    isOpen={showEditModal} 
    onclose={handleModalClose}
    onsave={handleGameSave}
  />
{/if}