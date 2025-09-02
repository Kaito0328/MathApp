"use client"
import React, { useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { SizeKey, RoundKey, SizeViewProperty, SizeTextProperty, FontWeightKey } from '../../design/tokens'
import type {
  NormalParams, UniformParams, ExponentialParams, GammaParams, ChiSquareParams, StudentTParams, FParams,
  BernoulliParams, BinomialParams, PoissonParams, CategoricalParams,
} from '../../types/distributions'
import { ApplyRow, NumberField, TextAreaNumbers, parseNumbers } from './shared'

export function NormalInput({ value, onChange, label = 'Normal(μ, σ)' }: { value?: NormalParams; onChange: (v: NormalParams) => void; label?: string }) {
  const [mu, setMu] = useState<number | ''>(value?.mu ?? 0)
  const [sigma, setSigma] = useState<number | ''>(value?.sigma ?? 1)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ mu: Number(mu) || 0, sigma: Number(sigma) || 1 })} />
        </div>
      </div>
      <div style={{ display: 'flex', gap: 12 }}>
        <NumberField label="mu" value={mu} onChange={setMu} />
        <NumberField label="sigma" value={sigma} onChange={setSigma} min={0} step={0.01} />
      </div>
    </BaseBox>
  )
}

export function UniformInput({ value, onChange, label = 'Uniform(a, b)' }: { value?: UniformParams; onChange: (v: UniformParams) => void; label?: string }) {
  const [a, setA] = useState<number | ''>(value?.a ?? 0)
  const [b, setB] = useState<number | ''>(value?.b ?? 1)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ a: Number(a) || 0, b: Number(b) || 1 })} />
        </div>
      </div>
      <div style={{ display: 'flex', gap: 12 }}>
        <NumberField label="a" value={a} onChange={setA} />
        <NumberField label="b" value={b} onChange={setB} />
      </div>
    </BaseBox>
  )
}

export function ExponentialInput({ value, onChange, label = 'Exponential(λ)' }: { value?: ExponentialParams; onChange: (v: ExponentialParams) => void; label?: string }) {
  const [lambda, setLambda] = useState<number | ''>(value?.lambda ?? 1)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ lambda: Number(lambda) || 1 })} />
        </div>
      </div>
      <NumberField label="lambda" value={lambda} onChange={setLambda} min={0} step={0.01} />
    </BaseBox>
  )
}

export function GammaInput({ value, onChange, label = 'Gamma(k, θ⁻¹)' }: { value?: GammaParams; onChange: (v: GammaParams) => void; label?: string }) {
  const [shape, setShape] = useState<number | ''>(value?.shape ?? 1)
  const [rate, setRate] = useState<number | ''>(value?.rate ?? 1)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ shape: Number(shape) || 1, rate: Number(rate) || 1 })} />
        </div>
      </div>
      <div style={{ display: 'flex', gap: 12 }}>
        <NumberField label="shape" value={shape} onChange={setShape} min={0} step={0.01} />
        <NumberField label="rate" value={rate} onChange={setRate} min={0} step={0.01} />
      </div>
    </BaseBox>
  )
}

export function ChiSquareInput({ value, onChange, label = 'ChiSquare(k)' }: { value?: ChiSquareParams; onChange: (v: ChiSquareParams) => void; label?: string }) {
  const [k, setK] = useState<number | ''>(value?.k ?? 1)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ k: Number(k) || 1 })} />
        </div>
      </div>
      <NumberField label="k" value={k} onChange={setK} min={1} step={1} />
    </BaseBox>
  )
}

export function StudentTInput({ value, onChange, label = 'StudentT(df)' }: { value?: StudentTParams; onChange: (v: StudentTParams) => void; label?: string }) {
  const [df, setDf] = useState<number | ''>(value?.df ?? 1)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ df: Number(df) || 1 })} />
        </div>
      </div>
      <NumberField label="df" value={df} onChange={setDf} min={1} step={1} />
    </BaseBox>
  )
}

export function FInput({ value, onChange, label = 'F(d1, d2)' }: { value?: FParams; onChange: (v: FParams) => void; label?: string }) {
  const [d1, setD1] = useState<number | ''>(value?.d1 ?? 1)
  const [d2, setD2] = useState<number | ''>(value?.d2 ?? 1)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ d1: Number(d1) || 1, d2: Number(d2) || 1 })} />
        </div>
      </div>
      <div style={{ display: 'flex', gap: 12 }}>
        <NumberField label="d1" value={d1} onChange={setD1} min={1} step={1} />
        <NumberField label="d2" value={d2} onChange={setD2} min={1} step={1} />
      </div>
    </BaseBox>
  )
}

export function BernoulliInput({ value, onChange, label = 'Bernoulli(p)' }: { value?: BernoulliParams; onChange: (v: BernoulliParams) => void; label?: string }) {
  const [p, setP] = useState<number | ''>(value?.p ?? 0.5)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ p: Math.max(0, Math.min(1, Number(p) || 0.5)) })} />
        </div>
      </div>
      <NumberField label="p" value={p} onChange={setP} min={0} step={0.01} />
    </BaseBox>
  )
}

export function BinomialInput({ value, onChange, label = 'Binomial(n, p)' }: { value?: BinomialParams; onChange: (v: BinomialParams) => void; label?: string }) {
  const [n, setN] = useState<number | ''>(value?.n ?? 1)
  const [p, setP] = useState<number | ''>(value?.p ?? 0.5)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ n: Math.max(0, Math.floor(Number(n) || 1)), p: Math.max(0, Math.min(1, Number(p) || 0.5)) })} />
        </div>
      </div>
      <div style={{ display: 'flex', gap: 12 }}>
        <NumberField label="n" value={n} onChange={setN} min={0} step={1} />
        <NumberField label="p" value={p} onChange={setP} min={0} step={0.01} />
      </div>
    </BaseBox>
  )
}

export function PoissonInput({ value, onChange, label = 'Poisson(λ)' }: { value?: PoissonParams; onChange: (v: PoissonParams) => void; label?: string }) {
  const [lambda, setLambda] = useState<number | ''>(value?.lambda ?? 1)
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ lambda: Number(lambda) || 1 })} />
        </div>
      </div>
      <NumberField label="lambda" value={lambda} onChange={setLambda} min={0} step={0.01} />
    </BaseBox>
  )
}

export function CategoricalInput({ value, onChange, label = 'Categorical(p_i)' }: { value?: CategoricalParams; onChange: (v: CategoricalParams) => void; label?: string }) {
  const [probsText, setProbsText] = useState<string>(value?.probs?.join(', ') ?? '')
  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={() => onChange({ probs: parseNumbers(probsText) })} />
        </div>
      </div>
      <TextAreaNumbers label="probs" value={probsText} onChange={setProbsText} rows={2} placeholder={'e.g. 0.2, 0.3, 0.5'} />
    </BaseBox>
  )
}
