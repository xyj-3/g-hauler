import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { ApplicationPayload } from '../types';

class WebSocketService {
    private eventListenersSetup = false;
    private readonly DEFAULT_URI = 'ws://localhost:9010';

    async connect(uri: string): Promise<void> {
        try {
            await invoke('ws_connect', { uri });
            this.setupEventListenersOnce();
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

    async send(verb: string, path: string, payload: Record<string, any> = {}): Promise<any> {
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

    async isConnected(): Promise<boolean> {
        try {
            return await invoke<boolean>('ws_is_connected');
        } catch (error) {
            console.error('Failed to check connection status:', error);
            return false;
        }
    }

    private setupEventListenersOnce(): void {
        if (this.eventListenersSetup) return;
        this.eventListenersSetup = true;

        listen('websocket-connected', () => {
            console.log('WebSocket connected');
        });

        listen('websocket-disconnected', (event) => {
            console.log('WebSocket disconnected:', event.payload);
        });

        listen('websocket-reconnecting', (event) => {
            console.log('WebSocket reconnecting:', event.payload);
        });

        listen('websocket-reconnected', () => {
            console.log('WebSocket reconnected');
        });

        listen('websocket-reconnection-failed', () => {
            console.log('WebSocket reconnection failed');
        });

        listen('websocket-closed', (event) => {
            console.log('WebSocket closed:', event.payload);
        });

        listen('websocket-message', (event) => {
            try {
                const message = typeof event.payload === 'string' ? JSON.parse(event.payload) : event.payload;
                // Just log the message - no processing needed
                // Applications can listen to this event directly if they need the data
            } catch (error) {
                console.error('[WebSocket] Failed to parse message as JSON:', error, event.payload);
            }
        });
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

    // Convenient action helpers - now fire-and-forget
    getApplications = async () =>
        this.send('GET', '/applications');

    getApplication = async (applicationId: string) =>
        this.send('GET', '/application', { id: applicationId });

    setApplication = async (
        applicationPath: string,
        name: string,
        options?: Partial<Omit<ApplicationPayload, 'applicationPath' | 'name'>>
    ) => {
        console.log('Setting application:', { applicationPath, name, ...options });
        return this.send('SET', '/application', {
            applicationPath,
            name,
            ...options
        });
    }

    deleteApplication = async (applicationId: string) =>
        this.send('DELETE', '/application', { id: applicationId });
}

// Export singleton instance
export const ws = new WebSocketService();