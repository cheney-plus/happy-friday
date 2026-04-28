import { defineStore } from 'pinia';

export const useAppStore = defineStore('app', {
  state: () => ({
    sidebarCollapsed: true,
    language: 'zh-CN',
    theme: 'light',
    loading: false
  }),
  actions: {
    toggleSidebar() {
      this.sidebarCollapsed = !this.sidebarCollapsed;
    },
    setLanguage(lang: string) {
      this.language = lang;
    },
    setTheme(theme: string) {
      this.theme = theme;
      document.documentElement.setAttribute('data-theme', theme);
    }
  }
});
