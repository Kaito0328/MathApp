"use client"
import React from 'react'
import ReactMarkdown from 'react-markdown'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import 'katex/dist/katex.min.css'

type Props = {
  docPath: string // path under /workspaces/docs, e.g. 'notes/linalg/matrix_qr.md'
  rustUrl?: string // optional github url to show as placeholder
}

export default function Document({ docPath, rustUrl }: Props) {
  const [md, setMd] = React.useState<string>('')
  const [error, setError] = React.useState<string>('')

  React.useEffect(() => {
    let cancelled = false
    setMd('')
    setError('')
    const load = async () => {
      try {
        const res = await fetch(`/api/docs?path=${encodeURIComponent(docPath)}`)
        if (!res.ok) throw new Error(`HTTP ${res.status}`)
        const json = await res.json()
        if (!cancelled) setMd(String(json.content || ''))
      } catch (e: any) {
        if (!cancelled) setError(String(e?.message || e))
      }
    }
    load()
    return () => { cancelled = true }
  }, [docPath])

  return (
    <div style={{ display:'grid', gap:8 }}>
      {rustUrl && (
        <div style={{ textAlign: 'right' }}>
          <a href={rustUrl} target="_blank" rel="noreferrer" style={{ fontSize: 12, opacity: 0.8 }}>
            Rust 実装（GitHub）
          </a>
        </div>
      )}
      {error ? (
        <span style={{ color:'crimson' }}>ドキュメントの読み込みに失敗しました: {error}</span>
      ) : (
        <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>{md}</ReactMarkdown>
      )}
    </div>
  )
}
