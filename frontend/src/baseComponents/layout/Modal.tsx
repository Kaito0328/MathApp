"use client"
import React from 'react'
import { View } from '../foundation/View'
import { Text } from '../foundation/Text'
import { Button } from '../controls/Button'
import { CoreColorKey, SizeKey } from '../../design/tokens'

export type ModalProps = {
  open: boolean
  title?: string
  onClose: () => void
  children: React.ReactNode
  width?: number | string
}

export const Modal: React.FC<ModalProps> = ({ open, title, onClose, children, width = 480 }) => {
  if (!open) return null
  return (
    <div style={{ position:'fixed', inset:0, background:'rgba(0,0,0,0.45)', display:'grid', placeItems:'center', zIndex: 50 }} onClick={onClose}>
      <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth:1, width, maxWidth:'90vw', maxHeight:'85vh', overflow:'auto', borderRadius:8, boxShadow:'0 8px 24px rgba(0,0,0,0.25)' }} onClick={(e)=>e.stopPropagation()}>
        <div style={{ display:'flex', alignItems:'center', padding:'8px 12px', borderBottom:'1px solid var(--c-border)' }}>
          <Text style={{ fontWeight:700 }}>{title ?? ''}</Text>
          <Button onClick={onClose} color={CoreColorKey.Base} style={{ marginLeft:'auto' }} aria-label="閉じる" title="閉じる">×</Button>
        </div>
        <div style={{ padding:12, display:'grid', gap:8 }}>
          {children}
        </div>
      </View>
    </div>
  )
}

export default Modal
