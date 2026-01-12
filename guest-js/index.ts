import { invoke } from '@tauri-apps/api/core';

export interface ConnectOptions {
    url: string;
    dbName?: string;
}

export async function connect(options: ConnectOptions): Promise<void>;
export async function connect(url: string, dbName?: string): Promise<void>;
export async function connect(urlOrOptions: string | ConnectOptions, dbName?: string): Promise<void> {
    if (typeof urlOrOptions === 'string') {
        await invoke('plugin:mongoose|connect', { url: urlOrOptions, dbName });
    } else {
        await invoke('plugin:mongoose|connect', { url: urlOrOptions.url, dbName: urlOrOptions.dbName });
    }
}

export { default as Model } from './model';
export * from './schema';
