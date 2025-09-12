"use client"
import React from 'react'
import SaveButton from './SaveButton'
import CopyButton from './CopyButton'
import Row from '../../../baseComponents/layout/Row'

export interface SaveCopyButtonsProps {
  buildPayload: () => any // object stored into variable store
  promptMessage?: string
  copyContent?: string
  disabledSave?: boolean
  disabledCopy?: boolean
  copyAriaLabel?: string
  saveAriaLabel?: string
  onAfterSave?: (name: string) => void
}

export const SaveCopyButton: React.FC<SaveCopyButtonsProps> = ({
  buildPayload,
  promptMessage = '保存する変数名',
  copyContent,
  disabledSave,
  disabledCopy,
  copyAriaLabel='コピー',
  saveAriaLabel='保存',
  onAfterSave,
}) => {
  return (
    <Row
      center={
        <>
          <SaveButton
            buildPayload={buildPayload}
            promptMessage={promptMessage}
            disabled={disabledSave}
            ariaLabel={saveAriaLabel}
            onAfterSave={onAfterSave}
          />
          <CopyButton
            copyContent={copyContent}
            disabled={disabledCopy}
            ariaLabel={copyAriaLabel}
          />
        </>
      }
    />

  )
}

export default SaveCopyButton
