"use client"
import React from 'react'

export const ActionGroup: React.FC<React.HTMLAttributes<HTMLDivElement>> = ({ style, children, ...rest }) => (
  <div
    style={{
      display:'flex',
      gap:6,
      alignItems:'center',
      ...(style||{})
    }}
    {...rest}
  >{children}</div>
)

export default ActionGroup
