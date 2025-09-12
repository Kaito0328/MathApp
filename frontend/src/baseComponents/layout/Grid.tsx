"use client"
import React from 'react'

export interface GridProps {
  children?: React.ReactNode
  gap?: number | string
  /**
   * 最小カラム幅。repeat(auto-fit, minmax(minColumnWidth, 1fr)) で自動折り返し
   * 例: 560, "560px", "min(560px, 50vw)"
   */
  minColumnWidth?: number | string
  /**
   * 固定列数を指定したい場合に使用（auto-fit と併用しない）
   */
  columns?: number
  /**
   * 固定カラム幅。columns と併用時に `repeat(columns, columnWidth)` で固定幅化します。
   * 例: 560, "560px"
   */
  columnWidth?: number | string
  style?: React.CSSProperties
}

/**
 * 汎用 Grid レイアウト。
 * - columns + columnWidth 指定: 固定列×固定カラム幅
 * - columns 指定のみ: 固定列×auto（1fr）
 * - minColumnWidth 指定: auto-fit で幅に応じてカラム数が自動決定
 */
export const Grid: React.FC<GridProps> = ({ children, gap = 12, minColumnWidth, columns, columnWidth, style }) => {
  const template = React.useMemo(() => {
    if (typeof columns === 'number' && columns > 0) {
      if (columnWidth !== undefined) {
        const cw = typeof columnWidth === 'number' ? `${columnWidth}px` : columnWidth
        return `repeat(${columns}, ${cw})`
      }
      return `repeat(${columns}, minmax(0, 1fr))`
    }
    const minw = typeof minColumnWidth === 'number' ? `${minColumnWidth}px` : (minColumnWidth || '560px')
    return `repeat(auto-fit, minmax(${minw}, 1fr))`
  }, [columns, minColumnWidth, columnWidth])

  return (
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: template,
        gap,
        alignItems: 'stretch',
        justifyContent: 'center',
        ...style,
      }}
    >
      {children}
    </div>
  )
}

export default Grid
