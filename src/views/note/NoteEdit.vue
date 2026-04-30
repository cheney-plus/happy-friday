<template>
  <div class="note-edit" :class="{ 'is-dark': isDark }">
    <div class="note-edit-header">
      <div class="header-left">
        <button class="header-btn" @click="goBack" :title="t('note.back')">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"></polyline></svg>
        </button>
        <span class="header-divider"></span>
        <button class="header-btn" :title="t('note.undo')" @click="handleUndo">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"></polyline><path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path></svg>
        </button>
        <button class="header-btn" :title="t('note.redo')" @click="handleRedo">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 4 23 10 17 10"></polyline><path d="M20.49 15a9 9 0 1 1-2.13-9.36L23 10"></path></svg>
        </button>
      </div>

      <div class="header-center">
        <input
          v-model="noteTitle"
          class="note-title-input"
          type="text"
          :placeholder="t('note.untitled')"
          @input="onTitleChange"
        />
      </div>

      <div class="header-right">
        <span class="save-status" :class="{ saved: isSaved }">
          <svg v-if="isSaved" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
          <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle></svg>
          {{ isSaved ? t('note.saved') : t('note.unsaved') }}
        </span>
        <button class="header-btn" :title="t('note.export')" @click="handleExport">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
        </button>
        <button class="header-btn" :title="t('note.more')" @click="toggleMoreMenu">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="1"></circle><circle cx="12" cy="5" r="1"></circle><circle cx="12" cy="19" r="1"></circle></svg>
        </button>
      </div>

      <Teleport to="body">
        <div v-if="moreMenuVisible" class="note-more-menu" :style="moreMenuStyle">
          <div class="more-menu-item" @click="handleToggleReadonly">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <template v-if="isReadonly">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle>
              </template>
              <template v-else>
                <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path><line x1="1" y1="1" x2="23" y2="23"></line>
              </template>
            </svg>
            {{ isReadonly ? t('note.editMode') : t('note.readonlyMode') }}
          </div>
          <div class="more-menu-item" @click="handleCopyMarkdown">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
            {{ t('note.copyMarkdown') }}
          </div>
          <div class="more-menu-divider"></div>
          <div class="more-menu-item danger" @click="handleDelete">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
            {{ t('note.delete') }}
          </div>
        </div>
      </Teleport>
    </div>

    <div class="note-edit-body">
      <MilkdownProvider>
        <NoteEditor
          :placeholder="t('note.editorPlaceholder')"
          @ready="onEditorReady"
          @change="onEditorChange"
        />
      </MilkdownProvider>
    </div>

    <div class="note-edit-footer">
      <div class="footer-left">
        <span class="word-count">{{ wordCount }} {{ t('note.words') }}</span>
        <span class="char-count">{{ charCount }} {{ t('note.characters') }}</span>
      </div>
      <div class="footer-right">
        <span class="note-id" v-if="noteId">ID: {{ noteId }}</span>
        <span class="last-saved" v-if="lastSavedTime">{{ t('note.lastSaved') }} {{ lastSavedTime }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted, onBeforeUnmount, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { MilkdownProvider } from '@milkdown/vue';
import { Crepe } from '@milkdown/crepe';
import { undoCommand, redoCommand } from '@milkdown/kit/plugin/history';
import { useAppStore } from '@/store';
import NoteEditor from './NoteEditor.vue';

import '@milkdown/crepe/theme/common/style.css';
import '@milkdown/crepe/theme/classic.css';

const route = useRoute();
const router = useRouter();
const { t } = useI18n();
const appStore = useAppStore();

const noteId = computed(() => route.params.id as string);
const isDark = computed(() => appStore.theme === 'dark');
const noteTitle = ref('');
const isSaved = ref(true);
const isReadonly = ref(false);
const wordCount = ref(0);
const charCount = ref(0);
const lastSavedTime = ref('');
const moreMenuVisible = ref(false);
const moreMenuStyle = reactive({ left: '0px', top: '0px' });

let crepeInstance: Crepe | null = null;

const onEditorReady = (crepe: Crepe) => {
  crepeInstance = crepe;
};

const onEditorChange = (markdown: string) => {
  updateStats(markdown);
  isSaved.value = false;
};

const updateStats = (markdown: string) => {
  const text = markdown.replace(/[#*_~`>\[\]()!|\-]/g, ' ').replace(/\s+/g, ' ').trim();
  wordCount.value = text ? text.split(/\s+/).filter(Boolean).length : 0;
  charCount.value = text.length;
};

const goBack = () => {
  router.push({ name: 'note' });
};

const onTitleChange = () => {
  isSaved.value = false;
};

const handleUndo = () => {
  try {
    undoCommand.run();
  } catch {}
};

const handleRedo = () => {
  try {
    redoCommand.run();
  } catch {}
};

const handleExport = () => {
  if (!crepeInstance) return;
  const markdown = crepeInstance.getMarkdown();
  const blob = new Blob([markdown], { type: 'text/markdown;charset=utf-8' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${noteTitle.value || t('note.untitled')}.md`;
  a.click();
  URL.revokeObjectURL(url);
};

const toggleMoreMenu = async (e: MouseEvent) => {
  if (moreMenuVisible.value) {
    moreMenuVisible.value = false;
    return;
  }
  const target = e.currentTarget as HTMLElement;
  const rect = target.getBoundingClientRect();
  moreMenuStyle.left = `${rect.right - 180}px`;
  moreMenuStyle.top = `${rect.bottom + 4}px`;
  moreMenuVisible.value = true;
};

const handleToggleReadonly = () => {
  isReadonly.value = !isReadonly.value;
  if (crepeInstance) {
    crepeInstance.setReadonly(isReadonly.value);
  }
  moreMenuVisible.value = false;
};

const handleCopyMarkdown = () => {
  if (!crepeInstance) return;
  const markdown = crepeInstance.getMarkdown();
  navigator.clipboard.writeText(markdown);
  moreMenuVisible.value = false;
};

const handleDelete = () => {
  moreMenuVisible.value = false;
  router.push({ name: 'note' });
};

const handleClickOutside = () => {
  if (moreMenuVisible.value) {
    moreMenuVisible.value = false;
  }
};

watch(isDark, async (dark) => {
  const milkdownEl = document.querySelector('.milkdown');
  if (milkdownEl) {
    milkdownEl.classList.toggle('crepe-dark', dark);
  }
});

onMounted(() => {
  document.addEventListener('click', handleClickOutside);

  if (isDark.value) {
    watch(() => crepeInstance, () => {
      const milkdownEl = document.querySelector('.milkdown');
      if (milkdownEl) {
        milkdownEl.classList.add('crepe-dark');
      }
    }, { once: true });
  }
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
  crepeInstance = null;
});
</script>

<style scoped>
.note-edit {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  background-color: var(--bg-primary);
}

.note-edit-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  border-bottom: 1px solid var(--border-color);
  gap: 12px;
  min-height: 48px;
}

.header-left,
.header-right {
  display: flex;
  align-items: center;
  gap: 2px;
  flex-shrink: 0;
}

.header-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  border: none;
  background-color: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s;
}

.header-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.header-divider {
  width: 1px;
  height: 16px;
  background-color: var(--border-color);
  margin: 0 4px;
}

.header-center {
  flex: 1;
  display: flex;
  justify-content: center;
  min-width: 0;
}

.note-title-input {
  width: 100%;
  max-width: 480px;
  border: none;
  outline: none;
  background: transparent;
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
  text-align: center;
  padding: 6px 12px;
  border-radius: 6px;
  transition: background-color 0.15s;
}

.note-title-input::placeholder {
  color: var(--text-tertiary);
}

.note-title-input:hover {
  background-color: var(--bg-hover);
}

.note-title-input:focus {
  background-color: var(--bg-hover);
}

.save-status {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-tertiary);
  padding: 4px 8px;
  border-radius: 4px;
  user-select: none;
}

.save-status.saved {
  color: #22c55e;
}

.note-edit-body {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.note-edit-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 16px;
  border-top: 1px solid var(--border-color);
  font-size: 12px;
  color: var(--text-tertiary);
  user-select: none;
}

.footer-left,
.footer-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.note-more-menu {
  position: fixed;
  z-index: 99999;
  background-color: white;
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.12), 0 0 1px rgba(0, 0, 0, 0.08);
  padding: 6px 0;
  min-width: 180px;
  animation: menu-in 0.12s ease-out;
}

.is-dark .note-more-menu {
  background-color: #2a2725;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.4), 0 0 1px rgba(0, 0, 0, 0.2);
}

@keyframes menu-in {
  from { opacity: 0; transform: scale(0.96) translateY(-4px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}

.more-menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 16px;
  font-size: 13px;
  color: var(--text-primary);
  cursor: pointer;
  transition: background-color 0.1s;
  user-select: none;
}

.more-menu-item:hover {
  background-color: var(--bg-hover);
}

.more-menu-item.danger {
  color: #ef4444;
}

.more-menu-item svg {
  flex-shrink: 0;
  color: var(--text-secondary);
}

.more-menu-item.danger svg {
  color: #ef4444;
}

.more-menu-divider {
  height: 1px;
  background-color: var(--border-color);
  margin: 4px 12px;
}
</style>

<style>
.milkdown.crepe-dark {
  --crepe-color-background: #1c1917;
  --crepe-color-on-background: rgba(255, 255, 255, 0.92);
  --crepe-color-surface: #232120;
  --crepe-color-surface-low: #2a2725;
  --crepe-color-on-surface: rgba(255, 255, 255, 0.85);
  --crepe-color-on-surface-variant: rgba(255, 255, 255, 0.6);
  --crepe-color-outline: rgba(255, 255, 255, 0.2);
  --crepe-color-primary: #f4bd6f;
  --crepe-color-secondary: #56442a;
  --crepe-color-on-secondary: #fbdebc;
  --crepe-color-inverse: #ede0d4;
  --crepe-color-on-inverse: #362f27;
  --crepe-color-inline-code: #ffb4ab;
  --crepe-color-error: #ffb4ab;
  --crepe-color-hover: #2e2b28;
  --crepe-color-selected: #3b342b;
  --crepe-color-inline-area: #3f3830;
  --crepe-shadow-1: 0px 1px 2px 0px rgba(0, 0, 0, 0.6), 0px 1px 3px 1px rgba(0, 0, 0, 0.3);
  --crepe-shadow-2: 0px 2px 6px 2px rgba(0, 0, 0, 0.4), 0px 1px 2px 0px rgba(0, 0, 0, 0.5);
}

.milkdown {
  --crepe-color-background: transparent;
  --crepe-shadow-1: none;
  --crepe-shadow-2: none;
}

.milkdown .ProseMirror {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
}

.milkdown .ProseMirror h1 {
  font-size: 32px;
  font-weight: 700;
  line-height: 1.3;
  margin-bottom: 8px;
  color: var(--text-primary);
}

.milkdown .ProseMirror h2 {
  font-size: 24px;
  font-weight: 600;
  line-height: 1.35;
  margin-top: 24px;
  margin-bottom: 8px;
  color: var(--text-primary);
}

.milkdown .ProseMirror h3 {
  font-size: 20px;
  font-weight: 600;
  line-height: 1.4;
  margin-top: 20px;
  margin-bottom: 6px;
  color: var(--text-primary);
}

.milkdown .ProseMirror p {
  font-size: 15px;
  line-height: 1.75;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.milkdown .ProseMirror blockquote {
  border-left: 3px solid var(--accent-color);
  padding-left: 16px;
  margin: 12px 0;
  color: var(--text-secondary);
}

.milkdown .ProseMirror code {
  font-size: 13px;
  padding: 2px 6px;
  border-radius: 4px;
  background-color: var(--bg-hover);
  color: var(--crepe-color-inline-code, #ba1a1a);
}

.milkdown .ProseMirror pre {
  border-radius: 8px;
  margin: 12px 0;
}

.milkdown .ProseMirror hr {
  border: none;
  border-top: 1px solid var(--border-color);
  margin: 24px 0;
}

.milkdown .ProseMirror a {
  color: var(--accent-color);
  text-decoration: underline;
  text-underline-offset: 2px;
}

.milkdown .ProseMirror img {
  border-radius: 8px;
  max-width: 100%;
}

.milkdown .ProseMirror ul,
.milkdown .ProseMirror ol {
  padding-left: 24px;
  margin: 8px 0;
}

.milkdown .ProseMirror li {
  font-size: 15px;
  line-height: 1.75;
  color: var(--text-primary);
}

.milkdown .ProseMirror .tableWrapper {
  border-radius: 8px;
  overflow: hidden;
  margin: 12px 0;
}

.milkdown .ProseMirror table {
  border-collapse: collapse;
  width: 100%;
}

.milkdown .ProseMirror th,
.milkdown .ProseMirror td {
  border: 1px solid var(--border-color);
  padding: 8px 12px;
  text-align: left;
}

.milkdown .ProseMirror th {
  background-color: var(--bg-hover);
  font-weight: 600;
}
</style>
