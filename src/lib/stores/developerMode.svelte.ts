import { invoke } from '@tauri-apps/api/core';

class DeveloperModeStore {
	#enabled = $state<boolean>(false);
	#initialized = $state<boolean>(false);  // Not used right now, but may be used later

	get enabled() {
		return this.#enabled;
	}

	get initialized() {
		return this.#initialized;
	}

	async initialize() {
		try {
			const isDeveloperMode = await invoke<boolean>('is_developer_mode');
			this.#enabled = isDeveloperMode;
			this.#initialized = true;
		} catch (error) {
			console.error('Failed to check developer mode:', error);
			this.#enabled = false;
			this.#initialized = true;
		}
	}

	async openDevTools() {
		if (!this.#enabled) {
			console.warn('Cannot open DevTools: Developer mode is not enabled');
			return;
		}

		try {
			await invoke('open_devtools');
		} catch (error) {
			console.error('Failed to open DevTools:', error);
		}
	}

	async closeDevTools() {
		if (!this.#enabled) {
			console.warn('Cannot close DevTools: Developer mode is not enabled');
			return;
		}

		try {
			await invoke('close_devtools');
		} catch (error) {
			console.error('Failed to close DevTools:', error);
		}
	}

	async isDevToolsOpen(): Promise<boolean> {
		try {
			return await invoke<boolean>('is_devtools_open');
		} catch (error) {
			console.error('Failed to check if DevTools is open:', error);
			return false;
		}
	}

	async toggleDevTools() {
		if (!this.#enabled) {
			console.warn('Cannot toggle DevTools: Developer mode is not enabled');
			return;
		}

		const isOpen = await this.isDevToolsOpen();
		if (isOpen) {
			await this.closeDevTools();
		} else {
			await this.openDevTools();
		}
	}
}

export const developerMode = new DeveloperModeStore();
