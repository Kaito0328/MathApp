"use client"
import React from 'react'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'

export interface ResultBlockProps {
    children?: React.ReactNode
}

export const VerficationBlock: React.FC<ResultBlockProps> = ({children}) => {
    if (!children) {
        return null
    }
  return (
    <SectionPanelWithTitle
      title="検証"
      showCopy={false}
      showSave={false}
    >
        {children}
    </SectionPanelWithTitle>
  )
}

export default VerficationBlock
