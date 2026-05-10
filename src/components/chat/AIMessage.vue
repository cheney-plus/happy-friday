<template>
  <div class="ai-message-wrapper">
    <div class="ai-message-block">
      <div class="ai-header">
        <div class="avatar ai-avatar">
          <span class="avatar-icon">✦</span>
        </div>
        <span class="ai-name">{{ displayName }}</span>
      </div>

      <div class="ai-body">
        <div class="markdown-body" v-html="renderedContent"></div>
        <span v-if="isStreaming" class="streaming-cursor"></span>
      </div>

      <div v-if="!isStreaming" class="ai-footer">
        <button class="action-icon-btn" title="分享" @click="$emit('action', 'share')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="18" cy="5" r="3"></circle>
            <circle cx="6" cy="12" r="3"></circle>
            <circle cx="18" cy="19" r="3"></circle>
            <line x1="8.59" y1="13.51" x2="15.42" y2="17.49"></line>
            <line x1="15.41" y1="6.51" x2="8.59" y2="10.49"></line>
          </svg>
        </button>
        <button class="action-icon-btn" title="添加" @click="$emit('action', 'add')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="16"></line>
            <line x1="8" y1="12" x2="16" y2="12"></line>
          </svg>
        </button>
        <button class="action-icon-btn" title="复制" @click="handleCopy">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
          </svg>
        </button>
        <button class="action-icon-btn" title="回退" @click="$emit('action', 'rollback')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="9 14 4 9 9 4"></polyline>
            <path d="M20 20v-7a4 4 0 0 0-4-4H4"></path>
          </svg>
        </button>
      </div>
    </div>

    <div v-if="showDivider && !isStreaming" class="message-divider"></div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { marked } from 'marked';

marked.setOptions({ breaks: true, gfm: true });

const props = withDefaults(defineProps<{
  content: string;
  displayName?: string;
  showDivider?: boolean;
  isStreaming?: boolean;
}>(), {
  displayName: '周五',
  showDivider: true,
  isStreaming: false
});

defineEmits<{
  (e: 'action', type: string): void;
}>();

const renderedContent = computed(() => marked.parse(props.content) as string);

async function handleCopy() {
  try {
    await navigator.clipboard.writeText(props.content);
  } catch {
  }
}
</script>

<style scoped>
.ai-message-wrapper {
  display: flex;
  flex-direction: column;
}

.ai-message-block {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
}

.ai-header {
  display: flex;
  align-items: center;
  gap: 10px;
}

.avatar {
  width: 34px;
  height: 34px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.ai-avatar {
  background: linear-gradient(135deg, #6ee7b7 0%, #34d399 50%, #10b981 100%);
}

.avatar-icon {
  font-size: 16px;
  color: #ffffff;
  font-weight: 700;
}

.ai-name {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: -0.01em;
}

.ai-body {
  padding-left: 44px;
  font-size: 14.5px;
  line-height: 1.7;
  color: var(--text-primary);
}

.markdown-body {
  white-space: normal;
}

.markdown-body :deep(p) {
  margin: 0 0 8px;
}

.markdown-body :deep(p:last-child) {
  margin-bottom: 0;
}

.markdown-body :deep(h1),
.markdown-body :deep(h2),
.markdown-body :deep(h3) {
  margin: 16px 0 8px;
  font-weight: 600;
  color: var(--text-primary);
}

.markdown-body :deep(h1) { font-size: 1.3em; }
.markdown-body :deep(h2) { font-size: 1.15em; }
.markdown-body :deep(h3) { font-size: 1.05em; }

.markdown-body :deep(ul),
.markdown-body :deep(ol) {
  margin: 8px 0;
  padding-left: 20px;
}

.markdown-body :deep(li) {
  margin: 4px 0;
}

.markdown-body :deep(blockquote) {
  margin: 10px 0;
  padding: 8px 14px;
  border-left: 3px solid #10b981;
  background: rgba(16, 185, 129, 0.06);
  border-radius: 0 8px 8px 0;
  color: var(--text-secondary);
}

.markdown-body :deep(code) {
  background: rgba(0, 0, 0, 0.06);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 0.9em;
  font-family: 'SF Mono', 'Fira Code', monospace;
}

[data-theme='dark'] .markdown-body :deep(code) {
  background: rgba(255, 255, 255, 0.1);
}

.markdown-body :deep(pre) {
  margin: 10px 0;
  padding: 14px;
  background: rgba(0, 0, 0, 0.04);
  border-radius: 10px;
  overflow-x: auto;
}

[data-theme='dark'] .markdown-body :deep(pre) {
  background: rgba(255, 255, 255, 0.06);
}

.markdown-body :deep(pre code) {
  background: transparent;
  padding: 0;
  font-size: 0.85em;
}

.markdown-body :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin: 10px 0;
  font-size: 0.9em;
}

.markdown-body :deep(th),
.markdown-body :deep(td) {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  text-align: left;
}

.markdown-body :deep(th) {
  background: var(--bg-hover);
  font-weight: 600;
}

.markdown-body :deep(hr) {
  border: none;
  border-top: 1px solid var(--border-color);
  margin: 12px 0;
}

.streaming-cursor {
  display: inline-block;
  width: 2px;
  height: 16px;
  background: #10b981;
  margin-left: 2px;
  vertical-align: text-bottom;
  animation: blink 0.8s infinite;
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.ai-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 2px;
  padding-left: 44px;
}

.action-icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 30px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  border-radius: 8px;
  transition: all 0.15s ease;
}

.action-icon-btn:hover {
  background: var(--bg-hover);
  color: var(--text-secondary);
}

.message-divider {
  width: 100%;
  height: 1px;
  background: var(--border-color);
}
</style>
