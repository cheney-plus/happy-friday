<template>
  <div class="note-page">
    <div
      class="note-sidebar"
      :class="{ collapsed: sidebarCollapsed, 'is-resizing': isResizing }"
      :style="{ width: sidebarCollapsed ? '0px' : sidebarWidth + 'px' }"
      @selectstart.prevent
    >
      <div class="sidebar-inner">
        <div class="sidebar-topbar" v-if="!searchMode">
          <button class="topbar-btn" @click="toggleSidebar" :title="sidebarCollapsed ? '' : '收起侧边栏'">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="3" y="3" width="18" height="18" rx="2"></rect><line x1="9" y1="3" x2="9" y2="21"></line></svg>
          </button>
          <div class="topbar-actions">
            <div class="new-note-btn" @click="createNewNote">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"></rect><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg>
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 12 15 18 9"></polyline></svg>
            </div>
            <button class="topbar-btn" @click="enterSearchMode">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
            </button>
          </div>
        </div>

        <div class="sidebar-search" v-else>
          <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
          <input
            ref="searchInputRef"
            v-model="searchQuery"
            class="search-input"
            type="text"
            placeholder="搜索笔记..."
            @keydown.enter="onSearch"
            @keydown.escape="exitSearchMode"
            @blur="exitSearchMode"
          />
        </div>

      <div class="sidebar-header">
        <div class="folder-trigger" ref="folderTriggerRef" @click.stop="toggleFolderMenu">
          <span class="folder-name">{{ currentFolderName }}</span>
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 12 15 18 9"></polyline></svg>
        </div>

        <Teleport to="body">
          <div v-if="folderMenuVisible" class="folder-dropdown" :style="folderMenuStyle">
            <div
              v-for="folder in folders"
              :key="folder.id"
              :class="['folder-item', { active: currentFolder === folder.id }]"
              @click="selectFolder(folder.id)"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path></svg>
              <div class="folder-info">
                <span class="folder-item-name">{{ folder.name }}</span>
                <span class="folder-count">{{ folder.count }}篇笔记</span>
              </div>
            </div>
          </div>
        </Teleport>
      </div>

      <div class="note-items" @contextmenu.prevent>
        <div
          v-for="note in notes"
          :key="note.id"
          :class="['note-item', { active: selectedNoteId === note.id }]"
          @click="selectNote(note.id)"
          @contextmenu.prevent="showContextMenu($event, note)"
        >
          <div class="note-title">{{ note.title }}</div>
          <div class="note-meta">
            <span class="note-time">{{ note.time }}</span>
            <span class="note-subtitle">{{ note.subtitle }}</span>
            <span v-if="note.extra" class="note-extra">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="9" x2="15" y2="15"></line><line x1="15" y1="9" x2="9" y2="15"></line></svg>
              {{ note.extra }}
            </span>
          </div>
        </div>
      </div>

      <Teleport to="body">
        <div v-if="contextMenu.visible" class="context-menu" :style="contextMenu.style">
          <div class="context-item" @click="handleAction('addToKnowledge')">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><path d="M12 8v8M8 12h8"></path></svg>
            添加到知识库
            <svg class="arrow-right" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"></polyline></svg>
          </div>
          <div class="context-item" @click="handleAction('moveToNotebook')">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path></svg>
            移动到笔记本
            <svg class="arrow-right" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"></polyline></svg>
          </div>
          <div class="context-item" @click="handleAction('duplicate')">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
            创建副本
          </div>
          <div class="context-divider"></div>
          <div class="context-item danger" @click="handleAction('delete')">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
            删除
          </div>
        </div>
      </Teleport>
      </div>
    </div>

    <div
      v-if="!sidebarCollapsed"
      class="sidebar-resize-handle"
      @mousedown="onResizeStart"
    ></div>

    <button
      v-if="sidebarCollapsed"
      class="sidebar-expand-btn"
      @click="toggleSidebar"
      title="展开侧边栏"
    >
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="3" y="3" width="18" height="18" rx="2"></rect><line x1="9" y1="3" x2="9" y2="21"></line></svg>
    </button>

    <div class="note-editor-area">
      <div v-if="selectedNote" class="editor-container">
        <NoteEditor
          :key="selectedNoteId"
          v-model="selectedNote.content"
          :placeholder="t('note.editorPlaceholder')"
          @change="onEditorChange"
        />
      </div>
      <div v-else class="editor-empty">
        <div class="empty-hint">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line><polyline points="10 9 9 9 8 9"></polyline></svg>
          <p>{{ t('note.selectToEdit') }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { useI18n } from 'vue-i18n';
import NoteEditor from './NoteEditor.vue';

const { t } = useI18n();

interface Note {
  id: number;
  title: string;
  time: string;
  subtitle: string;
  extra?: string;
  content: string;
}

interface Folder {
  id: string;
  name: string;
  count: number;
}

const currentFolder = ref('all');
const selectedNoteId = ref(3);
const folderMenuVisible = ref(false);
const folderTriggerRef = ref<HTMLElement | null>(null);

const SIDEBAR_MIN_WIDTH = 200;
const SIDEBAR_MAX_WIDTH = 280;
const SIDEBAR_DEFAULT_WIDTH = 200;
const sidebarWidth = ref(SIDEBAR_DEFAULT_WIDTH);
const sidebarCollapsed = ref(false);
const isResizing = ref(false);
const searchMode = ref(false);
const searchQuery = ref('');
const searchInputRef = ref<HTMLInputElement | null>(null);

const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value;
};

