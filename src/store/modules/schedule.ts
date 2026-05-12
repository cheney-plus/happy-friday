import { defineStore } from 'pinia';

export interface ScheduleEvent {
  id: string;
  title: string;
  start: string;
  end: string;
  startTime: string;
  endTime: string;
  allDay: boolean;
  description: string;
  color: string;
  reminder: boolean;
  completed: boolean;
}

export const EVENT_COLORS = [
  '#60a5fa',
  '#fbbf24',
  '#c084fc',
  '#f87171',
  '#22d3ee',
  '#fb7185',
  '#2563eb',
  '#6366f1',
  '#ea580c',
  '#9333ea',
  '#dc2626',
  '#0891b2',
  '#ca8a04',
  '#be185d',
  '#ff9de2',
  '#8c82fc',
  '#ffaa64',
  '#ff6464',
  '#6730ec',
];

const STORAGE_KEY = 'happy-friday-schedule-events';

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).substr(2, 9);
}

function loadEvents(): ScheduleEvent[] {
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    return data ? JSON.parse(data) : [];
  } catch {
    return [];
  }
}

function saveEvents(events: ScheduleEvent[]) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(events));
  } catch {
    // ignore storage errors
  }
}

export const useScheduleStore = defineStore('schedule', {
  state: () => ({
    events: loadEvents() as ScheduleEvent[],
    selectedDate: new Date().toISOString().split('T')[0],
    currentView: 'month' as 'month' | 'week' | 'year',
  }),

  getters: {
    getEventsForDateRange(state) {
      return (start: string, end: string) => {
        return state.events.filter(
          (e) => e.start <= end && e.end >= start
        );
      };
    },

    getEventById(state) {
      return (id: string) => {
        return state.events.find((e) => e.id === id);
      };
    },
  },

  actions: {
    addEvent(event: Omit<ScheduleEvent, 'id'>) {
      const newEvent: ScheduleEvent = {
        ...event,
        id: generateId(),
      };
      this.events.push(newEvent);
      saveEvents(this.events);
      return newEvent;
    },

    updateEvent(id: string, updates: Partial<ScheduleEvent>) {
      const idx = this.events.findIndex((e) => e.id === id);
      if (idx >= 0) {
        this.events[idx] = { ...this.events[idx], ...updates };
        saveEvents(this.events);
      }
    },

    removeEvent(id: string) {
      this.events = this.events.filter((e) => e.id !== id);
      saveEvents(this.events);
    },

    setSelectedDate(date: string) {
      this.selectedDate = date;
    },

    setCurrentView(view: 'month' | 'week' | 'year') {
      this.currentView = view;
    },
  },
});
