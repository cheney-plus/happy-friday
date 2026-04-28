<template>
  <div class="userprofile">
    <h1>{{ t('user.title') }}</h1>
    <p>{{ t('user.placeholder') }}</p>
    <!-- TODO: i18n -->
    <p v-if="$route.params.id">ID: {{ $route.params.id }}</p>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<style scoped>
.userprofile {
  padding: 24px;
}
</style>
