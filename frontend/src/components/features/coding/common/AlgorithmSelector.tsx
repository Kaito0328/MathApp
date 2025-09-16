"use client"
import React from 'react'

export type AlgorithmOption = {
  id: string
  label: string
}

type Props = {
  options: AlgorithmOption[]
  value: string
  onChange: (id: string) => void
  inline?: boolean
}

export default function AlgorithmSelector({ options, value, onChange, inline }: Props) {
  return (
    <div style={{ display: inline ? 'inline-flex' : 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
      {options.map(opt => (
        <label key={opt.id} style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
          <input
            type="radio"
            name="algorithm"
            checked={value === opt.id}
            onChange={() => onChange(opt.id)}
          />
          {opt.label}
        </label>
      ))}
    </div>
  )
}
