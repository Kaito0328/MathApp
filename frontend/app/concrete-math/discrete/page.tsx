"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import DiscretePolyPanel from '../../../src/components/features/concrete-math/DiscretePolyPanel'
import Document from '../../../src/components/features/document/Document'

export default function DiscretePolyPage() {
  return (
    <PageContainer title="Concrete Math / 離散多項式" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <DiscretePolyPanel />
        <SectionPanelWithTitle title="ドキュメント">
          <Document docPath="notes/concrete-math/combinatorics_polynomials.md" />
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
