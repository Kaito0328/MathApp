"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, SizeKey, RoundKey, ColorViewProperty, SizeViewProperty, SizeTextProperty, FontWeightKey, ColorTextProperty } from '../../design/tokens'

export function MatrixCard({ rows, cols, data, title = 'Matrix', previewRows = 6, previewCols = 6, showSizeBadge = false }: { rows: number; cols: number; data: number[]; title?: string; previewRows?: number; previewCols?: number; showSizeBadge?: boolean }) {
  const cell = (r: number, c: number) => data[r * cols + c]
  const pr = Math.min(rows, previewRows)
  const pc = Math.min(cols, previewCols)
  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1, position: 'relative' }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8, display: 'grid', gridTemplateColumns: `repeat(${pc}, minmax(40px, 1fr))`, gap: 4 }}>
        {Array.from({ length: pr }).map((_, r) => (
          <div key={r} style={{ display: 'contents' }}>
            {Array.from({ length: pc }).map((__, c) => (
              <BaseBox key={c} styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.SM, apply: { default: [] } }, roundKey: RoundKey.Sm }} style={{ padding: 6, textAlign: 'right', fontFamily: 'ui-monospace, monospace', borderWidth: 1 }}>
                {cell(r, c)?.toFixed?.(3) ?? cell(r, c)}
              </BaseBox>
            ))}
          </div>
        ))}
      </div>
      {showSizeBadge && (
        <div style={{ position: 'absolute', right: 8, bottom: 8 }}>
          <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } }, size: { sizeKey: SizeKey.SM, apply: { default: [SizeTextProperty.FontSize] } } }}>{rows}×{cols}</BaseText>
        </div>
      )}
    </BaseBox>
  )
}
