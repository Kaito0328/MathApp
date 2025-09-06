"use client"
import React from 'react'
import { AppHeader, AppDrawer } from './Header'
import PageContainer, { PageContainerProps } from '../baseComponents/patterns/PageContainer'

export type AppPageProps = Omit<PageContainerProps, 'children'> & {
  children?: React.ReactNode
  footer?: React.ReactNode
}

export const AppPage: React.FC<AppPageProps> = ({ title, actions, children, footer, breadcrumbs, tabs, stickyHeader, maxWidth = 1080, gutters = 12 }) => {
  const [open, setOpen] = React.useState(false)
  return (
    <div>
      <AppHeader onMenu={() => setOpen(true)} />
      <AppDrawer open={open} onClose={() => setOpen(false)} />
  <PageContainer title={title} actions={actions} breadcrumbs={breadcrumbs} tabs={tabs} stickyHeader={stickyHeader} maxWidth={maxWidth} gutters={gutters}>
        {children}
      </PageContainer>
      {footer}
    </div>
  )
}

export default AppPage
