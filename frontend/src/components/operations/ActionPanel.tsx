"use client"
import React from 'react'
import Panel from '../../baseComponents/layout/Panel'
import { Text } from '../../baseComponents/foundation/Text'
import SaveCopyButtons from './SaveCopyButtons'

export interface ActionPanelProps {
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

export const ActionPanel: React.FC<ActionPanelProps> = ({ title, showSave, showCopy, buildSavePayload, copyContent, disabledSave, disabledCopy, onAfterSave, children }) => {
  return (
    <Panel
      header={(
        <div style={{ display:'flex', alignItems:'center', gap:8 }}>
          <Text weight={600 as any}>{title}</Text>
          {(showSave || showCopy) && (
            <div style={{ marginLeft:'auto' }}>
              <SaveCopyButtons
                buildPayload={buildSavePayload || (()=>null)}
                copyContent={showCopy ? copyContent : undefined}
                disabledSave={disabledSave ?? !showSave}
                disabledCopy={disabledCopy ?? !showCopy}
                onAfterSave={onAfterSave}
              />
            </div>
          )}
        </div>
      )}
    >
      {children}
    </Panel>
  )
}

export default ActionPanel
