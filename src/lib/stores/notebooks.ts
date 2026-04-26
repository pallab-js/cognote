import { writable } from 'svelte/store';
import type { Notebook } from '../commands';
import { getNotebookTree } from '../commands';

export const notebooks = writable<Notebook[]>([]);

export async function refreshNotebooks() {
  try {
    notebooks.set(await getNotebookTree());
  } catch (e) {
    console.error('Failed to load notebooks:', e);
  }
}
