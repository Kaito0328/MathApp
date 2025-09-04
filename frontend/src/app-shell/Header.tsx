"use client"
import { useState } from 'react'
import { BaseBox } from '../design/base/BaseBox'
import { BaseText } from '../design/base/BaseText'
import { CoreColorKey, ColorViewProperty, SizeKey, SizeViewProperty, RoundKey, SizeTextProperty, FontWeightKey } from '../design/tokens'
import Link from 'next/link'

export function AppHeader({ onMenu }: { onMenu: () => void }) {
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Primary, apply: { default: [ColorViewProperty.Bg] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } } }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
        <button aria-label="menu" onClick={onMenu} style={{ background: 'transparent', border: 'none', color: 'inherit', fontSize: 20 }}>
          ☰
        </button>
        <Link href="/" style={{ textDecoration: 'none' }}>
          <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>MathApp</BaseText>
        </Link>
        <div style={{ marginLeft: 'auto' }} />
      </div>
    </BaseBox>
  )
}

export function AppDrawer({ open, onClose }: { open: boolean; onClose: () => void }) {
  const [openLinalg, setOpenLinalg] = useState<boolean>(true)
  return (
    <div style={{ position: 'fixed', inset: 0, pointerEvents: open ? 'auto' : 'none', zIndex: 1000 }}>
      {/* backdrop */}
      <div onClick={onClose} style={{ position: 'absolute', inset: 0, background: 'rgba(0,0,0,0.35)', opacity: open ? 1 : 0, transition: 'opacity 150ms' }} />
      {/* drawer */}
      <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}
        style={{ position: 'absolute', top: 0, bottom: 0, left: 0, width: 300, borderRightWidth: 1, transform: `translateX(${open ? 0 : -320}px)`, transition: 'transform 220ms ease', boxShadow: '0 10px 30px rgba(0,0,0,0.25)' }}>
        <div style={{ display: 'flex', alignItems: 'center' }}>
          <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>メニュー</BaseText>
          <div style={{ marginLeft: 'auto' }}>
            <button aria-label="close" onClick={onClose} style={{ background: 'transparent', border: 'none', fontSize: 18 }}>✕</button>
          </div>
        </div>
        <nav style={{ display: 'grid', gap: 10, marginTop: 12 }}>
          <Link href="/" onClick={onClose}>ホーム</Link>

          <div>
            <button onClick={() => setOpenLinalg((v: boolean) => !v)} style={{ display: 'flex', alignItems: 'center', gap: 8, width: '100%', background: 'transparent', border: 'none', padding: 0, cursor: 'pointer' }}>
              <span style={{ transform: `rotate(${openLinalg ? 90 : 0}deg)`, transition: 'transform 150ms' }}>▶</span>
              <span>線形代数</span>
            </button>
            <div style={{ display: 'grid', gap: 8, marginTop: 8, maxHeight: openLinalg ? 500 : 0, overflow: 'hidden', transition: 'max-height 200ms ease' }}>
              <Link href="/linalg" onClick={onClose} style={{ paddingLeft: 16 }}>ホーム</Link>
              <Link href="/linalg/add" onClick={onClose} style={{ paddingLeft: 16 }}>加算 A + B</Link>
              <Link href="/linalg/mul" onClick={onClose} style={{ paddingLeft: 16 }}>乗算 A × B</Link>
              <Link href="/linalg/decomp" onClick={onClose} style={{ paddingLeft: 16 }}>分解 (LU/QR/SVD)</Link>
            </div>
          </div>

          {/* 将来: FFT, 信号処理, 統計 など追加 */}
        </nav>
      </BaseBox>
    </div>
  )
}
