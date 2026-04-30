<template>
  <div class="friday-container">
    <div class="friday-content">
      <div class="logo-section">
        <div class="logo-badge">培养专属copilot</div>
        <div class="logo-main">
          <span class="logo-icon">✦</span>
          <h1 class="logo-text">tma</h1>
        </div>
        <p class="logo-subtitle">copilot</p>
      </div>

      <div class="input-section">
        <div class="input-wrapper">
          <textarea
            v-model="inputText"
            class="main-input"
            :placeholder="t('friday.placeholder')"
            rows="1"
            @input="autoResize"
            ref="textareaRef"
          ></textarea>

          <div class="input-actions">
            <div class="action-left">
              <button class="action-btn dropdown-btn">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
                </svg>
                <span>对话模式</span>
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="6 9 12 15 18 9"></polyline>
                </svg>
              </button>

              <button class="action-btn dropdown-btn">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"></circle>
                  <line x1="2" y1="12" x2="22" y2="12"></line>
                  <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
                </svg>
                <span>DS 快速</span>
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="6 9 12 15 18 9"></polyline>
                </svg>
              </button>

              <button class="action-btn icon-only">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="4"></circle>
                  <path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94"></path>
                </svg>
              </button>
            </div>

            <div class="action-right">
              <button class="action-btn icon-only">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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

              <button class="send-btn" :class="{ active: inputText.trim() }">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="22" y1="2" x2="11" y2="13"></line>
                  <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="features-section">
        <div
          v-for="feature in features"
          :key="feature.id"
          class="feature-card"
          @click="handleFeatureClick(feature.id)"
        >
          <div class="feature-icon" v-html="feature.icon"></div>
          <span class="feature-label">{{ feature.label }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const inputText = ref('');
const textareaRef = ref<HTMLTextAreaElement | null>(null);

const features = [
  {
    id: 'record',
    label: '录音纪要',
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"/><path d="M19 10v2a7 7 0 0 1-14 0v-2"/><line x1="12" y1="19" x2="12" y2="22"/></svg>'
  },
  {
    id: 'document',
    label: '文档解读',
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>'
  },
  {
    id: 'writing',
    label: '智能写作',
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/></svg>'
  },
  {
    id: 'image',
    label: '图像生成',
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>'
  },
  {
    id: 'quick',
    label: '快捷访问',
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>'
  }
];

const autoResize = () => {
  const textarea = textareaRef.value;
  if (textarea) {
    textarea.style.height = 'auto';
    textarea.style.height = Math.min(textarea.scrollHeight, 200) + 'px';
  }
};

const handleFeatureClick = (id: string) => {
  console.log('Feature clicked:', id);
};
</script>

<style scoped>
<<<<<<< HEAD
.friday-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: calc(100vh - var(--tab-bar-height));
  padding: 40px 20px;
  background-color: var(--bg-primary);
}

.friday-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 40px;
  max-width: 720px;
  width: 100%;
}

.logo-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.logo-badge {
  padding: 4px 12px;
  background: linear-gradient(135deg, #a7f3d0 0%, #6ee7b7 100%);
  color: #065f46;
  font-size: 12px;
  font-weight: 500;
  border-radius: 12px;
  margin-bottom: 4px;
}

.logo-main {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-icon {
  font-size: 32px;
  color: #1c1917;
}

.logo-text {
  font-size: 56px;
  font-weight: 700;
  letter-spacing: -2px;
  color: #1c1917;
  margin: 0;
  line-height: 1;
}

.logo-subtitle {
  font-size: 16px;
  font-weight: 400;
  color: var(--text-secondary);
  letter-spacing: 3px;
  margin: 0;
}

.input-section {
  width: 100%;
}

.input-wrapper {
  background: #ffffff;
  border: 1px solid var(--border-color);
  border-radius: 16px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.08);
  overflow: hidden;
}

.main-input {
  width: 100%;
  padding: 20px 24px;
  border: none;
  outline: none;
  resize: none;
  font-size: 16px;
  line-height: 1.5;
  color: var(--text-primary);
  background: transparent;
  font-family: inherit;
  min-height: 60px;
  max-height: 200px;
}

.main-input::placeholder {
  color: var(--text-tertiary);
}

.input-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  border-top: 1px solid var(--border-color);
  background: #fafafa;
}

.action-left,
.action-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.action-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border: none;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  border-radius: 8px;
  font-size: 13px;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.action-btn:hover {
  background: var(--bg-hover);
}

.dropdown-btn span {
  font-size: 13px;
}

.icon-only {
  padding: 8px;
}

.send-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  background: var(--bg-active);
  color: var(--text-tertiary);
  cursor: pointer;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.send-btn.active {
  background: var(--text-primary);
  color: #ffffff;
}

.send-btn:hover {
  transform: scale(1.05);
}

.features-section {
  display: flex;
  gap: 24px;
  justify-content: center;
  flex-wrap: wrap;
}

.feature-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 20px 24px;
  background: transparent;
  border: none;
  cursor: pointer;
  transition: all 0.2s ease;
  border-radius: 12px;
  min-width: 90px;
}

.feature-card:hover {
  background: var(--bg-hover);
  transform: translateY(-2px);
}

.feature-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 52px;
  height: 52px;
  color: var(--text-secondary);
  transition: all 0.2s ease;
}

.feature-card:hover .feature-icon {
  color: var(--text-primary);
}

.feature-label {
  font-size: 13px;
  color: var(--text-secondary);
  text-align: center;
  transition: all 0.2s ease;
}

.feature-card:hover .feature-label {
  color: var(--text-primary);
}
</style>
