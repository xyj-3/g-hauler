import { invoke } from '@tauri-apps/api/core';
import { STORE_KEY_BUILD_ID, STORE_KEY_DATA_PATH, STORE_KEY_VERSION_NUMBER } from './constants';


export async function getStoreValue(key: string): Promise<string | null> {
    try {
        return await invoke<string | null>('store_get_key', { key });
    } catch (error) {
        console.error(`Failed to get store value for ${key}:`, error);
        return null;
    }
}

export async function setStoreValue(key: string, value: string): Promise<void> {
    try {
        await invoke('store_set_key', { key, value });
    } catch (error) {
        console.error(`Failed to set store value for ${key}:`, error);
    }
}

export async function getBuildId(): Promise<string | null> {
    return getStoreValue(STORE_KEY_BUILD_ID);
}

export async function getVersionNumber(): Promise<string | null> {
    return getStoreValue(STORE_KEY_VERSION_NUMBER);
}

export async function getLghubDataPath(): Promise<string | null> {
    return getStoreValue(STORE_KEY_DATA_PATH);
}

export async function setBuildId(value: string): Promise<void> {
    await setStoreValue(STORE_KEY_BUILD_ID, value);
}

export async function setVersionNumber(value: string): Promise<void> {
    await setStoreValue(STORE_KEY_VERSION_NUMBER, value);
}

export async function setLghubDataPath(value: string): Promise<void> {
    await setStoreValue(STORE_KEY_DATA_PATH, value);
}