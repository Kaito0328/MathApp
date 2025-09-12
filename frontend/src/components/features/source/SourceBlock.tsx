"use client"
import React from 'react'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'
import ReactMarkdown from 'react-markdown'

type Props = {
  title?: string
  path: string // relative path under /workspaces/backend
  showPath?: boolean
}

export default function SourceBlock({ title = 'ソースコード', path, showPath = true }: Props) {
  const [content, setContent] = React.useState<string | null>(null)
  const [language, setLanguage] = React.useState<string>('')
  const [error, setError] = React.useState<string>('')
  const [info, setInfo] = React.useState<string>('')

  React.useEffect(() => {
    let cancelled = false
    const run = async () => {
      setError(''); setInfo(''); setContent(null)
      if (!path) { setError('パスが未指定です'); return }
      try {
        const res = await fetch(`/api/source?path=${encodeURIComponent(path)}`)
        const json = await res.json()
        if (json?.type !== 'file') throw new Error(json?.error || 'ファイルを開けませんでした')
        if (json.binary) { setInfo('バイナリのためプレビュー不可'); return }
        if (json.tooLarge) { setInfo('サイズが大きいためプレビューを省略しました'); return }
        setLanguage(String(json.language || ''))
        setContent(String(json.content ?? ''))
      } catch (e: any) {
        if (!cancelled) setError(String(e?.message || e))
      }
    }
    run()
    return () => { cancelled = true }
  }, [path])

  const md = content != null ? `\n\n\`\`\`${language || ''}\n${content}\n\`\`\`\n` : ''

  return (
    <SectionPanelWithTitle title={title}>
      <div style={{ display:'grid', gap:8 }}>
        {showPath && (
          <div style={{ fontSize: 12, opacity: 0.8 }}>backend/{path}</div>
        )}
        {error && <div style={{ color:'crimson' }}>{error}</div>}
        {info && !error && <div>{info}</div>}
        {!error && !info && content == null && <div>読み込み中...</div>}
        {!error && content != null && (
          <ReactMarkdown>{md}</ReactMarkdown>
        )}
      </div>
    </SectionPanelWithTitle>
  )
}
