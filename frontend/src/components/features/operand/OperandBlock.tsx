"use client"
import React from 'react'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'

export interface OperandBlockProps {
    title?: string
  children?: React.ReactNode
  copyContent?: string
    buildSavePayload: () => any
    onAfterSave: (name: string) => void
    disabledSave?: boolean
    disabledCopy?: boolean
}

export const OperandBlock: React.FC<OperandBlockProps> = ({children, title = "入力", copyContent, buildSavePayload, onAfterSave, disabledSave, disabledCopy}) => {
  return (
    <SectionPanelWithTitle
      title={title}
      showCopy={true}
      showSave={true}
        copyContent={copyContent}
        buildSavePayload={buildSavePayload}
        onAfterSave={onAfterSave}
        disabledSave={disabledSave}
        disabledCopy={disabledCopy}
    >
        {children}  
    </SectionPanelWithTitle>
  )
}

export default OperandBlock
