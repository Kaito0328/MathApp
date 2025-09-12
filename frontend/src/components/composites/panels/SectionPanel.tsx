"use client"
import React from 'react'
import Panel from '../../../baseComponents/layout/Panel'

export interface SectionPanelProps {
  children?: React.ReactNode
  header?: React.ReactNode
}

export const SectionPanel: React.FC<SectionPanelProps> = ({children, header}) => {
  return (
    <Panel
      header={header}
    >
      {children}
    </Panel>
  )
}

export default SectionPanel
