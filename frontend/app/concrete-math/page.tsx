"use client"
import React from 'react'
import Link from 'next/link'
import PageContainer from '../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../src/components/composites/panels/SectionPanelWithTitle'
import Document from '../../src/components/features/document/Document'
import { useVariableStore } from '../../src/state/VariableStore'

export default function ConcreteMathHome() {
  const { names, vars, remove, clear } = useVariableStore()
  return (
    <PageContainer title="Concrete Math" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <SectionPanelWithTitle title="ドキュメント">
          <Document docPath="notes/concrete-math/overview.md" />
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="ページ">
          <div style={{ display:'grid', gap:8 }}>
            <Link href="/concrete-math/combinatorics">組合せ論</Link>
            <Link href="/concrete-math/sum">和（部分和）</Link>
            <Link href="/concrete-math/sum">和（部分和）</Link>
            <Link href="/concrete-math/sequence">数列（漸化式）</Link>
          </div>
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="変数の管理" showSave={false} showCopy={false}>
          {names.length === 0 ? (
            <div style={{ opacity: 0.8 }}>保存された変数はありません。</div>
          ) : (
            <div style={{ display:'grid', gap:6 }}>
              {names.map((n) => (
                <div key={n} style={{ display:'flex', gap:8, alignItems:'center' }}>
                  <code style={{ opacity: 0.8 }}>{n}</code>
                  <span style={{ opacity: 0.6, fontSize:12 }}>{JSON.stringify(vars[n])}</span>
                  <div style={{ marginLeft:'auto' }}>
                    <button onClick={()=> remove(n)}>削除</button>
                  </div>
                </div>
              ))}
              <div>
                <button onClick={clear}>すべて削除</button>
              </div>
            </div>
          )}
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
