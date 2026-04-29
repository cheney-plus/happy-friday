<template>
  <div class="tab-bar-container" data-tauri-drag-region>
    <div class="mac-traffic-lights-spacer" data-tauri-drag-region></div>

    <div class="tabs-list">
      <div
        v-for="tab in tabStore.openedTabs"
        :key="tab.id"
        :class="['tab-item', { active: tabStore.activeTabId === tab.id }]"
        role="tab"
        @click="switchTab(tab)"
      >
        <component
          v-if="tab.icon"
          :is="iconMap[tab.icon]"
          :size="14"
          :stroke-width="1.8"
          class="tab-icon"
        />
        <span class="tab-title">{{ t(tab.i18nKey) }}</span>
        <button class="tab-close-btn" @click.stop="closeTab(tab.id)">
          <X :size="12" :stroke-width="2" />
        </button>
      </div>
    </div>

    <div class="tab-bar-empty-space" data-tauri-drag-region></div>
  </div>
</template>

<script setup lang="ts">
import { useTabStore, type Tab, type IconName } from '@/store';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import {
  X,
  UserRound,
  FolderKanban,
  FileText,
  CalendarDays,
  Bot,
  Clock,
  Settings
} from 'lucide-vue-next';
import type { Component } from 'vue';

const tabStore = useTabStore();
const { t } = useI18n();
const router = useRouter();

const iconMap: Record<IconName, Component> = {
  UserRound,
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

.tabs-list {
  display: flex;
  align-items: center;
  gap: 1px;
  height: 100%;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.tab-item {
  display: flex;
  align-items: center;
  height: 28px;
  padding: 0 10px;
  border-radius: 6px;
  cursor: pointer;
  color: var(--text-secondary);
  transition: background-color 0.15s, color 0.15s;
  gap: 5px;
  max-width: 180px;
}

.tab-item:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.tab-item.active {
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-weight: 500;
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
