<template>
  <div class="notelist">
    <h1>{{ t('note.title') }}</h1>
    <p>{{ t('note.placeholder') }}</p>
    <!-- TODO: i18n -->
    <p v-if="$route.params.id">ID: {{ $route.params.id }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.notelist {
  padding: 24px;
}
</style>
