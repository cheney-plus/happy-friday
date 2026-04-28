import { createI18n } from 'vue-i18n';

// Automatically load all json files
const modules = import.meta.glob('./locales/*/*.json', { eager: true }) as Record<string, any>;

const messages: Record<string, any> = {
  'zh-CN': {},
  'en-US': {}
};

for (const path in modules) {
  const match = path.match(/\.\/locales\/([^/]+)\/([^/]+)\.json$/);
  if (match) {
    const lang = match[1];
    const moduleName = match[2];
    messages[lang][moduleName] = modules[path].default || modules[path];
  }
}

export const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'en-US',
  messages
});

export function setI18nLanguage(lang: string) {
  i18n.global.locale.value = lang as any;
}
