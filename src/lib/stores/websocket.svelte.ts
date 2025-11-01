import { writable, derived } from 'svelte/store';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
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
let unlistenFunctions: UnlistenFn[] = [];

// Track if we're in a hot reload scenario (dev mode)
let isHotReload = false;
if (import.meta.hot) {
  import.meta.hot.on('vite:beforeUpdate', () => {
    isHotReload = true;
  });
}

export async function initializeWebSocketStores() {
  // Allow re-initialization during hot reload in development
  if (initialized && !isHotReload) {
    console.warn('[WebSocket Stores] Already initialized, skipping');
    return;
  }

  if (initialized && isHotReload) {
    console.log('[WebSocket Stores] Hot reload detected, re-initializing');
    // Clean up old listeners
    unlistenFunctions.forEach(unlisten => unlisten());
    unlistenFunctions = [];
    isHotReload = false; // Reset the flag
  }

  initialized = true;
  console.log('[WebSocket Stores] Initializing event listeners');

  try {
    // Connection state listeners
    const unlistenConnected = await listen('websocket-connected', () => {
      console.log('[WebSocket Stores] Connected');
      wsConnected.set(true);
      wsReconnecting.set(false);
    });
    unlistenFunctions.push(unlistenConnected);

    const unlistenDisconnected = await listen('websocket-disconnected', (event) => {
      console.log('[WebSocket Stores] Disconnected:', event.payload);
      wsConnected.set(false);
    });
    unlistenFunctions.push(unlistenDisconnected);

    const unlistenReconnecting = await listen('websocket-reconnecting', (event) => {
      console.log('[WebSocket Stores] Reconnecting:', event.payload);
      wsReconnecting.set(true);
    });
    unlistenFunctions.push(unlistenReconnecting);

    const unlistenReconnected = await listen('websocket-reconnected', () => {
      console.log('[WebSocket Stores] Reconnected');
      wsConnected.set(true);
      wsReconnecting.set(false);
    });
    unlistenFunctions.push(unlistenReconnected);

    const unlistenReconnectionFailed = await listen('websocket-reconnection-failed', () => {
      console.log('[WebSocket Stores] Reconnection failed');
      wsConnected.set(false);
      wsReconnecting.set(false);
    });
    unlistenFunctions.push(unlistenReconnectionFailed);

    const unlistenClosed = await listen('websocket-closed', (event) => {
      console.log('[WebSocket Stores] Closed:', event.payload);
      wsConnected.set(false);
    });
    unlistenFunctions.push(unlistenClosed);

    // Message routing listener
    const unlistenMessage = await listen('websocket-message', (event) => {
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
    unlistenFunctions.push(unlistenMessage);

    console.log('[WebSocket Stores] Initialization complete');
  } catch (error) {
    console.error('[WebSocket Stores] Failed to initialize event listeners:', error);
    throw error;
  }
}

export function cleanupWebSocketStores() {
  console.log('[WebSocket Stores] Cleaning up event listeners');
  unlistenFunctions.forEach(unlisten => unlisten());
  unlistenFunctions = [];
  initialized = false;
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
