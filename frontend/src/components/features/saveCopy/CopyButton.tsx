"use client"
import React from 'react'
import { Button } from '../../../baseComponents/controls/Button'
import { SizeKey, VariantKey, CoreColorKey } from '../../../design/tokens'
import { IconCopy } from '../../../baseComponents/icons'

export interface SaveCopyButtonsProps {
  copyContent?: string
  disabled?: boolean
  ariaLabel?: string
  size?: SizeKey,
  color?: CoreColorKey,
  variant?: VariantKey
}

export const CopyButton: React.FC<SaveCopyButtonsProps> = ({
  copyContent,
  disabled,
  ariaLabel='コピー',
    size = SizeKey.SM,
    color = CoreColorKey.Base,
    variant = VariantKey.Solid
}) => {
  return (
    <>
      {copyContent != null && (
        <Button aria-label={ariaLabel} title={ariaLabel} size={size} color={color} variant={variant} disabled={disabled} onClick={() => navigator.clipboard?.writeText(copyContent)}>
                <IconCopy />
            </Button>
        )}
    </>

  )
}

export default CopyButton
