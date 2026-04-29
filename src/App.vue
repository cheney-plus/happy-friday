<template>
  <div class="app-container">
    <TabBar />
    <div class="main-body">
      <Sidebar />
      <main class="main-content">
        <div class="content-wrapper">
          <router-view />
        </div>
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
import type { IconName } from '@/store';

const appStore = useAppStore();
const tabStore = useTabStore();
const route = useRoute();

const allMenus: { key: string; path: string; icon: IconName; i18nKey: string }[] = [
  { key: 'user', path: '/user', icon: 'UserRound', i18nKey: 'user.title' },
  { key: 'workspace', path: '/workspace', icon: 'FolderKanban', i18nKey: 'workspace.title' },
  { key: 'note', path: '/note', icon: 'FileText', i18nKey: 'note.title' },
  { key: 'schedule', path: '/schedule', icon: 'CalendarDays', i18nKey: 'schedule.title' },
  { key: 'history', path: '/history', icon: 'Clock', i18nKey: 'history.title' },
  { key: 'settings', path: '/settings', icon: 'Settings', i18nKey: 'settings.title' },
  { key: 'friday', path: '/friday', icon: 'Bot', i18nKey: 'friday.title' }
];

watch(
  () => route.path,
  (newPath) => {
    if (newPath && newPath !== '/') {
      const rootPath = '/' + newPath.split('/')[1];
      const menu = allMenus.find(m => m.path === rootPath);

      if (menu) {
        tabStore.addTab({
          id: newPath,
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
  if ((window as any).__TAURI_INTERNALS__) {
    try {
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
  background-color: var(--bg-secondary);
}

.main-body {
  display: flex;
  flex: 1;
  overflow: hidden;
  padding: 0 6px 6px 0;
}

.main-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.content-wrapper {
  flex: 1;
  overflow-y: auto;
  background-color: var(--bg-primary);
  border-radius: var(--content-radius);
  margin: 0;
}
</style>
