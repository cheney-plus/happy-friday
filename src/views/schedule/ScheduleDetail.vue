<template>
  <div class="schedule-detail">
    <h1>{{ t('schedule.title') }}</h1>
    <p>{{ t('schedule.placeholder') }}</p>
    <p v-if="$route.params.id">ID: {{ $route.params.id }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.schedule-detail {
  padding: 24px;
}
</style>
