"use client"
import React from 'react'
import { useVariableStore } from '../../../state/VariableStore'
import { Text } from '../../../baseComponents/foundation/Text'
import { View } from '../../../baseComponents/foundation/View'
import { Button } from '../../../baseComponents/controls/Button'
import FilePickerButton from '../../composites/FilePickerButton'
import Modal from '../../../baseComponents/layout/Modal'
import VariableList from './parts/VariableList'
import VariableCreator from './VariableCreator'
import { CoreColorKey, SizeKey } from '../../../design/tokens'
import { parseLatexArray } from '../../../utils/parse/latex'
import { TextInput } from '../../../baseComponents/input/TextInput'
import { TextArea } from '../../../baseComponents/input/TextArea'
// import features are temporarily removed per request

export type VariableSectionProps = {
  kind: 'matrix' | 'vector'
}

export const VariableSection: React.FC<VariableSectionProps> = ({ kind }) => {
  const { names, vars, remove, upsert } = useVariableStore()
  const sectionNames = React.useMemo(() => names.filter((n) => (vars as any)[n]?.kind === kind), [names, vars, kind])
  const [selectionMode, setSelectionMode] = React.useState(false)
  const [selected, setSelected] = React.useState<Set<string>>(new Set())
  const [showAdd, setShowAdd] = React.useState(false)
  const [editing, setEditing] = React.useState<string | null>(null)

  React.useEffect(() => {
    // prune selections when list changes
    setSelected((prev) => new Set(Array.from(prev).filter((n) => sectionNames.includes(n))))
  }, [sectionNames])

  const title = kind === 'matrix' ? '行列' : 'ベクトル'
  const [importOpen, setImportOpen] = React.useState(false)
  const [mdName, setMdName] = React.useState('')
  const [mdText, setMdText] = React.useState('')
  const [mdError, setMdError] = React.useState<string | null>(null)
  const allNames = React.useMemo(() => new Set(names), [names])

  function uniqueName(base: string): string {
    const trimmed = base.trim()
    if (!trimmed) return ''
    if (!allNames.has(trimmed) && !(vars as any)[trimmed]) return trimmed
    let i = 2
    while (true) {
      const cand = `${trimmed}_${i}`
      if (!allNames.has(cand) && !(vars as any)[cand]) return cand
      i++
    }
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
          if (v.kind === 'matrix') {
            const rows = Number(v.rows), cols = Number(v.cols)
            const data = Array.isArray(v.data) ? v.data.map(Number) : null
            if (!Number.isFinite(rows) || !Number.isFinite(cols) || !data || data.some((x: any) => !Number.isFinite(x))) { invalid++; continue }
            if (rows * cols !== data.length) { invalid++; continue }
            const nm = uniqueName(String(key))
            if (!nm) { invalid++; continue }
            upsert(nm, { kind: 'matrix', rows, cols, data })
            added++
          } else if (v.kind === 'vector') {
            const data = Array.isArray(v.data) ? v.data.map(Number) : null
            let length = Number(v.length)
            if (!data || data.some((x: any) => !Number.isFinite(x))) { invalid++; continue }
            if (!Number.isFinite(length) || length !== data.length) length = data.length
            const nm = uniqueName(String(key))
            if (!nm) { invalid++; continue }
            upsert(nm, { kind: 'vector', length, data })
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

  React.useEffect(() => {
  const handler = () => { setShowAdd(false); setEditing(null) }
    window.addEventListener('variable-creator:close', handler as any)
    return () => window.removeEventListener('variable-creator:close', handler as any)
  }, [])

  React.useEffect(() => {
    const onEdit = (e: any) => setEditing(e?.detail?.name ?? null)
    window.addEventListener('variable:edit', onEdit as any)
    return () => window.removeEventListener('variable:edit', onEdit as any)
  }, [])

  const exportKind = () => {
    const obj: Record<string, any> = {}
    for (const n of sectionNames) obj[n] = (vars as any)[n]
    const blob = new Blob([JSON.stringify(obj, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = kind === 'matrix' ? 'matrices.json' : 'vectors.json'
    a.click()
    URL.revokeObjectURL(url)
  }

  // JSON import and markdown import are temporarily disabled

  // quick import UI removed temporarily

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
          <Button onClick={exportKind} color={CoreColorKey.Base}>エクスポート</Button>
          <Button onClick={() => setImportOpen(true)} color={CoreColorKey.Base}>インポート</Button>
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
        editingName={editing}
        kind={kind}
      />

  <div>
  {showAdd ? (
          <div style={{ display: 'grid', gap: 8 }}>
            <VariableCreator /* fixed kind rendering */ fixedMode={kind} key={`add-${kind}`} />
          </div>
        ) : (
          <Button onClick={() => setShowAdd(true)}>＋ 追加</Button>
        )}
      </div>

      <Modal open={importOpen} onClose={() => setImportOpen(false)} title={`インポート（${title}）`}>
        <div style={{ display:'grid', gap:12 }}>
          <div>
            <Text style={{ fontWeight:600 }}>ファイルからインポート（JSON）</Text>
            <div style={{ marginTop:6 }}>
              <FilePickerButton accept="application/json,.json" onFiles={handleJsonFiles} color={CoreColorKey.Base}>ファイルを選択</FilePickerButton>
            </div>
          </div>
          <div style={{ borderTop:'1px solid var(--c-border)', paddingTop:8 }}>
            <Text style={{ fontWeight:600 }}>Markdown（LaTeX）からインポート</Text>
            <div style={{ display:'grid', gap:8, marginTop:6 }}>
              <div style={{ display:'flex', gap:8, alignItems:'center' }}>
                <Text>変数名</Text>
                <div style={{ flex:1 }}>
                  <TextInput value={mdName} onChange={(e)=>setMdName(e.target.value)} placeholder={'例: A'} style={{ width:'100%' }} invalid={!!mdError && !mdName.trim()} />
                </div>
              </div>
              <div>
                <Text>Markdown/LaTeX</Text>
                <TextArea value={mdText} onChange={(e)=>{ setMdText(e.target.value); setMdError(null) }} placeholder={kind==='matrix'? '例) 1 & 2 \\ 3 & 4  または  \\begin{bmatrix} 1 & 2 \\ 3 & 4 \\end{bmatrix}' : '例) 1 \\ 2 \\ 3  または  \\begin{bmatrix} 1 \\ 2 \\ 3 \\end{bmatrix}'} rows={6} style={{ width:'100%' }} invalid={!!mdError && !mdText.trim()} />
              </div>
              {mdError && <Text style={{ color:'crimson' }}>{mdError}</Text>}
              <div style={{ display:'flex', gap:8, justifyContent:'flex-end' }}>
                <Button color={CoreColorKey.Base} onClick={()=>{ setMdName(''); setMdText(''); setMdError(null); setImportOpen(false) }}>キャンセル</Button>
                <Button onClick={()=>{
                  const name = mdName.trim()
                  const body = mdText.trim()
                  if (!name || !body) { setMdError('変数名とMarkdownを入力してください'); return }
                  const res = parseLatexArray(body)
                  if ('error' in res) { setMdError(res.error); return }
                  if (res.kind !== kind) {
                    setMdError(kind==='matrix' ? '行列セクションです。ベクトルが検出されました。' : 'ベクトルセクションです。行列が検出されました。')
                    return
                  }
                  const nm = uniqueName(name)
                  if (!nm) { setMdError('無効な変数名です'); return }
                  if (res.kind === 'matrix') {
                    upsert(nm, { kind: 'matrix', rows: res.rows, cols: res.cols, data: res.data })
                  } else {
                    upsert(nm, { kind: 'vector', length: res.data.length, data: res.data })
                  }
                  alert(`${nm} を追加しました`)
                  setMdName(''); setMdText(''); setMdError(null); setImportOpen(false)
                }} color={CoreColorKey.Primary}>追加</Button>
              </div>
            </div>
          </div>
        </div>
      </Modal>
    </View>
  )
}

export default VariableSection
