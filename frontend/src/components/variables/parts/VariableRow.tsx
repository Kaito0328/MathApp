"use client"
import React from 'react'
import { Text } from '../../../baseComponents/foundation/Text'
import { Button } from '../../../baseComponents/controls/Button'
import { variableLabel, variablePreview, variableToMarkdown } from './VariableUtils'
import { MatrixView, VectorView } from '../../../widgets/display'
import { CoreColorKey, VariantKey, SizeKey } from '../../../design/tokens'

export type VariableRowProps = {
  name: string
  value: any
  onRemove: (name: string) => void
  selectionMode?: boolean
  checked?: boolean
  onCheckChange?: (name: string, checked: boolean) => void
}

export const VariableRow: React.FC<VariableRowProps> = ({ name, value, onRemove, selectionMode, checked, onCheckChange }) => {
  const label = variableLabel(value)
  const preview = variablePreview(value)
  const copyMarkdown = () => {
    const md = variableToMarkdown(value)
    if (md) {
      navigator.clipboard?.writeText(md)
      // Lightweight feedback
      try {
        const el = document.createElement('div')
        el.textContent = 'コピーしました'
        el.style.position = 'fixed'
        el.style.bottom = '16px'
        el.style.left = '50%'
        el.style.transform = 'translateX(-50%)'
        el.style.background = 'rgba(0,0,0,0.75)'
        el.style.color = 'white'
        el.style.padding = '6px 10px'
        el.style.borderRadius = '8px'
        el.style.fontSize = '12px'
        el.style.zIndex = '100'
        document.body.appendChild(el)
        setTimeout(() => el.remove(), 1200)
      } catch {
        // ignore clipboard/toast errors silently
      }
    }
  }
  return (
    <div style={{ display: 'grid', gridTemplateColumns: selectionMode ? 'auto 1fr auto' : '1fr auto', alignItems: 'center', gap: 8 }}>
      {selectionMode && (
        <input type="checkbox" checked={!!checked} onChange={(e) => onCheckChange?.(name, e.target.checked)} />
      )}
      <div>
        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
          <Text style={{ fontWeight: 600 }}>{name}</Text>
          <Text style={{ opacity: 0.8, fontWeight: 500 }}>— {label}</Text>
        </div>
        <div style={{ marginTop: 4 }}>
          {value?.kind === 'matrix' ? (
            <MatrixView rows={value.rows} cols={value.cols} data={value.data} />
          ) : value?.kind === 'vector' ? (
            <VectorView orientation='col' values={value.data} />
          ) : preview ? (
            <Text style={{ opacity: 0.8 }}>{preview}</Text>
          ) : null}
        </div>
      </div>
      <div style={{ display: 'flex', gap: 4 }}>
        <Button
          onClick={copyMarkdown}
          color={CoreColorKey.Base}
          variant={VariantKey.Solid}
          size={SizeKey.SM}
          aria-label="コピー"
          title="コピー"
        >
          {/* Copy icon */}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
          </svg>
        </Button>
        <Button
          onClick={() => window.dispatchEvent(new CustomEvent('variable:edit', { detail: { name } }))}
          variant={VariantKey.Solid}
          size={SizeKey.SM}
          aria-label="編集"
          title="編集"
        >
          {/* Edit (pencil) icon */}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M12 20h9" />
            <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4 12.5-12.5z" />
          </svg>
        </Button>
        <Button
          onClick={() => onRemove(name)}
          color={CoreColorKey.Danger}
          variant={VariantKey.Solid}
          size={SizeKey.SM}
          aria-label="削除"
          title="削除"
        >
          {/* Trash icon */}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <polyline points="3 6 5 6 21 6" />
            <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" />
            <path d="M10 11v6" />
            <path d="M14 11v6" />
            <path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" />
          </svg>
        </Button>
      </div>
    </div>
  )
}

export default VariableRow
