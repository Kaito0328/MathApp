"use client"
import React from 'react'
import SaveCopyButtons from './SaveCopyButtons'

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

export const WithActions: React.FC<Props> = ({
  children,
  buildSavePayload,
  copyContent,
  showSave = true,
  showCopy = true,
  disabledSave,
  disabledCopy,
  onAfterSave,
  alignRight = false,
}) => {
  return (
    <div style={{ display:'inline-flex', alignItems:'center', gap:8 }}>
      {children}
      {(showSave || showCopy) && (
        <div style={{ marginLeft: alignRight ? 'auto' : undefined }}>
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
  )
}

export default WithActions
