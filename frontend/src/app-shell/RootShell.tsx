"use client"
import React from 'react'
import { VariableStoreProvider } from '../state/VariableStore'

export function RootShell({ children }: { children: React.ReactNode }) {
  return (
    <VariableStoreProvider>
  {children}
    </VariableStoreProvider>
  )
}
