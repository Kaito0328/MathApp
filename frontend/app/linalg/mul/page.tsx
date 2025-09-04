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

function mulMatrices(A: MatrixDTO, B: MatrixDTO): MatrixDTO | { error: string } {
  if (A.cols !== B.rows) return { error: 'サイズが一致しません（乗算は A.cols === B.rows が必要）' }
  const rows = A.rows, cols = B.cols, kdim = A.cols
  const data = new Array(rows * cols).fill(0)
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      let s = 0
      for (let k = 0; k < kdim; k++) s += (A.data[r * A.cols + k] ?? 0) * (B.data[k * B.cols + c] ?? 0)
      data[r * cols + c] = s
    }
  }
  return { rows, cols, data }
}

export default function LinalgMul() {
  const { get, upsert } = useVariableStore()
  const [A, setA] = useState<MatrixDTO>({ rows: 2, cols: 3, data: [1,2,3,4,5,6] })
  const [B, setB] = useState<MatrixDTO>({ rows: 3, cols: 2, data: [7,8,9,10,11,12] })

  const result = useMemo(() => mulMatrices(A, B), [A, B])

  return (
    <div style={{ display: 'grid', gap: 12 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>A × B（乗算）</BaseText>
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
            <MatrixInput value={A} onChange={setA} />
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
            <MatrixInput value={B} onChange={setB} />
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
            <MatrixCard rows={result.rows} cols={result.cols} data={result.data} showSizeBadge title="A × B" />
          )}
        </div>
      </BaseBox>
    </div>
  )
}