const enterSearchMode = () => {
  searchMode.value = true;
  nextTick(() => {
    searchInputRef.value?.focus();
  });
};

const exitSearchMode = () => {
  searchMode.value = false;
  searchQuery.value = '';
};

const onSearch = () => {
};

const onResizeStart = (e: MouseEvent) => {
  e.preventDefault();
  isResizing.value = true;
  const startX = e.clientX;
  const startWidth = sidebarWidth.value;

  const onResizeMove = (moveEvent: MouseEvent) => {
    const delta = moveEvent.clientX - startX;
    const newWidth = Math.min(SIDEBAR_MAX_WIDTH, Math.max(SIDEBAR_MIN_WIDTH, startWidth + delta));
    sidebarWidth.value = newWidth;
  };

  const onResizeEnd = () => {
    isResizing.value = false;
    document.removeEventListener('mousemove', onResizeMove);
    document.removeEventListener('mouseup', onResizeEnd);
    document.body.style.cursor = '';
    document.body.style.userSelect = '';
  };

  document.body.style.cursor = 'col-resize';
  document.body.style.userSelect = 'none';
  document.addEventListener('mousemove', onResizeMove);
  document.addEventListener('mouseup', onResizeEnd);
};

const folders = reactive<Folder[]>([
  { id: 'all', name: '全部', count: 13 },
  { id: '123', name: '123', count: 1 },
  { id: 'freya', name: 'freya 项目', count: 1 }
]);

const currentFolderName = computed(() => {
  const f = folders.find(f => f.id === currentFolder.value);
  return f ? f.name : '全部';
});

