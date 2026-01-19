import { invoke } from '@tauri-apps/api/core';

export interface UserPreferences {
	theme: 'light' | 'dark' | 'system';
	sidebarCollapsed: boolean;
	gridViewDensity: 'compact' | 'comfortable' | 'spacious';
	notificationsEnabled: boolean;
}

const DEFAULTS: UserPreferences = {
	theme: 'dark',
	sidebarCollapsed: false,
	gridViewDensity: 'comfortable',
	notificationsEnabled: true
};

class UserPreferencesStore {
	#preferences = $state<UserPreferences>({ ...DEFAULTS });
	#loaded = $state(false);
	#error = $state<string | null>(null);

	get preferences() {
		return this.#preferences;
	}

	get loaded() {
		return this.#loaded;
	}

	get error() {
		return this.#error;
	}

	get theme() {
		return this.#preferences.theme;
	}

	get sidebarCollapsed() {
		return this.#preferences.sidebarCollapsed;
	}

	get gridViewDensity() {
		return this.#preferences.gridViewDensity;
	}

	get notificationsEnabled() {
		return this.#preferences.notificationsEnabled;
	}

	async initialize() {
		try {
			const stored = await invoke<UserPreferences | null>('get_user_preferences');
			if (stored) {
				this.#preferences = { ...DEFAULTS, ...stored };
			}
			this.#loaded = true;
			console.log('[UserPreferences] Initialized:', this.#preferences);
		} catch (error) {
			console.error('[UserPreferences] Failed to load:', error);
			this.#error = error instanceof Error ? error.message : String(error);
			this.#loaded = true;
		}
	}

	async update(partial: Partial<UserPreferences>) {
		this.#preferences = { ...this.#preferences, ...partial };
		await this.#persist();
	}

	async setTheme(theme: UserPreferences['theme']) {
		this.#preferences.theme = theme;
		await this.#persist();
	}

	async toggleSidebar() {
		this.#preferences.sidebarCollapsed = !this.#preferences.sidebarCollapsed;
		await this.#persist();
	}

	async setGridDensity(density: UserPreferences['gridViewDensity']) {
		this.#preferences.gridViewDensity = density;
		await this.#persist();
	}

	async reset() {
		this.#preferences = { ...DEFAULTS };
		await this.#persist();
	}

	async #persist() {
		try {
			await invoke('set_user_preferences', { preferences: this.#preferences });
			console.log('[UserPreferences] Persisted');
		} catch (error) {
			console.error('[UserPreferences] Failed to persist:', error);
			this.#error = error instanceof Error ? error.message : String(error);
		}
	}
}

export const userPreferences = new UserPreferencesStore();
