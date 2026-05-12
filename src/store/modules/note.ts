import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

export interface Note {
  id: string;
  knowledgeBaseId: string | null;
  title: string;
  content: string;
  contentText: string;
  isDeleted: boolean;
  createdAt: string;
  updatedAt: string;
}

const DEBOUNCE_MS = 800;

export const useNoteStore = defineStore('note', {
  state: () => ({
    notes: [] as Note[],
    currentNoteId: null as string | null,
    loading: false,
    saving: false,
    _saveTimer: null as ReturnType<typeof setTimeout> | null,
    _pendingSave: null as { noteId: string; title: string; content: string; contentText: string } | null,
  }),

  getters: {
    currentNote(state): Note | undefined {
      return state.notes.find(n => n.id === state.currentNoteId);
    },
  },

  actions: {
    async fetchNotes(knowledgeBaseId?: string) {
      this.loading = true;
      try {
        const notes = await invoke<Note[]>('get_notes', {
          knowledgeBaseId: knowledgeBaseId ?? null,
        });
        this.notes = notes;
      } finally {
        this.loading = false;
      }
    },

    async fetchNote(noteId: string) {
      const note = await invoke<Note | null>('get_note', { noteId });
      if (note) {
        const idx = this.notes.findIndex(n => n.id === noteId);
        if (idx >= 0) {
          this.notes[idx] = note;
        } else {
          this.notes.unshift(note);
        }
      }
      return note;
    },

    async createNote(knowledgeBaseId?: string, title?: string) {
      const note = await invoke<Note>('create_note', {
        knowledgeBaseId: knowledgeBaseId ?? null,
        title: title ?? null,
      });
      this.notes.unshift(note);
      this.currentNoteId = note.id;
      return note;
    },

    async deleteNote(noteId: string) {
      await invoke('delete_note', { noteId });
      this.notes = this.notes.filter(n => n.id !== noteId);
      if (this.currentNoteId === noteId) {
        this.currentNoteId = this.notes.length > 0 ? this.notes[0].id : null;
      }
    },

    async searchNotes(query: string) {
      this.loading = true;
      try {
        const notes = await invoke<Note[]>('search_notes', { query });
        this.notes = notes;
      } finally {
        this.loading = false;
      }
    },

    selectNote(noteId: string) {
      this.currentNoteId = noteId;
    },

    scheduleSave(noteId: string, title: string, content: string, contentText: string) {
      this._pendingSave = { noteId, title, content, contentText };

      if (this._saveTimer) {
        clearTimeout(this._saveTimer);
      }

      this._saveTimer = setTimeout(() => {
        this._flushSave();
      }, DEBOUNCE_MS);
    },

    async flushPendingSave() {
      if (this._saveTimer) {
        clearTimeout(this._saveTimer);
        this._saveTimer = null;
      }
      if (this._pendingSave) {
        await this._flushSave();
      }
    },

    async _flushSave() {
      this._saveTimer = null;
      const pending = this._pendingSave;
      if (!pending) return;

      this._pendingSave = null;
      this.saving = true;
      try {
        await invoke('update_note', {
          noteId: pending.noteId,
          title: pending.title,
          content: pending.content,
          contentText: pending.contentText,
        });

        const note = this.notes.find(n => n.id === pending.noteId);
        if (note) {
          note.title = pending.title;
          note.content = pending.content;
          note.contentText = pending.contentText;
          note.updatedAt = new Date().toISOString();
        }
      } finally {
        this.saving = false;
      }
    },
  },
});
