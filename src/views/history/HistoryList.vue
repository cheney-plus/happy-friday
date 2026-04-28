<template>
  <div class="historylist">
    <h1>{{ t('history.title') }}</h1>
    <p>{{ t('history.placeholder') }}</p>
    <!-- TODO: i18n -->
    <p v-if="$route.params.id">ID: {{ $route.params.id }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.historylist {
  padding: 24px;
}
</style>
