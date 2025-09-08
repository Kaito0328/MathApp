"use client"
import React from 'react'
import { useVariableStore } from '../../state/VariableStore'
import { Text } from '../../baseComponents/foundation/Text'
import { View } from '../../baseComponents/foundation/View'
import { Button } from '../../baseComponents/patterns/Button'
import VariableList from './parts/VariableList'
import { CoreColorKey, SizeKey } from '../../design/tokens'
import Modal from '../../baseComponents/patterns/Modal'
import FilePickerButton from '../../baseComponents/patterns/FilePickerButton'
import PolyVariableCreator from './PolyVariableCreator'

export type PolyKinds = 'polynomial' | 'rational'

export const PolyVariableSection: React.FC<{ kind: PolyKinds }> = ({ kind }) => {
  const { names, vars, remove } = useVariableStore()
  const { upsert } = useVariableStore()
  const sectionNames = React.useMemo(() => names.filter((n) => (vars as any)[n]?.kind === kind), [names, vars, kind])
  const [selectionMode, setSelectionMode] = React.useState(false)
  const [selected, setSelected] = React.useState<Set<string>>(new Set())
  const [showAdd, setShowAdd] = React.useState(false)

  React.useEffect(() => {
    setSelected((prev) => new Set(Array.from(prev).filter((n) => sectionNames.includes(n))))
  }, [sectionNames])

  const title = kind === 'polynomial' ? '多項式' : '有理関数'
  const [importOpen, setImportOpen] = React.useState(false)

  const exportKind = () => {
    const obj: Record<string, any> = {}
    for (const n of sectionNames) obj[n] = (vars as any)[n]
    const blob = new Blob([JSON.stringify(obj, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = kind === 'polynomial' ? 'polynomials.json' : 'rationals.json'
    a.click()
    URL.revokeObjectURL(url)
  }

  async function handleJsonFiles(files: FileList) {
    const file = files[0]
    if (!file) return
    try {
      const text = await file.text()
      const parsed = JSON.parse(text)
      let added = 0
      let skipped = 0
      let invalid = 0
      if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) {
        for (const key of Object.keys(parsed)) {
          const v = (parsed as any)[key]
          if (!v || typeof v !== 'object') { invalid++; continue }
          if (v.kind !== kind) { skipped++; continue }
          if (v.kind === 'polynomial') {
            const coeffs = Array.isArray(v.coeffs) ? v.coeffs.map(Number) : null
            if (!coeffs || coeffs.some((x: any) => !Number.isFinite(x))) { invalid++; continue }
            upsert(key, { kind: 'polynomial', coeffs })
            added++
          } else if (v.kind === 'rational') {
            const num = Array.isArray(v.numerator) ? v.numerator.map(Number) : null
            const den = Array.isArray(v.denominator) ? v.denominator.map(Number) : null
            if (!num || !den || num.some((x: any) => !Number.isFinite(x)) || den.some((x: any) => !Number.isFinite(x))) { invalid++; continue }
            upsert(key, { kind: 'rational', numerator: num, denominator: den })
            added++
          } else {
            invalid++
          }
        }
      } else {
        invalid++
      }
      alert(`${added}件を追加しました（スキップ: ${skipped}, 無効: ${invalid}）`)
      setImportOpen(false)
    } catch (e: any) {
      alert(`JSONの解析に失敗しました: ${e?.message ?? e}`)
    }
  }

  const onDeleteSelected = () => {
    if (!selected.size) return
    if (!confirm(`${selected.size}件を削除しますか？`)) return
    for (const n of Array.from(selected)) remove(n)
    setSelected(new Set())
    setSelectionMode(false)
  }

  const toggleSelect = (name: string, checked: boolean) => {
    setSelected((prev) => {
      const next = new Set(prev)
      if (checked) next.add(name); else next.delete(name)
      return next
    })
  }

  return (
    <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12, display: 'grid', gap: 8 }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
        <Text style={{ fontWeight: 700 }}>変数</Text>
        <Text style={{ fontWeight: 600 }}>{title}</Text>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: 8, alignItems: 'center' }}>
          <Button onClick={() => setImportOpen(true)} color={CoreColorKey.Base}>インポート</Button>
          <Button onClick={exportKind} color={CoreColorKey.Base}>エクスポート</Button>
          {!selectionMode ? (
            <Button onClick={() => setSelectionMode(true)} color={CoreColorKey.Primary}>選択</Button>
          ) : (
            <>
              <Button onClick={onDeleteSelected} disabled={!selected.size} color={CoreColorKey.Danger}>一括削除</Button>
              <Button onClick={() => { setSelectionMode(false); setSelected(new Set()) }} color={CoreColorKey.Base}>キャンセル</Button>
            </>
          )}
        </div>
      </div>

      <VariableList
        names={sectionNames}
        vars={vars as any}
        onRemove={remove}
        selectionMode={selectionMode}
        selected={selected}
        onToggleSelect={toggleSelect}
        kind={undefined}
      />

      <div>
        {showAdd ? (
          <div style={{ display: 'grid', gap: 8 }}>
            <PolyVariableCreator kind={kind} />
          </div>
        ) : (
          <Button onClick={() => setShowAdd(true)}>＋ 追加</Button>
        )}
      </div>

      <Modal open={importOpen} onClose={() => setImportOpen(false)} title={`${title}のインポート（JSON）`}>
        <div style={{ display:'grid', gap:12 }}>
          <div>
            <Text>JSONファイルを選択してください</Text>
            <div style={{ marginTop:6 }}>
              <FilePickerButton accept="application/json,.json" onFiles={handleJsonFiles} color={CoreColorKey.Base}>ファイルを選択</FilePickerButton>
            </div>
          </div>
          <div style={{ opacity:0.8 }}>
            形式の例:
            <pre style={{ whiteSpace:'pre-wrap' }}>{`{
  "P1": { "kind": "polynomial", "coeffs": [1, 0, -1] },
  "F": { "kind": "rational", "numerator": [1, 0], "denominator": [1, 1] }
}`}</pre>
          </div>
        </div>
      </Modal>
    </View>
  )
}

export default PolyVariableSection
