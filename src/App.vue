<template>
  <div class="app-container">
    <TabBar />
    <div class="main-body">
      <Sidebar />
      <main class="main-content">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import Sidebar from '@/components/layout/Sidebar.vue';
import TabBar from '@/components/layout/TabBar.vue';
import { onMounted, watch } from 'vue';
import { useAppStore, useTabStore } from '@/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { setI18nLanguage } from '@/i18n';
import { useRoute } from 'vue-router';

const appStore = useAppStore();
const tabStore = useTabStore();
const route = useRoute();

// Sync route changes to tabs
watch(
  () => route.path,
  (newPath) => {
    if (newPath && newPath !== '/') {
      // Find matching menu config to get title and icon
      const allMenus = [
        { key: 'user', path: '/user', icon: '👤', i18nKey: 'user.title' },
        { key: 'workspace', path: '/workspace', icon: '📁', i18nKey: 'workspace.title' },
        { key: 'note', path: '/note', icon: '📝', i18nKey: 'note.title' },
        { key: 'schedule', path: '/schedule', icon: '📅', i18nKey: 'schedule.title' },
        { key: 'history', path: '/history', icon: '🕒', i18nKey: 'history.title' },
        { key: 'settings', path: '/settings', icon: '⚙️', i18nKey: 'settings.title' },
        { key: 'friday', path: '/friday', icon: '🤖', i18nKey: 'friday.title' }
      ];

      // Simple matching logic based on root path
      const rootPath = '/' + newPath.split('/')[1];
      const menu = allMenus.find(m => m.path === rootPath);

      if (menu) {
        tabStore.addTab({
          id: newPath, // use full path as id so /workspace and /workspace/1 are distinct
          path: newPath,
          i18nKey: menu.i18nKey,
          icon: menu.icon
        });
      }
    }
  },
  { immediate: true }
);

onMounted(async () => {
  // Only invoke Tauri APIs when running in Tauri context
  if ((window as any).__TAURI_INTERNALS__) {
    try {
      // Attempt to load config from backend
      const config = await invoke<any>('get_config');
      if (config) {
        if (config.language) {
          appStore.setLanguage(config.language);
          setI18nLanguage(config.language);
        }
        if (config.theme) {
          appStore.setTheme(config.theme);
        }
      }
    } catch (error) {
      console.error('Failed to load config:', error);
    }

    listen('config-changed', (event: any) => {
      if (event.payload.language) {
        appStore.setLanguage(event.payload.language);
        setI18nLanguage(event.payload.language);
      }
      if (event.payload.theme) {
        appStore.setTheme(event.payload.theme);
      }
    });
  } else {
    console.log('Running in browser mode, Tauri APIs are disabled.');
  }
});
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

.main-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  background-color: var(--bg-primary);
}
</style>
