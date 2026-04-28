<!-- ANCHOR: EDITOR_WIKI_LINKS_ACTIVE -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Editor } from '@tiptap/core';
  import StarterKit from '@tiptap/starter-kit';
  import Link from '@tiptap/extension-link';
  import Mention from '@tiptap/extension-mention';
  import TaskList from '@tiptap/extension-task-list';
  import TaskItem from '@tiptap/extension-task-item';
  import Table from '@tiptap/extension-table';
  import TableRow from '@tiptap/extension-table-row';
  import TableCell from '@tiptap/extension-table-cell';
  import TableHeader from '@tiptap/extension-table-header';
  import Image from '@tiptap/extension-image';
  import Highlight from '@tiptap/extension-highlight';
  import { MathExtension } from '@aarkue/tiptap-math-extension';
  import 'katex/dist/katex.min.css';
  import { activeNoteId, showToast } from '$lib/stores/app';
  import { wordCount, lastSaved } from '$lib/stores/notes';
  import { getNote, updateNote, listNotes, createNoteLink } from '$lib/commands';
  import type { Note } from '$lib/commands';
  import { Bold, Italic, Heading1, Heading2, Code, Link as LinkIcon, List, ListOrdered, CheckSquare, Table as TableIcon, Image as ImageIcon, Highlighter, Sigma } from 'lucide-svelte';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';

  let editorEl: HTMLElement;
  let editor: Editor | null = null;
  let note: Note | null = null;
  let titleValue = '';
  let autosaveTimer: ReturnType<typeof setTimeout>;
  let loading = false;
  let mentionPopup: HTMLElement | null = null;
  let saveInProgress = false;
  let pendingSave = false;

  // Load note when activeNoteId changes
  $: if ($activeNoteId) loadNote($activeNoteId);
  $: if (!$activeNoteId) { note = null; titleValue = ''; editor?.commands.setContent(''); }

  async function loadNote(id: string) {
    loading = true;
    try {
      note = await getNote(id);
      titleValue = note.title;
      if (editor) {
        const content = note.content ? JSON.parse(note.content) : '';
        editor.commands.setContent(content, false);
        updateWordCount();
      }
    } catch (e) {
      console.error('Failed to load note:', e);
    } finally {
      loading = false;
    }
  }

  function updateWordCount() {
    if (!editor) return;
    const text = editor.getText();
    wordCount.set(text.trim() ? text.trim().split(/\s+/).length : 0);
  }

  function scheduleAutosave() {
    clearTimeout(autosaveTimer);
    autosaveTimer = setTimeout(save, 1000);
  }

  async function save() {
    if (!note || !editor) return;
    if (saveInProgress) {
      pendingSave = true;
      return;
    }
    
    saveInProgress = true;
    const content = JSON.stringify(editor.getJSON());
    try {
      await updateNote(note.id, titleValue || 'Untitled', content);
      lastSaved.set(new Date());
    } catch (e) {
      console.error('Autosave failed:', e);
    } finally {
      saveInProgress = false;
      if (pendingSave) {
        pendingSave = false;
        scheduleAutosave();
      }
    }
  }

  async function saveTitleBlur() {
    if (!note) return;
    await updateNote(note.id, titleValue || 'Untitled');
    lastSaved.set(new Date());
  }

  // Build mention suggestion list
  let noteCache: { id: string; title: string }[] = [];
  let cacheStale = true;

  async function getMentionItems(query: string) {
    if (cacheStale) {
      try {
        const notes = await listNotes();
        noteCache = notes.map(n => ({ id: n.id, title: n.title }));
        cacheStale = false;
        setTimeout(() => { cacheStale = true; }, 30_000);
      } catch { return []; }
    }
    const q = query.toLowerCase();
    return noteCache
      .filter(n => n.id !== $activeNoteId && n.title.toLowerCase().includes(q))
      .slice(0, 8)
      .map(n => ({ id: n.id, label: n.title }));
  }

  onMount(() => {
    editor = new Editor({
      element: editorEl,
      extensions: [
        StarterKit,
        TaskList,
        TaskItem.configure({ nested: true }),
        Table.configure({ resizable: true }),
        TableRow,
        TableHeader,
        TableCell,
        Image.configure({ inline: true, allowBase64: true }),
        Highlight,
        MathExtension.configure({ evaluation: false }),
        Link.configure({ openOnClick: false }),
        Mention.configure({
          HTMLAttributes: { class: 'wiki-link' },
          suggestion: {
            items: ({ query }: { query: string }) => getMentionItems(query),
            render: () => {
              let items: { id: string; label: string }[] = [];
              let selectedIndex = 0;

              return {
                onStart(props: any) {
                  mentionPopup = document.createElement('div');
                  mentionPopup.className = 'mention-popup';
                  document.body.appendChild(mentionPopup);
                  items = props.items;
                  selectedIndex = 0;
                  renderPopup(mentionPopup, items, selectedIndex, props);
                  positionPopup(mentionPopup, props.clientRect?.());
                },
                onUpdate(props: any) {
                  items = props.items;
                  selectedIndex = 0;
                  if (mentionPopup) {
                    renderPopup(mentionPopup, items, selectedIndex, props);
                    positionPopup(mentionPopup, props.clientRect?.());
                  }
                },
                onKeyDown(props: any) {
                  if (props.event.key === 'ArrowDown') { selectedIndex = (selectedIndex + 1) % items.length; if (mentionPopup) renderPopup(mentionPopup, items, selectedIndex, props); return true; }
                  if (props.event.key === 'ArrowUp') { selectedIndex = (selectedIndex - 1 + items.length) % items.length; if (mentionPopup) renderPopup(mentionPopup, items, selectedIndex, props); return true; }
                  if (props.event.key === 'Enter') { selectItem(items[selectedIndex], props); return true; }
                  return false;
                },
                onExit() { mentionPopup?.remove(); mentionPopup = null; },
              };

              function renderPopup(el: HTMLElement, its: any[], idx: number, props: any) {
                if (its.length === 0) {
                  el.innerHTML = '<div class="mention-item empty">No notes found</div>';
                  return;
                }
                
                el.innerHTML = '';
                its.forEach((item, i) => {
                  const div = document.createElement('div');
                  div.className = `mention-item${i === idx ? ' selected' : ''}`;
                  div.dataset.idx = i.toString();
                  div.textContent = item.label;
                  div.addEventListener('click', () => selectItem(item, props));
                  el.appendChild(div);
                });
              }

              function positionPopup(el: HTMLElement, rect: DOMRect | undefined) {
                if (!rect) return;
                el.style.position = 'fixed';
                el.style.top = `${rect.bottom + 4}px`;
                el.style.left = `${rect.left}px`;
              }

              function selectItem(item: any, props: any) {
                if (!item) return;
                props.command({ id: item.id, label: item.label });
                // Create backlink
                if ($activeNoteId) {
                  createNoteLink($activeNoteId, item.id)
                    .catch(() => showToast('Failed to create link', 'error'));
                }
              }
            },
          },
        }),
      ],
      content: '',
      onUpdate: ({ editor: e }) => {
        updateWordCount();
        scheduleAutosave();
      },
      editorProps: {
        attributes: { class: 'tiptap-editor', spellcheck: 'true' },
      },
    });

    // Handle wiki-link clicks
    if (editorEl) {
      editorEl.addEventListener('click', (e) => {
        const target = (e.target as HTMLElement).closest('.wiki-link') as HTMLElement | null;
        if (target) {
          const id = target.dataset.id;
          if (id) activeNoteId.set(id);
        }
      });
    }

    // Load initial note if already selected
    if ($activeNoteId) loadNote($activeNoteId);
  });

  onDestroy(() => {
    clearTimeout(autosaveTimer);
    mentionPopup?.remove();
    editor?.destroy();
  });

  function cmd(command: (chain: ReturnType<Editor['chain']>) => ReturnType<Editor['chain']>) {
    if (editor) command(editor.chain().focus()).run();
  }
  $: isBold = editor?.isActive('bold') ?? false;
  $: isItalic = editor?.isActive('italic') ?? false;
  $: isH1 = editor?.isActive('heading', { level: 1 }) ?? false;
  $: isH2 = editor?.isActive('heading', { level: 2 }) ?? false;
  $: isCode = editor?.isActive('code') ?? false;
  $: isTaskList = editor?.isActive('taskList') ?? false;
  $: isHighlight = editor?.isActive('highlight') ?? false;
  $: isTable = editor?.isActive('table') ?? false;

  function insertTable() {
    editor?.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run();
  }

  let fileInput: HTMLInputElement;

  function triggerAddImage() {
    fileInput.click();
  }

  async function handleFileSelect(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      const file = target.files[0];
      await uploadAndInsertImage(file);
      target.value = ''; // Reset input
    }
  }

  async function handleDrop(e: DragEvent) {
    if (!e.dataTransfer?.files.length) return;
    const file = e.dataTransfer.files[0];
    if (file.type.startsWith('image/')) {
      e.preventDefault();
      await uploadAndInsertImage(file);
    }
  }

  async function uploadAndInsertImage(file: File) {
    try {
      const buffer = await file.arrayBuffer();
      const bytes = Array.from(new Uint8Array(buffer));
      const absPath = await invoke<string>('save_image', { filename: file.name, bytes });
      const assetUrl = convertFileSrc(absPath);
      editor?.chain().focus().setImage({ src: assetUrl }).run();
    } catch (err) {
      showToast('Failed to save image', 'error');
      console.error(err);
    }
  }

  function insertMath() {
    editor?.chain().focus().insertContent('<math></math>').run();
  }
