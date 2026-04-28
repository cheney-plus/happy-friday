<template>
  <aside :class="['sidebar', { collapsed: appStore.sidebarCollapsed }]">
    <div class="sidebar-header">
      <div class="avatar" @click="toggleSidebar">
        <!-- Avatar icon placeholder -->
        <span v-if="appStore.sidebarCollapsed">👤</span>
        <span v-else>👤 User</span>
      </div>
    </div>
    
    <nav class="sidebar-menu">
      <router-link 
        v-for="item in menuConfig" 
        :key="item.key" 
        :to="item.path"
        class="menu-item"
        active-class="active"
      >
        <span class="icon">{{ item.icon }}</span>
        <span v-if="!appStore.sidebarCollapsed" class="label">{{ t(item.i18nKey) }}</span>
      </router-link>
    </nav>
    
    <div class="sidebar-bottom">
      <router-link 
        v-for="item in bottomMenuConfig" 
        :key="item.key" 
        :to="item.path"
        class="menu-item"
        active-class="active"
      >
        <span class="icon">{{ item.icon }}</span>
        <span v-if="!appStore.sidebarCollapsed" class="label">{{ t(item.i18nKey) }}</span>
      </router-link>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useAppStore } from '@/store';
import { useI18n } from 'vue-i18n';

const appStore = useAppStore();
const { t } = useI18n();

const toggleSidebar = () => {
  appStore.toggleSidebar();
};

const menuConfig = [
  { key: 'user', path: '/user', icon: '👤', i18nKey: 'user.title' },
  { key: 'workspace', path: '/workspace', icon: '📁', i18nKey: 'workspace.title' },
  { key: 'note', path: '/note', icon: '📝', i18nKey: 'note.title' },
  { key: 'schedule', path: '/schedule', icon: '📅', i18nKey: 'schedule.title' },
  { key: 'friday', path: '/friday', icon: '🤖', i18nKey: 'friday.title' }
];

const bottomMenuConfig = [
  { key: 'history', path: '/history', icon: '🕒', i18nKey: 'history.title' },
  { key: 'settings', path: '/settings', icon: '⚙️', i18nKey: 'settings.title' }
];
</script>

<style scoped>
.sidebar {
  width: 240px;
  height: 100%;
  background-color: var(--bg-sidebar);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease;
  overflow: hidden;
}

.sidebar.collapsed {
  width: var(--sidebar-width);
}

.sidebar-header {
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
}

.avatar {
  display: flex;
  align-items: center;
  gap: 12px;
  font-weight: bold;
}

.sidebar-menu {
  flex: 1;
  padding: 12px 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.sidebar-bottom {
  padding: 12px 0;
  border-top: 1px solid var(--border-color);
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  color: var(--text-primary);
  transition: background-color 0.2s;
}

.sidebar.collapsed .menu-item {
  justify-content: center;
  padding: 8px 0;
}

.menu-item:hover {
  background-color: var(--bg-hover);
}

.menu-item.active {
  background-color: var(--bg-hover);
  color: var(--accent-color);
  font-weight: bold;
}

.icon {
  font-size: 1.2rem;
  min-width: 24px;
  text-align: center;
}

.label {
  margin-left: 12px;
  white-space: nowrap;
}
</style>
