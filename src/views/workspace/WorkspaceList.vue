<template>
  <div class="workspace-list">
    <h1>{{ t('workspace.title') }}</h1>
    <p>{{ t('workspace.placeholder') }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.workspace-list {
  padding: 24px;
}
</style>
