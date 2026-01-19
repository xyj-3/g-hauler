import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { ApplicationPayload, WebSocketMessage, GHUBApp } from '$lib/types';
import { applicationPayloadToGHUBApp } from '$lib/types';

class GHubStore {
	#connected = $state(false);
	#reconnecting = $state(false);
	#applications = $state<ApplicationPayload[]>([]);
	#lastUpdate = $state<Date | null>(null);

	#initialized = false;
	#listeners: UnlistenFn[] = [];

	get connected() {
		return this.#connected;
	}

	get reconnecting() {
		return this.#reconnecting;
	}

	get applications() {
		return this.#applications;
	}

	get lastUpdate() {
		return this.#lastUpdate;
	}

	get applicationCount() {
		return this.#applications.length;
	}

	get isReady() {
		return this.#connected && this.#applications.length > 0;
	}

	get asGHUBApps(): GHUBApp[] {
		return this.#applications.map(applicationPayloadToGHUBApp);
	}

	async initialize() {
		if (this.#initialized) {
			console.warn('[GHubStore] Already initialized');
			return;
		}

		this.#initialized = true;
		console.log('[GHubStore] Initializing event listeners');

		try {
			const listeners = await Promise.all([
				listen('websocket-connected', () => {
					console.log('[GHubStore] Connected');
					this.#connected = true;
					this.#reconnecting = false;
				}),
				listen('websocket-disconnected', (event) => {
					console.log('[GHubStore] Disconnected:', event.payload);
					this.#connected = false;
				}),
				listen('websocket-reconnecting', (event) => {
					console.log('[GHubStore] Reconnecting:', event.payload);
					this.#reconnecting = true;
				}),
				listen('websocket-reconnected', () => {
					console.log('[GHubStore] Reconnected');
					this.#connected = true;
					this.#reconnecting = false;
				}),
				listen('websocket-reconnection-failed', () => {
					console.log('[GHubStore] Reconnection failed');
					this.#connected = false;
					this.#reconnecting = false;
				}),
				listen('websocket-closed', (event) => {
					console.log('[GHubStore] Closed:', event.payload);
					this.#connected = false;
				}),
				listen<WebSocketMessage>('websocket-message', (event) => {
					try {
						const message: WebSocketMessage =
							typeof event.payload === 'string'
								? JSON.parse(event.payload)
								: event.payload;

						console.log('[GHubStore] Message received:', message.path);
						this.#handleMessage(message);
					} catch (error) {
						console.error('[GHubStore] Failed to parse message:', error, event.payload);
					}
				})
			]);

			this.#listeners.push(...listeners);
			console.log('[GHubStore] Initialization complete');
		} catch (error) {
			console.error('[GHubStore] Failed to initialize event listeners:', error);
			throw error;
		}
	}

	destroy() {
		console.log('[GHubStore] Cleaning up event listeners');
		this.#listeners.forEach((unlisten) => unlisten());
		this.#listeners = [];
		this.#initialized = false;
	}

	#handleMessage(message: WebSocketMessage) {
		switch (message.path) {
			case '/applications':
				this.#handleApplicationsResponse(message);
				break;

			case '/application':
				this.#handleApplicationResponse(message);
				break;

			default:
				console.log('[GHubStore] Unhandled message path:', message.path);
		}
	}

	#handleApplicationsResponse(message: WebSocketMessage) {
		if (message.payload?.applications) {
			console.log(
				'[GHubStore] Updating applications with',
				message.payload.applications.length,
				'items'
			);
			this.#applications = message.payload.applications;
			this.#lastUpdate = new Date();
		} else {
			console.warn('[GHubStore] /applications response has no applications array');
			this.#applications = [];
		}
	}

	#handleApplicationResponse(message: WebSocketMessage) {
		const updatedApp: ApplicationPayload = message.payload;

		if (!updatedApp.applicationId && !updatedApp.databaseId) {
			console.warn('[GHubStore] /application response missing ID');
			return;
		}

		const appId = updatedApp.applicationId || updatedApp.databaseId;

		const index = this.#applications.findIndex(
			(app) => (app.applicationId || app.databaseId) === appId
		);

		if (index !== -1) {
			console.log('[GHubStore] Updating application at index', index);
			const newApps = [...this.#applications];
			newApps[index] = updatedApp;
			this.#applications = newApps;
		} else {
			console.log('[GHubStore] Application not found, adding new one');
			this.#applications = [...this.#applications, updatedApp];
		}

		this.#lastUpdate = new Date();
	}
}

export const ghub = new GHubStore();
