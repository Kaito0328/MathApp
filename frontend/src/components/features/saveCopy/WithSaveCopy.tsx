"use client"
import React from 'react'
import SaveCopyButton from './SaveCopyButton'
import Row from '../../../baseComponents/layout/Row'

type Props = {
  children: React.ReactNode
  buildSavePayload?: () => any
  copyContent?: string
  showSave?: boolean
  showCopy?: boolean
  disabledSave?: boolean
  disabledCopy?: boolean
  onAfterSave?: (name: string) => void
  // レイアウト: 子要素の右側にアクションを寄せる
  alignRight?: boolean
}

export const WithSaveCopy: React.FC<Props> = ({
  children,
  buildSavePayload,
  copyContent,
  showSave = true,
  showCopy = true,
  disabledSave,
  disabledCopy,
  onAfterSave,
}) => {
  return (
    <Row
      left={children}
      right={(showSave || showCopy) && (
          <SaveCopyButton
            buildPayload={buildSavePayload || (()=>null)}
            copyContent={showCopy ? copyContent : undefined}
            disabledSave={disabledSave ?? !showSave}
            disabledCopy={disabledCopy ?? !showCopy}
            onAfterSave={onAfterSave}
          />
      )}
    />
  )
}

export default WithSaveCopy