const notes = reactive<Note[]>([
  { id: 3, title: '项目规划', time: '26分钟前', subtitle: '无附加文本', content: '# 项目规划\n\n## 目标\n\n- 完成核心功能开发\n- 优化用户体验\n- 提升系统性能\n\n## 时间线\n\n第一阶段：基础架构搭建\n\n第二阶段：功能实现\n\n第三阶段：测试与优化' },
  { id: 4, title: '会议记录', time: '26分钟前', subtitle: '无附加文本', content: '# 会议记录\n\n**日期**：2026年4月30日\n\n**参会人**：产品组全员\n\n## 议题\n\n1. 需求评审\n2. 进度同步\n3. 风险评估\n\n## 结论\n\n- 需求已确认\n- 下周进入开发阶段' },
  { id: 5, title: '技术方案', time: '26分钟前', subtitle: '无附加文本', content: '# 技术方案\n\n## 架构设计\n\n采用微服务架构，主要模块包括：\n\n- **API 网关**：统一入口\n- **用户服务**：认证与授权\n- **业务服务**：核心业务逻辑\n\n## 技术选型\n\n| 模块 | 技术 | 版本 |\n|------|------|------|\n| 前端 | Vue 3 | 3.4 |\n| 后端 | Node.js | 20 |\n| 数据库 | PostgreSQL | 16 |' },
  { id: 6, title: '学习笔记', time: '26分钟前', subtitle: '无附加文本', content: '# 学习笔记\n\n## TypeScript 高级类型\n\n### 条件类型\n\n```typescript\ntype IsString<T> = T extends string ? true : false;\n```\n\n### 映射类型\n\n```typescript\ntype Readonly<T> = {\n  readonly [P in keyof T]: T[P];\n};\n```\n\n> 类型系统是 TypeScript 最强大的特性之一。' },
  { id: 7, title: '周报', time: '26分钟前', subtitle: '无附加文本', content: '# 本周工作总结\n\n## 已完成\n\n- [x] 用户模块重构\n- [x] API 文档更新\n- [x] 性能优化方案\n\n## 进行中\n\n- [ ] 编辑器集成\n- [ ] 数据迁移脚本\n\n## 下周计划\n\n1. 完成编辑器功能\n2. 开始集成测试\n3. 准备发布环境' },
  { id: 8, title: '读书笔记', time: '26分钟前', subtitle: '无附加文本', content: '# 读书笔记\n\n## 《设计模式》\n\n### 观察者模式\n\n定义对象间一对多的依赖关系，当一个对象状态改变时，所有依赖它的对象都会收到通知。\n\n### 策略模式\n\n定义一系列算法，把它们一个个封装起来，并使它们可互相替换。\n\n---\n\n*阅读进度：第 7 章*' },
  { id: 9, title: '需求文档', time: '26分钟前', subtitle: '无附加文本', content: '# 需求文档\n\n## 功能需求\n\n### FR-001 用户登录\n\n**描述**：用户可以通过邮箱和密码登录系统\n\n**优先级**：高\n\n### FR-002 笔记管理\n\n**描述**：用户可以创建、编辑、删除笔记\n\n**优先级**：高\n\n## 非功能需求\n\n- 响应时间 < 200ms\n- 支持 1000 并发用户' },
  { id: 10, title: '灵感记录', time: '26分钟前', subtitle: '无附加文本', content: '# 灵感记录\n\n## 产品想法\n\n一个基于 AI 的智能笔记应用，能够：\n\n- 自动整理笔记内容\n- 智能推荐相关笔记\n- 生成摘要和思维导图\n\n## 设计灵感\n\n极简主义风格，注重内容本身，减少视觉干扰。\n\n> 好的工具应该是隐形的。' },
  { id: 11, title: 'API 设计', time: '26分钟前', subtitle: '无附加文本', content: '# API 设计文档\n\n## 笔记相关接口\n\n### 获取笔记列表\n\n```\nGET /api/notes\n```\n\n### 创建笔记\n\n```\nPOST /api/notes\nBody: { title: string, content: string }\n```\n\n### 更新笔记\n\n```\nPUT /api/notes/:id\nBody: { title?: string, content?: string }\n```' },
  { id: 12, title: '项目配置', time: '26分钟前', subtitle: '无附加文本', extra: '123', content: '# 项目配置\n\n## 环境变量\n\n| 变量名 | 说明 | 默认值 |\n|--------|------|--------|\n| PORT | 服务端口 | 3000 |\n| DB_URL | 数据库地址 | localhost |\n| REDIS_URL | 缓存地址 | localhost |\n\n## 构建命令\n\n- `pnpm dev` - 开发模式\n- `pnpm build` - 生产构建\n- `pnpm preview` - 预览构建结果' },
  { id: 13, title: '代码规范', time: '26分钟前', subtitle: '无附加文本', content: '# 代码规范\n\n## 命名约定\n\n- **组件**：PascalCase（如 `NoteList`）\n- **函数**：camelCase（如 `getData`）\n- **常量**：UPPER_SNAKE_CASE（如 `MAX_COUNT`）\n\n## Git 规范\n\n提交信息格式：\n\n```\ntype(scope): description\n```\n\n类型包括：feat, fix, docs, style, refactor, test, chore' },
  { id: 14, title: '部署方案', time: '26分钟前', subtitle: '无附加文本', content: '# 部署方案\n\n## 架构图\n\n```\n用户 → CDN → Nginx → Node.js → PostgreSQL\n                    ↓\n                  Redis\n```\n\n## 部署步骤\n\n1. 构建前端资源\n2. 推送 Docker 镜像\n3. 更新 K8s 配置\n4. 滚动更新服务\n5. 验证部署结果' },
  { id: 15, title: '测试计划', time: '26分钟前', subtitle: '无附加文本', content: '# 测试计划\n\n## 单元测试\n\n覆盖核心业务逻辑，目标覆盖率 > 80%\n\n## 集成测试\n\n- API 接口测试\n- 数据库操作测试\n- 第三方服务 Mock 测试\n\n## E2E 测试\n\n模拟用户操作流程：\n\n1. 登录 → 创建笔记 → 编辑 → 保存\n2. 搜索笔记 → 查看结果\n3. 删除笔记 → 确认删除' }
]);

const selectedNote = computed(() => notes.find(n => n.id === selectedNoteId.value));

const selectNote = (id: number) => {
  selectedNoteId.value = id;
};

const createNewNote = () => {
  const newId = Math.max(...notes.map(n => n.id)) + 1;
  notes.unshift({
    id: newId,
    title: '新建笔记',
    time: '刚刚',
    subtitle: '无附加文本',
    content: '# 新建笔记\n\n',
  });
  selectedNoteId.value = newId;
};

