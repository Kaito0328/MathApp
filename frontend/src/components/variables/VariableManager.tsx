"use client"
import React from 'react'
import { useVariableStore } from '../../state/VariableStore'
import { Card } from '../../baseComponents/patterns/Card'
import { CoreColorKey, SizeKey } from '../../design/tokens'
import VariableToolbar from './parts/VariableToolbar'
import VariableList from './parts/VariableList'

export function VariableManager() {
  const { names, vars, remove, clear, upsert } = useVariableStore()

  const exportAll = () => {
    const blob = new Blob([JSON.stringify(vars, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'variables.json'
    a.click()
    URL.revokeObjectURL(url)
  }

  const onImportFiles = async (files: FileList) => {
    try {
      const tasks = Array.from(files).map(async (file) => {
        const text = await file.text()
        const obj = JSON.parse(text)
        if (obj && typeof obj === 'object') {
          for (const [k, v] of Object.entries(obj)) {
            const mv = v as any
            if (mv && mv.kind === 'matrix' && Number.isInteger(mv.rows) && Number.isInteger(mv.cols) && Array.isArray(mv.data)) {
              upsert(k, { kind: 'matrix', rows: mv.rows, cols: mv.cols, data: mv.data })
            }
          }
        }
      })
      await Promise.all(tasks)
    } catch {
      alert('インポートに失敗しました')
    }
  }

  const onClearAll = () => { if (confirm('全削除しますか？')) clear() }

  return (
    <Card color={CoreColorKey.Base} size={SizeKey.MD} style={{ gap: 8 }}>
      <VariableToolbar onExport={exportAll} onImportFiles={onImportFiles} onClearAll={onClearAll} />
      <VariableList names={names} vars={vars as any} onRemove={remove} />
    </Card>
  )
}
