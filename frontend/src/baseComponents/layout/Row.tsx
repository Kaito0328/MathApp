"use client"
import React from 'react'

export type ThreeZoneProps = React.HTMLAttributes<HTMLDivElement> & {
  gap?: number // grid column gap
  zoneGap?: number // inner gap inside each zone
  left?: React.ReactNode
  center?: React.ReactNode
  right?: React.ReactNode
  leftWrap?: boolean
  centerWrap?: boolean
  rightWrap?: boolean
}

// Row: three-zone row layout (left | center | right)
export const Row: React.FC<ThreeZoneProps> = ({
  gap = 8,
  zoneGap,
  left,
  center,
  right,
  leftWrap = false,
  centerWrap = false,
  rightWrap = false,
  style,
  ...rest
}) => {
  const innerGap = zoneGap ?? gap
  const Zone: React.FC<{ align?: 'start' | 'center' | 'end'; wrap?: boolean; children?: React.ReactNode }> = ({ align = 'start', wrap, children }) => (
    <div style={{ display: 'flex', alignItems: 'center', justifyContent: align === 'start' ? 'flex-start' : align === 'center' ? 'center' : 'flex-end', gap: innerGap, flexWrap: wrap ? 'wrap' : 'nowrap' as any }}>
      {children}
    </div>
  )
  return (
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: '1fr auto 1fr',
        alignItems: 'center',
        gap,
        ...(style || {}),
      }}
      {...rest}
    >
      <Zone align="start" wrap={leftWrap}>{left}</Zone>
      <Zone align="center" wrap={centerWrap}>{center}</Zone>
      <Zone align="end" wrap={rightWrap}>{right}</Zone>
    </div>
  )
}

export default Row