const onEditorChange = (content: string) => {
  if (selectedNote.value) {
    const titleMatch = content.match(/^#\s+(.+)/m);
    if (titleMatch) {
      selectedNote.value.title = titleMatch[1].trim();
    }
  }
};

let folderMenuStyle = reactive({ left: '0px', top: '0px' });

const toggleFolderMenu = async () => {
  if (folderMenuVisible.value) {
    folderMenuVisible.value = false;
    return;
  }
  await nextTick();
  if (folderTriggerRef.value) {
    const rect = folderTriggerRef.value.getBoundingClientRect();
    folderMenuStyle.left = `${rect.left}px`;
    folderMenuStyle.top = `${rect.bottom + 4}px`;
  }
  folderMenuVisible.value = true;
};

const selectFolder = (id: string) => {
  currentFolder.value = id;
  folderMenuVisible.value = false;
};

const contextMenu = reactive({
  visible: false,
  x: 0,
  y: 0,
  targetNote: null as Note | null,
  get style() {
    return {
      left: `${this.x}px`,
      top: `${this.y}px`
    };
  }
});

const showContextMenu = (e: MouseEvent, note: Note) => {
  contextMenu.visible = true;
  contextMenu.x = e.clientX;
  contextMenu.y = e.clientY;
  contextMenu.targetNote = note;
};

const hideContextMenu = () => {
  contextMenu.visible = false;
  contextMenu.targetNote = null;
};

const handleAction = (action: string) => {
  if (action === 'delete' && contextMenu.targetNote) {
    const idx = notes.findIndex(n => n.id === contextMenu.targetNote!.id);
    if (idx !== -1) {
      notes.splice(idx, 1);
      if (selectedNoteId.value === contextMenu.targetNote!.id) {
        selectedNoteId.value = notes.length > 0 ? notes[0].id : 0;
      }
    }
  }
  hideContextMenu();
};

const handleClickOutside = () => {
  if (contextMenu.visible || folderMenuVisible.value) {
    contextMenu.visible = false;
    folderMenuVisible.value = false;
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>

<style scoped>
.note-page {
  display: flex;
  height: 100%;
  overflow: hidden;
}

.note-sidebar {
  min-width: 0;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  position: relative;
  overflow: hidden;
  transition: width 0.28s cubic-bezier(0.4, 0, 0.2, 1);
}

.note-sidebar :deep(*) {
  user-select: none;
  -webkit-user-select: none;
}

.note-sidebar :deep(::selection) {
  background: transparent;
}

.note-sidebar.is-resizing {
  transition: none;
}

.note-sidebar.collapsed {
  width: 0 !important;
}

.sidebar-inner {
  min-width: 200px;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar-expand-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  border: none;
  background-color: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: background-color 0.12s;
  flex-shrink: 0;
  margin: 12px 0 0 12px;
}

.sidebar-expand-btn:hover {
  background-color: var(--bg-hover);
}

.sidebar-resize-handle {
  width: 6px;
  cursor: col-resize;
  flex-shrink: 0;
}

.sidebar-topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 12px;
}

.sidebar-search {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 12px;
  height: 56px;
  box-sizing: border-box;
}

.search-icon {
  flex-shrink: 0;
  color: var(--text-tertiary);
}

.search-input {
  flex: 1;
  border: none;
  outline: none;
  background: transparent;
  font-size: 14px;
  color: var(--text-primary);
  min-width: 0;
  height: 32px;
}

.search-input::placeholder {
  color: var(--text-tertiary);
}

.topbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  border: none;
  background-color: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: background-color 0.12s;
}

.topbar-btn:hover {
  background-color: var(--bg-hover);
}

.topbar-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.new-note-btn {
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 5px 8px;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.12s;
  color: var(--text-primary);
}

.new-note-btn:hover {
  background-color: var(--bg-hover);
}

.sidebar-header {
  padding: 0 12px 8px;
  position: relative;
}

.folder-trigger {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 8px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  cursor: pointer;
  transition: background-color 0.12s;
  user-select: none;
}

.folder-trigger:hover {
  background-color: var(--bg-hover);
}

.folder-name {
  line-height: 1;
}

.note-items {
  flex: 1;
  overflow-y: auto;
  padding: 2px 0;
  scrollbar-width: thin;
  scrollbar-color: rgba(0, 0, 0, 0.15) transparent;
}

.note-items::-webkit-scrollbar {
  width: 5px;
}

.note-items::-webkit-scrollbar-track {
  background: transparent;
}

.note-items::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.15);
  border-radius: 10px;
}

