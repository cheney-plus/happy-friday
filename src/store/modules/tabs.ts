import { defineStore } from 'pinia';

export type IconName = 'FolderKanban' | 'FileText' | 'CalendarDays' | 'Bot' | 'Clock' | 'Settings';

export interface Tab {
  id: string;
  path: string;
  i18nKey: string;
  icon?: IconName;
}

export const useTabStore = defineStore('tabs', {
  state: () => ({
    openedTabs: [] as Tab[],
    activeTabId: '',
    fridayCounter: 0
  }),
  actions: {
    addTab(tab: Tab) {
      const existingTabIndex = this.openedTabs.findIndex(t => t.id === tab.id);
      if (existingTabIndex === -1) {
        this.openedTabs.push(tab);
      }
      this.activeTabId = tab.id;
    },
    addFridayTab() {
      this.fridayCounter++;
      const tab: Tab = {
        id: `friday-${this.fridayCounter}`,
        path: '/friday',
        i18nKey: 'friday.title',
        icon: 'Bot'
      };
      this.openedTabs.push(tab);
      this.activeTabId = tab.id;
      return tab;
    },
    removeTab(id: string) {
      const index = this.openedTabs.findIndex(t => t.id === id);
      if (index !== -1) {
        this.openedTabs.splice(index, 1);

        if (this.activeTabId === id) {
          if (this.openedTabs.length > 0) {
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
