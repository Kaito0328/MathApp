import js from '@eslint/js'
import globals from 'globals'
import tseslint from 'typescript-eslint'
import reactHooks from 'eslint-plugin-react-hooks'
import { FlatCompat } from '@eslint/eslintrc'
import path from 'node:path'

const compat = new FlatCompat({
  baseDirectory: path.resolve('.'),
})

export default tseslint.config([
  {
  ignores: ['.next/**', 'node_modules/**', 'dist/**', 'src/wasm-pkg/**'],
  },
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      // Next.js recommended (via legacy config compat)
      ...compat.extends('next'),
      ...compat.extends('next/core-web-vitals'),
    ],
    plugins: { 'react-hooks': reactHooks },
    rules: {
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',
      // Temporarily disable until tokens APIs are fully typed without casts
      '@typescript-eslint/no-explicit-any': 'off',
    },
    languageOptions: {
      ecmaVersion: 2022,
      globals: {
        ...globals.browser,
      },
    },
  },
  {
    files: ['next-env.d.ts'],
    rules: {
      '@typescript-eslint/triple-slash-reference': 'off',
    },
  },
])
