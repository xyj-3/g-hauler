import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { ApplicationPayload, WebSocketMessage } from '../types';

interface PendingRequest {
	resolve: (value: any) => void;
	reject: (error: any) => void;
	timeout: ReturnType<typeof setTimeout>;
}

class WebSocketService {
	private readonly DEFAULT_URI = 'ws://localhost:9010';
	private pendingRequests = new Map<string, PendingRequest>();
	private messageListener: (() => void) | null = null;
	private initialized = false;

	async initialize() {
		if (this.initialized) return;

		this.messageListener = await listen<WebSocketMessage>('websocket-message', (event) => {
			try {
				const message: WebSocketMessage =
					typeof event.payload === 'string' ? JSON.parse(event.payload) : event.payload;

				this.handleResponse(message);
			} catch (error) {
				console.error('[WebSocket] Failed to parse message:', error);
			}
		});

		this.initialized = true;
	}

	private handleResponse(message: WebSocketMessage) {
		const pending = this.pendingRequests.get(message.msgId);
		if (pending) {
			clearTimeout(pending.timeout);
			this.pendingRequests.delete(message.msgId);
			pending.resolve(message);
		}
	}

	async connect(uri: string): Promise<void> {
		try {
			await invoke('ws_connect', { uri });
		} catch (error) {
			console.error('WebSocket connection failed:', error);
			throw error;
		}
	}

	async disconnect(): Promise<void> {
		try {
			await invoke('ws_disconnect');
		} catch (error) {
			console.error('WebSocket disconnect failed:', error);
			throw error;
		}
	}

	async send(verb: string, path: string, payload: Record<string, any> = {}): Promise<void> {
		console.log('[WebSocket] Sending message:', { verb, path, payload });

		try {
			await invoke('ws_send_message', {
				verb,
				path,
				payload
			});
			console.log('[WebSocket] Message sent successfully:', { verb, path });
		} catch (error) {
			console.error('[WebSocket] Failed to send message:', { verb, path, error });
			throw error;
		}
	}

	async request<T = any>(
		verb: string,
		path: string,
		payload: Record<string, any> = {},
		timeoutMs: number = 10000
	): Promise<T> {
		await this.initialize();

		const msgId = crypto.randomUUID();

		return new Promise((resolve, reject) => {
			const timeout = setTimeout(() => {
				this.pendingRequests.delete(msgId);
				reject(new Error(`Request timeout: ${verb} ${path}`));
			}, timeoutMs);

			this.pendingRequests.set(msgId, {
				resolve: (message: WebSocketMessage) => resolve(message.payload as T),
				reject,
				timeout
			});

			invoke('ws_send_message', {
				verb,
				path,
				payload: { ...payload, msgId }
			}).catch((error) => {
				clearTimeout(timeout);
				this.pendingRequests.delete(msgId);
				reject(error);
			});
		});
	}

	async isConnected(): Promise<boolean> {
		try {
			return await invoke<boolean>('ws_is_connected');
		} catch (error) {
			console.error('Failed to check connection status:', error);
			return false;
		}
	}

	async autoConnect(): Promise<void> {
		try {
			console.log('Attempting to connect to WebSocket at:', this.DEFAULT_URI);
			await this.connect(this.DEFAULT_URI);
			console.log('WebSocket connected successfully');
		} catch (error) {
			console.error('Auto-connect failed:', error);
			console.error('Make sure a WebSocket server is running at:', this.DEFAULT_URI);
		}
	}

	destroy() {
		if (this.messageListener) {
			this.messageListener();
			this.messageListener = null;
		}
		this.pendingRequests.forEach(({ timeout }) => clearTimeout(timeout));
		this.pendingRequests.clear();
		this.initialized = false;
	}
}

export const ws = new WebSocketService();
