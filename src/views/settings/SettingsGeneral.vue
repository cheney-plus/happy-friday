<template>
  <div class="settings-general">
    <h1>{{ t('settings.title') }}</h1>
    <p>{{ t('settings.placeholder') }}</p>

    <div class="settings-section">
      <h3>{{ t('settings.theme') }}</h3>
      <select v-model="appStore.theme" @change="handleThemeChange">
        <option value="light">{{ t('settings.themeLight') }}</option>
        <option value="dark">{{ t('settings.themeDark') }}</option>
      </select>
    </div>

    <div class="settings-section">
      <h3>{{ t('settings.language') }}</h3>
      <select v-model="appStore.language" @change="handleLanguageChange">
        <option value="zh-CN">简体中文</option>
        <option value="en-US">English</option>
      </select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useAppStore } from '@/store';
import { setI18nLanguage } from '@/i18n';
import { invoke } from '@tauri-apps/api/core';
import { isTauriEnvironment } from '@/config/menu';

const { t } = useI18n();
const appStore = useAppStore();

const saveConfig = async () => {
  if (isTauriEnvironment()) {
    try {
      await invoke('update_config', {
        config: {
          language: appStore.language,
          theme: appStore.theme
        }
      });
    } catch (e) {
      console.error('Failed to save config:', e);
    }
  }
};

const handleThemeChange = () => {
  appStore.setTheme(appStore.theme);
  saveConfig();
};

const handleLanguageChange = () => {
  appStore.setLanguage(appStore.language);
  setI18nLanguage(appStore.language);
  saveConfig();
};
</script>

<style scoped>
.settings-general {
  padding: 24px;
}

.settings-section {
  margin-top: 24px;
}

.settings-section h3 {
  margin-bottom: 8px;
}

select {
  padding: 8px;
  border-radius: 4px;
  border: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  color: var(--text-primary);
}
</style>
