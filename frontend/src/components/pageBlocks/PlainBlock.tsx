"use client"
import React from 'react'
import { View } from '../../baseComponents/foundation/View'
import { VariantKey } from '../../design/tokens'

export type PlainBlockProps = React.HTMLAttributes<HTMLDivElement> & {
  borderColor?: string
  radius?: number
  padding?: number
}

// PlainBlock: page-level block using Card but forcing transparent background & thin secondary-like border.
export const PlainBlock: React.FC<PlainBlockProps> = ({
  borderColor = 'rgba(0,0,0,0.18)',
  radius = 6,
  padding = 12,
  style,
  children,
  ...rest
}) => {
  return (
    <View
      color={'base' as any}
      variant={VariantKey.Outline}
      style={{
        background: 'transparent',
        border: `1px solid ${borderColor}`,
        borderRadius: radius,
        padding,
        ...(style||{}),
      }}
      {...rest}
    >
      {children}
    </View>
  )
}

export default PlainBlock
