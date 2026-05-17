<template>
  <BubbleMenu
    v-if="editor"
    :editor="editor"
    :tippy-options="{ duration: 150, placement: 'top' }"
    class="note-bubble-menu"
  >
    <div v-if="!showAIPanel" class="bubble-menu-container">
      <button class="bubble-btn ai-write-btn" @click="openAIPanel" title="Friday 帮写">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
          <path d="M2 17l10 5 10-5"></path>
          <path d="M2 12l10 5 10-5"></path>
        </svg>
        <span>帮写</span>
      </button>

      <div class="bubble-divider"></div>

      <button class="bubble-btn" @click="handleTranslate" title="翻译">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="m5 8 6 6"></path>
          <path d="m4 14 6-6 2-3"></path>
          <path d="M2 5h12"></path>
          <path d="M7 2h1"></path>
          <path d="m22 22-5-10-5 10"></path>
          <path d="M14 18h6"></path>
        </svg>
        <span>翻译</span>
      </button>

      <button class="bubble-btn" @click="handleRefine" title="精炼">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
        </svg>
        <span>精炼</span>
      </button>

      <button class="bubble-btn" @click="handlePolish" title="润色">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 20h9"></path>
          <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path>
        </svg>
        <span>润色</span>
      </button>

      <button class="bubble-btn" @click="handleExpand" title="扩写">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="15 3 21 3 21 9"></polyline>
          <polyline points="9 21 3 21 3 15"></polyline>
          <line x1="21" y1="3" x2="14" y2="10"></line>
          <line x1="3" y1="21" x2="10" y2="14"></line>
        </svg>
        <span>扩写</span>
      </button>

      <div class="bubble-divider"></div>

      <button class="bubble-btn chat-open-btn" @click="handleOpenInChat" title="对话中打开">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
        </svg>
        <span>对话中打开</span>
      </button>
    </div>

    <div v-else class="ai-input-wrapper" :class="{ 'is-dark': isDark }">
      <textarea
        ref="inputRef"
        v-model="inputText"
        class="ai-textarea"
        :placeholder="selectedText ? `基于选中文本：${selectedText.slice(0, 50)}${selectedText.length > 50 ? '...' : ''}` : '输入问题，或从下方场景提问'"
        rows="1"
        @input="autoResize"
        @keydown.enter.exact.prevent="handleSend"
      ></textarea>

      <div class="input-actions">
        <div class="action-left">
          <div class="command-dropdown">
            <button class="command-btn" @click.stop="toggleCommandMenu" :class="{ active: showCommandMenu }">
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
              </svg>
              <span>AI 指令</span>
              <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                <polyline points="6 9 12 15 18 9"></polyline>
              </svg>
            </button>

            <Transition name="dropdown">
              <div v-if="showCommandMenu" class="command-menu" :class="{ 'menu-up': commandMenuDirection === 'up', 'menu-down': commandMenuDirection === 'down' }">
                <div class="command-item" @click="selectCommand('解读')">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="10"></circle>
                    <line x1="12" y1="16" x2="12" y2="12"></line>
                    <line x1="12" y1="8" x2="12.01" y2="8"></line>
                  </svg>
                  <span>解读</span>
                </div>

                <div class="command-item" @click="selectCommand('总结')">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                    <polyline points="14 2 14 8 20 8"></polyline>
                    <line x1="16" y1="13" x2="8" y2="13"></line>
                    <line x1="16" y1="17" x2="8" y2="17"></line>
                  </svg>
                  <span>总结</span>
                </div>

                <div class="command-item" @click="selectCommand('续写')">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M12 20h9"></path>
                    <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path>
                  </svg>
                  <span>续写</span>
                </div>

                <div class="command-item" @click="selectCommand('语法修正')">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="4 17 10 11 4 5"></polyline>
                    <line x1="12" y1="19" x2="20" y2="19"></line>
                  </svg>
                  <span>语法修正</span>
                </div>

                <div class="command-item" @click="selectCommand('生成任务计划')">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                    <line x1="16" y1="2" x2="16" y2="6"></line>
                    <line x1="8" y1="2" x2="8" y2="6"></line>
                    <line x1="3" y1="10" x2="21" y2="10"></line>
                  </svg>
                  <span>生成任务计划</span>
                </div>

                <div class="command-item" @click="selectCommand('生成表格')">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
                    <line x1="3" y1="9" x2="21" y2="9"></line>
                    <line x1="3" y1="15" x2="21" y2="15"></line>
                    <line x1="9" y1="3" x2="9" y2="21"></line>
                    <line x1="15" y1="3" x2="15" y2="21"></line>
                  </svg>
                  <span>生成表格</span>
                </div>
              </div>
            </Transition>
          </div>
        </div>

        <div class="action-right">
          <Transition name="btn-switch" mode="out-in">
            <button
              key="send"
              class="send-btn"
              :class="{ active: inputText.trim() }"
              @click="handleSend"
              :disabled="!inputText.trim()"
              title="发送"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <line x1="22" y1="2" x2="11" y2="13"></line>
                <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
              </svg>
            </button>
          </Transition>
        </div>
      </div>
    </div>
  </BubbleMenu>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { BubbleMenu } from '@tiptap/vue-3/menus';

