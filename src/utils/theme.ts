import { ref, watch } from 'vue';

export type ThemeMode = 'light' | 'dark' | 'system';

const THEME_STORAGE_KEY = 'happy-friday-theme-mode';

const currentMode = ref<ThemeMode>('light');
const appliedTheme = ref<'light' | 'dark'>('light');
let mediaQuery: MediaQueryList | null = null;
let systemChangeListener: ((e: MediaQueryListEvent) => void) | null = null;

const getSystemTheme = (): 'light' | 'dark' => {
  if (typeof window !== 'undefined' && window.matchMedia) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  return 'light';
};

const applyThemeToDOM = (theme: 'light' | 'dark') => {
  if (typeof document !== 'undefined') {
    const root = document.documentElement;
    if (theme === 'dark') {
      root.setAttribute('data-theme', 'dark');
    } else {
      root.removeAttribute('data-theme');
    }
  }
};

const updateAppliedTheme = () => {
  if (currentMode.value === 'system') {
    appliedTheme.value = getSystemTheme();
  } else {
    appliedTheme.value = currentMode.value;
  }
  applyThemeToDOM(appliedTheme.value);
};

const saveToStorage = (mode: ThemeMode) => {
  try {
    localStorage.setItem(THEME_STORAGE_KEY, mode);
  } catch (error) {
    console.error('Failed to save theme:', error);
  }
};

const loadFromStorage = (): ThemeMode => {
  try {
    const stored = localStorage.getItem(THEME_STORAGE_KEY);
    if (stored && ['light', 'dark', 'system'].includes(stored)) {
      return stored as ThemeMode;
    }
  } catch (error) {
    console.error('Failed to load theme:', error);
  }
  return 'light';
};

export const startSystemThemeListener = () => {
  if (typeof window === 'undefined' || !window.matchMedia) return;

  stopSystemThemeListener();

  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  
  systemChangeListener = (e: MediaQueryListEvent) => {
    if (currentMode.value === 'system') {
      appliedTheme.value = e.matches ? 'dark' : 'light';
      applyThemeToDOM(appliedTheme.value);
    }
  };

  mediaQuery.addEventListener('change', systemChangeListener);
};

export const stopSystemThemeListener = () => {
  if (mediaQuery && systemChangeListener) {
    mediaQuery.removeEventListener('change', systemChangeListener);
    mediaQuery = null;
    systemChangeListener = null;
  }
};

export const useTheme = () => {
  const setTheme = (mode: ThemeMode) => {
    currentMode.value = mode;
    saveToStorage(mode);
    updateAppliedTheme();
    
    if (mode === 'system') {
      startSystemThemeListener();
    } else {
      stopSystemThemeListener();
    }
  };

  const initTheme = () => {
    const savedMode = loadFromStorage();
    currentMode.value = savedMode;
    updateAppliedTheme();

    if (savedMode === 'system') {
      startSystemThemeListener();
    }

    watch(currentMode, (newMode) => {
      if (newMode === 'system') {
        startSystemThemeListener();
      } else {
        stopSystemThemeListener();
      }
      updateAppliedTheme();
    });
  };

  return {
    currentMode,
    appliedTheme,
    setTheme,
    initTheme,
    startSystemThemeListener,
    stopSystemThemeListener
  };
};
