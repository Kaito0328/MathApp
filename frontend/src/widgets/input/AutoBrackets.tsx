"use client"
import React from 'react'
import { MarkdownMath } from '../display/MarkdownMath'

type Props = {
  anchorRef: React.RefObject<HTMLElement | null>
  side: 'left' | 'right'
  paddingPx?: number
}

// Automatically sized KaTeX bracket that matches the height of the anchor element.
export const AutoBrackets: React.FC<Props> = ({ anchorRef, side, paddingPx = 0 }) => {
  const holderRef = React.useRef<HTMLSpanElement>(null)
  const [scaleY, setScaleY] = React.useState<number>(1)
  const [targetH, setTargetH] = React.useState<number>(0)
  const baseMath = React.useMemo(() => {
    const phantom = `\\vphantom{\\rule{0pt}{1em}}`
    return side === 'left' ? `\\left[ ${phantom} \\right.` : `\\left. ${phantom} \\right]`
  }, [side])

  const update = React.useCallback(() => {
    const el = anchorRef.current
    if (!el) return
    const rect = el.getBoundingClientRect()
    const h = Math.max(0, rect.height + paddingPx)
    setTargetH(h)
    // 次フレームでホルダーの実高さを計測してスケール決定
    requestAnimationFrame(() => {
      const baseH = holderRef.current?.getBoundingClientRect().height || 1
      const s = baseH > 0 ? h / baseH : 1
      setScaleY(s)
    })
  }, [anchorRef, paddingPx])

  React.useEffect(() => {
    let ro: ResizeObserver | null = null
    let observed: Element | null = null
    const attach = () => {
      const el = anchorRef.current
      if (!el) return
      if (observed === el && ro) return
      if (ro) {
  try { if (observed) ro.unobserve(observed) } catch { /* ignore */ }
        ro.disconnect()
      }
      ro = new ResizeObserver(update)
      ro.observe(el)
      observed = el
    }
    attach()
    update()
    // window リサイズ時も再計算
    window.addEventListener('resize', update)
    const id = setInterval(attach, 250) // current の入れ替わりに保険
    return () => {
      window.removeEventListener('resize', update)
      clearInterval(id)
      if (ro && observed) {
        try { ro.unobserve(observed) } catch { /* ignore */ }
        ro.disconnect()
      }
    }
  }, [anchorRef, update])

  return (
    <span
      className="katex-bracket"
      ref={holderRef}
      style={{
        display: 'inline-block',
        lineHeight: 1,
        height: targetH ? `${targetH}px` : undefined,
        transform: `scaleY(${scaleY})`,
        transformOrigin: 'center',
      }}
    >
      <MarkdownMath math={baseMath} block={false} />
    </span>
  )
}

export default AutoBrackets
