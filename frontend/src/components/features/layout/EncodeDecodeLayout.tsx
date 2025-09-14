"use client"
import React from 'react'

export type EncodeDecodeLayoutProps = {
  title?: string
  description?: React.ReactNode
  left?: React.ReactNode // 入力パラメータやメッセージ入力
  onEncode?: () => void
  canEncode?: boolean
  encodeButtonLabel?: string
  encodedView?: React.ReactNode // 符号語表示や誤り注入UI
  onDecode?: () => void
  canDecode?: boolean
  decodeButtonLabel?: string
  decodedView?: React.ReactNode // 復号結果や比較
  footer?: React.ReactNode
}

const sectionStyle: React.CSSProperties = { display: 'grid', gap: 10, padding: 12, border: '1px solid #ddd', borderRadius: 8 }

const EncodeDecodeLayout: React.FC<EncodeDecodeLayoutProps> = ({
  title,
  description,
  left,
  onEncode,
  canEncode = true,
  encodeButtonLabel = 'Encode',
  encodedView,
  onDecode,
  canDecode = true,
  decodeButtonLabel = 'Decode',
  decodedView,
  footer,
}) => {
  return (
    <div style={{ display:'grid', gap:16 }}>
      {(title || description) && (
        <div>
          {title && <h2 style={{ margin:'6px 0' }}>{title}</h2>}
          {description && <div style={{ opacity:0.85 }}>{description}</div>}
        </div>
      )}
      <div style={{ display:'grid', gap:12 }}>
        <div style={sectionStyle}>
          {left}
          {onEncode && (
            <div>
              <button onClick={onEncode} disabled={!canEncode}>{encodeButtonLabel}</button>
            </div>
          )}
        </div>
        <div style={sectionStyle}>
          {encodedView}
          {onDecode && (
            <div>
              <button onClick={onDecode} disabled={!canDecode}>{decodeButtonLabel}</button>
            </div>
          )}
        </div>
        <div style={sectionStyle}>
          {decodedView}
        </div>
      </div>
      {footer && <div>{footer}</div>}
    </div>
  )
}

export default EncodeDecodeLayout
