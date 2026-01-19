import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { ApplicationPayload, WebSocketMessage, GHUBApp } from '$lib/types';
import { applicationPayloadToGHUBApp } from '$lib/types';
import { ws } from '$lib/services/websocket';

type StoreStatus = 'disconnected' | 'connecting' | 'connected' | 'loading' | 'ready' | 'error';

class GHubStore {
	#status = $state<StoreStatus>('disconnected');
	#error = $state<string | null>(null);
	#applications = $state<ApplicationPayload[]>([]);
	#lastUpdate = $state<Date | null>(null);

	#initialized = false;
	#listeners: UnlistenFn[] = [];
	#loadingTimeout: ReturnType<typeof setTimeout> | null = null;

	get status() {
		return this.#status;
	}

	get error() {
		return this.#error;
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
		return this.#status === 'ready';
	}

	get isLoading() {
		return this.#status === 'loading' || this.#status === 'connecting';
	}

	get isConnected() {
		return this.#status === 'connected' || this.#status === 'loading' || this.#status === 'ready';
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
		console.log('[GHubStore] Initializing with full lifecycle ownership');

		try {
			await this.#setupEventListeners();
			await this.#connect();
		} catch (error) {
			console.error('[GHubStore] Initialization failed:', error);
			this.#status = 'error';
			this.#error = error instanceof Error ? error.message : 'Initialization failed';
		}
	}

	async #setupEventListeners() {
		const listeners = await Promise.all([
			listen('websocket-connected', () => {
				console.log('[GHubStore] WebSocket connected');
				this.#onConnected();
			}),
			listen('websocket-disconnected', (event) => {
				console.log('[GHubStore] WebSocket disconnected:', event.payload);
				this.#onDisconnected();
			}),
			listen('websocket-reconnecting', () => {
				console.log('[GHubStore] WebSocket reconnecting');
				this.#status = 'connecting';
			}),
			listen('websocket-reconnected', () => {
				console.log('[GHubStore] WebSocket reconnected');
				this.#onConnected();
			}),
			listen('websocket-reconnection-failed', () => {
				console.log('[GHubStore] Reconnection failed');
				this.#status = 'error';
				this.#error = 'Failed to reconnect to G HUB WebSocket';
			}),
			listen('websocket-closed', (event) => {
				console.log('[GHubStore] WebSocket closed:', event.payload);
				this.#onDisconnected();
			}),
			listen<WebSocketMessage>('websocket-message', (event) => {
				try {
					const message: WebSocketMessage =
						typeof event.payload === 'string' ? JSON.parse(event.payload) : event.payload;

					this.#handleMessage(message);
				} catch (error) {
					console.error('[GHubStore] Failed to parse message:', error, event.payload);
				}
			})
		]);

		this.#listeners.push(...listeners);
		console.log('[GHubStore] Event listeners set up');
	}

	async #connect() {
		this.#status = 'connecting';
		console.log('[GHubStore] Initiating connection to G HUB WebSocket');

		try {
			await ws.autoConnect();
		} catch (error) {
			console.error('[GHubStore] Connection failed:', error);
			this.#status = 'error';
			this.#error = 'Failed to connect to G HUB. Is G HUB running?';
		}
	}

	#onConnected() {
		this.#status = 'connected';
		this.#error = null;
		this.#loadApplications();
	}

	#onDisconnected() {
		if (this.#status === 'error') {
			return;
		}
		this.#status = 'disconnected';
		this.#clearLoadingTimeout();
	}

	async #loadApplications() {
		this.#status = 'loading';
		console.log('[GHubStore] Auto-loading applications');

		this.#clearLoadingTimeout();

		this.#loadingTimeout = setTimeout(() => {
			if (this.#status === 'loading') {
				console.error('[GHubStore] Loading timeout - no response from G HUB');
				this.#status = 'error';
				this.#error = 'G HUB did not respond to application request';
			}
		}, 10000);

		try {
			await ws.getApplications();
			console.log('[GHubStore] Application request sent');
		} catch (error) {
			console.error('[GHubStore] Failed to request applications:', error);
			this.#status = 'error';
			this.#error = error instanceof Error ? error.message : 'Failed to load applications';
			this.#clearLoadingTimeout();
		}
	}

	#clearLoadingTimeout() {
		if (this.#loadingTimeout) {
			clearTimeout(this.#loadingTimeout);
			this.#loadingTimeout = null;
		}
	}

	#handleMessage(message: WebSocketMessage) {
		console.log('[GHubStore] Message received:', message.path);

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
		this.#clearLoadingTimeout();

		if (message.payload?.applications) {
			console.log('[GHubStore] Received', message.payload.applications.length, 'applications');
			this.#applications = message.payload.applications;
			this.#lastUpdate = new Date();
			this.#status = 'ready';
			this.#error = null;
		} else {
			console.warn('[GHubStore] Applications response has no applications array');
			this.#applications = [];
			this.#status = 'ready';
		}
	}

	#handleApplicationResponse(message: WebSocketMessage) {
		const updatedApp: ApplicationPayload = message.payload;

		if (!updatedApp.applicationId && !updatedApp.databaseId) {
			console.warn('[GHubStore] Application response missing ID');
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

	destroy() {
		console.log('[GHubStore] Cleaning up');
		this.#clearLoadingTimeout();
		this.#listeners.forEach((unlisten) => unlisten());
		this.#listeners = [];
		this.#initialized = false;
		this.#status = 'disconnected';
	}
}

export const ghub = new GHubStore();
