"use client"
import React from 'react'
import { Button } from '../../baseComponents/controls/Button'
import { SizeKey, VariantKey, CoreColorKey } from '../../design/tokens'
import { useVariableStore } from '../../state/VariableStore'
import { IconSave, IconCopy } from './icons'
import ActionGroup from './ActionGroup'

export interface SaveCopyButtonsProps {
  buildPayload: () => any // object stored into variable store
  promptMessage?: string
  copyContent?: string
  disabledSave?: boolean
  disabledCopy?: boolean
  copyAriaLabel?: string
  saveAriaLabel?: string
  onAfterSave?: (name: string) => void
}

export const SaveCopyButtons: React.FC<SaveCopyButtonsProps> = ({
  buildPayload,
  promptMessage = '保存する変数名',
  copyContent,
  disabledSave,
  disabledCopy,
  copyAriaLabel='コピー',
  saveAriaLabel='保存',
  onAfterSave,
}) => {
  const { upsert } = useVariableStore()
  return (
    <ActionGroup>
      <Button aria-label={saveAriaLabel} title={saveAriaLabel} size={SizeKey.SM} variant={VariantKey.Solid} disabled={disabledSave} onClick={()=>{
        const name = window.prompt(promptMessage)?.trim();
        if(!name) return;
        const payload = buildPayload();
        if(!payload) return;
        upsert(name, payload);
        onAfterSave?.(name);
      }}>
        <IconSave />
      </Button>
      {copyContent != null && (
        <Button aria-label={copyAriaLabel} title={copyAriaLabel} size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} disabled={disabledCopy} onClick={()=> navigator.clipboard?.writeText(copyContent)}>
          <IconCopy />
        </Button>
      )}
    </ActionGroup>
  )
}

export default SaveCopyButtons
