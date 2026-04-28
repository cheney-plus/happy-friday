<template>
  <div class="workspacedetail">
    <h1>{{ t('workspace.title') }}</h1>
    <p>{{ t('workspace.placeholder') }}</p>
    <!-- TODO: i18n -->
    <p v-if="$route.params.id">ID: {{ $route.params.id }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.workspacedetail {
  padding: 24px;
}
</style>
