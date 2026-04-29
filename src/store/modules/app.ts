import { defineStore } from 'pinia';

export const useAppStore = defineStore('app', {
  state: () => ({
    sidebarVisible: true,
    language: 'zh-CN',
    theme: 'light',
    loading: false
  }),
  actions: {
    toggleSidebar() {
      this.sidebarVisible = !this.sidebarVisible;
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
