"use client"
import React from 'react'

export type StackProps = React.HTMLAttributes<HTMLDivElement> & {
  gap?: number
  align?: React.CSSProperties['alignItems']
  justify?: React.CSSProperties['justifyContent']
  as?: keyof HTMLElementTagNameMap
}

// Vertical layout helper: stacks children in a column with a consistent gap.
export const Stack: React.FC<StackProps> = ({ gap = 8, align = 'stretch', justify = 'flex-start', as = 'div', style, children, ...rest }) => {
  const Tag: any = as
  return (
    <Tag
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap,
        alignItems: align,
        justifyContent: justify,
        ...(style || {}),
      }}
      {...rest}
    >
      {children}
    </Tag>
  )
}

export default Stack
