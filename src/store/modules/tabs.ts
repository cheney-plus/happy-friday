import { defineStore } from 'pinia';

export interface Tab {
  id: string; // unique identifier (usually route path)
  path: string; // route path
  i18nKey: string; // translation key for title
  icon?: string; // icon to display
}

export const useTabStore = defineStore('tabs', {
  state: () => ({
    openedTabs: [] as Tab[],
    activeTabId: ''
  }),
  actions: {
    addTab(tab: Tab) {
      const existingTabIndex = this.openedTabs.findIndex(t => t.id === tab.id);
      if (existingTabIndex === -1) {
        this.openedTabs.push(tab);
      }
      this.activeTabId = tab.id;
    },
    removeTab(id: string) {
      const index = this.openedTabs.findIndex(t => t.id === id);
      if (index !== -1) {
        this.openedTabs.splice(index, 1);
        
        // If we closed the active tab, we need to switch to another tab
        if (this.activeTabId === id) {
          if (this.openedTabs.length > 0) {
            // Switch to the previous tab if possible, otherwise the first tab
            const nextIndex = Math.max(0, index - 1);
            this.activeTabId = this.openedTabs[nextIndex].id;
          } else {
            this.activeTabId = '';
          }
        }
      }
    },
    setActiveTab(id: string) {
      this.activeTabId = id;
    }
  }
});
