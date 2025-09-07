export type ThemeMode = 'light' | 'dark';

export const getSystemTheme = (): ThemeMode => {
  if (typeof window === 'undefined') return 'light';
  return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'dark'
    : 'light';
};
