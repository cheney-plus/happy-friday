<template>
  <div class="note-edit">
    <h1>{{ t('note.title') }}</h1>
    <p>{{ t('note.placeholder') }}</p>
    <p v-if="$route.params.id">ID: {{ $route.params.id }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.note-edit {
  padding: 24px;
}
</style>
