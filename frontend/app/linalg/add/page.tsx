"use client"
import { useMemo, useState } from 'react'
import { BaseBox } from '../../../src/design/base/BaseBox'
import { BaseText } from '../../../src/design/base/BaseText'
import { CoreColorKey, ColorViewProperty, SizeKey, SizeViewProperty, SizeTextProperty, FontWeightKey } from '../../../src/design/tokens'
import { MatrixInput } from '../../components'
import { MatrixCard } from '../../../src/components/base/MatrixCard'
import { useVariableStore } from '../../../src/state/VariableStore'
import { VariablePicker } from '../../../src/components/variables/VariablePicker'

type MatrixDTO = { rows: number; cols: number; data: number[] }

function addMatrices(A: MatrixDTO, B: MatrixDTO): MatrixDTO | { error: string } {
  if (A.rows !== B.rows || A.cols !== B.cols) return { error: 'サイズが一致しません（加算は同型行列が必要）' }
  const data = new Array(A.rows * A.cols)
  for (let i = 0; i < data.length; i++) data[i] = (A.data[i] ?? 0) + (B.data[i] ?? 0)
  return { rows: A.rows, cols: A.cols, data }
}

export default function LinalgAdd() {
  const { get, upsert } = useVariableStore()
  const [A, setA] = useState<MatrixDTO>({ rows: 2, cols: 2, data: [1, 2, 3, 4] })
  const [B, setB] = useState<MatrixDTO>({ rows: 2, cols: 2, data: [5, 6, 7, 8] })
  const [size, setSize] = useState<{ rows: number, cols: number }>({ rows: 2, cols: 2 })

  const result = useMemo(() => addMatrices(A, B), [A, B])

  return (
    <div style={{ display: 'grid', gap: 12 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>A + B（加算）</BaseText>
      <BaseBox style={{ padding: 8, borderWidth: 0 }}>
        <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
          <BaseText>サイズ</BaseText>
          <label>rows <input type="number" min={1} value={size.rows} onChange={(e) => setSize(s => ({ ...s, rows: Math.max(1, Math.floor(Number(e.target.value)||1)) }))} /></label>
          <label>cols <input type="number" min={1} value={size.cols} onChange={(e) => setSize(s => ({ ...s, cols: Math.max(1, Math.floor(Number(e.target.value)||1)) }))} /></label>
          <button onClick={() => {
            setA(a => ({ rows: size.rows, cols: size.cols, data: Array(size.rows * size.cols).fill(0).map((_, i) => a.data[i] ?? 0) }))
            setB(b => ({ rows: size.rows, cols: size.cols, data: Array(size.rows * size.cols).fill(0).map((_, i) => b.data[i] ?? 0) }))
          }}>適用</button>
        </div>
      </BaseBox>

      <div style={{ display: 'grid', gap: 12, gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))' }}>
        <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } } }} style={{ borderWidth: 1 }}>
          <div style={{ display: 'flex', alignItems: 'center' }}>
            <BaseText>A</BaseText>
            <div style={{ marginLeft: 'auto', display: 'flex', gap: 6 }}>
              <VariablePicker placeholder="変数から代入" onPick={(name) => {
                const v = get(name)
                if (v && (v as any).kind === 'matrix') {
                  const m = v as any as MatrixDTO
                  setA({ rows: m.rows, cols: m.cols, data: m.data })
                }
              }} />
              <button onClick={() => {
                const name = window.prompt('保存する変数名')?.trim()
                if (name) upsert(name, { kind: 'matrix', rows: A.rows, cols: A.cols, data: A.data })
              }}>変数に保存</button>
            </div>
          </div>
          <div style={{ marginTop: 8 }}>
            <MatrixInput value={A} onChange={setA} controlledSize={size} hideSizeControls />
          </div>
        </BaseBox>

        <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } } }} style={{ borderWidth: 1 }}>
          <div style={{ display: 'flex', alignItems: 'center' }}>
            <BaseText>B</BaseText>
            <div style={{ marginLeft: 'auto', display: 'flex', gap: 6 }}>
              <VariablePicker placeholder="変数から代入" onPick={(name) => {
                const v = get(name)
                if (v && (v as any).kind === 'matrix') {
                  const m = v as any as MatrixDTO
                  setB({ rows: m.rows, cols: m.cols, data: m.data })
                }
              }} />
              <button onClick={() => {
                const name = window.prompt('保存する変数名')?.trim()
                if (name) upsert(name, { kind: 'matrix', rows: B.rows, cols: B.cols, data: B.data })
              }}>変数に保存</button>
            </div>
          </div>
          <div style={{ marginTop: 8 }}>
            <MatrixInput value={B} onChange={setB} controlledSize={size} hideSizeControls />
          </div>
        </BaseBox>
      </div>

      <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } } }} style={{ borderWidth: 1 }}>
        <div style={{ display: 'flex', alignItems: 'center' }}>
          <BaseText>結果</BaseText>
          <div style={{ marginLeft: 'auto' }}>
            <button onClick={() => {
              if ('error' in result) return
              const name = window.prompt('保存する変数名')?.trim()
              if (!name) return
              upsert(name, { kind: 'matrix', rows: result.rows, cols: result.cols, data: result.data })
            }}>結果を変数に保存</button>
          </div>
        </div>
        <div style={{ marginTop: 8 }}>
          {'error' in result ? (
            <BaseText>{result.error}</BaseText>
          ) : (
            <MatrixCard rows={result.rows} cols={result.cols} data={result.data} showSizeBadge title="A + B" />
          )}
        </div>
      </BaseBox>
    </div>
  )
}
