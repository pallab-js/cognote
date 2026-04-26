import { invoke } from '@tauri-apps/api/core';

export interface Notebook {
  id: string; name: string; parent_id: string | null;
  created_at: string; updated_at: string;
}
export interface Note {
  id: string; title: string; content: string | null;
  notebook_id: string | null; is_pinned: boolean;
  created_at: string; updated_at: string;
}
export interface Tag { id: string; name: string; }
export interface NoteLink { source_note_id: string; target_note_id: string; context: string | null; }
export interface FileInfo {
  id: string; name: string; path: string;
  mime_type: string | null; size: number | null;
  notebook_id: string | null; created_at: string;
}
export interface GraphData {
  nodes: { id: string; label: string; notebook_id: string | null }[];
  edges: { source: string; target: string }[];
}
export interface SearchResult { id: string; title: string; snippet: string; }
export interface DailyStats {
  today_count: number; streak: number; total_notes: number;
  recent_days: { date: string; count: number }[];
}
export interface AppConfig { theme: string; vault_path: string | null; }

// Notebooks
export const createNotebook = (name: string, parent_id?: string) =>
  invoke<Notebook>('create_notebook', { name, parentId: parent_id });
export const renameNotebook = (id: string, name: string) =>
  invoke<void>('rename_notebook', { id, name });
export const deleteNotebook = (id: string) =>
  invoke<void>('delete_notebook', { id });
export const getNotebookTree = () =>
  invoke<Notebook[]>('get_notebook_tree');

// Notes
export const createNote = (title: string, notebook_id?: string) =>
  invoke<Note>('create_note', { title, notebookId: notebook_id });
export const getNote = (id: string) =>
  invoke<Note>('get_note', { id });
export const updateNote = (id: string, title?: string, content?: string, notebook_id?: string | null) =>
  invoke<Note>('update_note', { id, title, content, notebookId: notebook_id });
export const deleteNote = (id: string) =>
  invoke<void>('delete_note', { id });
export const listNotes = (notebook_id?: string, tag_id?: string) =>
  invoke<Note[]>('list_notes', { notebookId: notebook_id, tagId: tag_id });

// Tags
export const addTag = (note_id: string, tag_name: string) =>
  invoke<Tag>('add_tag', { noteId: note_id, tagName: tag_name });
export const removeTag = (note_id: string, tag_id: string) =>
  invoke<void>('remove_tag', { noteId: note_id, tagId: tag_id });
export const listTags = () =>
  invoke<Tag[]>('list_tags');

// Links
export const createNoteLink = (source_id: string, target_id: string, context?: string) =>
  invoke<void>('create_note_link', { sourceId: source_id, targetId: target_id, context });
export const removeNoteLink = (source_id: string, target_id: string) =>
  invoke<void>('remove_note_link', { sourceId: source_id, targetId: target_id });
export const getBacklinks = (note_id: string) =>
  invoke<NoteLink[]>('get_backlinks', { noteId: note_id });
export const getKnowledgeGraph = () =>
  invoke<GraphData>('get_knowledge_graph');
export const getMindmapData = (note_id: string) =>
  invoke<{ id: string; label: string; children: any[] }>('get_mindmap_data', { noteId: note_id });

// Files
export const importFile = (source_path: string, notebook_id?: string) =>
  invoke<FileInfo>('import_file', { sourcePath: source_path, notebookId: notebook_id });
export const listFiles = (notebook_id?: string) =>
  invoke<FileInfo[]>('list_files', { notebookId: notebook_id });
export const deleteFile = (id: string) =>
  invoke<void>('delete_file', { id });
export const openFileExternal = (id: string) =>
  invoke<void>('open_file_external', { id });

// Search & Stats
export const searchNotes = (query: string) =>
  invoke<SearchResult[]>('search_notes', { query });
export const getDailyStats = () =>
  invoke<DailyStats>('get_daily_stats');

// Config
export const getAppConfig = () =>
  invoke<AppConfig>('get_app_config');
export const updateAppConfig = (theme?: string, vault_path?: string) =>
  invoke<void>('update_app_config', { theme, vaultPath: vault_path });
