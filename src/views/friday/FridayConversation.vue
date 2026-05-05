<template>
  <div class="conversation-container">
    <header class="conversation-header">
      <button class="header-btn back-btn" @click="goBack">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M19 12H5"></path>
          <polyline points="12 19 5 12 12 5"></polyline>
        </svg>
      </button>

      <div class="header-center">
        <span class="header-title">{{ chatTitle }}</span>
        <span class="header-time">{{ chatTime }}</span>
      </div>

      <button class="header-btn knowledge-btn" @click="handleAddToKnowledge">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
          <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
          <line x1="12" y1="6" x2="12" y2="13"></line>
          <line x1="9" y1="10" x2="15" y2="10"></line>
        </svg>
      </button>
    </header>

    <main class="conversation-messages" ref="messagesContainer">
      <div class="messages-inner">
        <template v-for="(msg, index) in messages" :key="index">
          <div
            class="message-row"
            :class="msg.role"
          >
            <div v-if="msg.role === 'user'" class="message-content-wrap user">
              <div class="message-bubble user">
                {{ msg.content }}
              </div>
            </div>

            <div v-else class="ai-message-block">
              <div class="ai-header">
                <div class="avatar ai-avatar">
                  <span class="avatar-icon">✦</span>
                </div>
                <span class="ai-name">周五</span>
              </div>
              <div class="ai-body">
                <div class="markdown-body" v-html="renderMarkdown(msg.content)"></div>
              </div>
              <div class="ai-footer">
                <button class="action-icon-btn" title="分享" @click="handleAction('share', index)">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="18" cy="5" r="3"></circle>
                    <circle cx="6" cy="12" r="3"></circle>
                    <circle cx="18" cy="19" r="3"></circle>
                    <line x1="8.59" y1="13.51" x2="15.42" y2="17.49"></line>
                    <line x1="15.41" y1="6.51" x2="8.59" y2="10.49"></line>
                  </svg>
                </button>
                <button class="action-icon-btn" title="添加" @click="handleAction('add', index)">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="10"></circle>
                    <line x1="12" y1="8" x2="12" y2="16"></line>
                    <line x1="8" y1="12" x2="16" y2="12"></line>
                  </svg>
                </button>
                <button class="action-icon-btn" title="复制" @click="handleCopy(msg.content)">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                  </svg>
                </button>
                <button class="action-icon-btn" title="回溯" @click="handleAction('backtrack', index)">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="1 4 1 10 7 10"></polyline>
                    <path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path>
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <div v-if="msg.role === 'assistant'" class="message-divider"></div>
        </template>

        <div v-if="isStreaming" class="message-row assistant">
          <div class="ai-message-block">
            <div class="ai-header">
              <div class="avatar ai-avatar">
                <span class="avatar-icon">✦</span>
              </div>
              <span class="ai-name">周五</span>
            </div>
            <div class="ai-body">
              <div class="markdown-body" v-html="renderMarkdown(streamingContent)"></div>
              <span class="streaming-cursor"></span>
            </div>
          </div>
        </div>
      </div>
    </main>

    <footer class="conversation-input">
      <div class="input-wrapper">
        <textarea
          v-model="inputText"
          class="main-input"
          placeholder="输入消息..."
          rows="1"
          @input="autoResize"
          @keydown.enter.exact="handleSend"
          ref="textareaRef"
        ></textarea>

        <div class="input-actions">
          <div class="action-left">
          </div>

          <div class="action-right">
            <button class="action-btn icon-only">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="4"></circle>
                <path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94"></path>
              </svg>
            </button>

            <button class="action-btn icon-only">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path>
              </svg>
            </button>

            <button class="action-btn icon-only">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="6" cy="6" r="3"></circle>
                <circle cx="6" cy="18" r="3"></circle>
                <line x1="20" y1="4" x2="8.12" y2="15.88"></line>
                <line x1="14.47" y1="14.48" x2="20" y2="20"></line>
                <line x1="8.12" y1="8.12" x2="12" y2="12"></line>
              </svg>
            </button>

            <button class="send-btn" :class="{ active: inputText.trim() }" @click="handleSend">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="22" y1="2" x2="11" y2="13"></line>
                <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { marked } from 'marked';

const router = useRouter();
const route = useRoute();

const inputText = ref('');
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const messagesContainer = ref<HTMLElement | null>(null);
const isStreaming = ref(false);
const streamingContent = ref('');

const chatTitle = ref('与 Friday 的对话');
const chatTime = ref(formatTime(new Date()));

interface Message {
  role: 'user' | 'assistant';
  content: string;
}

const messages = ref<Message[]>([]);

marked.setOptions({
  breaks: true,
  gfm: true
});

function formatTime(date: Date): string {
  const h = date.getHours().toString().padStart(2, '0');
  const m = date.getMinutes().toString().padStart(2, '0');
  return `${h}:${m}`;
}

function renderMarkdown(content: string): string {
  return marked.parse(content) as string;
}

function goBack() {
  router.push('/friday');
}

function handleAddToKnowledge() {}

function handleAction(_action: string, _index: number) {}

async function handleCopy(content: string) {
  try {
    await navigator.clipboard.writeText(content);
  } catch {
    // fallback
  }
}

function autoResize() {
  const textarea = textareaRef.value;
  if (textarea) {
    textarea.style.height = 'auto';
    textarea.style.height = Math.min(textarea.scrollHeight, 160) + 'px';
  }
}

function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
}

