"use client"
import React from 'react'
import { Text } from '../../../baseComponents/foundation/Text'
import SaveCopyButton from '../../features/saveCopy/SaveCopyButton'
import SectionPanel from './SectionPanel'
import Row from '../../../baseComponents/layout/Row'

export interface SectionPanelWithTitleProps {
  title: string
  showSave?: boolean
  showCopy?: boolean
  buildSavePayload?: () => any
  copyContent?: string
  disabledSave?: boolean
  disabledCopy?: boolean
  onAfterSave?: (name: string) => void
  children?: React.ReactNode
}

export const SectionPanelWithTitle: React.FC<SectionPanelWithTitleProps> = ({ title, showSave, showCopy, buildSavePayload, copyContent, disabledSave, disabledCopy, onAfterSave, children }) => {
  return (
    <SectionPanel
      header={(
        <Row
          left = {
            <Text weight={600 as any}>{title}</Text>
          }
          
          right = {
              (showSave || showCopy) && (
              <div style={{ marginLeft:'auto' }}>
                <SaveCopyButton
                  buildPayload={buildSavePayload || (()=>null)}
                  copyContent={showCopy ? copyContent : undefined}
                  disabledSave={disabledSave ?? !showSave}
                  disabledCopy={disabledCopy ?? !showCopy}
                  onAfterSave={onAfterSave}
                />
              </div>
            )
          }
        />
      )}
    >
      {children}
    </SectionPanel>
  )
}

export default SectionPanelWithTitle
