<template>
  <div class="settings-page">
    <h1 class="settings-title">设置</h1>

    <div class="settings-content">
      <!-- 通用设置 -->
      <div class="settings-group">
        <div class="group-title">通用设置</div>
        <div class="group-content">
          <div class="setting-item">
            <span class="item-label">界面显示</span>
            <select v-model="settings.displayMode" class="item-select">
              <option value="system">跟随系统</option>
              <option value="light">浅色模式</option>
              <option value="dark">深色模式</option>
            </select>
          </div>
          <div class="setting-item">
            <span class="item-label">字体大小</span>
            <div class="slider-wrapper">
              <span class="slider-label slider-min">小</span>
              <input type="range" min="12" max="20" v-model.number="settings.fontSize" class="item-slider" />
              <span class="slider-label slider-max">大</span>
              <span class="slider-mid">{{ settings.fontSize === 16 ? '标准' : '' }}</span>
            </div>
          </div>
          <div class="setting-item">
            <span class="item-label">开机自动启动</span>
            <label class="toggle-switch">
              <input type="checkbox" v-model="settings.autoStart" />
              <span class="toggle-slider"></span>
            </label>
          </div>
          <div class="setting-item">
            <span class="item-label">接收消息提醒</span>
            <label class="toggle-switch">
              <input type="checkbox" v-model="settings.messageNotify" />
              <span class="toggle-slider"></span>
            </label>
          </div>
          <div class="setting-item clickable" @click="goToModelSettings">
            <span class="item-label">模型设置</span>
            <span class="item-link">
              支持自定义模型
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"></polyline></svg>
            </span>
          </div>
        </div>
      </div>

      <!-- AI工具 -->
      <div class="settings-group">
        <div class="group-title">AI工具</div>
        <div class="group-content">
          <div class="setting-item">
            <span class="item-label">随时唤起ima</span>
            <span class="shortcut-key">⌘ + Space</span>
          </div>
          <div class="setting-item">
            <span class="item-label">快捷截图</span>
            <span class="shortcut-key">⌘ + J</span>
          </div>
          <div class="setting-item clickable">
            <span class="item-label">AI划词工具栏</span>
            <span class="item-link">
              仅在ima中使用
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"></polyline></svg>
            </span>
          </div>
        </div>
      </div>

      <!-- 浏览设置 -->
      <div class="settings-group">
        <div class="group-title">浏览设置</div>
        <div class="group-content">
          <div class="setting-item">
            <span class="item-label">网页默认用 ima 打开</span>
            <button class="action-btn">去设置</button>
          </div>
          <div class="setting-item clickable">
            <span class="item-label">搜索方式</span>
            <span class="item-link">
              Microsoft Bing
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"></polyline></svg>
            </span>
          </div>
          <div class="setting-item">
            <span class="item-label">启动时候复上次打开的标签页</span>
            <label class="toggle-switch">
              <input type="checkbox" v-model="settings.restoreTabs" />
              <span class="toggle-slider"></span>
            </label>
          </div>
        </div>
      </div>

      <!-- 书签 -->
      <div class="settings-group">
        <div class="group-title">书签</div>
        <div class="group-content">
          <div class="setting-item">
            <span class="item-label">展示书签栏</span>
            <label class="toggle-switch">
              <input type="checkbox" v-model="settings.showBookmarkBar" />
              <span class="toggle-slider"></span>
            </label>
          </div>
          <div class="setting-item clickable">
            <span class="item-label">书签管理器</span>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="arrow-icon"><polyline points="9 18 15 12 9 6"></polyline></svg>
          </div>
          <div class="setting-item clickable">
            <span class="item-label">导入书签</span>
            <div class="import-action">
              <span class="red-dot"></span>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="arrow-icon"><polyline points="9 18 15 12 9 6"></polyline></svg>
            </div>
          </div>
        </div>
      </div>

      <!-- 关于 -->
      <div class="settings-group">
        <div class="group-title">关于</div>
        <div class="group-content">
          <div class="setting-item clickable">
            <span class="item-label">关于ima.copilot</span>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="arrow-icon"><polyline points="9 18 15 12 9 6"></polyline></svg>
          </div>
          <div class="setting-item">
            <span class="item-label">版本号 &nbsp; 2.5.0(4215)</span>
            <button class="text-btn">检查并更新</button>
          </div>
          <div class="setting-item clickable">
            <span class="item-label">功能介绍</span>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="arrow-icon"><polyline points="9 18 15 12 9 6"></polyline></svg>
          </div>
          <div class="setting-item clickable">
            <span class="item-label">帮助与反馈</span>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="arrow-icon"><polyline points="9 18 15 12 9 6"></polyline></svg>
          </div>
        </div>
      </div>

      <div class="footer-links">
        <a href="#" class="footer-link">服务协议</a>
        <span class="footer-divider">|</span>
        <a href="#" class="footer-link">开源版权声明</a>
        <span class="footer-divider">|</span>
        <a href="#" class="footer-link">隐私保护指引</a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const settings = reactive({
  displayMode: 'system',
  fontSize: 16,
  autoStart: true,
  messageNotify: false,
  restoreTabs: true,
  showBookmarkBar: false
});

