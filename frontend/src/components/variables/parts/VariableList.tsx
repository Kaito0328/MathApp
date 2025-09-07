"use client"
import React from 'react'
import { Text } from '../../../baseComponents/foundation/Text'
import VariableRow from './VariableRow'
import VariableCreator from '../VariableCreator'

export type VariableListProps = {
  names: string[]
  vars: Record<string, any>
  onRemove: (name: string) => void
  selectionMode?: boolean
  selected?: Set<string>
  onToggleSelect?: (name: string, checked: boolean) => void
  editingName?: string | null
  kind?: 'matrix' | 'vector'
}

export const VariableList: React.FC<VariableListProps> = ({ names, vars, onRemove, selectionMode, selected, onToggleSelect, editingName, kind }) => {
  if (!names?.length) return <Text>登録された変数はありません</Text>
  return (
    <div style={{ display: 'grid', gap: 6 }}>
      {names.map((name) => (
        name === editingName ? (
          <div key={`edit-${name}`} style={{ display: 'grid', gap: 8 }}>
            <VariableCreator fixedMode={kind} initialName={name} />
          </div>
        ) : (
          <VariableRow
            key={name}
            name={name}
            value={vars[name]}
            onRemove={onRemove}
            selectionMode={!!selectionMode}
            checked={selected ? selected.has(name) : false}
            onCheckChange={onToggleSelect}
          />
        )
      ))}
    </div>
  )
}

export default VariableList
