import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

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
  createdAt: string;
  updatedAt: string;
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

export const useScheduleStore = defineStore('schedule', {
  state: () => ({
    events: [] as ScheduleEvent[],
    selectedDate: new Date().toISOString().split('T')[0],
    currentView: 'month' as 'month' | 'week' | 'year',
    loading: false,
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
    async loadEvents() {
      this.loading = true;
      try {
        this.events = await invoke<ScheduleEvent[]>('get_schedule_events');
      } catch (e) {
        console.error('Failed to load schedule events:', e);
        this.events = [];
      } finally {
        this.loading = false;
      }
    },

    async addEvent(event: Omit<ScheduleEvent, 'id' | 'createdAt' | 'updatedAt'>) {
      try {
        const newEvent = await invoke<ScheduleEvent>('create_schedule_event', {
          title: event.title,
          startDate: event.start,
          endDate: event.end,
          startTime: event.startTime,
          endTime: event.endTime,
          allDay: event.allDay,
          description: event.description,
          color: event.color,
          reminder: event.reminder,
          completed: event.completed,
        });
        this.events.push(newEvent);
        return newEvent;
      } catch (e) {
        console.error('Failed to create schedule event:', e);
        throw e;
      }
    },

    async updateEvent(id: string, updates: Partial<ScheduleEvent>) {
      const existing = this.events.find((e) => e.id === id);
      if (!existing) return;

      const merged = { ...existing, ...updates };
      try {
        await invoke('update_schedule_event', {
          eventId: id,
          title: merged.title,
          startDate: merged.start,
          endDate: merged.end,
          startTime: merged.startTime,
          endTime: merged.endTime,
          allDay: merged.allDay,
          description: merged.description,
          color: merged.color,
          reminder: merged.reminder,
          completed: merged.completed,
        });
        const idx = this.events.findIndex((e) => e.id === id);
        if (idx >= 0) {
          this.events[idx] = { ...this.events[idx], ...updates };
        }
      } catch (e) {
        console.error('Failed to update schedule event:', e);
        throw e;
      }
    },

    async removeEvent(id: string) {
      try {
        await invoke('delete_schedule_event', { eventId: id });
        this.events = this.events.filter((e) => e.id !== id);
      } catch (e) {
        console.error('Failed to delete schedule event:', e);
        throw e;
      }
    },

    setSelectedDate(date: string) {
      this.selectedDate = date;
    },

    setCurrentView(view: 'month' | 'week' | 'year') {
      this.currentView = view;
    },
  },
});
