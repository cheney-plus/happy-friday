<template>
  <div class="tab-bar-container" data-tauri-drag-region>
    <div class="mac-traffic-lights-spacer" data-tauri-drag-region></div>

    <button class="sidebar-toggle-btn" @click="appStore.toggleSidebar()">
      <PanelLeftClose v-if="appStore.sidebarVisible" :size="16" :stroke-width="1.8" />
      <PanelLeftOpen v-else :size="16" :stroke-width="1.8" />
    </button>

    <div class="tabs-list">
      <template v-for="(tab, index) in tabStore.openedTabs" :key="tab.id">
        <span v-if="index > 0" class="tab-divider">|</span>
        <div
          :class="['tab-item', { active: tabStore.activeTabId === tab.id }]"
          role="tab"
          @click="switchTab(tab)"
        >
          <component
            v-if="tab.icon"
            :is="iconMap[tab.icon]"
            :size="14"
            :stroke-width="2"
            class="tab-icon"
          />
          <span class="tab-title">{{ t(tab.i18nKey) }}</span>
          <button class="tab-close-btn" @click.stop="closeTab(tab.id)">
            <X :size="12" :stroke-width="2" />
          </button>
        </div>
      </template>
    </div>

    <div class="tab-bar-empty-space" data-tauri-drag-region></div>
  </div>
</template>

<script setup lang="ts">
import { useTabStore, useAppStore, type Tab, type IconName } from '@/store';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import {
  X,
  PanelLeftClose,
  PanelLeftOpen,
  FolderKanban,
  FileText,
  CalendarDays,
  Bot,
  Clock,
  Settings
} from 'lucide-vue-next';
import type { Component } from 'vue';

const tabStore = useTabStore();
const appStore = useAppStore();
const { t } = useI18n();
const router = useRouter();

const iconMap: Record<IconName, Component> = {
  FolderKanban,
  FileText,
  CalendarDays,
  Bot,
  Clock,
  Settings
};

const switchTab = (tab: Tab) => {
  if (tabStore.activeTabId !== tab.id) {
    router.push(tab.path);
  }
};

const closeTab = (id: string) => {
  tabStore.removeTab(id);
  if (tabStore.activeTabId) {
    const activeTab = tabStore.openedTabs.find(t => t.id === tabStore.activeTabId);
    if (activeTab) {
      router.push(activeTab.path);
    }
  } else {
    router.push('/workspace');
  }
};
</script>

<style scoped>
.tab-bar-container {
  display: flex;
  align-items: center;
  height: var(--tab-bar-height);
  background-color: var(--bg-secondary);
  padding-left: 4px;
  user-select: none;
  -webkit-app-region: drag;
  app-region: drag;
  flex-shrink: 0;
}

.mac-traffic-lights-spacer {
  width: 80px;
  height: 100%;
}

.sidebar-toggle-btn {
  background: none;
  border: none;
  color: var(--text-primary);
  opacity: 0.5;
  cursor: pointer;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: background-color 0.15s, opacity 0.15s;
  -webkit-app-region: no-drag;
  app-region: no-drag;
  flex-shrink: 0;
}

.sidebar-toggle-btn:hover {
  background-color: var(--bg-hover);
  opacity: 0.85;
}

.tabs-list {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 100%;
  -webkit-app-region: no-drag;
  app-region: no-drag;
  padding-left: 4px;
}

.tab-divider {
  color: var(--border-color);
  font-size: 12px;
  line-height: 1;
  user-select: none;
  opacity: 0.7;
}

.tab-item {
  display: flex;
  align-items: center;
  height: 28px;
  padding: 0 10px;
  border-radius: 6px;
  cursor: pointer;
  color: var(--text-primary);
  opacity: 0.7;
  transition: background-color 0.15s, opacity 0.15s;
  gap: 5px;
  max-width: 140px;
  min-width: 80px;
  font-weight: 600;
}

.tab-item:hover {
  background-color: var(--bg-hover);
  opacity: 0.8;
}

.tab-item.active {
  background-color: var(--bg-primary);
  opacity: 1;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
}

.tab-icon {
  flex-shrink: 0;
  display: flex;
  align-items: center;
}

.tab-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12.5px;
  line-height: 1;
  font-weight: inherit;
}

.tab-close-btn {
  background: none;
  border: none;
  color: var(--text-tertiary);
  cursor: pointer;
  margin-left: 2px;
  border-radius: 4px;
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.15s, background-color 0.15s, color 0.15s;
  flex-shrink: 0;
}

.tab-item:hover .tab-close-btn {
  opacity: 1;
}

.tab-close-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.tab-bar-empty-space {
  flex: 1;
  height: 100%;
}
</style>