const props = defineProps<{
  editor: any;
  isDark?: boolean;
}>();

const emit = defineEmits<{
  aiWrite: [text: string, command?: string];
  translate: [text: string];
  refine: [text: string];
  polish: [text: string];
  expand: [text: string];
  openInChat: [text: string];
}>();

const showAIPanel = ref(false);
const showCommandMenu = ref(false);
const inputText = ref('');
const selectedText = ref('');
const inputRef = ref<HTMLTextAreaElement | null>(null);
const currentCommand = ref('');
const commandMenuDirection = ref<'up' | 'down'>('down');
const isJustOpened = ref(false);

const getSelectedText = () => {
  const { from, to } = props.editor.state.selection;
  return props.editor.state.doc.textBetween(from, to, ' ');
};

const resetAIPanel = () => {
  if (showAIPanel.value) {
    showAIPanel.value = false;
    showCommandMenu.value = false;
    inputText.value = '';
    currentCommand.value = '';
    selectedText.value = '';
  }
};

const openAIPanel = async () => {
  selectedText.value = getSelectedText();
  showAIPanel.value = true;
  await nextTick();
  inputRef.value?.focus();
  if (selectedText.value) {
    inputText.value = '';
  }
  
  await nextTick();
  const commandBtn = document.querySelector('.command-btn') as HTMLElement;
  if (commandBtn) {
    const rect = commandBtn.getBoundingClientRect();
    const menuHeight = 260;
    const spaceBelow = window.innerHeight - rect.bottom;
    const spaceAbove = rect.top;
    
    if (spaceBelow < menuHeight && spaceAbove > spaceBelow) {
      commandMenuDirection.value = 'up';
    } else {
      commandMenuDirection.value = 'down';
    }
  }
  
  showCommandMenu.value = true;
  isJustOpened.value = true;
  setTimeout(() => {
    isJustOpened.value = false;
  }, 100);
};

const closeAIPanel = () => {
  showAIPanel.value = false;
  showCommandMenu.value = false;
  inputText.value = '';
  currentCommand.value = '';
  selectedText.value = '';
};

const toggleCommandMenu = async (e: MouseEvent) => {
  e.stopPropagation();
  
  if (!showCommandMenu.value) {
    const btn = (e.currentTarget as HTMLElement);
    const rect = btn.getBoundingClientRect();
    const menuHeight = 260; 
    const spaceBelow = window.innerHeight - rect.bottom;
    const spaceAbove = rect.top;
    
    if (spaceBelow < menuHeight && spaceAbove > spaceBelow) {
      commandMenuDirection.value = 'up';
    } else {
      commandMenuDirection.value = 'down';
    }
  }
  
  showCommandMenu.value = !showCommandMenu.value;
};

const selectCommand = (command: string) => {
  currentCommand.value = command;
  showCommandMenu.value = false;
  const prefixMap: Record<string, string> = {
    '解读': '请解读以下内容：',
    '总结': '请总结以下内容：',
    '续写': '请续写以下内容：',
    '语法修正': '请修正以下内容的语法错误：',
    '生成任务计划': '请根据以下内容生成任务计划：',
    '生成表格': '请根据以下内容生成表格：'
  };
  
  if (selectedText.value) {
    inputText.value = prefixMap[command] + '\n' + selectedText.value;
  } else {
    inputText.value = prefixMap[command] + '\n';
  }
  
  autoResize();
  nextTick(() => {
    inputRef.value?.focus();
  });
};

const autoResize = () => {
  const textarea = inputRef.value;
  if (!textarea) return;

  textarea.style.height = 'auto';
  const lineHeight = 24;
  const maxHeight = lineHeight * 6;
  const newHeight = Math.min(textarea.scrollHeight, maxHeight);
  
  textarea.style.height = newHeight + 'px';
  textarea.style.overflowY = textarea.scrollHeight > maxHeight ? 'auto' : 'hidden';
};

