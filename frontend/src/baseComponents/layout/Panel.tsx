"use client"
import React from 'react'
import { View } from '../foundation/View'
import { CoreColorKey, SizeKey, VariantKey, RoundKey } from '../../design/tokens'

export type PanelProps = React.HTMLAttributes<HTMLDivElement> & {
  color?: CoreColorKey
  variant?: VariantKey
  size?: SizeKey
  bordered?: boolean
  round?: RoundKey
  header?: React.ReactNode
  footer?: React.ReactNode
}

export const Panel: React.FC<PanelProps> = ({
  color = CoreColorKey.Base,
  variant = VariantKey.Soft,
  size = SizeKey.MD,
  bordered = true,
  round = RoundKey.Md,
  header,
  footer,
  children,
  style,
  ...rest
}) => {
  return (
    <View
      color={color}
      variant={variant}
      size={size}
      round={round}
      style={{ borderWidth: bordered ? 1 : undefined, ...(style||{}) }}
      {...rest}
    >
      {header && <div style={{ marginBottom: 8 }}>{header}</div>}
      {children}
      {footer && <div style={{ marginTop: 8 }}>{footer}</div>}
    </View>
  )
}

export default Panel
