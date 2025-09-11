"use client"
import React from 'react'
import { Text } from '../../baseComponents/foundation/Text'
import { View } from '../../baseComponents/foundation/View'
import { Button } from '../../baseComponents/controls/Button'
import { TextInput } from '../../baseComponents/input/TextInput'
import MatrixInput from '../../widgets/input/MatrixInput'
import VectorInput from '../../widgets/input/VectorInput'
import type { Matrix, Vector } from '../../widgets/dto/linalg'
import { useVariableStore } from '../../state/VariableStore'
import { CoreColorKey, SizeKey, VariantKey } from '../../design/tokens'

type Mode = 'matrix' | 'vector'

export const VariableCreator: React.FC<{ fixedMode?: Mode; initialName?: string }> = ({ fixedMode, initialName }) => {
  const { upsert, names, get } = useVariableStore()
  const [mode, setMode] = React.useState<Mode>(fixedMode ?? 'matrix')
  const [name, setName] = React.useState(initialName ?? '')
  const [matrix, setMatrix] = React.useState<Matrix>({ rows: 2, cols: 2, data: [0, 0, 0, 0] })
  const [vector, setVector] = React.useState<Vector>({ data: [0, 0, 0] })
  const [rows, setRows] = React.useState(2)
  const [cols, setCols] = React.useState(2)
  const [len, setLen] = React.useState(3)

  React.useEffect(() => {
    setMatrix((m) => ({ rows, cols, data: Array.from({ length: rows * cols }, (_, i) => m.data[i] ?? 0) }))
  }, [rows, cols])
  React.useEffect(() => {
    setVector((v) => ({ data: Array.from({ length: len }, (_, i) => v.data[i] ?? 0) }))
  }, [len])

  // Prefill for editing
  React.useEffect(() => {
    if (!initialName) return
    const v = get(initialName)
    if (!v) return
    if (v.kind === 'matrix') {
      setMode('matrix')
      setRows(v.rows)
      setCols(v.cols)
      setMatrix({ rows: v.rows, cols: v.cols, data: v.data.slice() })
    } else if (v.kind === 'vector') {
      setMode('vector')
      setLen(v.length)
      setVector({ data: v.data.slice() })
    }
  }, [initialName, get])

  const exists = (n: string) => names.includes(n)
  const isEditing = !!initialName
  const canSave = name.trim().length > 0 && (isEditing || !exists(name))

  const save = () => {
    if (!canSave) return
    if (mode === 'matrix') {
      upsert(name, { kind: 'matrix', rows: matrix.rows, cols: matrix.cols, data: matrix.data })
    } else {
      upsert(name, { kind: 'vector', length: vector.data.length, data: vector.data })
    }
    window.dispatchEvent(new CustomEvent('variable-creator:close'))
  }

  const canSwitch = !fixedMode
  const panelVariant = VariantKey.Solid

  return (
  <View color={CoreColorKey.Secondary} variant={panelVariant} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12, display: 'grid', gap: 8 }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
        <Text style={{ fontWeight: 600 }}>{isEditing ? '変数編集' : '変数追加'}</Text>
        {canSwitch && (
          <div style={{ marginLeft: 'auto', display: 'flex', gap: 8 }}>
            <Button onClick={() => setMode('matrix')} disabled={mode === 'matrix'}>行列</Button>
            <Button onClick={() => setMode('vector')} disabled={mode === 'vector'}>ベクトル</Button>
          </div>
        )}
      </div>
      <div style={{ display: 'grid', gap: 8 }}>
        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
          <Text>名前</Text>
          <TextInput value={name} onChange={(e) => setName(e.target.value)} placeholder={'名前を入力してください...'} style={{ background: 'transparent' }} disabled={isEditing} />
          {!isEditing && exists(name) && <Text style={{ color: 'crimson' }}>同名の変数が存在します</Text>}
        </div>
        {mode === 'matrix' ? (
          <div style={{ display: 'grid', gap: 8 }}>
            <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
              <Text>サイズ</Text>
              <TextInput type="number" value={rows} min={1} onChange={(e) => setRows(Math.max(1, Number(e.target.value || 0)))} style={{ width: 80 }} />
              <Text>×</Text>
              <TextInput type="number" value={cols} min={1} onChange={(e) => setCols(Math.max(1, Number(e.target.value || 0)))} style={{ width: 80 }} />
            </div>
            <MatrixInput value={matrix} onChange={setMatrix} rows={rows} cols={cols} cellWidth={64} gap={8} />
          </div>
        ) : (
          <div style={{ display: 'grid', gap: 8 }}>
            <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
              <Text>長さ</Text>
              <TextInput type="number" value={len} min={1} onChange={(e) => setLen(Math.max(1, Number(e.target.value || 0)))} style={{ width: 80 }} />
            </div>
            <VectorInput value={vector} onChange={setVector} orientation={'col'} length={len} cellWidth={64} gap={8} />
          </div>
        )}
        <div style={{ display: 'flex', gap: 8 }}>
          <Button onClick={save} disabled={!canSave}>
            保存
          </Button>
          <Button onClick={() => window.dispatchEvent(new CustomEvent('variable-creator:close'))} color={CoreColorKey.Base}>閉じる</Button>
        </div>
      </div>
    </View>
  )
}

export default VariableCreator
