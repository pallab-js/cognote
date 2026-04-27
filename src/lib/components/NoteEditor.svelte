<!-- ANCHOR: EDITOR_WIKI_LINKS_ACTIVE -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Editor } from '@tiptap/core';
  import StarterKit from '@tiptap/starter-kit';
  import Link from '@tiptap/extension-link';
  import Mention from '@tiptap/extension-mention';
  import { activeNoteId, showToast } from '$lib/stores/app';
  import { wordCount, lastSaved } from '$lib/stores/notes';
  import { getNote, updateNote, listNotes, createNoteLink } from '$lib/commands';
  import type { Note } from '$lib/commands';
  import { Bold, Italic, Heading1, Heading2, Code, Link as LinkIcon, List, ListOrdered } from 'lucide-svelte';

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
  async function getMentionItems(query: string) {
    try {
      const notes = await listNotes();
      return notes
        .filter(n => n.id !== $activeNoteId && n.title.toLowerCase().includes(query.toLowerCase()))
        .slice(0, 8)
        .map(n => ({ id: n.id, label: n.title }));
    } catch { return []; }
  }

  onMount(() => {
    editor = new Editor({
      element: editorEl,
      extensions: [
        StarterKit,
        Link.configure({ openOnClick: false }),
        Mention.configure({
          HTMLAttributes: { class: 'wiki-link' },
          suggestion: {
            items: ({ query }) => getMentionItems(query),
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
                el.innerHTML = its.map((item, i) =>
                  `<div class="mention-item${i === idx ? ' selected' : ''}" data-idx="${i}">${item.label}</div>`
                ).join('') || '<div class="mention-item empty">No notes found</div>';
                el.querySelectorAll('.mention-item[data-idx]').forEach(div => {
                  div.addEventListener('click', () => selectItem(its[parseInt((div as HTMLElement).dataset.idx!)], props));
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
      <button class="tb-btn" title="Bullet list" onclick={() => editor?.chain().focus().toggleBulletList().run()}><List size={13}/></button>
      <button class="tb-btn" title="Ordered list" onclick={() => editor?.chain().focus().toggleOrderedList().run()}><ListOrdered size={13}/></button>
      <div class="tb-sep"></div>
      <span class="tb-hint">Type <code>[[</code> to link a note</span>
    </div>

    <!-- Editor -->
    <div class="editor-content" bind:this={editorEl}></div>
  {:else}
    <div class="empty-state">
      <p>Select a note or create a new one</p>
      <p class="hint">Use the note list on the left</p>
    </div>
  {/if}
</div>

<style>
  .editor-wrap { display: flex; flex-direction: column; height: 100%; overflow: hidden; }

  .title-bar { padding: 16px 24px 0; flex-shrink: 0; }
  .note-title {
    width: 100%; background: none; border: none; outline: none;
    font-size: 24px; font-weight: 400; color: var(--text-primary);
    font-family: var(--font-sans); letter-spacing: -0.16px;
    border-bottom: 1px solid var(--border-subtle); padding-bottom: 8px;
  }
  .note-title::placeholder { color: var(--text-muted); }

  .toolbar {
    display: flex; align-items: center; gap: 2px;
    padding: 8px 24px; border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0; flex-wrap: wrap;
  }
  .tb-btn {
    background: transparent; color: var(--text-secondary);
    padding: 6px 8px; border-radius: 6px; border: 1px solid transparent;
    font: 500 12px var(--font-sans); cursor: pointer;
    display: flex; align-items: center;
  }
  .tb-btn:hover { border-color: var(--border-prominent); color: var(--text-primary); }
  .tb-btn.active { color: var(--green-brand); border-color: var(--green-border); }
  .tb-sep { width: 1px; height: 16px; background: var(--border-standard); margin: 0 4px; }
  .tb-hint { font-size: 11px; color: var(--text-muted); margin-left: auto; }
  .tb-hint code { font-family: var(--font-mono); color: var(--green-brand); }

  .editor-content { flex: 1; overflow-y: auto; padding: 16px 24px; }

  :global(.tiptap-editor) {
    outline: none; min-height: 100%;
    font-size: 15px; line-height: 1.6; color: var(--text-primary);
    font-family: var(--font-sans);
  }
  :global(.tiptap-editor h1) { font-size: 28px; font-weight: 400; margin: 16px 0 8px; line-height: 1.25; }
  :global(.tiptap-editor h2) { font-size: 22px; font-weight: 400; margin: 14px 0 6px; line-height: 1.3; }
  :global(.tiptap-editor p) { margin: 0 0 8px; }
  :global(.tiptap-editor code) { font-family: var(--font-mono); background: var(--border-subtle); padding: 1px 4px; border-radius: 4px; font-size: 13px; color: var(--green-brand); }
  :global(.tiptap-editor pre) { background: var(--bg-button); border: 1px solid var(--border-standard); border-radius: 8px; padding: 12px 16px; margin: 8px 0; overflow-x: auto; }
  :global(.tiptap-editor pre code) { background: none; padding: 0; }
  :global(.tiptap-editor ul, .tiptap-editor ol) { padding-left: 20px; margin: 4px 0 8px; }
  :global(.tiptap-editor li) { margin: 2px 0; }
  :global(.tiptap-editor a) { color: var(--green-link); text-decoration: underline; }
  :global(.tiptap-editor .wiki-link) {
    color: var(--green-link); background: rgba(0, 197, 115, 0.08);
    border-radius: 4px; padding: 1px 4px; cursor: pointer;
    border-bottom: 1px solid var(--green-border);
  }
  :global(.tiptap-editor .wiki-link:hover) { background: rgba(0, 197, 115, 0.15); }

  :global(.mention-popup) {
    background: var(--surface-glass); border: 1px solid var(--border-standard);
    border-radius: 8px; padding: 4px; min-width: 200px; max-width: 300px;
    z-index: 1000; backdrop-filter: blur(8px);
    box-shadow: 0 4px 16px rgba(0,0,0,0.3);
  }
  :global(.mention-item) {
    padding: 6px 10px; border-radius: 6px; cursor: pointer;
    font-size: 13px; color: var(--text-secondary);
  }
  :global(.mention-item.selected) { background: var(--border-standard); color: var(--text-primary); }
  :global(.mention-item:hover) { background: var(--border-subtle); }
  :global(.mention-item.empty) { color: var(--text-muted); cursor: default; }

  .empty-state {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    height: 100%; gap: 8px; color: var(--text-muted);
  }
  .empty-state p { font-size: 15px; }
  .empty-state .hint { font-size: 12px; }
</style>
