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
          <UserMessage v-if="msg.role === 'user'" :content="msg.content" />
          <AIMessage
            v-else
            :content="msg.content"
            :show-divider="true"
            @action="(type) => handleAction(type, index)"
          />
        </template>

        <AIMessage
          v-if="isStreaming"
          :content="streamingContent"
          :is-streaming="true"
          :show-divider="false"
        />
      </div>
    </main>

    <ChatInputBox
      v-model="inputText"
      placeholder="输入消息..."
      @send="handleSend"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted, onUnmounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import UserMessage from '@/components/chat/UserMessage.vue';
import AIMessage from '@/components/chat/AIMessage.vue';
import ChatInputBox from '@/components/chat/ChatInputBox.vue';

const router = useRouter();
const route = useRoute();

const inputText = ref('');
const messagesContainer = ref<HTMLElement | null>(null);
const isStreaming = ref(false);
const streamingContent = ref('');

const chatTitle = ref('与 Friday 的对话');
const chatTime = ref(formatTime(new Date()));

interface Message {
  role: 'user' | 'assistant';
  content: string;
  id?: number;
}

const messages = ref<Message[]>([]);

const currentMode = ref<string>('');
const currentSessionId = ref<string>('');
let unlistenChunk: UnlistenFn | null = null;
let unlistenDone: UnlistenFn | null = null;
let unlistenError: UnlistenFn | null = null;
let unlistenTitle: UnlistenFn | null = null;
let activeRequestId = '';
let isDoneReceived = false;

function formatTime(date: Date): string {
  const h = date.getHours().toString().padStart(2, '0');
  const m = date.getMinutes().toString().padStart(2, '0');
  return `${h}:${m}`;
}

function goBack() {
  router.push('/friday');
}

function handleAddToKnowledge() {}

function handleAction(_action: string, _index: number) {}

function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
}

function loadModelConfig(modelId: string) {
  try {
    const stored = localStorage.getItem('happy-friday-custom-models');
    if (stored) {
      const models = JSON.parse(stored);
      let model = models.find((m: any) => m.id === modelId);
      if (!model && models.length > 0) {
        const selectedId = localStorage.getItem('happy-friday-selected-model');
        model = selectedId ? models.find((m: any) => m.id === selectedId) : models[0];
      }
      return model || null;
    }
  } catch (e) {
    console.error('Failed to load model config:', e);
  }
  return null;
}

async function sendChatMessage(text: string) {
  if (isStreaming.value || !text.trim()) return;

  const mode = route.query.mode as string || 'chat';
  const modelId = route.query.modelId as string || '';
  const sessionId = route.query.sessionId as string || '';
  const model = loadModelConfig(modelId);

  if (!model) {
    console.error('No model config found');
    return;
  }

  messages.value.push({
    role: 'user',
    content: text
  });

  inputText.value = '';
  isStreaming.value = true;
  streamingContent.value = '';
  scrollToBottom();

  activeRequestId = `req_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
  isDoneReceived = false;

  try {
    if (mode === 'chat') {
      await invoke<Record<string, any>>('chat_with_memory', {
        requestId: activeRequestId,
        sessionId: currentSessionId.value || '',
        model: model,
        message: text
      });
    } else {
      await invoke<void>('chat_without_memory', {
        requestId: activeRequestId,
        model: model,
        message: text
      });
    }
  } catch (err) {
    console.error('Chat invoke error:', err);
    isStreaming.value = false;
    streamingContent.value = '';
  }
}

function handleSend(e?: Event) {
  if (e instanceof KeyboardEvent && e.isComposing) return;
  sendChatMessage(inputText.value);
}

async function loadSessionHistory(sessionId: string) {
  try {
    const history = await invoke<Array<{ id: number; session_id: string; role: string; content: string; created_at: string }>>(
      'get_session_messages',
      { sessionId }
    );
    messages.value = history.map(m => ({
      role: m.role as 'user' | 'assistant',
      content: m.content,
      id: m.id
    }));
  } catch (err) {
    console.error('Failed to load session history:', err);
  }
}

async function triggerAiResponse() {
  if (isStreaming.value) return;

  const mode = route.query.mode as string || 'chat';
  const modelId = route.query.modelId as string || '';
  const model = loadModelConfig(modelId);

  if (!model) return;

  isStreaming.value = true;
  streamingContent.value = '';
  scrollToBottom();

  activeRequestId = `req_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
  isDoneReceived = false;

  try {
    if (mode === 'chat') {
      await invoke<Record<string, any>>('chat_with_memory', {
        requestId: activeRequestId,
        sessionId: currentSessionId.value || '',
        model: model,
        message: ''
      });
    }
  } catch (err) {
    console.error('Chat invoke error:', err);
    isStreaming.value = false;
    streamingContent.value = '';
  }
}

