'use client'

import { useEffect, useState } from 'react'
import { initWasm } from './lib/wasm'
import type { DftResult } from './types'
import { SignalView, SpectrumView, PolynomialView } from './components'
import { BaseBox } from '../src/design/base/BaseBox'
import { BaseText } from '../src/design/base/BaseText'
import { useTheme } from '../src/design/ThemeProvider'

export default function Page() {
  const [sum, setSum] = useState<number | null>(null)
  const [dft, setDft] = useState<DftResult | null>(null)
  const [demoSignal, setDemoSignal] = useState<number[]>([])
  const { theme, setTheme } = useTheme()

  useEffect(() => {
    let mounted = true
    initWasm()
      .then((m) => {
        if (!mounted) return
        setSum(m.add(1, 2))
        const sr = 64
        const n = 64
        const data = Array.from({ length: n }, (_, i) => Math.sin((2 * Math.PI * 4 * i) / sr))
        setDemoSignal(data)
        const obj = m.dft_real_obj(data, sr)
        setDft(obj)
      })
      .catch((e) => console.error('Failed to init wasm', e))
    return () => {
      mounted = false
    }
  }, [])

  return (
    <main>
      <h1 style={{ marginTop: 0 }}>MathApp (Next.js + WASM)</h1>
      <div style={{ display: 'flex', gap: 8, marginBottom: 12 }}>
        <button onClick={() => setTheme(theme === 'light' ? 'dark' : 'light')}>Theme: {theme}</button>
      </div>
      <div style={{ border: '1px solid #333', padding: 12, borderRadius: 8, margin: '12px 0' }}>
        <p>WASM add(1,2) = {sum ?? 'loading…'}</p>
      </div>
      {/* Design System Demo */}
      <BaseBox styleKit={{ color: { colorKey: 'primary' as any, apply: { default: ['bg','border'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any, shadowKey: 'sm' as any }}
        className="border-base" style={{ borderWidth: 1, marginBottom: 12 }}>
        <BaseText styleKit={{ color: { colorKey: 'base' as any, apply: { default: ['text'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['fontSize'] as any } }, fontWeightKey: 'bold' as any }}>
          Base components (Box/Text) via tokens
        </BaseText>
      </BaseBox>
      {demoSignal.length > 0 && (
        <SignalView value={{ data: demoSignal, sample_rate: 64 }} />
      )}
      {dft && <SpectrumView value={dft} />}
      <PolynomialView value={{ coeffs: [1, -3, 2] }} />
    </main>
  )
}
