import React from 'react'

export function Sparkline({ data, width = 240, height = 48, color = '#0af' }: { data: number[]; width?: number; height?: number; color?: string }) {
  if (!data || data.length === 0) return <div style={{ width, height, background: '#111' }} />
  const w = width
  const h = height
  const max = Math.max(...data)
  const min = Math.min(...data)
  const range = max - min || 1
  const points = data
    .map((v, i) => {
      const x = (i / (data.length - 1)) * w
      const y = h - ((v - min) / range) * h
      return `${x.toFixed(2)},${y.toFixed(2)}`
    })
    .join(' ')
  return (
    <svg width={w} height={h} viewBox={`0 0 ${w} ${h}`}>
      <rect width={w} height={h} fill="#111" />
      <polyline fill="none" stroke={color} strokeWidth={1} points={points} />
    </svg>
  )
}

export function KV({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div style={{ display: 'flex', gap: 8 }}>
      <div style={{ width: 140, color: '#999' }}>{label}</div>
      <div>{children}</div>
    </div>
  )
}

export function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section style={{ border: '1px solid #333', padding: 12, borderRadius: 8, margin: '12px 0' }}>
      <h3 style={{ margin: '4px 0 8px', fontSize: 16 }}>{title}</h3>
      {children}
    </section>
  )
}