.note-items::-webkit-scrollbar-thumb:hover {
  background-color: rgba(0, 0, 0, 0.25);
}

[data-theme='dark'] .note-items {
  scrollbar-color: rgba(255, 255, 255, 0.15) transparent;
}

[data-theme='dark'] .note-items::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.15);
}

[data-theme='dark'] .note-items::-webkit-scrollbar-thumb:hover {
  background-color: rgba(255, 255, 255, 0.25);
}

.note-item {
  padding: 10px 16px;
  cursor: pointer;
  transition: background-color 0.12s;
  border-left: 3px solid transparent;
}

.note-item:hover {
  background-color: var(--bg-hover);
}

.note-item.active {
  background-color: #f0f0ee;
  border-left-color: var(--text-primary);
}

.note-title {
  font-size: 14px;
  font-weight: 400;
  color: var(--text-primary);
  margin-bottom: 1px;
  line-height: 1.35;
}

.note-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--text-tertiary);
}

.note-time {
  white-space: nowrap;
}

.note-subtitle {
  white-space: nowrap;
}

.note-extra {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  color: var(--text-tertiary);
}

.note-editor-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--bg-primary);
}

.editor-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.editor-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-hint {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  color: var(--text-tertiary);
}

.empty-hint svg {
  opacity: 0.4;
}

.empty-hint p {
  font-size: 14px;
}
</style>

<style>
.folder-dropdown {
  position: fixed;
  z-index: 99999;
  background-color: #ffffff;
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.12), 0 0 1px rgba(0, 0, 0, 0.08);
  padding: 6px 0;
  min-width: 200px;
  animation: dropdown-in 0.12s ease-out;
}

[data-theme='dark'] .folder-dropdown {
  background-color: #2a2725;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.4), 0 0 1px rgba(0, 0, 0, 0.2);
}

@keyframes dropdown-in {
  from { opacity: 0; transform: scale(0.96) translateY(-4px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}

.folder-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 16px;
  font-size: 13px;
  color: #1c1917;
  cursor: pointer;
  transition: background-color 0.1s;
  user-select: none;
}

[data-theme='dark'] .folder-item {
  color: rgba(255, 255, 255, 0.92);
}

.folder-item:hover {
  background-color: rgba(0, 0, 0, 0.06);
}

[data-theme='dark'] .folder-item:hover {
  background-color: rgba(255, 255, 255, 0.06);
}

.folder-item.active {
  background-color: rgba(0, 0, 0, 0.08);
}

[data-theme='dark'] .folder-item.active {
  background-color: rgba(255, 255, 255, 0.08);
}

.folder-item svg {
  flex-shrink: 0;
  color: #78716c;
}

[data-theme='dark'] .folder-item svg {
  color: rgba(255, 255, 255, 0.55);
}

.folder-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
}

.folder-item-name {
  font-weight: 500;
  line-height: 1.3;
}

.folder-count {
  font-size: 11px;
  color: #a8a29e;
}

[data-theme='dark'] .folder-count {
  color: rgba(255, 255, 255, 0.35);
}

.context-menu {
  position: fixed;
  z-index: 99999;
  background-color: #ffffff;
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.12), 0 0 1px rgba(0, 0, 0, 0.08);
  padding: 6px 0;
  min-width: 180px;
  animation: dropdown-in 0.12s ease-out;
}

[data-theme='dark'] .context-menu {
  background-color: #2a2725;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.4), 0 0 1px rgba(0, 0, 0, 0.2);
}

.context-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 16px;
  font-size: 13px;
  color: #1c1917;
  cursor: pointer;
  transition: background-color 0.1s;
  user-select: none;
}

[data-theme='dark'] .context-item {
  color: rgba(255, 255, 255, 0.92);
}

.context-item:hover {
  background-color: rgba(0, 0, 0, 0.06);
}

[data-theme='dark'] .context-item:hover {
  background-color: rgba(255, 255, 255, 0.06);
}

.context-item svg {
  flex-shrink: 0;
  color: #78716c;
}

[data-theme='dark'] .context-item svg {
  color: rgba(255, 255, 255, 0.55);
}

.context-item.danger {
  color: #ef4444;
}

.context-item.danger svg {
  color: #ef4444;
}

.context-item .arrow-right {
  margin-left: auto;
  opacity: 0.4;
}

.context-divider {
  height: 1px;
  background-color: #e7e5e4;
  margin: 4px 12px;
}

[data-theme='dark'] .context-divider {
  background-color: #3f3f46;
}
</style>
