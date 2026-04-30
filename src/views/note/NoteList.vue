<template>
  <div class="note-list">
    <h1>{{ t('note.title') }}</h1>
    <p>{{ t('note.placeholder') }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.note-list {
  padding: 24px;
}
</style>
