<template>
  <div class="schedule-calendar">
    <h1>{{ t('schedule.title') }}</h1>
    <p>{{ t('schedule.placeholder') }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.schedule-calendar {
  padding: 24px;
}
</style>
