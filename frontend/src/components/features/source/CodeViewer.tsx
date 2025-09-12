"use client"
import React from 'react'

type Entry = { name: string; kind: 'dir' | 'file' }

type Props = {
  rootRelPath: string // e.g. '' or 'crates/linalg/src'
  initialPath?: string // e.g. 'crates/linalg/src/lib.rs'
}

export default function CodeViewer({ rootRelPath, initialPath }: Props) {
  const [cwd, setCwd] = React.useState<string>(rootRelPath)
  const [sel, setSel] = React.useState<string | null>(initialPath ?? null)
  const [entries, setEntries] = React.useState<Entry[] | null>(null)
  const [file, setFile] = React.useState<{ content?: string; language?: string; tooLarge?: boolean; binary?: boolean } | null>(null)
  const [error, setError] = React.useState<string>('')

  const list = React.useCallback(async (rel: string) => {
    setError(''); setEntries(null)
    try {
      const res = await fetch(`/api/source?path=${encodeURIComponent(rel)}`)
      const json = await res.json()
      if (json?.type !== 'dir') throw new Error(json?.error || 'Not a directory')
      setEntries(json.items as Entry[])
    } catch (e: any) {
      setError(String(e?.message || e))
    }
  }, [])

  const load = React.useCallback(async (rel: string) => {
    setError(''); setFile(null)
    try {
      const res = await fetch(`/api/source?path=${encodeURIComponent(rel)}`)
      const json = await res.json()
      if (json?.type !== 'file') throw new Error(json?.error || 'Not a file')
      setFile({ content: json.content, language: json.language, tooLarge: json.tooLarge, binary: json.binary })
    } catch (e: any) {
      setError(String(e?.message || e))
    }
  }, [])

  React.useEffect(() => { list(cwd) }, [cwd, list])
  React.useEffect(() => { if (sel) load(sel) }, [sel, load])

  const pushDir = (name: string) => setCwd(cwd ? `${cwd}/${name}` : name)
  const upDir = () => setCwd(cwd.split('/').slice(0, -1).join('/'))

  return (
    <div style={{ display:'grid', gridTemplateColumns:'minmax(220px, 28%) 1fr', gap:12, minHeight: 320 }}>
      <div style={{ border:'1px solid var(--border, #ddd)', borderRadius:6, padding:8, overflow:'auto' }}>
        <div style={{ display:'flex', gap:8, alignItems:'center', borderBottom:'1px solid #eee', paddingBottom:6, marginBottom:6 }}>
          <strong>backend/{cwd || '.'}</strong>
          <div style={{ marginLeft:'auto' }}>
            <button onClick={upDir} disabled={!cwd}>Up</button>
          </div>
        </div>
        {error && <div style={{ color:'crimson' }}>{error}</div>}
        {!entries && !error && <div>読み込み中...</div>}
        {entries && (
          <ul style={{ listStyle:'none', padding:0, margin:0 }}>
            {entries.map((e)=> (
              <li key={e.name}>
                {e.kind === 'dir' ? (
                  <button style={{ all:'unset', cursor:'pointer', padding:'4px 6px', display:'block' }} onClick={()=> pushDir(e.name)}>📁 {e.name}</button>
                ) : (
                  <button style={{ all:'unset', cursor:'pointer', padding:'4px 6px', display:'block' }} onClick={()=> setSel(cwd ? `${cwd}/${e.name}` : e.name)}>📄 {e.name}</button>
                )}
              </li>
            ))}
          </ul>
        )}
      </div>
      <div style={{ border:'1px solid var(--border, #ddd)', borderRadius:6, padding:8, overflow:'auto' }}>
        {sel ? (
          file ? (
            file.binary ? <div>バイナリのため表示できません</div>
            : file.tooLarge ? <div>ファイルサイズが大きいためプレビューを省略しました</div>
            : <pre style={{ margin:0, whiteSpace:'pre', overflow:'auto' }}>
                <code>
                  {file.content}
                </code>
              </pre>
          ) : <div>読み込み中...</div>
        ) : (
          <div>左のツリーからファイルを選択してください</div>
        )}
      </div>
    </div>
  )
}
