"use client"
import React from 'react'
import { Button } from '../../../baseComponents/controls/Button'
import { CoreColorKey, SizeKey, VariantKey } from '../../../design/tokens'
import { useVariableStore } from '../../../state/VariableStore'
import { IconSave } from '../../../baseComponents/icons'

export interface SaveCopyButtonsProps {
  buildPayload: () => any // object stored into variable store
  promptMessage?: string
  disabled?: boolean
  ariaLabel?: string
  onAfterSave?: (name: string) => void
  size?: SizeKey
  color?: CoreColorKey,
    variant?: VariantKey
}

export const SaveButton: React.FC<SaveCopyButtonsProps> = ({
  buildPayload,
  promptMessage = '保存する変数名',
  disabled,
  ariaLabel='保存',
  onAfterSave,
  size = SizeKey.SM,
  color = CoreColorKey.Primary,
  variant = VariantKey.Solid
}) => {
  const { upsert } = useVariableStore()
  return (
      <Button aria-label={ariaLabel} title={ariaLabel} size={size} color={color} variant={variant} disabled={disabled} onClick={()=>{
        const name = window.prompt(promptMessage)?.trim();
        if(!name) return;
        const payload = buildPayload();
        if(!payload) return;
        upsert(name, payload);
        onAfterSave?.(name);
      }}>
        <IconSave />
      </Button>
  )
}

export default SaveButton