</script>

<div class="editor-wrap">
  {#if note}
    <!-- Title -->
    <div class="title-bar">
      <input
        class="note-title"
        bind:value={titleValue}
        onblur={saveTitleBlur}
        oninput={scheduleAutosave}
        placeholder="Untitled"
      />
    </div>

    <!-- Toolbar -->
    <div class="toolbar">
      <button class="tb-btn" class:active={isBold} title="Bold (Ctrl+B)" onclick={() => editor?.chain().focus().toggleBold().run()}><Bold size={13}/></button>
      <button class="tb-btn" class:active={isItalic} title="Italic (Ctrl+I)" onclick={() => editor?.chain().focus().toggleItalic().run()}><Italic size={13}/></button>
      <div class="tb-sep"></div>
      <button class="tb-btn" class:active={isH1} title="Heading 1" onclick={() => editor?.chain().focus().toggleHeading({ level: 1 }).run()}><Heading1 size={13}/></button>
      <button class="tb-btn" class:active={isH2} title="Heading 2" onclick={() => editor?.chain().focus().toggleHeading({ level: 2 }).run()}><Heading2 size={13}/></button>
      <div class="tb-sep"></div>
      <button class="tb-btn" class:active={isCode} title="Code" onclick={() => editor?.chain().focus().toggleCode().run()}><Code size={13}/></button>
      <button class="tb-btn" class:active={isHighlight} title="Highlight" onclick={() => editor?.chain().focus().toggleHighlight().run()}><Highlighter size={13}/></button>
      <div class="tb-sep"></div>
      <button class="tb-btn" title="Bullet list" onclick={() => editor?.chain().focus().toggleBulletList().run()}><List size={13}/></button>
      <button class="tb-btn" title="Ordered list" onclick={() => editor?.chain().focus().toggleOrderedList().run()}><ListOrdered size={13}/></button>
      <button class="tb-btn" class:active={isTaskList} title="Task list" onclick={() => editor?.chain().focus().toggleTaskList().run()}><CheckSquare size={13}/></button>
      <div class="tb-sep"></div>
      <button class="tb-btn" class:active={isTable} title="Insert Table" onclick={insertTable}><TableIcon size={13}/></button>
      <button class="tb-btn" title="Add Image" onclick={triggerAddImage}><ImageIcon size={13}/></button>
      <button class="tb-btn" title="Insert Math" onclick={insertMath}><Sigma size={13}/></button>
      <input type="file" accept="image/*" style="display: none;" bind:this={fileInput} onchange={handleFileSelect} />
      <div class="tb-sep"></div>
      <span class="tb-hint">Type <code>[[</code> to link a note</span>
    </div>

    <!-- Editor -->
    <div class="editor-content" role="presentation" bind:this={editorEl} ondrop={handleDrop} ondragover={(e) => e.preventDefault()}></div>
  {:else}
    <div class="empty-state">
      <p>Select a note or create a new one</p>
      <p class="hint">Use the note list on the left</p>
    </div>
  {/if}
</div>

<style>
  .editor-wrap { display: flex; flex-direction: column; height: 100%; overflow: hidden; background: var(--bg-primary); }

  .title-bar { padding: 24px 32px 0; flex-shrink: 0; }
  .note-title {
    width: 100%; background: none; border: none; outline: none;
    font-size: 24px; font-weight: 400; color: var(--text-primary);
    font-family: var(--font-sans); letter-spacing: -0.16px;
    border-bottom: 1px solid var(--border-subtle); padding-bottom: 12px;
    transition: border-color 0.15s ease;
  }
  .note-title::placeholder { color: var(--text-muted); }
  .note-title:focus { border-color: var(--border-standard); }

  .toolbar {
    display: flex; align-items: center; gap: 4px;
    padding: 12px 32px; border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0; flex-wrap: wrap; background: var(--bg-primary);
  }
  .tb-btn {
    background: transparent; color: var(--text-muted);
    padding: 8px; border-radius: 6px; border: 1px solid transparent;
    cursor: pointer; display: flex; align-items: center;
    transition: all 0.15s ease;
  }
  .tb-btn:hover { background: var(--border-subtle); color: var(--text-primary); }
  .tb-btn.active { color: var(--green-brand); background: var(--bg-secondary); border-color: var(--green-border); }
  .tb-sep { width: 1px; height: 18px; background: var(--border-subtle); margin: 0 6px; }
  .tb-hint { font-size: 12px; color: var(--text-muted); margin-left: auto; }
  .tb-hint code { font-family: var(--font-mono); color: var(--green-brand); font-weight: 400; }

  .editor-content { flex: 1; overflow-y: auto; padding: 24px 32px; }

  :global(.tiptap-editor) {
    outline: none; min-height: 100%;
    font-size: 16px; line-height: 1.5; color: var(--text-primary);
    font-family: var(--font-sans);
  }
  :global(.tiptap-editor h1) { font-size: 32px; font-weight: 400; margin: 24px 0 12px; line-height: 1.25; }
  :global(.tiptap-editor h2) { font-size: 24px; font-weight: 400; margin: 20px 0 8px; line-height: 1.33; letter-spacing: -0.16px; }
  :global(.tiptap-editor p) { margin: 0 0 12px; }
  :global(.tiptap-editor code) { font-family: var(--font-mono); background: var(--border-subtle); padding: 2px 4px; border-radius: 4px; font-size: 14px; color: var(--green-brand); }
  :global(.tiptap-editor pre) { background: var(--bg-secondary); border: 1px solid var(--border-standard); border-radius: 8px; padding: 16px; margin: 12px 0; overflow-x: auto; }
  :global(.tiptap-editor pre code) { background: none; padding: 0; color: var(--text-secondary); }
  :global(.tiptap-editor ul, .tiptap-editor ol) { padding-left: 24px; margin: 8px 0 16px; }
  :global(.tiptap-editor li) { margin: 4px 0; }
  :global(.tiptap-editor a) { color: var(--green-link); text-decoration: none; border-bottom: 1px solid transparent; }
  :global(.tiptap-editor a:hover) { border-bottom-color: var(--green-link); }
  :global(.tiptap-editor .wiki-link) {
    color: var(--green-link); background: rgba(0, 197, 115, 0.05);
    border-radius: 4px; padding: 1px 4px; cursor: pointer;
    border-bottom: 1px solid var(--green-border);
  }
  :global(.tiptap-editor .wiki-link:hover) { background: rgba(0, 197, 115, 0.1); }
  
  :global(.tiptap-editor ul[data-type="taskList"]) { list-style: none; padding: 0; }
  :global(.tiptap-editor ul[data-type="taskList"] li) { display: flex; align-items: flex-start; gap: 10px; }
  :global(.tiptap-editor ul[data-type="taskList"] li > label) { margin-top: 5px; display: flex; }
  :global(.tiptap-editor ul[data-type="taskList"] li > label input) { accent-color: var(--green-brand); }
  :global(.tiptap-editor ul[data-type="taskList"] li > div) { flex: 1; }

  :global(.tiptap-editor table) { border-collapse: collapse; margin: 16px 0; overflow: hidden; table-layout: fixed; width: 100%; border: 1px solid var(--border-standard); border-radius: 6px; }
  :global(.tiptap-editor td), :global(.tiptap-editor th) { min-width: 1em; border: 1px solid var(--border-subtle); padding: 8px 12px; vertical-align: top; box-sizing: border-box; position: relative; }
  :global(.tiptap-editor th) { background-color: var(--bg-secondary); text-align: left; font-weight: 500; }
  :global(.tiptap-editor .column-resize-handle) { background-color: var(--green-brand); bottom: -2px; pointer-events: none; position: absolute; right: -2px; top: -2px; width: 4px; z-index: 20; }
  
  :global(.tiptap-editor img) { max-width: 100%; height: auto; border-radius: 8px; margin: 16px 0; border: 1px solid var(--border-subtle); }
  :global(.tiptap-editor img.ProseMirror-selectednode) { outline: 2px solid var(--green-brand); }
  
  :global(.tiptap-editor mark) { background-color: rgba(62, 207, 142, 0.2); color: inherit; border-radius: 2px; padding: 0 2px; }
  
  :global(.tiptap-editor .math-node) { background: var(--bg-secondary); padding: 4px 8px; border-radius: 6px; font-family: var(--font-mono); color: var(--green-brand); border: 1px solid var(--border-subtle); }

  :global(.mention-popup) {
    background: var(--bg-primary); border: 1px solid var(--border-standard);
    border-radius: 8px; padding: 6px; min-width: 220px; max-width: 320px;
    z-index: 1000; box-shadow: 0 8px 32px rgba(0,0,0,0.4);
  }
  :global(.mention-item) {
    padding: 8px 12px; border-radius: 6px; cursor: pointer;
    font-size: 14px; color: var(--text-secondary); transition: all 0.15s ease;
  }
  :global(.mention-item.selected) { background: var(--bg-secondary); color: var(--text-primary); border: 1px solid var(--border-subtle); }
  :global(.mention-item:hover) { background: var(--bg-secondary); color: var(--text-primary); }
  :global(.mention-item.empty) { color: var(--text-muted); cursor: default; }

  .empty-state {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    height: 100%; gap: 12px; color: var(--text-muted); background: var(--bg-primary);
  }
  .empty-state p { font-size: 16px; font-weight: 400; }
  .empty-state .hint { font-size: 13px; color: var(--text-heavy); }
</style>
