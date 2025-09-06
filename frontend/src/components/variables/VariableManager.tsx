"use client"
import React from 'react'
import { useVariableStore } from '../../state/VariableStore'
import { View } from '../../baseComponents/foundation/View'
import { Text } from '../../baseComponents/foundation/Text'
import { CoreColorKey, SizeKey } from '../../design/tokens'
import { formatMatrixMarkdown } from '../../utils/format/markdown'

export function VariableManager() {
  const { names, vars, remove, clear, upsert } = useVariableStore()
  const fileRef = React.useRef<HTMLInputElement>(null)

  const exportAll = () => {
    const blob = new Blob([JSON.stringify(vars, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'variables.json'
    a.click()
    URL.revokeObjectURL(url)
  }

  const onImport = async (file: File) => {
    try {
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
    } catch {
      alert('インポートに失敗しました')
    }
  }

  return (
    <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12 }}>
      <div style={{ display: 'flex', alignItems: 'center' }}>
        <Text>変数</Text>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: 8 }}>
          <button onClick={exportAll}>エクスポート</button>
          <button onClick={() => fileRef.current?.click()}>インポート</button>
          <button onClick={() => { if (confirm('全削除しますか？')) clear() }}>全削除</button>
        </div>
        <input ref={fileRef} type="file" accept="application/json" style={{ display: 'none' }} onChange={(e) => {
          const f = e.target.files?.[0]
          if (f) onImport(f)
          e.currentTarget.value = ''
        }} />
      </div>
      <div style={{ marginTop: 8, display: 'grid', gap: 6 }}>
        {names.length === 0 && <Text>登録された変数はありません</Text>}
        {names.map((name) => {
          const v = vars[name] as any
          const label = v?.kind === 'matrix' ? `matrix [${v.rows} x ${v.cols}]` : 'unknown'
          const preview = v?.kind === 'matrix' ?
            Array.from({ length: Math.min(v.rows, 3) }, (_, r) =>
              Array.from({ length: Math.min(v.cols, 6) }, (_, c) => v.data[r * v.cols + c] ?? 0).join(' ')
            ).join(' | ') : ''
          const toMarkdown = () => {
            if (!(v && v.kind === 'matrix')) return ''
            return formatMatrixMarkdown(v.rows, v.cols, v.data)
          }
          return (
            <div key={name} style={{ display: 'grid', gridTemplateColumns: '1fr auto', alignItems: 'center', gap: 8 }}>
              <div>
                <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                  <Text>{name}</Text>
                  <Text style={{ opacity: 0.7 }}>— {label}</Text>
                </div>
                {preview && <div style={{ marginTop: 4 }}><Text style={{ opacity: 0.8 }}>{preview}</Text></div>}
              </div>
              <div style={{ display: 'flex', gap: 8 }}>
                <button onClick={() => {
                  const md = toMarkdown()
                  if (!md) return
                  navigator.clipboard?.writeText(md)
                }}>Markdownコピー</button>
                <button onClick={() => remove(name)}>削除</button>
              </div>
            </div>
          )
        })}
      </div>
    </View>
  )
}
