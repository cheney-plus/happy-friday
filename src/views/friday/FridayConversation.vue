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
import { ref, nextTick, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
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
}

const messages = ref<Message[]>([]);

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
</style>
