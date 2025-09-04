"use client"
import React from 'react'
import { useVariableStore } from '../../state/VariableStore'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, ColorViewProperty, SizeKey, SizeViewProperty } from '../../design/tokens'

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
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } } }} style={{ borderWidth: 1 }}>
      <div style={{ display: 'flex', alignItems: 'center' }}>
        <BaseText>変数</BaseText>
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
        {names.length === 0 && <BaseText>登録された変数はありません</BaseText>}
        {names.map((name) => {
          const v = vars[name] as any
          const label = v?.kind === 'matrix' ? `matrix [${v.rows} x ${v.cols}]` : 'unknown'
          const preview = v?.kind === 'matrix' ?
            Array.from({ length: Math.min(v.rows, 3) }, (_, r) =>
              Array.from({ length: Math.min(v.cols, 6) }, (_, c) => v.data[r * v.cols + c] ?? 0).join(' ')
            ).join(' | ') : ''
          const toMarkdown = () => {
            if (!(v && v.kind === 'matrix')) return ''
            const lines = [] as string[]
            for (let r = 0; r < v.rows; r++) {
              const row = [] as string[]
              for (let c = 0; c < v.cols; c++) row.push(String(v.data[r * v.cols + c] ?? 0))
              lines.push(row.join(' & '))
            }
            return lines.join(' // ')
          }
          return (
            <div key={name} style={{ display: 'grid', gridTemplateColumns: '1fr auto', alignItems: 'center', gap: 8 }}>
              <div>
                <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                  <BaseText>{name}</BaseText>
                  <BaseText style={{ opacity: 0.7 }}>— {label}</BaseText>
                </div>
                {preview && <div style={{ marginTop: 4 }}><BaseText style={{ opacity: 0.8 }}>{preview}</BaseText></div>}
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
    </BaseBox>
  )
}
