import { invoke } from '@tauri-apps/api/core';
import type { ApplicationPayload } from '../types';

class WebSocketService {
    private readonly DEFAULT_URI = 'ws://localhost:9010';

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

    // Action helpers - fire-and-forget commands that trigger responses via WebSocket events
    async getApplications(): Promise<void> {
        return this.send('GET', '/applications');
    }

    async getApplication(applicationId: string): Promise<void> {
        return this.send('GET', '/application', { id: applicationId });
    }

    async setApplication(
        applicationPath: string,
        name: string,
        options?: Partial<Omit<ApplicationPayload, 'applicationPath' | 'name'>>
    ): Promise<void> {
        console.log('Setting application:', { applicationPath, name, ...options });
        return this.send('SET', '/application', {
            applicationPath,
            name,
            ...options
        });
    }

    async deleteApplication(applicationId: string): Promise<void> {
        return this.send('DELETE', '/application', { id: applicationId });
    }
}

// Export singleton instance
export const ws = new WebSocketService();