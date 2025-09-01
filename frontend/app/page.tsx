'use client'

import { useEffect, useState } from 'react'
import { getWasm } from '../src/wasm'
import * as sp from '../src/wasm/signal_processing'
import type { DftResult, Signal, Vector, Matrix, PolynomialR, TransferFunction } from './types'
import {
  SignalView,
  SpectrumView,
  PolynomialView,
  VectorView,
  MatrixView,
  TransferFunctionView,
  SignalInput,
  VectorInput,
  MatrixInput,
  PolynomialInput,
  TransferFunctionInput,
} from './components'
import { BaseBox } from '../src/design/base/BaseBox'
import { BaseText } from '../src/design/base/BaseText'
import { useTheme } from '../src/design/ThemeProvider'

type Wasm = { add(a: number, b: number): number }

export default function Page() {
  const { theme, setTheme } = useTheme()
  const [wasm, setWasm] = useState<Wasm | null>(null)
  const [sum, setSum] = useState<number | null>(null)

  // Inputs
  const [signal, setSignal] = useState<Signal>(() => {
    const sr = 64, n = 64, f = 4
    const data = Array.from({ length: n }, (_, i) => Math.sin((2 * Math.PI * f * i) / sr))
    return { data, sample_rate: sr }
  })
  const [dft, setDft] = useState<DftResult | null>(null)
  const [vector, setVector] = useState<Vector>({ data: [1, 2, 3, 4] })
  const [matrix, setMatrix] = useState<Matrix>({ rows: 2, cols: 3, data: [1, 2, 3, 4, 5, 6] })
  const [poly, setPoly] = useState<PolynomialR>({ coeffs: [1, -3, 2] })
  const [tf, setTf] = useState<TransferFunction>({ num: [1], den: [1, -1, 0.25], sample_time: null })

  useEffect(() => {
    let mounted = true
    getWasm()
      .then((m) => {
        if (!mounted) return
        setWasm(m as Wasm)
        setSum(m.add(1, 2))
      })
      .catch((e) => console.error('Failed to init wasm', e))
    return () => { mounted = false }
  }, [])

  // Compute DFT when signal or wasm changes
  useEffect(() => {
    if (!wasm) return
    sp.dftReal(signal)
      .then(setDft)
      .catch((e) => console.error('DFT failed', e))
  }, [wasm, signal])

  return (
    <main>
      <h1 style={{ marginTop: 0 }}>MathApp (Next.js + WASM)</h1>
      <div style={{ display: 'flex', gap: 8, marginBottom: 12 }}>
        <button onClick={() => setTheme(theme === 'light' ? 'dark' : 'light')}>Theme: {theme}</button>
      </div>

      {/* Design System Demo */}
      <BaseBox className="border-base" style={{ borderWidth: 1, marginBottom: 12 }}
        styleKit={{ color: { colorKey: 'primary' as any, apply: { default: ['bg','border'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any, shadowKey: 'sm' as any }}>
        <BaseText styleKit={{ color: { colorKey: 'base' as any, apply: { default: ['text'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['fontSize'] as any } }, fontWeightKey: 'bold' as any }}>
          Base components (Box/Text) via tokens
        </BaseText>
      </BaseBox>

      {/* WASM smoke test */}
      <div style={{ border: '1px solid #333', padding: 12, borderRadius: 8, margin: '12px 0' }}>
        <p>WASM add(1,2) = {sum ?? 'loading…'}</p>
      </div>

      {/* Signal I/O */}
      <section style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
        <div>
          <SignalInput value={signal} onChange={setSignal} />
        </div>
        <div>
          <SignalView value={signal} />
          {dft && <SpectrumView value={dft} />}
        </div>
      </section>

      {/* Vector */}
      <section style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
        <div>
          <VectorInput value={vector} onChange={setVector} />
        </div>
        <div>
          <VectorView value={vector} />
        </div>
      </section>

      {/* Matrix */}
      <section style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
        <div>
          <MatrixInput value={matrix} onChange={setMatrix} />
        </div>
        <div>
          <MatrixView value={matrix} />
        </div>
      </section>

      {/* Polynomial */}
      <section style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
        <div>
          <PolynomialInput value={poly} onChange={setPoly} />
        </div>
        <div>
          <PolynomialView value={poly} />
        </div>
      </section>

      {/* Transfer Function */}
      <section style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
        <div>
          <TransferFunctionInput value={tf} onChange={setTf} />
        </div>
        <div>
          <TransferFunctionView value={tf} />
        </div>
      </section>
    </main>
  )
}
