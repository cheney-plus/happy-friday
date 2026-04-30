<template>
  <div class="editor-wrapper">
    <Milkdown />
  </div>
</template>

<script setup lang="ts">
import { onMounted, nextTick, watch } from 'vue';
import { Milkdown, useEditor } from '@milkdown/vue';
import { Crepe, CrepeFeature } from '@milkdown/crepe';
import { useAppStore } from '@/store';

const props = withDefaults(defineProps<{
  placeholder?: string;
  defaultValue?: string;
}>(), {
  placeholder: 'Start writing...',
  defaultValue: '',
});

const emit = defineEmits<{
  ready: [crepe: Crepe];
  change: [markdown: string];
}>();

const appStore = useAppStore();
const isDark = () => appStore.theme === 'dark';

const { loading } = useEditor((root) => {
  const crepe = new Crepe({
    root,
    defaultValue: props.defaultValue || '',
    features: {
      [CrepeFeature.CodeMirror]: true,
      [CrepeFeature.ListItem]: true,
      [CrepeFeature.LinkTooltip]: true,
      [CrepeFeature.Cursor]: true,
      [CrepeFeature.ImageBlock]: true,
      [CrepeFeature.BlockEdit]: true,
      [CrepeFeature.Toolbar]: true,
      [CrepeFeature.Placeholder]: true,
      [CrepeFeature.Table]: true,
      [CrepeFeature.Latex]: true,
      [CrepeFeature.TopBar]: false,
    },
    featureConfigs: {
      [CrepeFeature.Placeholder]: {
        text: props.placeholder,
        mode: 'doc',
      },
    },
  });

  crepe.on((listener) => {
    listener.markdownUpdated((_ctx, markdown) => {
      emit('change', markdown);
    });
  });

  nextTick(() => {
    emit('ready', crepe);
  });

  return crepe;
});

watch(loading, (isLoading) => {
  if (!isLoading && isDark()) {
    nextTick(() => {
      const milkdownEl = document.querySelector('.milkdown');
      if (milkdownEl) {
        milkdownEl.classList.add('crepe-dark');
      }
    });
  }
});

onMounted(() => {
  if (isDark()) {
    nextTick(() => {
      const milkdownEl = document.querySelector('.milkdown');
      if (milkdownEl) {
        milkdownEl.classList.add('crepe-dark');
      }
    });
  }
});
</script>

<style scoped>
.editor-wrapper {
  flex: 1;
  overflow-y: auto;
  padding: 0 24px;
}

.editor-wrapper :deep(.milkdown) {
  min-height: 100%;
  outline: none;
}

.editor-wrapper :deep(.ProseMirror) {
  padding: 24px 0;
  min-height: 300px;
}

.editor-wrapper :deep(.milkdown .editor) {
  padding: 0;
}
</style>
