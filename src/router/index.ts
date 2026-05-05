import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router';

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/workspace'
  },
  {
    path: '/workspace',
    name: 'workspace',
    component: () => import('@/views/workspace/WorkspaceList.vue')
  },
  {
    path: '/workspace/:id',
    name: 'workspace-detail',
    component: () => import('@/views/workspace/WorkspaceDetail.vue')
  },
  {
    path: '/note',
    name: 'note',
    component: () => import('@/views/note/NoteList.vue')
  },
  {
    path: '/note/:id',
    name: 'note-edit',
    component: () => import('@/views/note/NoteEdit.vue')
  },
  {
    path: '/schedule',
    name: 'schedule',
    component: () => import('@/views/schedule/ScheduleCalendar.vue')
  },
  {
    path: '/schedule/:id',
    name: 'schedule-detail',
    component: () => import('@/views/schedule/ScheduleDetail.vue')
  },
  {
    path: '/history',
    name: 'history',
    component: () => import('@/views/history/HistoryList.vue')
  },
  {
    path: '/history/:id',
    name: 'history-diff',
    component: () => import('@/views/history/HistoryDiff.vue')
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('@/views/settings/SettingsGeneral.vue')
  },
  {
    path: '/friday',
    name: 'friday',
    component: () => import('@/views/friday/FridayChat.vue')
  },
  {
    path: '/friday/chat',
    name: 'friday-chat',
    component: () => import('@/views/friday/FridayConversation.vue')
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/workspace'
  }
];

export const router = createRouter({
  history: createWebHistory(),
  routes
});
