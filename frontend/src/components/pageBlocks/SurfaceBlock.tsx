"use client"
import React from 'react'

export type SurfaceBlockProps = React.HTMLAttributes<HTMLDivElement> & {
  borderColor?: string
  transparent?: boolean
  padding?: number
  radius?: number
}

// Lightweight wrapper for page sections: transparent background + subtle secondary border.
export const SurfaceBlock: React.FC<SurfaceBlockProps> = ({
  borderColor = 'var(--surface-block-border, var(--color-secondary-border, rgba(0,0,0,0.15)))',
  transparent = true,
  padding = 12,
  radius = 6,
  style,
  children,
  ...rest
}) => {
  return (
    <div
      style={{
        background: transparent ? 'transparent' : 'var(--surface-block-bg, #fff)',
        border: `1px solid ${borderColor}`,
        borderRadius: radius,
        padding,
        ...(style || {}),
      }}
      {...rest}
    >
      {children}
    </div>
  )
}

export default SurfaceBlock
