<template>
  <div class="editor-wrapper">
    <textarea
      ref="textareaRef"
      class="note-textarea"
      :value="modelValue"
      :placeholder="placeholder"
      @input="onInput"
    ></textarea>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, watch } from 'vue';

const props = withDefaults(defineProps<{
  placeholder?: string;
  modelValue?: string;
}>(), {
  placeholder: 'Start writing...',
  modelValue: '',
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  change: [value: string];
}>();

const textareaRef = ref<HTMLTextAreaElement | null>(null);

const onInput = (e: Event) => {
  const value = (e.target as HTMLTextAreaElement).value;
  emit('update:modelValue', value);
  emit('change', value);
};

const autoResize = () => {
  const el = textareaRef.value;
  if (!el) return;
  el.style.height = 'auto';
  el.style.height = el.scrollHeight + 'px';
};

watch(() => props.modelValue, () => {
  nextTick(autoResize);
});

onMounted(() => {
  nextTick(autoResize);
});
</script>

<style scoped>
.editor-wrapper {
  flex: 1;
  overflow-y: auto;
  padding: 0 24px;
}

.note-textarea {
  width: 100%;
  min-height: 300px;
  padding: 24px 0;
  border: none;
  outline: none;
  resize: none;
  background: transparent;
  font-size: 15px;
  line-height: 1.75;
  color: var(--text-primary);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
}

.note-textarea::placeholder {
  color: var(--text-tertiary);
}
</style>
