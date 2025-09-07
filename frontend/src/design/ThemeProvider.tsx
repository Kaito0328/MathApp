"use client";
import React, { createContext, useContext, useEffect, useState } from 'react';
import { ThemeMode, getSystemTheme } from './tokens';

type Ctx = { theme: ThemeMode; setTheme: (t: ThemeMode) => void };
const ThemeCtx = createContext<Ctx | undefined>(undefined);

export const useTheme = () => {
  const ctx = useContext(ThemeCtx);
  if (!ctx) throw new Error('useTheme must be used within ThemeProvider');
  return ctx;
};

export const ThemeProvider: React.FC<React.PropsWithChildren> = ({ children }) => {
  const [theme, setTheme] = useState<ThemeMode>('light');
  useEffect(() => {
    const initial = getSystemTheme();
    setTheme(initial);
  }, []);
  useEffect(() => {
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('data-theme', theme);
    }
  }, [theme]);
  return <ThemeCtx.Provider value={{ theme, setTheme }}>{children}</ThemeCtx.Provider>;
};
