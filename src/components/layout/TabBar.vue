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
        <span class="tab-icon" v-if="tab.icon">{{ tab.icon }}</span>
        <span class="tab-title">{{ t(tab.i18nKey) }}</span>
        <button class="tab-close-btn" @click.stop="closeTab(tab.id)">
          &times;
        </button>
      </div>
    </div>
    
    <div class="tab-bar-empty-space" data-tauri-drag-region></div>
  </div>
</template>

<script setup lang="ts">
import { useTabStore, type Tab } from '@/store';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

const tabStore = useTabStore();
const { t } = useI18n();
const router = useRouter();

const switchTab = (tab: Tab) => {
  if (tabStore.activeTabId !== tab.id) {
    router.push(tab.path);
  }
};

const closeTab = (id: string) => {
  tabStore.removeTab(id);
  if (tabStore.activeTabId) {
    // Navigate to the newly active tab if there's still one
    const activeTab = tabStore.openedTabs.find(t => t.id === tabStore.activeTabId);
    if (activeTab) {
      router.push(activeTab.path);
    }
  } else {
    // If no tabs left, maybe go to a default route or show an empty state
    router.push('/workspace');
  }
};
</script>

<style scoped>
.tab-bar-container {
  display: flex;
  align-items: flex-end;
  height: 48px;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  padding-top: 12px;
  user-select: none;
  -webkit-app-region: drag;
  app-region: drag;
}

.mac-traffic-lights-spacer {
  width: 80px;
  height: 100%;
}

.tabs-list {
  display: flex;
  height: 36px;
  align-items: flex-end;
  gap: 4px;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.tab-item {
  display: flex;
  align-items: center;
  height: 32px;
  min-width: 120px;
  max-width: 200px;
  padding: 0 12px;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-bottom: none;
  border-top-left-radius: 8px;
  border-top-right-radius: 8px;
  cursor: pointer;
  color: var(--text-secondary);
  position: relative;
  transition: all 0.2s;
  box-shadow: 0 2px 4px rgba(0,0,0,0.02);
}

.tab-item:hover {
  background-color: var(--bg-hover);
}

.tab-item.active {
  height: 36px;
  color: var(--text-primary);
  background-color: var(--bg-primary);
  z-index: 2;
  font-weight: 500;
  box-shadow: 0 -2px 6px rgba(0,0,0,0.05);
}

.tab-item.active::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 1px;
  background-color: var(--bg-primary);
}

.tab-icon {
  margin-right: 6px;
  font-size: 14px;
}

.tab-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
}

.tab-close-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 16px;
  cursor: pointer;
  margin-left: 8px;
  border-radius: 50%;
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.2s, background-color 0.2s;
}

.tab-item:hover .tab-close-btn {
  opacity: 1;
}

.tab-close-btn:hover {
  background-color: var(--border-color);
  color: var(--text-primary);
}

.tab-bar-empty-space {
  flex: 1;
  height: 100%;
}
</style>