const goToModelSettings = () => {
  router.push('/settings/model');
};
</script>

<style scoped>
.settings-page {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 32px 40px;
}

.settings-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 28px;
  max-width: 720px;
  width: 100%;
  text-align: left;
}

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-width: 720px;
  width: 100%;
}

.settings-group {
  background-color: var(--bg-primary);
}

.group-title {
  font-size: 14px;
  color: var(--text-tertiary);
  padding: 16px 0 10px;
  font-weight: 400;
}

.group-content {
  background-color: #f7f6f3;
  border-radius: 10px;
  overflow: hidden;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.04);
  min-height: 52px;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item.clickable {
  cursor: pointer;
}

.item-label {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
}

.item-select {
  appearance: none;
  background-color: transparent;
  border: none;
  outline: none;
  font-size: 14px;
  color: var(--text-primary);
  cursor: pointer;
  padding-right: 20px;
  background-image: url("data:image/svg+xml,%3Csvg width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2378716c' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right center;
  font-weight: 500;
}

.slider-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  position: relative;
  flex: 1;
  max-width: 280px;
  justify-content: flex-end;
}

.slider-label {
  font-size: 11px;
  color: var(--text-tertiary);
  white-space: nowrap;
}

.slider-min {
  position: absolute;
  left: 0;
}

.slider-max {
  position: absolute;
  right: 0;
}

.slider-mid {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  top: 18px;
  font-size: 11px;
  color: var(--text-tertiary);
}

.item-slider {
  width: 160px;
  height: 4px;
  appearance: none;
  background: #e5e5e5;
  border-radius: 2px;
  outline: none;
  cursor: pointer;
}

.item-slider::-webkit-slider-thumb {
  appearance: none;
  width: 16px;
  height: 16px;
  background: var(--text-primary);
  border-radius: 50%;
  cursor: pointer;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
}

.item-slider::-moz-range-thumb {
  width: 16px;
  height: 16px;
  background: var(--text-primary);
  border-radius: 50%;
  cursor: pointer;
  border: none;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
}

.toggle-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
  flex-shrink: 0;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  inset: 0;
  background-color: #d4d4d4;
  border-radius: 24px;
  transition: background-color 0.25s ease;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  border-radius: 50%;
  transition: transform 0.25s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
}

.toggle-switch input:checked + .toggle-slider {
  background-color: #10b981;
}

.toggle-switch input:checked + .toggle-slider::before {
  transform: translateX(20px);
}

.shortcut-key {
  font-size: 13px;
  color: var(--text-primary);
  background-color: #f0efe9;
  padding: 5px 12px;
  border-radius: 6px;
  font-family: inherit;
  letter-spacing: 0.3px;
}

.item-link {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 13px;
  color: var(--text-tertiary);
}

.item-link svg {
  color: var(--text-tertiary);
}

.action-btn {
  background-color: var(--text-primary);
  color: white;
  border: none;
  padding: 5px 14px;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  font-weight: 500;
  font-family: inherit;
  transition: opacity 0.15s;
}

.action-btn:hover {
  opacity: 0.85;
}

.arrow-icon {
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.import-action {
  display: flex;
  align-items: center;
  gap: 8px;
}

.red-dot {
  width: 8px;
  height: 8px;
  background-color: #ef4444;
  border-radius: 50%;
  flex-shrink: 0;
}

.text-btn {
  background-color: transparent;
  color: var(--text-primary);
  border: none;
  padding: 5px 14px;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  font-weight: 500;
  font-family: inherit;
}

.text-btn:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

.footer-links {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 28px 0 12px;
  flex-wrap: wrap;
}

.footer-link {
  font-size: 13px;
  color: var(--text-tertiary);
  text-decoration: none;
  transition: color 0.15s;
}

.footer-link:hover {
  color: var(--text-secondary);
}

.footer-divider {
  font-size: 13px;
  color: var(--text-tertiary);
}
</style>
