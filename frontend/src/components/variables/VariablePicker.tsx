"use client"
import { useVariableStore } from '../../state/VariableStore'

export function VariablePicker({ onPick, placeholder = '選択' }: { onPick: (name: string) => void; placeholder?: string }) {
  const { names } = useVariableStore()
  return (
    <select onChange={(e) => { const v = e.target.value; if (v) onPick(v) }} defaultValue="">
      <option value="" disabled>{placeholder}</option>
      {names.map((n) => (<option key={n} value={n}>{n}</option>))}
    </select>
  )
}
