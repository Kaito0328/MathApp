'use client'

import { useEffect, useState } from 'react'
import { initWasm } from './lib/wasm'
import { VectorInput, MatrixInput, SpectrumCard } from './components'
import { SpectrumInput, TransferFunctionInput, ZpkInput, PolynomialTermInput } from '../src/components/inputs'
import { PolynomialCardSimple } from '../src/components/base/PolynomialCardSimple'
import { SignalCardSimple } from '../src/components/base/SignalCardSimple'
import { SignalInputSimple } from '../src/components/inputs/SignalInputSimple'
import { RationalFunctionInputSimple } from '../src/components/inputs/RationalFunctionInputSimple'
import { VectorCard } from '../src/components/base/VectorCard'
import { MatrixCard } from '../src/components/base/MatrixCard'
import { BaseBox } from '../src/design/base/BaseBox'
import { BaseText } from '../src/design/base/BaseText'
import { useTheme } from '../src/design/ThemeProvider'

import { ContinuousPdfCard, DiscretePmfCard } from '../src/components/base/DistributionCards'
import { TransferFunctionCardSimple } from '../src/components/base/LtiCardsSimple'
import { formatPolynomialMarkdown } from '../src/components/utils/polynomial'
import { RationalFunctionCard } from '../src/components/base/RationalFunctionCard'

