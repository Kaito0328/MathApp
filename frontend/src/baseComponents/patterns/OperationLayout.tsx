"use client"
import React from 'react'

export function BinaryOperationLayout({
  header,
  leftOperand,
  rightOperand,
  result,
}: {
  header: React.ReactNode
  leftOperand: React.ReactNode
  rightOperand: React.ReactNode
  result: React.ReactNode
}) {
  return (
    <div style={{ display: 'grid', gap: 12, borderWidth: 1, padding: 12 }}>
      {/* header controls: 左/演算/右 */}
      <div style={{ borderWidth: 1, padding: 12 }}>{header}</div>

      {/* operands grid */}
      <div style={{ display: 'grid', gap: 12, gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))', alignItems: 'start' }}>
        <div style={{ borderWidth: 1, padding: 12 }}>{leftOperand}</div>
        <div style={{ borderWidth: 1, padding: 12 }}>{rightOperand}</div>
      </div>

      {/* result */}
      <div style={{ borderWidth: 1, padding: 12 }}>{result}</div>
    </div>
  )
}

export function UnaryOperationLayout({
  header,
  input,
  result,
  validation,
}: {
  header: React.ReactNode
  input: React.ReactNode
  result: React.ReactNode
  validation?: React.ReactNode
}) {
  return (
    <div style={{ display: 'grid', gap: 12, borderWidth: 1, padding: 12 }}>
      <div style={{ borderWidth: 1, padding: 12 }}>{header}</div>
      <div style={{ borderWidth: 1, padding: 12 }}>{input}</div>
      <div style={{ borderWidth: 1, padding: 12 }}>{result}</div>
      {validation && <div style={{ borderWidth: 1, padding: 12 }}>{validation}</div>}
    </div>
  )
}

export default BinaryOperationLayout
