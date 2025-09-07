"use client"
import React from 'react'
import { Text } from '../../../baseComponents/foundation/Text'
import { Button } from '../../../baseComponents/patterns/Button'
import FilePickerButton from '../../../baseComponents/patterns/FilePickerButton'

export type VariableToolbarProps = {
  onExport: () => void
  onImportFiles: (files: FileList) => void
  onClearAll: () => void
}

export const VariableToolbar: React.FC<VariableToolbarProps> = ({ onExport, onImportFiles, onClearAll }) => {
  return (
    <div style={{ display: 'flex', alignItems: 'center' }}>
      <Text>変数</Text>
      <div style={{ marginLeft: 'auto', display: 'flex', gap: 8 }}>
        <Button onClick={onExport}>エクスポート</Button>
        <FilePickerButton accept="application/json" onFiles={onImportFiles}>インポート</FilePickerButton>
        <Button onClick={onClearAll}>全削除</Button>
      </div>
    </div>
  )
}

export default VariableToolbar
