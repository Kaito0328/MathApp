"use client"
import { useMemo, useState } from 'react'
import { View } from '../../../src/baseComponents/foundation/View'
import { Text } from '../../../src/baseComponents/foundation/Text'
import { CoreColorKey, SizeKey, FontWeightKey } from '../../../src/design/tokens'
import { MatrixInput } from '../../../src/widgets/input'
import { MatrixView } from '../../../src/widgets/display'
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
    <Text weight={FontWeightKey.Medium}>A + B（加算）</Text>
    <View style={{ padding: 8, borderWidth: 0 }}>
        <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
      <Text>サイズ</Text>
          <label>rows <input type="number" min={1} value={size.rows} onChange={(e) => setSize(s => ({ ...s, rows: Math.max(1, Math.floor(Number(e.target.value)||1)) }))} /></label>
          <label>cols <input type="number" min={1} value={size.cols} onChange={(e) => setSize(s => ({ ...s, cols: Math.max(1, Math.floor(Number(e.target.value)||1)) }))} /></label>
          <button onClick={() => {
            setA(a => ({ rows: size.rows, cols: size.cols, data: Array(size.rows * size.cols).fill(0).map((_, i) => a.data[i] ?? 0) }))
            setB(b => ({ rows: size.rows, cols: size.cols, data: Array(size.rows * size.cols).fill(0).map((_, i) => b.data[i] ?? 0) }))
          }}>適用</button>
        </div>
  </View>

      <div style={{ display: 'grid', gap: 12, gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))' }}>
    <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12 }}>
          <div style={{ display: 'flex', alignItems: 'center' }}>
      <Text>A</Text>
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
            <MatrixInput value={A} onChange={setA} rows={size.rows} cols={size.cols} />
          </div>
        </View>

    <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12 }}>
          <div style={{ display: 'flex', alignItems: 'center' }}>
      <Text>B</Text>
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
            <MatrixInput value={B} onChange={setB} rows={size.rows} cols={size.cols} />
          </div>
        </View>
      </div>

    <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12 }}>
        <div style={{ display: 'flex', alignItems: 'center' }}>
      <Text>結果</Text>
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
            <Text>{result.error}</Text>
          ) : (
            <MatrixView rows={result.rows} cols={result.cols} data={result.data} />
          )}
        </div>
      </View>
    </div>
  )
}
