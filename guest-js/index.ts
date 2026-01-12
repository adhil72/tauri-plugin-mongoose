import { invoke } from '@tauri-apps/api/core';

export async function connect(url: string): Promise<void> {
    await invoke('plugin:mongoose|connect', {
        url,
    });
}
