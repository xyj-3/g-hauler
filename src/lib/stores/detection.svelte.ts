import type { GameScanResult, ScanOptions } from '$lib/types';

// Svelte 5 runes-based state
class DetectionStore {
  scanResults = $state<GameScanResult | null>(null);
  selectedGames = $state<Set<string>>(new Set());
  scanOptions = $state<ScanOptions>({
    scanSteam: true,
    scanEpicGames: true,
    scanUplay: true,
    scanGogGalaxy: true,
    scanRiotGames: true,
    scanWinRegistry: false,
    scanOsxBundle: false,
    scanEaApp: true
  });
  showCustomScan = $state(false);

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

  setScanOptions(options: ScanOptions) {
    this.scanOptions = options;
  }

  toggleCustomScan() {
    this.showCustomScan = !this.showCustomScan;
  }

  reset() {
    this.scanResults = null;
    this.selectedGames = new Set();
    // Note: scanOptions and showCustomScan persist between page navigations
  }
}

export const detectionStore = new DetectionStore();
