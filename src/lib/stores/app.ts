import { writable } from 'svelte/store';
import type { AppConfig } from '../commands';

export type View = 'editor' | 'graph' | 'mindmap' | 'dashboard' | 'files';

export const currentView = writable<View>('editor');
export const activeNoteId = writable<string | null>(null);
export const activeNotebookId = writable<string | null>(null);
export const activeTagId = writable<string | null>(null);
export const searchQuery = writable('');
export const appConfig = writable<AppConfig>({ theme: 'dark', vault_path: null });
