"use client"
import React from 'react'
import Link from 'next/link'
import PageContainer from '../../src/baseComponents/layout/PageContainer'

export default function CodingIndexPage() {
  return (
    <PageContainer title="符号理論" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <div>実装済みのページ:</div>
        <ul>
          <li><Link href="/coding/hamming74">Hamming(7,4)</Link></li>
          <li><Link href="/coding/rs">Reed–Solomon</Link></li>
        </ul>
        <div style={{ opacity:0.8, fontSize:13 }}>ユーティリティや他の符号（Linear, Cyclic, BCH）は順次追加します。</div>
      </div>
    </PageContainer>
  )
}