async function initConversation() {
  isStreaming.value = false;
  streamingContent.value = '';
  messages.value = [];
  activeRequestId = '';
  isDoneReceived = false;
  chatTitle.value = '与 Friday 的对话';
  chatTime.value = formatTime(new Date());

  currentMode.value = route.query.mode as string || 'chat';
  currentSessionId.value = (route.params.sessionId as string) || '';
  if (currentSessionId.value.startsWith('new-')) {
    currentSessionId.value = '';
  }

  if (currentMode.value === 'chat' && currentSessionId.value) {
    const queryTitle = route.query.title as string;
    if (queryTitle) {
      chatTitle.value = queryTitle;
    }
    await loadSessionHistory(currentSessionId.value);
    try {
      const sessionInfo = await invoke<{ id: string; title: string }>('get_session', { sessionId: currentSessionId.value });
      if (sessionInfo) {
        chatTitle.value = sessionInfo.title;
      }
    } catch {}
  }

  const query = route.query.q as string;
  if (query) {
    const alreadyHasMessage = messages.value.length > 0
      && messages.value[messages.value.length - 1].role === 'user'
      && messages.value[messages.value.length - 1].content === query;

    if (alreadyHasMessage) {
      await triggerAiResponse();
    } else {
      sendChatMessage(query);
    }
  }
}

onMounted(async () => {
  unlistenChunk = await listen<{ requestId: string; sessionId?: string; content: string }>(
    'chat-chunk',
    (event) => {
      if (event.payload.requestId !== activeRequestId) return;
      streamingContent.value += event.payload.content;
      scrollToBottom();
    }
  );

  unlistenDone = await listen<{ requestId: string; sessionId?: string; fullContent: string; messageId?: number }>(
    'chat-done',
    (event) => {
      if (event.payload.requestId !== activeRequestId) return;
      if (isDoneReceived) return;
      isDoneReceived = true;

      isStreaming.value = false;
      if (streamingContent.value || event.payload.fullContent) {
        messages.value.push({
          role: 'assistant',
          content: event.payload.fullContent || streamingContent.value,
          id: event.payload.messageId
        });
      }

      if (event.payload.sessionId && !currentSessionId.value) {
        currentSessionId.value = event.payload.sessionId;
      }

      streamingContent.value = '';
      scrollToBottom();
    }
  );

  unlistenError = await listen<{ requestId: string; sessionId?: string; error: string }>(
    'chat-error',
    (event) => {
      if (event.payload.requestId !== activeRequestId) return;
      isStreaming.value = false;
      streamingContent.value = '';
      console.error('Stream error:', event.payload.error);
    }
  );

  unlistenTitle = await listen<{ sessionId: string; title: string }>(
    'session-title-updated',
    (event) => {
      if (event.payload.sessionId === currentSessionId.value) {
        chatTitle.value = event.payload.title;
      }
    }
  );

  await initConversation();
});

onUnmounted(() => {
  unlistenChunk?.();
  unlistenDone?.();
  unlistenError?.();
  unlistenTitle?.();
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
</style>
