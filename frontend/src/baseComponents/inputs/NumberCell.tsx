"use client"
import React from 'react'
import { TextInput } from '../patterns/TextInput'

export type NumberCellProps = {
  value: number
  onChange: (value: number) => void
  placeholder?: string
  step?: number
  min?: number
  max?: number
  width?: number | string
  disabled?: boolean
  title?: string
}

export const NumberCell: React.FC<NumberCellProps> = ({ value, onChange, placeholder, step = 1, min, max, width = 72, disabled, title }) => {
  const [text, setText] = React.useState<string>(Number.isFinite(value) ? String(value) : '')
  React.useEffect(() => {
    const incoming = Number.isFinite(value) ? String(value) : ''
    if (incoming !== text) setText(incoming)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [value])

  const commit = (raw: string) => {
    const n = parseFloat(raw)
    if (Number.isFinite(n)) onChange(n)
    else if (raw.trim() === '') onChange(0)
  }

  return (
    <TextInput
      type="number"
      className="numberCell"
      value={text}
      onChange={(e) => setText(e.target.value)}
      onBlur={(e) => commit(e.target.value)}
      step={step as any}
      min={min as any}
      max={max as any}
      placeholder={placeholder}
      disabled={disabled}
      title={title}
      style={{ width }}
    />
  )
}

export default NumberCell
