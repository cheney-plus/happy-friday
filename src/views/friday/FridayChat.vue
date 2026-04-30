<template>
  <div class="friday-chat">
    <h1>{{ t('friday.title') }}</h1>
    <p>{{ t('friday.placeholder') }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.friday-chat {
  padding: 24px;
}
</style>
