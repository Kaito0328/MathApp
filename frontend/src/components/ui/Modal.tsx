"use client"
import React from 'react'

type Props = {
  open: boolean
  onClose: () => void
  title?: string
  children: React.ReactNode
}

export default function Modal({ open, onClose, title, children }: Props) {
  if (!open) return null
  return (
    <div style={{ position:'fixed', inset:0, background:'rgba(0,0,0,0.32)', zIndex:1000 }} onClick={onClose}>
      <div style={{ position:'absolute', inset:'5% 5% auto 5%', background:'#fff', borderRadius:8, padding:12, maxHeight:'90%', overflow:'auto' }} onClick={(e)=> e.stopPropagation()}>
        <div style={{ display:'flex', alignItems:'center', gap:8, borderBottom:'1px solid #eee', paddingBottom:6, marginBottom:8 }}>
          <strong>{title ?? ''}</strong>
          <button onClick={onClose} style={{ marginLeft:'auto' }}>閉じる</button>
        </div>
        {children}
      </div>
    </div>
  )
}
