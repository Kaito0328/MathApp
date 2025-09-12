"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import RecurrencePanel from '../../../src/components/features/concrete-math/RecurrencePanel'
import Document from '../../../src/components/features/document/Document'

export default function SequencePage() {
  return (
    <PageContainer title="Concrete Math / 数列（漸化式）" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <RecurrencePanel />
        <SectionPanelWithTitle title="ドキュメント">
          <Document docPath="notes/concrete-math/recurrence_closed_form.md" />
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