const aiResponses = [
  `你好！我是 **Friday**，你的专属个人知识智能服务助手 🎉

我可以帮你完成以下任务：

- 📝 **智能写作** — 辅助创作与润色
- 📄 **文档解读** — 多格式智能分析
- 🔍 **知识检索** — 从你的知识库中查找信息
- 💡 **头脑风暴** — 帮你拓展思路

有什么我可以帮你的吗？`,

  `这是一个很好的问题！让我来为你详细解答。

## 核心要点

1. **理解需求** — 首先要明确目标和约束条件
2. **设计方案** — 制定可行的实施方案
3. **执行落地** — 按计划推进并迭代优化

> 成功的关键在于持续迭代和反馈循环。

如果你需要更深入的分析，随时告诉我！`,

  `好的，我来帮你梳理一下思路。

### 步骤一：分析现状
当前的情况需要从多个维度来评估，包括技术可行性、资源投入和预期收益。

### 步骤二：制定计划
基于分析结果，我建议采用以下方案：

| 维度 | 方案 | 优先级 |
|------|------|--------|
| 技术 | 渐进式迭代 | 高 |
| 资源 | 分阶段投入 | 中 |
| 验证 | A/B 测试 | 高 |

### 步骤三：执行与反馈
在执行过程中，需要持续监控关键指标，及时调整策略。

需要我进一步展开某个方面吗？`
];

function simulateAIResponse() {
  isStreaming.value = true;
  streamingContent.value = '';

  const fullText = aiResponses[messages.value.length % aiResponses.length];
  let charIndex = 0;

  const interval = setInterval(() => {
    if (charIndex < fullText.length) {
      const chunkSize = Math.floor(Math.random() * 3) + 1;
      streamingContent.value += fullText.slice(charIndex, charIndex + chunkSize);
      charIndex += chunkSize;
      scrollToBottom();
    } else {
      clearInterval(interval);
      isStreaming.value = false;
      messages.value.push({
        role: 'assistant',
        content: fullText
      });
      streamingContent.value = '';
      scrollToBottom();
    }
  }, 30);
}

function handleSend(e?: Event) {
  if (e instanceof KeyboardEvent && e.isComposing) return;

  const text = inputText.value.trim();
  if (!text || isStreaming.value) return;

  messages.value.push({
    role: 'user',
    content: text
  });

  inputText.value = '';
  if (textareaRef.value) {
    textareaRef.value.style.height = 'auto';
  }

  scrollToBottom();

  setTimeout(() => {
    simulateAIResponse();
  }, 500);
}

onMounted(() => {
  const query = route.query.q as string;
  if (query) {
    messages.value.push({
      role: 'user',
      content: query
    });
    nextTick(() => {
      simulateAIResponse();
    });
  }
});
</script>

<style scoped>
.conversation-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  background-color: var(--bg-primary);
  overflow: hidden;
}

.conversation-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  flex-shrink: 0;
  -webkit-app-region: drag;
  app-region: drag;
}

.header-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 10px;
  transition: all 0.15s ease;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.header-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.header-center {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.header-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: -0.01em;
}

.header-time {
  font-size: 12px;
  color: var(--text-tertiary);
}

.conversation-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px 0;
  scroll-behavior: smooth;
}

.conversation-messages::-webkit-scrollbar {
  width: 5px;
}

.conversation-messages::-webkit-scrollbar-track {
  background: transparent;
}

.conversation-messages::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 10px;
}

.messages-inner {
  max-width: 800px;
  margin: 0 auto;
  padding: 0 24px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.message-row {
  display: flex;
  align-items: flex-start;
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

.message-row.user {
  justify-content: flex-end;
}

.message-content-wrap.user {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  max-width: 70%;
}

.message-bubble {
  padding: 10px 18px;
  border-radius: 16px;
  font-size: 14.5px;
  line-height: 1.6;
  word-break: break-word;
}

.message-bubble.user {
  background: #2a2a2e;
  color: #f0f0f2;
  border-bottom-right-radius: 4px;
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

.conversation-input {
  flex-shrink: 0;
  padding: 8px 58px 14px;
}

.input-wrapper {
  max-width: 800px;
  margin: 0 auto;
  background: #ffffff;
  border: 1.5px solid #e5e5e5;
  border-radius: 22px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  overflow: hidden;
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.input-wrapper:focus-within {
  border-color: #d4d4d4;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.main-input {
  width: 100%;
  padding: 12px 18px 4px;
  border: none;
  outline: none;
  resize: none;
  font-size: 15px;
  line-height: 1.5;
  color: #1a1a1a;
  background: transparent;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  min-height: 38px;
  max-height: 160px;
  overflow-y: auto;
}

.main-input::-webkit-scrollbar {
  width: 5px;
}

.main-input::-webkit-scrollbar-track {
  background: transparent;
}

.main-input::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 10px;
}

.main-input::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

.main-input::placeholder {
  color: #9ca3af;
}

.input-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 4px 14px 8px;
}

.action-left,
.action-right {
  display: flex;
  align-items: center;
  gap: 6px;
}

.action-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 6px 10px;
  border: none;
  background: transparent;
  color: #374151;
  cursor: pointer;
  border-radius: 16px;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.15s ease;
  white-space: nowrap;
}

.action-btn:hover {
  background: #f3f4f6;
}

.icon-only {
  padding: 6px 8px;
}

.icon-only:not(.send-btn) {
  border: 1px solid #e5e7eb;
  border-radius: 50%;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
}

.send-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: #9ca3af;
  color: #ffffff;
  cursor: pointer;
  border-radius: 50%;
  transition: all 0.2s ease;
  margin-left: 2px;
}

.send-btn.active {
  background: #374151;
  color: #ffffff;
}

.send-btn:hover {
  transform: scale(1.06);
}
</style>
