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
      <Text weight={FontWeightKey.Medium}>A × B（乗算）</Text>
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
            <MatrixInput value={A} onChange={setA} />
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
            <MatrixInput value={B} onChange={setB} />
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
