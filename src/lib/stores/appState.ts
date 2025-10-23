import { writable } from 'svelte/store';

// Store to track if the home page has finished its initial load
export const homePageLoaded = writable(false);
