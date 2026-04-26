import { writable } from 'svelte/store';
import type { Note } from '../commands';
import { listNotes } from '../commands';

export const notes = writable<Note[]>([]);
export const wordCount = writable(0);
export const lastSaved = writable<Date | null>(null);

export async function refreshNotes(notebook_id?: string, tag_id?: string) {
  try {
    notes.set(await listNotes(notebook_id, tag_id));
  } catch (e) {
    console.error('Failed to load notes:', e);
  }
}
