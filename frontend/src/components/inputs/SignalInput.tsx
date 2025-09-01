"use client"
import React, { useMemo, useState } from 'react'
import type { Signal } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'

function parseNumbers(src: string): number[] {
	return src
		.split(/[,\s]+/)
		.map(s => s.trim())
		.filter(s => s.length > 0)
		.map(Number)
		.filter(n => Number.isFinite(n))
}

function genSine(n: number, sr: number, freq: number): number[] {
	return Array.from({ length: n }, (_, i) => Math.sin((2 * Math.PI * freq * i) / sr))
}

function genSquare(n: number, sr: number, freq: number): number[] {
	return Array.from({ length: n }, (_, i) => (Math.sin((2 * Math.PI * freq * i) / sr) >= 0 ? 1 : -1))
}

function genNoise(n: number): number[] {
	return Array.from({ length: n }, () => (Math.random() * 2 - 1))
}

export function SignalInput({ value, onChange }: { value?: Signal; onChange: (s: Signal) => void }) {
	const [mode, setMode] = useState<'manual' | 'sine' | 'square' | 'noise'>('sine')
	const [n, setN] = useState<number>(value?.data.length ?? 64)
	const [sr, setSr] = useState<number>(value?.sample_rate ?? 64)
	const [freq, setFreq] = useState<number>(4)
	const [text, setText] = useState<string>((value?.data ?? []).join(', '))

	const data = useMemo(() => {
		if (mode === 'manual') return parseNumbers(text)
		if (mode === 'sine') return genSine(n, sr, freq)
		if (mode === 'square') return genSquare(n, sr, freq)
		return genNoise(n)
	}, [mode, text, n, sr, freq])

	const apply = () => onChange({ data, sample_rate: sr })

	return (
		<BaseBox styleKit={{ size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} className="border-base" style={{ borderWidth: 1 }}>
			<BaseText styleKit={{ size: { sizeKey: 'md' as any, apply: { default: ['fontSize'] as any } }, fontWeightKey: 'medium' as any }}>Signal Input</BaseText>
			<div style={{ display: 'flex', flexDirection: 'column', gap: 8, marginTop: 8 }}>
				<div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
					<label>
						<BaseText>mode</BaseText>{' '}
						<select value={mode} onChange={(e) => setMode(e.target.value as any)}>
							<option value="sine">sine</option>
							<option value="square">square</option>
							<option value="noise">noise</option>
							<option value="manual">manual</option>
						</select>
					</label>
					<label>
						<BaseText>length</BaseText>{' '}
						<input type="number" value={n} min={2} onChange={(e) => setN(Number(e.target.value) || 0)} />
					</label>
					<label>
						<BaseText>sample_rate</BaseText>{' '}
						<input type="number" value={sr} min={1} onChange={(e) => setSr(Number(e.target.value) || 0)} />
					</label>
					{(mode === 'sine' || mode === 'square') && (
						<label>
							<BaseText>freq</BaseText>{' '}
							<input type="number" value={freq} min={0} onChange={(e) => setFreq(Number(e.target.value) || 0)} />
						</label>
					)}
				</div>
				{mode === 'manual' && (
					<textarea
						value={text}
						onChange={(e) => setText(e.target.value)}
						rows={4}
						placeholder={'e.g. 0, 1, 0, -1, ...'}
						style={{ width: '100%', fontFamily: 'ui-monospace, monospace' }}
					/>
				)}
				<div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
					<button onClick={apply}>Apply</button>
					<BaseText>preview length: {data.length}</BaseText>
				</div>
			</div>
		</BaseBox>
	)
}

