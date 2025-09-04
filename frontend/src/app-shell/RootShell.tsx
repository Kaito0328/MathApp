"use client"
import React from 'react'
import { VariableStoreProvider } from '../state/VariableStore'
import { AppHeader, AppDrawer } from './Header'

export function RootShell({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = React.useState(false)
  return (
    <VariableStoreProvider>
      <div>
        <AppHeader onMenu={() => setOpen(true)} />
        <AppDrawer open={open} onClose={() => setOpen(false)} />
        <main style={{ padding: 16 }}>{children}</main>
      </div>
    </VariableStoreProvider>
  )
}
