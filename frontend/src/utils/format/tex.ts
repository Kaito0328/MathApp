// Utility to compose TeX strings with optional left/right parts and mode hints.
export type ComposeTeXOptions = {
  left?: string | null
  body: string
  right?: string | null
  mode?: 'inline' | 'block' | 'aligned'
}

export function composeTeX({ left, body, right, mode }: ComposeTeXOptions): { tex: string; block: boolean } {
  const core = [left?.trim(), body.trim(), right?.trim()].filter(Boolean).join(' ')
  if (mode === 'aligned') {
    // Caller provides a body that may already contain aligned rows; we just pass block=true
    return { tex: core, block: true }
  }
  const block = mode === 'block'
  return { tex: core, block }
}
