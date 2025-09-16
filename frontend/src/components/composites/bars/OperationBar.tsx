"use client"
import React from 'react'

type Props = {
  left?: React.ReactNode
  center?: React.ReactNode
  right?: React.ReactNode
  ariaLabel?: string
}

export default function OperationBar({ left, center, right, ariaLabel }: Props) {
  return (
    <div aria-label={ariaLabel || 'operation bar'}
      style={{
        display: 'grid',
        gridTemplateColumns: '1fr auto 1fr',
        gap: 12,
        alignItems: 'center',
        padding: '8px 12px',
        border: 'none',
        background: 'transparent'
      }}>
      <div style={{ minWidth: 0, display:'flex', gap:8, flexWrap:'wrap' }}>{left}</div>
      <div style={{ display:'flex', justifyContent:'center' }}>{center}</div>
      <div style={{ minWidth: 0, display:'flex', gap:8, justifyContent:'flex-end', flexWrap:'wrap' }}>{right}</div>
    </div>
  )
}
