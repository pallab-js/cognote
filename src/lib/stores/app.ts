import { writable } from 'svelte/store';
import type { AppConfig } from '../commands';

export type View = 'editor' | 'graph' | 'mindmap' | 'dashboard' | 'files' | 'tasks';

export const currentView = writable<View>('editor');
export const leftSidebarOpen = writable(true);
export const rightPanelOpen = writable(true);
export const activeNoteId = writable<string | null>(null);
export const activeNotebookId = writable<string | null>(null);
export const searchQuery = writable('');
export const commandPaletteOpen = writable(false);
export const appConfig = writable<AppConfig>({ theme: 'dark', vault_path: null });

export interface Toast {
  id: number;
  message: string;
  type: 'error' | 'success' | 'info';
}
export const toasts = writable<Toast[]>([]);
let toastId = 0;

export function showToast(message: string, type: Toast['type'] = 'info') {
  const id = ++toastId;
  toasts.update(t => [...t, { id, message, type }]);
  setTimeout(() => {
    toasts.update(t => t.filter(x => x.id !== id));
  }, 4000);
}
