<template>
  <aside v-show="appStore.sidebarVisible" class="sidebar">
    <nav class="sidebar-menu">
      <router-link
        v-for="item in menuConfig"
        :key="item.key"
        :to="item.path"
        class="menu-item"
        active-class="active"
      >
        <component :is="item.iconComponent" :size="20" :stroke-width="1.6" />
        <span class="menu-tooltip">{{ t(item.i18nKey) }}</span>
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
        <component :is="item.iconComponent" :size="20" :stroke-width="1.6" />
        <span class="menu-tooltip">{{ t(item.i18nKey) }}</span>
      </router-link>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useAppStore } from '@/store';
import { useI18n } from 'vue-i18n';
import {
  FolderKanban,
  FileText,
  CalendarDays,
  Bot,
  Clock,
  Settings
} from 'lucide-vue-next';

const appStore = useAppStore();
const { t } = useI18n();

const menuConfig = [
  { key: 'workspace', path: '/workspace', iconComponent: FolderKanban, i18nKey: 'workspace.title' },
  { key: 'note', path: '/note', iconComponent: FileText, i18nKey: 'note.title' },
  { key: 'schedule', path: '/schedule', iconComponent: CalendarDays, i18nKey: 'schedule.title' },
  { key: 'friday', path: '/friday', iconComponent: Bot, i18nKey: 'friday.title' }
];

const bottomMenuConfig = [
  { key: 'history', path: '/history', iconComponent: Clock, i18nKey: 'history.title' },
  { key: 'settings', path: '/settings', iconComponent: Settings, i18nKey: 'settings.title' }
];
</script>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  height: 100%;
  background-color: var(--bg-sidebar);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 0;
  flex-shrink: 0;
}

.sidebar-menu {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  width: 100%;
  padding-top: 4px;
}

.sidebar-bottom {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  width: 100%;
  padding-top: 8px;
}

.menu-item {
  position: relative;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  color: var(--text-primary);
  opacity: 0.55;
  transition: background-color 0.15s, opacity 0.15s, color 0.15s;
}

.menu-item:hover {
  background-color: var(--bg-hover);
  opacity: 0.85;
}

.menu-item.active {
  background-color: var(--accent-light);
  color: var(--accent-color);
  opacity: 1;
}

.menu-tooltip {
  position: absolute;
  left: calc(100% + 10px);
  top: 50%;
  transform: translateY(-50%);
  background-color: var(--text-primary);
  color: var(--bg-primary);
  font-size: 12px;
  font-weight: 500;
  padding: 4px 10px;
  border-radius: 6px;
  white-space: nowrap;
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.15s;
  z-index: 100;
}

.menu-tooltip::before {
  content: '';
  position: absolute;
  left: -4px;
  top: 50%;
  transform: translateY(-50%);
  border: 4px solid transparent;
  border-right-color: var(--text-primary);
  border-left: none;
}

.menu-item:hover .menu-tooltip {
  opacity: 1;
}
</style>
