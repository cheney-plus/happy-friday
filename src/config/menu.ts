import {
  FolderKanban,
  FileText,
  CalendarDays,
  Bot,
  Clock,
  Settings,
  type LucideIcon
} from 'lucide-vue-next';
import type { IconName } from '@/store';

export interface MenuItem {
  key: string;
  path: string;
  icon: IconName;
  iconComponent: LucideIcon;
  i18nKey: string;
}

export const sidebarMenuConfig: MenuItem[] = [
  { key: 'workspace', path: '/workspace', icon: 'FolderKanban', iconComponent: FolderKanban, i18nKey: 'workspace.title' },
  { key: 'note', path: '/note', icon: 'FileText', iconComponent: FileText, i18nKey: 'note.title' },
  { key: 'schedule', path: '/schedule', icon: 'CalendarDays', iconComponent: CalendarDays, i18nKey: 'schedule.title' }
];

export const sidebarBottomMenuConfig: MenuItem[] = [
  { key: 'history', path: '/history', icon: 'Clock', iconComponent: Clock, i18nKey: 'history.title' },
  { key: 'settings', path: '/settings', icon: 'Settings', iconComponent: Settings, i18nKey: 'settings.title' }
];

export const fridayMenuConfig: MenuItem = {
  key: 'friday',
  path: '/friday',
  icon: 'Bot',
  iconComponent: Bot,
  i18nKey: 'friday.title'
};

export const allMenuConfigs: MenuItem[] = [
  ...sidebarMenuConfig,
  ...sidebarBottomMenuConfig,
  fridayMenuConfig
];

export function isTauriEnvironment(): boolean {
  return !!(window as unknown as { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
}
