import { invoke } from '@tauri-apps/api/core';

export interface ConnectOptions {
    url: string;
    dbName?: string;
}

export interface ConnectCallbacks {
    onSuccess?: () => void;
    onError?: (error: string) => void;
}

export interface MongoDBRole {
    role: string;
    db: string;
}

export interface MongoDBUser {
    _id: string;
    userId: { $binary: { base64: string; subType: string } };
    user: string;
    db: string;
    roles: MongoDBRole[];
    mechanisms?: string[];
}

export async function connect(options: ConnectOptions, callbacks?: ConnectCallbacks): Promise<void>;
export async function connect(url: string, dbName?: string, callbacks?: ConnectCallbacks): Promise<void>;
export async function connect(
    urlOrOptions: string | ConnectOptions,
    dbNameOrCallbacks?: string | ConnectCallbacks,
    callbacks?: ConnectCallbacks
): Promise<void> {
    let url: string;
    let dbName: string | undefined;
    let cbs: ConnectCallbacks | undefined;

    if (typeof urlOrOptions === 'string') {
        url = urlOrOptions;
        if (typeof dbNameOrCallbacks === 'string') {
            dbName = dbNameOrCallbacks;
            cbs = callbacks;
        } else {
            cbs = dbNameOrCallbacks;
        }
    } else {
        url = urlOrOptions.url;
        dbName = urlOrOptions.dbName;
        cbs = dbNameOrCallbacks as ConnectCallbacks | undefined;
    }

    try {
        await invoke('plugin:mongoose|connect', { url, dbName });
        cbs?.onSuccess?.();
    } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        cbs?.onError?.(errorMessage);
        throw error;
    }
}

export async function getUsers(): Promise<MongoDBUser[]> {
    return await invoke('plugin:mongoose|get_users');
}

export async function getUser(username: string, db?: string): Promise<MongoDBUser | null> {
    return await invoke('plugin:mongoose|get_user', { username, db });
}

export { default as Model } from './model';
export * from './schema';