export default function Page() {
  const [wasmInfo, setWasmInfo] = useState<string>('loading…')
  const [vec, setVec] = useState<number[]>([1, 2, 3, 4])
  const [mat, setMat] = useState<{ rows: number; cols: number; data: number[] }>({ rows: 2, cols: 3, data: [1, 2, 3, 4, 5, 6] })
  const [sig, setSig] = useState<{ data: number[]; sample_rate: number }>({ data: Array.from({ length: 64 }, (_, i) => Math.sin((2*Math.PI*4*i)/64)), sample_rate: 64 })
  const [spec, setSpec] = useState<{ spectrum: number[]; sample_rate: number }>({ spectrum: [], sample_rate: 64 })
  const [rf, setRf] = useState<{ num: number[]; den: number[] }>({ num: [1], den: [1, -1] })
  const [tf, setTf] = useState<{ num: number[]; den: number[]; sample_time?: number | null }>({ num: [1], den: [1, -1], sample_time: undefined })
  const [zpk, setZpk] = useState<{ zeros: number[]; poles: number[]; gain: number; sample_time?: number | null }>({ zeros: [], poles: [], gain: 1 })
  const [rfMd, setRfMd] = useState<string>('')
  const [tfMd, setTfMd] = useState<string>('')
  const [poly, setPoly] = useState<number[]>([1, -3, 2])
  const [polyMd, setPolyMd] = useState<string>('')
  const { theme, setTheme } = useTheme()
  // Stats demo states
  const [normalSvg, setNormalSvg] = useState<string>('')
  const [gammaSvg, setGammaSvg] = useState<string>('')
  const [binomSvg, setBinomSvg] = useState<string>('')
  const [poisSvg, setPoisSvg] = useState<string>('')

  useEffect(() => {
    let mounted = true
    initWasm()
      .then((m: any) => {
        if (!mounted) return
        // Demo: use MatrixF64 which is guaranteed by wasm.d.ts
        const MatrixF64 = (m as any).MatrixF64
        if (MatrixF64) {
          const A = MatrixF64.zeros(2, 3)
          const T = A.transpose()
          const info = `${A.rows()}x${A.cols()} -> transpose ${T.rows()}x${T.cols()}`
          setWasmInfo(info)
          A.free?.(); T.free?.()
        } else {
          setWasmInfo('WASM loaded (MatrixF64 not found)')
        }
      })
  .catch((e: unknown) => console.error('Failed to init wasm', e))
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
      <BaseBox styleKit={{ color: { colorKey: 'base' as any, apply: { default: ['bg','border'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} style={{ borderWidth: 1, margin: '12px 0' }}>
        <BaseText>WASM demo: {wasmInfo}</BaseText>
      </BaseBox>
      {/* Design System Demo */}
    <BaseBox styleKit={{ color: { colorKey: 'primary' as any, apply: { default: ['bg','border'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any, shadowKey: 'sm' as any }}
        className="border-base" style={{ borderWidth: 1, marginBottom: 12 }}>
        <BaseText styleKit={{ color: { colorKey: 'base' as any, apply: { default: ['text'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['fontSize'] as any } }, fontWeightKey: 'bold' as any }}>
          Base components (Box/Text) via tokens
        </BaseText>
      </BaseBox>
  <PolynomialCardSimple coeffs={poly} varName="x" />

      {/* Inputs (new grid-based) */}
      <div style={{ display: 'grid', gap: 12, gridTemplateColumns: '1fr', marginTop: 12 }}>
  <VectorInput value={vec} onChange={setVec} />
  <VectorCard data={vec} title="Vector" showSizeBadge={true} />

  <MatrixInput value={mat} onChange={setMat} />
  <MatrixCard rows={mat.rows} cols={mat.cols} data={mat.data} title="Matrix" showSizeBadge={true} />

  {/* Signal / Spectrum */}
  {/* Signal: モード入力UI（詳細） */}
  <BaseBox styleKit={{ color: { colorKey: 'base' as any, apply: { default: ['bg','border'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} style={{ borderWidth: 1 }}>
    <BaseText>Signal Input (modes)</BaseText>
    <div style={{ marginTop: 8 }}>
      {/* 既存の簡易UIも残す */}
      <SignalInputSimple value={sig as any} onChange={async (s) => {
    setSig(s)
  try {
      const wasm: any = await initWasm()
      const interleavedFa: Float64Array = wasm.dftComplexF64(new Float64Array(s.data))
      setSpec({ spectrum: Array.from(interleavedFa), sample_rate: s.sample_rate })
  } catch { /* ignore DFT errors */ }
  }} />
    </div>
  </BaseBox>
  {/* Signal 表示: プロット色はトークン適用で暗背景でも視認性改善 */}
  <SignalCardSimple data={sig.data} showPlot={true} showVector={false} />
  {/* Spectrum 入力: Re,Im ラベル整頓とモード説明 */}
  <BaseBox styleKit={{ color: { colorKey: 'base' as any, apply: { default: ['bg','border'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} style={{ borderWidth: 1 }}>
    <BaseText>Spectrum Input</BaseText>
    <div style={{ marginTop: 8 }}>
      <SpectrumInput value={spec} onChange={setSpec} />
    </div>
    <div style={{ marginTop: 8 }}>
      <BaseText styleKit={{ size: { sizeKey: 'sm' as any, apply: { default: ['fontSize'] as any } } }}>
        Spectrum(DFT) は、時間領域の信号を複素数の周波数領域に変換した結果（離散フーリエ変換）です。
      </BaseText>
    </div>
  </BaseBox>
  <SpectrumCard spectrum={spec.spectrum} sample_rate={spec.sample_rate} />

  {/* Rational / LTI */}
  {/* Polynomial Input */}
  <PolynomialTermInput value={poly} onChange={setPoly} varName="x" label="Polynomial Input" />
  <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
    <button onClick={() => {
      const md = `P(x) = ${formatPolynomialMarkdown(poly, 'x')}`
      setPolyMd(md)
    }}>Process Polynomial</button>
  </div>
  {polyMd && (
    <BaseBox styleKit={{ color: { colorKey: 'base' as any, apply: { default: ['bg','border'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} style={{ borderWidth: 1 }}>
      <BaseText>
        {`P(x) = ${polyMd.replace(/^P\(x\) =\s*/, '')}`}
      </BaseText>
    </BaseBox>
  )}
  {/* Rational Function 入力（セル x^n 形式）*/}
  <RationalFunctionInputSimple value={rf} onChange={setRf} varName="x" />
  <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
    <button onClick={() => {
      const md = `R(x) = \\frac{${formatPolynomialMarkdown(rf.num, 'x')}}{${formatPolynomialMarkdown(rf.den, 'x')}}`
      setRfMd(md)
    }}>Process Rational</button>
  </div>
  {rfMd && <RationalFunctionCard markdown={rfMd} title="Rational (Processed)" />}

  {/* TransferFunction: 係数入力はセル x^n、z/s の切替想定（varName使用） */}
  <TransferFunctionInput value={tf as any} onChange={setTf as any} varName="z" />
  <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
    <button onClick={() => {
      const md = `H(z) = \\frac{${formatPolynomialMarkdown(tf.num, 'z')}}{${formatPolynomialMarkdown(tf.den, 'z')}}`
      setTfMd(md)
    }}>Process TF</button>
  </div>
  {tfMd && <TransferFunctionCardSimple num={tf.num} den={tf.den} varName="z" title="Transfer Function (Processed)" />}
  {/* ZPK: 入力は Re/Im ペアで #0 などの余計な見出しが出ないよう整理済み */}
  <ZpkInput value={zpk as any} onChange={setZpk as any} />

  {/* Statistics / Distributions (WASM-backed) */}
  <BaseBox styleKit={{ color: { colorKey: 'base' as any, apply: { default: ['bg','border'] as any } }, size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} style={{ borderWidth: 1 }}>
    <BaseText>Distributions (PDF/PMF)</BaseText>
    <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', marginTop: 8 }}>
      <button onClick={async () => {
        const wasm: any = await initWasm()
        const inst = new wasm.Normal(0, 1)
        setNormalSvg(inst.pdf_svg(320, 160, 200))
      }}>Normal(0,1)</button>
      <button onClick={async () => {
        const wasm: any = await initWasm()
        const inst = new wasm.Gamma(2, 1)
        setGammaSvg(inst.pdf_svg(320, 160, 200))
      }}>Gamma(k=2, rate=1)</button>
      <button onClick={async () => {
        const wasm: any = await initWasm()
        const inst = new wasm.Binomial(20, 0.4)
        setBinomSvg(inst.pmf_svg(320, 160))
      }}>Binomial(n=20,p=0.4)</button>
      <button onClick={async () => {
        const wasm: any = await initWasm()
        const inst = new wasm.Poisson(5)
        setPoisSvg(inst.pmf_svg(320, 160))
      }}>Poisson(λ=5)</button>
    </div>
  </BaseBox>
  {normalSvg && <ContinuousPdfCard title="Normal PDF" svg={normalSvg} />}
  {gammaSvg && <ContinuousPdfCard title="Gamma PDF" svg={gammaSvg} />}
  {binomSvg && <DiscretePmfCard title="Binomial PMF" svg={binomSvg} />}
  {poisSvg && <DiscretePmfCard title="Poisson PMF" svg={poisSvg} />}
      </div>
    </main>
  )
}