const handleSend = () => {
  if (!inputText.value.trim()) return;
  
  emit('aiWrite', inputText.value, currentCommand.value || undefined);
  closeAIPanel();
};

const handleTranslate = () => {
  const text = getSelectedText();
  emit('translate', text);
};

const handleRefine = () => {
  const text = getSelectedText();
  emit('refine', text);
};

const handlePolish = () => {
  const text = getSelectedText();
  emit('polish', text);
};

const handleExpand = () => {
  const text = getSelectedText();
  emit('expand', text);
};

const handleOpenInChat = () => {
  const text = getSelectedText();
  emit('openInChat', text);
};

const handleClickOutside = (event: MouseEvent) => {
  if (isJustOpened.value) return;
  
  const target = event.target as HTMLElement;
  if (showCommandMenu.value && !target.closest('.command-dropdown')) {
    showCommandMenu.value = false;
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
  props.editor.on('selectionUpdate', resetAIPanel);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
  props.editor.off('selectionUpdate', resetAIPanel);
});
</script>

<style scoped>
.note-bubble-menu {
  z-index: 1000;
}

.bubble-menu-container {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px;
  background-color: var(--bg-primary, #ffffff);
  border-radius: 10px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12), 0 0 1px rgba(0, 0, 0, 0.08);
  border: 1px solid var(--border-color, #e5e7eb);
  animation: bubble-in 0.15s ease-out;
}

[data-theme='dark'] .bubble-menu-container {
  background-color: #1f2937;
  border-color: #374151;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4), 0 0 1px rgba(0, 0, 0, 0.2);
}

@keyframes bubble-in {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(4px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.bubble-btn {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 4px 10px;
  border: none;
  border-radius: 6px;
  background-color: transparent;
  color: var(--text-primary, #374151);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  white-space: nowrap;
  user-select: none;
}

.bubble-btn:hover {
  background-color: var(--bg-hover, #f3f4f6);
  color: var(--text-primary, #111827);
}

[data-theme='dark'] .bubble-btn:hover {
  background-color: #374151;
  color: #f9fafb;
}

.bubble-btn:active {
  transform: scale(0.97);
}

.bubble-btn svg {
  flex-shrink: 0;
  opacity: 0.8;
}

.bubble-btn:hover svg {
  opacity: 1;
}

.ai-write-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff !important;
  font-weight: 600;
  padding: 4px 12px;
}

.ai-write-btn svg {
  opacity: 1;
  color: #ffffff;
}

.ai-write-btn:hover {
  background: linear-gradient(135deg, #5568d3 0%, #653d91 100%);
  color: #ffffff !important;
  transform: scale(1.02);
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.4);
}

.ai-write-btn:active {
  transform: scale(0.98);
}

.bubble-divider {
  width: 1px;
  height: 20px;
  background-color: var(--border-color, #e5e7eb);
  margin: 0 2px;
}

[data-theme='dark'] .bubble-divider {
  background-color: #4b5563;
}

.ai-input-wrapper {
  position: relative;
  width: 380px;
  max-width: 480px;
  background: var(--bg-primary, #ffffff);
  border: 1.5px solid var(--border-color, #e5e7eb);
  border-radius: 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06), 0 0 1px rgba(0, 0, 0, 0.04);
  animation: panel-in 0.18s ease-out;
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.ai-input-wrapper:focus-within {
  border-color: var(--text-tertiary, #9ca3af);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1), 0 0 1px rgba(0, 0, 0, 0.04);
}

.ai-input-wrapper.is-dark {
  background: #1f2937;
  border-color: #374151;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.25), 0 0 1px rgba(0, 0, 0, 0.1);
}

.ai-input-wrapper.is-dark:focus-within {
  border-color: #6b7280;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.35), 0 0 1px rgba(0, 0, 0, 0.1);
}

@keyframes panel-in {
  from {
    opacity: 0;
    transform: scale(0.96) translateY(4px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.ai-textarea {
  width: 100%;
  padding: 12px 16px 4px;
  border: none;
  outline: none;
  resize: none;
  font-size: 14px;
  line-height: 1.5;
  color: var(--text-primary, #111827);
  background: transparent;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  min-height: 38px;
  max-height: 144px;
  overflow-y: auto;
  overflow-x: hidden;
  box-sizing: border-box;
  border-radius: 20px 20px 0 0;
}

.ai-textarea::-webkit-scrollbar {
  width: 5px;
}

.ai-textarea::-webkit-scrollbar-track {
  background: transparent;
}

.ai-textarea::-webkit-scrollbar-thumb {
  background: var(--border-color, #e5e7eb);
  border-radius: 10px;
}

.ai-textarea::-webkit-scrollbar-thumb:hover {
  background: var(--text-tertiary, #9ca3af);
}

.ai-textarea::placeholder {
  color: var(--text-tertiary, #9ca3af);
}

.is-dark .ai-textarea {
  color: #f9fafb;
}

.is-dark .ai-textarea::placeholder {
  color: #6b7280;
}

.input-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 4px 12px 8px;
}

.action-left,
.action-right {
  display: flex;
  align-items: center;
  gap: 6px;
}

.command-dropdown {
  position: relative;
  display: inline-block;
}

.command-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 6px 10px;
  border: 1px solid var(--border-color, #e5e7eb);
  border-radius: 16px;
  background: transparent;
  color: var(--text-secondary, #6b7280);
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  transition: all 0.15s ease;
  white-space: nowrap;
}

.command-btn:hover {
  background: var(--bg-secondary, #f9fafb);
  border-color: var(--border-color, #d1d5db);
  color: var(--text-primary, #374151);
}

.command-btn.active {
  background: var(--bg-secondary, #f9fafb);
  border-color: #667eea;
  color: #667eea;
}

.is-dark .command-btn {
  border-color: #4b5563;
  color: #9ca3af;
}

.is-dark .command-btn:hover {
  background: #374151;
  border-color: #6b7280;
  color: #d1d5db;
}

.is-dark .command-btn.active {
  background: #374151;
  border-color: #667eea;
  color: #a78bfa;
}

.command-btn svg:last-child {
  transition: transform 0.2s ease;
}

.command-btn.active svg:last-child {
  transform: rotate(180deg);
}

.command-menu {
  position: absolute;
  left: 0;
  min-width: 180px;
  background: var(--bg-primary, #ffffff);
  border: 1px solid var(--border-color, #e5e7eb);
  border-radius: 12px;
  padding: 4px;
  z-index: 1001;
  animation: menu-in 0.12s ease-out;
}

.command-menu.menu-down {
  top: calc(100% + 8px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12), 0 0 1px rgba(0, 0, 0, 0.06);
}

.command-menu.menu-up {
  bottom: calc(100% + 8px);
  box-shadow: 0 -8px 24px rgba(0, 0, 0, 0.12), 0 0 1px rgba(0, 0, 0, 0.06);
}

.is-dark .command-menu {
  background: #374151;
  border-color: #4b5563;
  box-shadow: 0 -8px 24px rgba(0, 0, 0, 0.35), 0 0 1px rgba(0, 0, 0, 0.15);
}

@keyframes menu-in {
  from {
    opacity: 0;
    transform: translateY(-4px) scale(0.96);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.command-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  border-radius: 8px;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary, #374151);
  cursor: pointer;
  transition: all 0.1s ease;
  user-select: none;
}

.command-item:hover {
  background: var(--bg-hover, #f3f4f6);
  color: var(--text-primary, #111827);
}

.is-dark .command-item {
  color: #d1d5db;
}

.is-dark .command-item:hover {
  background: #4b5563;
  color: #f9fafb;
}

.command-item svg {
  flex-shrink: 0;
  color: var(--text-tertiary, #9ca3af);
}

.command-item:hover svg {
  color: #667eea;
}

.send-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 30px;
  border: none;
  border-radius: 50%;
  background: var(--text-tertiary, #9ca3af);
  color: #ffffff;
  cursor: pointer;
  transition: all 0.2s ease;
  margin-left: 2px;
}

.send-btn.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff;
}

.send-btn:hover:not(:disabled) {
  transform: scale(1.06);
}

.send-btn:active:not(:disabled) {
  transform: scale(0.94);
}

.send-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-switch-enter-active {
  transition: all 0.2s ease;
}

.btn-switch-leave-active {
  transition: all 0.15s ease;
}

.btn-switch-enter-from {
  opacity: 0;
  transform: scale(0.7);
}

.btn-switch-leave-to {
  opacity: 0;
  transform: scale(0.7);
}

.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 0.12s ease, transform 0.12s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px) scale(0.96);
}
</style>
