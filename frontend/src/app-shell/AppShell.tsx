"use client"
import React from 'react'
import { AppHeader, AppDrawer } from './Header'
import Footer from './Footer'

export const AppShell: React.FC<{ children?: React.ReactNode }> = ({ children }) => {
  const [open, setOpen] = React.useState(false)
  return (
    <div>
      <AppHeader onMenu={() => setOpen(true)} />
      <AppDrawer open={open} onClose={() => setOpen(false)} />
      {children}
  <Footer />
    </div>
  )
}

export default AppShell
