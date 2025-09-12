"use client"
import React from 'react'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'

export interface ResultBlockProps {
    children?: React.ReactNode
}

export const ResultBlock: React.FC<ResultBlockProps> = ({children}) => {
    if (!children) {
        return null
    }
  return (
    <SectionPanelWithTitle
      title="結果"
      showCopy={false}
      showSave={false}
    >
        {children}
    </SectionPanelWithTitle>
  )
}

export default ResultBlock
