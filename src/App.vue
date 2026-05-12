<template>
  <div class="app-container">
    <TabBar />
    <div class="main-body">
      <Sidebar />
      <main class="main-content">
        <div class="content-wrapper">
          <router-view v-slot="{ Component }">
            <keep-alive>
              <component :is="Component" :key="route.fullPath" />
            </keep-alive>
          </router-view>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import Sidebar from '@/components/layout/Sidebar.vue';
import TabBar from '@/components/layout/TabBar.vue';
import { onMounted, onUnmounted, watch } from 'vue';
import { useAppStore, useTabStore } from '@/store';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { setI18nLanguage } from '@/i18n';
import { useRoute, useRouter } from 'vue-router';
import { allMenuConfigs, isTauriEnvironment } from '@/config/menu';
import { useTheme } from '@/utils/theme';

const appStore = useAppStore();
const tabStore = useTabStore();
const route = useRoute();
const router = useRouter();
const { initTheme, setTheme: applyThemeFromConfig } = useTheme();

let unlistenConfig: UnlistenFn | null = null;

watch(
  () => route.fullPath,
  (newPath) => {
    if (!newPath || newPath === '/') return;

    const rootPath = '/' + newPath.split('/')[1];
    const menu = allMenuConfigs.find(m => m.path === rootPath);
    if (!menu) return;

    const activeTab = tabStore.openedTabs.find(t => t.id === tabStore.activeTabId);
    if (activeTab) {
      const activeRootPath = '/' + activeTab.path.split('/')[1];
      if (activeRootPath === rootPath) {
        tabStore.updateTabFullPath(activeTab.id, newPath);
        return;
      }
    }

    if (rootPath === '/friday') {
      const tab = tabStore.addFridayTab();
      if (newPath !== '/friday') {
        tabStore.updateTabFullPath(tab.id, newPath);
      }
      router.replace(newPath !== '/friday' ? newPath : tab.fullPath);
    } else {
      tabStore.addTab({
        id: newPath,
        path: newPath,
        fullPath: newPath,
        i18nKey: menu.i18nKey,
        icon: menu.icon
      });
    }
  },
  { immediate: true }
);

onMounted(async () => {
  initTheme();

  if (isTauriEnvironment()) {
    try {
      const config = await invoke<{ language?: string; theme?: string }>('get_config');
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

    unlistenConfig = await listen<{ language?: string; theme?: string }>('config-changed', (event) => {
      if (event.payload.language) {
        appStore.setLanguage(event.payload.language);
        setI18nLanguage(event.payload.language);
      }
      if (event.payload.theme) {
        appStore.setTheme(event.payload.theme);
        applyThemeFromConfig(event.payload.theme as 'light' | 'dark' | 'system');
      }
    });
  } else {
    console.log('Running in browser mode, Tauri APIs are disabled.');
  }
});

onUnmounted(() => {
  if (unlistenConfig) {
    unlistenConfig();
    unlistenConfig = null;
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
  padding: 0 6px 6px 6px;
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
