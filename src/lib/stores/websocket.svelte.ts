import { writable, derived } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import type { ApplicationPayload, WebSocketMessage, GHUBApp } from '../types';
import { applicationPayloadToGHUBApp } from '../types';

// ============ Connection State Stores ============

export const wsConnected = writable(false);
export const wsReconnecting = writable(false);

// ============ Application Data Stores ============

export const applications = writable<ApplicationPayload[]>([]);

// ============ Derived Stores ============

export const applicationCount = derived(
  applications,
  ($apps) => $apps.length
);

export const applicationsAsGHUBApps = derived(
  applications,
  ($apps) => $apps.map(applicationPayloadToGHUBApp)
);

// ============ WebSocket Event Handling ============

let initialized = false;

export function initializeWebSocketStores() {
  if (initialized) {
    console.warn('[WebSocket Stores] Already initialized, skipping');
    return;
  }

  initialized = true;
  console.log('[WebSocket Stores] Initializing event listeners');

  // Connection state listeners
  listen('websocket-connected', () => {
    console.log('[WebSocket Stores] Connected');
    wsConnected.set(true);
    wsReconnecting.set(false);
  });

  listen('websocket-disconnected', (event) => {
    console.log('[WebSocket Stores] Disconnected:', event.payload);
    wsConnected.set(false);
  });

  listen('websocket-reconnecting', (event) => {
    console.log('[WebSocket Stores] Reconnecting:', event.payload);
    wsReconnecting.set(true);
  });

  listen('websocket-reconnected', () => {
    console.log('[WebSocket Stores] Reconnected');
    wsConnected.set(true);
    wsReconnecting.set(false);
  });

  listen('websocket-reconnection-failed', () => {
    console.log('[WebSocket Stores] Reconnection failed');
    wsConnected.set(false);
    wsReconnecting.set(false);
  });

  listen('websocket-closed', (event) => {
    console.log('[WebSocket Stores] Closed:', event.payload);
    wsConnected.set(false);
  });

  // Message routing listener
  listen('websocket-message', (event) => {
    try {
      const message: WebSocketMessage = typeof event.payload === 'string'
        ? JSON.parse(event.payload)
        : event.payload;

      console.log('[WebSocket Stores] Message received:', message.path);

      // Route message to appropriate store based on path
      routeMessage(message);
    } catch (error) {
      console.error('[WebSocket Stores] Failed to parse message:', error, event.payload);
    }
  });

  console.log('[WebSocket Stores] Initialization complete');
}

function routeMessage(message: WebSocketMessage) {
  switch (message.path) {
    case '/applications':
      handleApplicationsResponse(message);
      break;

    case '/application':
      handleApplicationResponse(message);
      break;

    default:
      console.log('[WebSocket Stores] Unhandled message path:', message.path);
  }
}

function handleApplicationsResponse(message: WebSocketMessage) {
  if (message.payload?.applications) {
    console.log('[WebSocket Stores] Updating applications store with', message.payload.applications.length, 'items');
    applications.set(message.payload.applications);
  } else {
    console.warn('[WebSocket Stores] /applications response has no applications array');
    applications.set([]);
  }
}

function handleApplicationResponse(message: WebSocketMessage) {
  const updatedApp: ApplicationPayload = message.payload;

  if (!updatedApp.applicationId && !updatedApp.databaseId) {
    console.warn('[WebSocket Stores] /application response missing ID');
    return;
  }

  const appId = updatedApp.applicationId || updatedApp.databaseId;

  applications.update(apps => {
    const index = apps.findIndex(
      app => (app.applicationId || app.databaseId) === appId
    );

    if (index !== -1) {
      console.log('[WebSocket Stores] Updating application at index', index);
      apps[index] = updatedApp;
      return [...apps]; // Create new array to trigger reactivity
    } else {
      console.log('[WebSocket Stores] Application not found, adding new one');
      return [...apps, updatedApp];
    }
  });
}
