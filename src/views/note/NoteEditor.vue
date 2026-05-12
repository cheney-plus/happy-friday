<template>
  <div ref="containerRef" class="editor-wrapper"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue';
import { createRoot, Root } from 'react-dom/client';
import React from 'react';
import BlockNoteEditorComponent from './BlockNoteEditor';

const props = withDefaults(defineProps<{
  placeholder?: string;
  modelValue?: string;
}>(), {
  placeholder: '开始写作...',
  modelValue: '',
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  change: [value: string];
}>();

const containerRef = ref<HTMLDivElement | null>(null);
let root: Root | null = null;

const handleChange = (markdown: string) => {
  emit('update:modelValue', markdown);
  emit('change', markdown);
};

const renderEditor = () => {
  if (!containerRef.value) return;
  if (!root) {
    root = createRoot(containerRef.value);
  }
  root.render(
    React.createElement(BlockNoteEditorComponent, {
      initialContent: props.modelValue,
      placeholder: props.placeholder,
      onChange: handleChange,
    })
  );
};

watch(() => props.modelValue, () => {
});

watch(() => props.placeholder, () => {
  renderEditor();
});

onMounted(() => {
  renderEditor();
});

onBeforeUnmount(() => {
  if (root) {
    root.unmount();
    root = null;
  }
});
</script>

<style scoped>
.editor-wrapper {
  flex: 1;
  overflow-y: auto;
  padding: 0 32px 0 48px;
}
</style>
