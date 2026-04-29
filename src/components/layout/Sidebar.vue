<template>
  <aside class="sidebar">
    <div class="sidebar-top">
      <div class="sidebar-avatar">
        <UserRound :size="22" :stroke-width="1.8" />
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
        <component :is="item.iconComponent" :size="20" :stroke-width="1.8" />
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
        <component :is="item.iconComponent" :size="20" :stroke-width="1.8" />
        <span class="menu-tooltip">{{ t(item.i18nKey) }}</span>
      </router-link>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import {
  UserRound,
  FolderKanban,
  FileText,
  CalendarDays,
  Bot,
  Clock,
  Settings
} from 'lucide-vue-next';

const { t } = useI18n();

const menuConfig = [
  { key: 'user', path: '/user', iconComponent: UserRound, i18nKey: 'user.title' },
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
  padding: 8px 0;
  flex-shrink: 0;
}

.sidebar-top {
  padding: 8px 0 12px;
}

.sidebar-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background-color: var(--bg-hover);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background-color 0.2s, color 0.2s;
}

.sidebar-avatar:hover {
  background-color: var(--bg-active);
  color: var(--text-primary);
}

.sidebar-menu {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  width: 100%;
}

.sidebar-bottom {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
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
  color: var(--text-secondary);
  transition: background-color 0.15s, color 0.15s;
}

.menu-item:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.menu-item.active {
  background-color: var(--accent-light);
  color: var(--accent-color);
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
