"use client"
import { useVariableStore } from '../../state/VariableStore'
import { Select } from '../../baseComponents/patterns/Select'

export function VariablePicker({ onPick, placeholder = '選択', allowedKinds, disabled }: { onPick: (name: string) => void; placeholder?: string; allowedKinds?: Array<'matrix' | 'vector'>; disabled?: boolean }) {
  const { names, vars } = useVariableStore() as any
  const filtered = (allowedKinds && allowedKinds.length > 0)
    ? names.filter((n: string) => allowedKinds!.includes((vars[n] as any)?.kind))
    : names
  const isDisabled = disabled || (allowedKinds && allowedKinds.length > 0 && filtered.length === 0)
  return (
    <Select onChange={(e) => { const v = (e.target as HTMLSelectElement).value; if (v) onPick(v) }} defaultValue="" disabled={isDisabled}>
      <option value="" disabled>{placeholder}</option>
  {filtered.map((n: string) => (<option key={n} value={n}>{n}</option>))}
    </Select>
  )
}
