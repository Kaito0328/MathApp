"use client"
import React, { useMemo } from 'react'
import { Section, KV } from './common'

export function ImagePreview({ rgba, width, height, title = 'Image' }: { rgba: Uint8Array; width: number; height: number; title?: string }) {
  const url = useMemo(() => {
    const canvas = document.createElement('canvas')
    canvas.width = width; canvas.height = height
    const ctx = canvas.getContext('2d')!
    const img = new ImageData(new Uint8ClampedArray(rgba), width, height)
    ctx.putImageData(img, 0, 0)
    return canvas.toDataURL('image/png')
  }, [rgba, width, height])
  return (
    <Section title={title}>
      <KV label="shape">{width} × {height}</KV>
      {/* eslint-disable-next-line @next/next/no-img-element */}
  <img alt={title} src={url} width={Math.min(300, width)} style={{ imageRendering: 'pixelated', border: '1px solid var(--c-base-border)' }} />
    </Section>
  )
}
