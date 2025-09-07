import React from 'react'
import ReactMarkdown from 'react-markdown'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import 'katex/dist/katex.min.css'

type Props = { math: string; block?: boolean }

export function MarkdownMath({ math, block = true }: Props) {
  const content = block ? `$$\\displaystyle ${math}$$` : `$${math}$`
  return (
    <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
      {content}
    </ReactMarkdown>
  )
}

export default MarkdownMath
