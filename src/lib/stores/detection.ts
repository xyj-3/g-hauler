import type { GameScanResult } from '$lib/types';

// Svelte 5 runes-based state
class DetectionStore {
  scanResults = $state<GameScanResult | null>(null);
  selectedGames = $state<Set<string>>(new Set());

  setScanResults(results: GameScanResult | null) {
    this.scanResults = results;
  }

  setSelectedGames(games: Set<string>) {
    this.selectedGames = games;
  }

  addSelectedGame(gameId: string) {
    this.selectedGames.add(gameId);
    // Trigger reactivity by creating a new Set
    this.selectedGames = new Set(this.selectedGames);
  }

  removeSelectedGame(gameId: string) {
    this.selectedGames.delete(gameId);
    // Trigger reactivity by creating a new Set
    this.selectedGames = new Set(this.selectedGames);
  }

  clearSelectedGames() {
    this.selectedGames = new Set();
  }

  reset() {
    this.scanResults = null;
    this.selectedGames = new Set();
  }
}

export const detectionStore = new DetectionStore();
