"use client"
import React from 'react'
import Link from 'next/link'
import PageContainer from '../../src/baseComponents/patterns/PageContainer'
import { View } from '../../src/baseComponents/foundation/View'
import { Text } from '../../src/baseComponents/foundation/Text'
import { CoreColorKey, SizeKey, FontWeightKey } from '../../src/design/tokens'
import PolyVariableSection from '../../src/components/variables/PolyVariableSection'

export default function PolyHomePage() {
  return (
    <PageContainer title="多項式 / 有理関数" stickyHeader maxWidth={1080}>
      <div style={{ display:'grid', gap:12 }}>
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12 }}>
          <Text weight={FontWeightKey.Medium}>ツール</Text>
          <div style={{ display:'grid', gap:8, marginTop: 8 }}>
            <Link className="nav-link" href="/poly/polynomial/binary">多項式の二項演算</Link>
            <Link className="nav-link" href="/poly/polynomial/unary">多項式の単項演算</Link>
            <Link className="nav-link" href="/poly/polynomial/generate">多項式の生成</Link>
            <Link className="nav-link" href="/poly/rational/binary">有理関数の二項演算</Link>
            <Link className="nav-link" href="/poly/rational/unary">有理関数の単項演算</Link>
          </div>
        </View>
        <PolyVariableSection kind="polynomial" />
        <PolyVariableSection kind="rational" />
      </div>
    </PageContainer>
  )
}
